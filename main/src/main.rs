// clawtype-rust is (a part of) firmware for chorded keyboards
// Copyright (C) 2025  Mateusz Czapliński akavel.pl
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Input, Output, Pull, Level};
use embassy_rp::peripherals::{I2C0, USB};
use embassy_rp::{usb as rp_usb, i2c as rp_i2c, spi as rp_spi};
use embassy_sync::mutex::Mutex;
use embassy_sync::blocking_mutex::raw::*;
use embassy_time::{Delay, Timer};
use embassy_usb::class::hid;
use embassy_usb::class::cdc_acm;
use embedded_graphics::prelude::*;
use embedded_graphics::image::{Image, ImageRaw};
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_hal_bus::spi as hal_spi;
use usbd_hid::descriptor::{self as hid_desc, SerializedDescriptor as _};
use {defmt_rtt as _, panic_probe as _};
use mpu6050_async::Mpu6050;
use nokia5110lcd::Pcd8544;
use u8g2_fonts::{FontRenderer, fonts, types as font_params};

use clawtype_chords::{
    self as chords,
    keycodes as new_keys,
    keycodes::{
        HACK_MOUSE_MARKER,
        HACK_MOUSE_ENABLE_TOGGLE,
        HACK_MOUSE_LEFT_DRAG_TOGGLE,
        HACK_MOUSE_LEFT_CLICK,
        HACK_MOUSE_RIGHT_CLICK,
        HACK_MOUSE_WHEEL_DOWN,
        HACK_MOUSE_WHEEL_UP,
        HACK_MOUSE_LEFT_BTN,
        HACK_MOUSE_RIGHT_BTN,
    },
    SwitchSet,
    UsbOutcome::*
};

pub mod usb_kbd;
pub mod usb_simpler;
mod layout;
mod futures;

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => rp_usb::InterruptHandler<USB>;
    I2C0_IRQ => rp_i2c::InterruptHandler<I2C0>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    ////
    //// USB initial setup
    ////

    let driver = rp_usb::Driver::new(p.USB, Irqs);
    use usb_simpler::buffers as usb_buffers;
    let mut usb_buf_dev = usb_buffers::ForDevice::new();
    let mut usb_buf_hid_kbd = usb_buffers::ForHid::new();
    let mut usb_buf_hid_mouse = usb_buffers::ForHid::new();
    let mut logger_state = cdc_acm::State::new();
    let mut usb_dev_builder =
        usb_simpler::new("akavel", "clawtype").into_device_builder(driver, &mut usb_buf_dev);
    let kbd_hid = usb_dev_builder.add_hid_reader_writer::<1, 8>(
        &mut usb_buf_hid_kbd,
        hid::Config {
            report_descriptor: hid_desc::KeyboardReport::desc(),
            request_handler: None,
            poll_ms: 60,
            max_packet_size: 64,
        },
    );
    let mouse_hid = usb_dev_builder.add_hid_reader_writer::<1, 8>(
        &mut usb_buf_hid_mouse,
        hid::Config {
            report_descriptor: hid_desc::MouseReport::desc(),
            request_handler: None,
            poll_ms: 60,
            max_packet_size: 64,
        },
    );
    let logger_class = usb_dev_builder.add_cdc_acm_class(&mut logger_state, 64);
    let mut usb = usb_dev_builder.build();
    let usb_fut = usb.run();
    let (kbd_reader, mut kbd_writer) = kbd_hid.split();
    let (mouse_reader, mouse_writer) = mouse_hid.split();
    let log_fut = embassy_usb_logger::with_class!(1024, log::LevelFilter::Info, logger_class);

    ////
    //// INPUT switches initial setup
    ////

    let mut p0 = Input::new(p.PIN_2, Pull::Up);
    let mut p1 = Input::new(p.PIN_3, Pull::Up);
    let mut p2 = Input::new(p.PIN_4, Pull::Up);
    let mut p3 = Input::new(p.PIN_5, Pull::Up);
    let mut p4 = Input::new(p.PIN_6, Pull::Up);
    let mut p5 = Input::new(p.PIN_7, Pull::Up);
    let mut p6 = Input::new(p.PIN_8, Pull::Up);
    let mut p7 = Input::new(p.PIN_9, Pull::Up);

    // Enable the schmitt trigger to slightly debounce.
    for p in [&mut p0, &mut p1, &mut p2, &mut p3, &mut p4, &mut p5, &mut p6, &mut p7] {
        p.set_schmitt(true);
    }

    let mut cho = chords::Engine::<layout::Layout>::default();

    ////
    //// GYRO MOUSE initial setup
    ////

    let mut i2c_cfg = rp_i2c::Config::default();
    i2c_cfg.frequency = 400_000;
    let i2c = rp_i2c::I2c::new_async(p.I2C0, p.PIN_29, p.PIN_28, Irqs, i2c_cfg);
    let mut mpu = Mpu6050::new(i2c);
    let _ = mpu.init(&mut Delay).await;

    // WARN: to avoid deadlocks, ALWAYS lock multiple ONLY in order like below
    let mouse_enabled = Mutex::<ThreadModeRawMutex, _>::new(false);
    let mouse_buttons = Mutex::<ThreadModeRawMutex, _>::new(0u8); // TODO: atomic
    let mouse_writer = Mutex::<ThreadModeRawMutex, _>::new(mouse_writer);

    ////
    //// NOKIA LCD initial setup
    ////

    let mut cfg = rp_spi::Config::default();
    cfg.frequency = 2_000_000;
    let spi_bus = rp_spi::Spi::new_blocking_txonly(p.SPI0, p.PIN_22, p.PIN_23, cfg);
    let lcd_ce = Output::new(p.PIN_21, Level::High);
    // TODO: don't use new_no_delay but regular new
    let spi_dev = hal_spi::ExclusiveDevice::new_no_delay(spi_bus, lcd_ce);

    let lcd_dc = Output::new(p.PIN_20, Level::Low);
    let lcd_rst = Output::new(p.PIN_26, Level::Low);
    let lcd_light = Output::new(p.PIN_27, Level::Low);

    let mut delayer = Delay{};
    let mut lcd = Pcd8544::new(spi_dev, lcd_dc, lcd_rst, &mut delayer).expect("better not fail");

    let mut lcd_buf = nokia5110lcd::Buffer::new();

    let raw_img = ImageRaw::<BinaryColor>::new(VAULT_BOY, nokia5110lcd::WIDTH.into());
    let img = Image::new(&raw_img, Point::new(11, 0));

    // let font = FontRenderer::new::<fonts::u8g2_font_u8glib_4_tr>();
    // let font = FontRenderer::new::<fonts::u8g2_font_tinyunicode_tf>();
    // let font = FontRenderer::new::<fonts::u8g2_font_boutique_bitmap_7x7_t_all>();
    // let font = FontRenderer::new::<fonts::u8g2_font_pxplustandynewtv_t_all>();
    let font_mini = FontRenderer::new::<fonts::u8g2_font_3x5im_te>();
    let font_unicode = FontRenderer::new::<fonts::u8g2_font_tiny5_t_all>();
    let top = font_params::VerticalPosition::Top;
    let fcol = font_params::FontColor::Transparent(BinaryColor::On);
    let pt = |x, y| Point::new(x, y);
    let lcd_fut = async {
        // First, draw a welcome screen
        let _ = img.draw(&mut lcd_buf);
        let rows = ["Hello,", "hackerman!", "", "Cześć,", "Ciao,", "Привіт!"];
        let dy = 8;
        for (i, s) in rows.iter().enumerate() {
            let _ = font_unicode.render(
                *s,
                pt(0, i as i32 * 8 + 1),
                top, fcol,
                &mut lcd_buf,
            );
        }
        let _ = lcd.position(0, 0);
        let _ = lcd.data(&lcd_buf.bytes);

        // Then, wait a short while...
        // Timer::after_millis(1200).await;
        Timer::after_millis(2000).await;

        // And switch to the cheatsheet.
        let _ = lcd_buf.clear(BinaryColor::Off);
        let _ = img.draw(&mut lcd_buf);
        let rows = [
            "v_vv ( ^_^vGUI",
            "^^_v < %_%v %",
            "%_^^ Cap",
            "_^^% +",
            "_^^v =",
            "%%v_ ` ^^_% ?",
        ];
        let dy = 8;
        for (i, s) in rows.iter().enumerate() {
            let _ = font_mini.render(
                *s,
                pt(0, i as i32 * 8 + 1),
                top, fcol,
                &mut lcd_buf,
            );
        }
        let _ = lcd.position(0, 0);
        let _ = lcd.data(&lcd_buf.bytes);
    };

    ////
    //// OTHER
    ////

    log::info!("Starting clawtype...");
    let gyro_fut = async {
        loop {
            // log::info!("loopsy...");
            // Timer::after_millis(20).await;
            Timer::after_millis(5).await;

            let Ok(gyro) = mpu.get_gyro().await else {
                continue;
            };
            let (gx, gy, gz) = gyro;
            // log::info!("gyro: {gx} {gy} {gz}");
            // let vx = (gx*30.0) as i8;
            // let vy = (-gz*20.0) as i8;
            let vx = (gx*30.0) as i8;
            let vy = (-gz*20.0) as i8;
            // let vx = (gx/250.0) as i8;
            // let vy = (-gz/200.0) as i8;
            // log::info!("mouse: {vx}\t{vy}");

            let m = { *mouse_enabled.lock().await };
            if m {
                // log::info!("m enabled");
                let b = mouse_buttons.lock().await;
                let mut mw = mouse_writer.lock().await;
                let _ = usb_send_mouse_report(&mut *mw, *b, vx, vy, 0).await;
            }
        }
    };

    let in_fut = async {
        loop {
            _ = Timer::after_millis(2).await;
            let switches =
                bit(0b01_00_00_00, p0.is_low()) | // pinky base
                bit(0b10_00_00_00, p1.is_low()) | // pinky tip
                bit(0b00_01_00_00, p2.is_low()) | // ring base
                bit(0b00_10_00_00, p3.is_low()) | // ring tip
                bit(0b00_00_01_00, p4.is_low()) | // middle base
                bit(0b00_00_10_00, p5.is_low()) | // middle tip
                bit(0b00_00_00_01, p6.is_low()) | // index base
                bit(0b00_00_00_10, p7.is_low());  // index tip

            let outcome = cho.handle(SwitchSet(switches));
            if outcome != Nothing {
                log::info!("got: {outcome:?}");
            }
            match outcome {
                Nothing => (),
                KeyPress(k) => {
                    let mask = mouse_mask_from_key_with_flags(k);
                    let mut b = mouse_buttons.lock().await;
                    *b |= mask;
                    let mut mw = mouse_writer.lock().await;
                    usb_send_mouse_report(&mut *mw, *b, 0, 0, 0).await;
                },
                KeyRelease(k) => {
                    let mask = mouse_mask_from_key_with_flags(k);
                    let mut b = mouse_buttons.lock().await;
                    *b &= !mask;
                    let mut mw = mouse_writer.lock().await;
                    usb_send_mouse_report(&mut *mw, *b, 0, 0, 0).await;
                },
                KeyHit(key_with_flags) => {
                    if key_with_flags & HACK_MOUSE_MARKER == HACK_MOUSE_MARKER {
                        match key_with_flags {
                            HACK_MOUSE_ENABLE_TOGGLE => {
                                let mut m = mouse_enabled.lock().await;
                                *m = !*m;
                            }
                            HACK_MOUSE_WHEEL_DOWN => {
                                let b = mouse_buttons.lock().await;
                                let mut mw = mouse_writer.lock().await;
                                usb_send_mouse_report(&mut *mw, *b, 0, 0, -10).await;
                            },
                            HACK_MOUSE_WHEEL_UP => {
                                let b = mouse_buttons.lock().await;
                                let mut mw = mouse_writer.lock().await;
                                usb_send_mouse_report(&mut *mw, *b, 0, 0, 10).await;
                            },
                            _ => (),
                        }
                    } else {
                        usb_send_key_with_flags(&mut kbd_writer, key_with_flags).await;
                    }
                }
            }
        }
    };

    let mut request_handler = MyRequestHandler {};
    let kbd_out_fut = async {
        kbd_reader.run(false, &mut request_handler).await;
    };

    // Run everything concurrently.
    // If we had made everything `'static` above instead, we could do this using separate tasks instead.
    join!(
        lcd_fut,
        usb_fut,
        log_fut,
        gyro_fut,
        in_fut,
        kbd_out_fut,
    ).await;
}

struct MyRequestHandler {}

impl hid::RequestHandler for MyRequestHandler { }

fn bit(mask: u8, apply: bool) -> u8 {
    if apply { mask } else { 0 }
}

async fn usb_send_key_with_flags<'d, D, const N: usize>(writer: &mut hid::HidWriter<'d, D, N>, k: new_keys::KeyWithFlags)
where
      D: embassy_usb::driver::Driver<'d>,
{
    let bytes = k.to_be_bytes();
    let modifier = bytes[0];
    let key = bytes[1];

    // press...
    use usbd_hid::descriptor::KeyboardReport;
    let mut report = KeyboardReport::default();
    report.modifier = modifier;
    report.keycodes[0] = key;
    let _ = writer.write_serialize(&report).await;

    // ...and release
    let empty_report = KeyboardReport::default();
    let _ = writer.write_serialize(&empty_report).await;
}

async fn usb_send_mouse_report<'d, D, const N: usize>(writer: &mut hid::HidWriter<'d, D, N>, buttons: u8, x: i8, y: i8, wheel: i8)
where
      D: embassy_usb::driver::Driver<'d>,
{
    use usbd_hid::descriptor::MouseReport;
    let report = MouseReport {
        buttons,
        x,
        y,
        wheel,
        pan: 0,
    };
    let _ = writer.write_serialize(&report).await;
}

fn mouse_mask_from_key_with_flags(k: new_keys::KeyWithFlags) -> u8 {
    match k {
        HACK_MOUSE_LEFT_BTN => MASK_MOUSE_BTN_LEFT,
        HACK_MOUSE_RIGHT_BTN => MASK_MOUSE_BTN_RIGHT,
        _ => 0u8,
    }
}

const MASK_MOUSE_BTN_LEFT: u8 = 0x1;
const MASK_MOUSE_BTN_RIGHT: u8 = 0x2;

// based on Vault Boy cross-stitch pattern by IFeel_Attacked (https://redd.it/rnt3ou)
// copied and cropped manually, converted with https://javl.github.io/image2cpp/
const VAULT_BOY: &[u8] = &[
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x24, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x43, 0xc0, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc4, 0xc0, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x01, 0x88, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x70, 0x00, 0x18,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x02, 0x38, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x02, 0x05, 0xc4, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x08, 0x03,
    0xf2, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0xb0, 0x00, 0x09, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x02, 0x40, 0x10, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0xb0,
    0x1c, 0x11, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x80, 0x04, 0x21, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x01, 0x30, 0x00, 0x15, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
    0x31, 0x18, 0x2a, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x32, 0x3c, 0x16, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x30, 0x00, 0x02, 0x04, 0x04, 0x2a, 0x00, 0x00, 0x00, 0x00, 0x00, 0x48, 0x00,
    0x02, 0x0c, 0x00, 0x14, 0x00, 0x00, 0x00, 0x00, 0x00, 0x84, 0x00, 0x02, 0x0c, 0x00, 0x24, 0x00,
    0x00, 0x00, 0x00, 0x01, 0x04, 0x00, 0x02, 0x04, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x00, 0x01, 0x08,
    0x00, 0x02, 0x02, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x01, 0x08, 0x00, 0x02, 0x00, 0x00, 0x02,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x88, 0x00, 0x02, 0x7f, 0xf0, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x88, 0x00, 0x01, 0x20, 0x20, 0x0c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x44, 0x00, 0x01, 0x19, 0xc0,
    0x30, 0x00, 0x00, 0x00, 0x00, 0x07, 0xe2, 0x00, 0x01, 0x06, 0x00, 0x40, 0x00, 0x00, 0x00, 0x00,
    0x08, 0x19, 0x00, 0x00, 0x80, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x10, 0x04, 0x80, 0x00, 0xc0,
    0x03, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x10, 0x04, 0xf0, 0x00, 0x60, 0x05, 0x38, 0x00, 0x00, 0x00,
    0x00, 0x09, 0xc4, 0x5e, 0x07, 0xd0, 0x01, 0x16, 0x00, 0x00, 0x00, 0x00, 0x06, 0x38, 0xa3, 0xfc,
    0x2c, 0x02, 0x11, 0x00, 0x00, 0x00, 0x00, 0x08, 0x0c, 0xa0, 0x00, 0x23, 0xc4, 0x20, 0x80, 0x00,
    0x00, 0x00, 0x08, 0x02, 0xa0, 0x00, 0x21, 0x08, 0x20, 0x60, 0x00, 0x00, 0x00, 0x07, 0xc2, 0xa0,
    0x00, 0x30, 0xf0, 0x40, 0x10, 0x00, 0x00, 0x00, 0x04, 0x3c, 0xa0, 0x00, 0x18, 0x00, 0x80, 0x08,
    0x00, 0x00, 0x00, 0x04, 0x04, 0xa0, 0x00, 0x08, 0x03, 0x00, 0x04, 0x00, 0x00, 0x00, 0x04, 0x05,
    0x60, 0x04, 0x08, 0x3c, 0x00, 0x02, 0x00, 0x00, 0x00, 0x02, 0xfa, 0xc0, 0x0c, 0x08, 0x20, 0x00,
    0x01, 0x00, 0x00, 0x00, 0x03, 0x05, 0x80, 0xf4, 0x18, 0x20, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00,
    0xf9, 0x7f, 0x04, 0x10, 0x20, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x0f, 0xc0, 0x0c, 0x10, 0x40,
    0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x10, 0x40, 0x30, 0x00, 0x20, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x08, 0x10, 0x40, 0x0c, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x10,
    0x40, 0x03, 0x80, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x10, 0x40, 0x02, 0x70, 0x10, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x08, 0x10, 0x40, 0x02, 0x20, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08,
    0x10, 0x40, 0x02, 0x40, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0c, 0x10, 0x40, 0x06, 0x40, 0x20,
];
