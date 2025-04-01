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
use embassy_rp::gpio::{Input, Pull};
use embassy_rp::peripherals::{I2C0, USB};
use embassy_rp::{usb as rp_usb, i2c as rp_i2c};
use embassy_sync::mutex::Mutex;
use embassy_sync::blocking_mutex::raw::*;
use embassy_time::{Delay, Timer};
use embassy_usb::class::hid;
use embassy_usb::class::cdc_acm::{CdcAcmClass, self};
use usbd_hid::descriptor::{self as hid_desc, SerializedDescriptor as _};
use {defmt_rtt as _, panic_probe as _};
use mpu6050_async::Mpu6050;

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
    let logger_class = CdcAcmClass::new(&mut usb_dev_builder.wrapped, &mut logger_state, 64);
    let log_fut = embassy_usb_logger::with_class!(1024, log::LevelFilter::Info, logger_class);
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
    let mut usb = usb_dev_builder.build();
    let usb_fut = usb.run();
    let (kbd_reader, mut kbd_writer) = kbd_hid.split();
    let (mouse_reader, mut mouse_writer) = mouse_hid.split();

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

    let mouse_enabled = Mutex::<ThreadModeRawMutex, _>::new(false);

    ////
    //// OTHER
    ////

    log::info!("Starting clawtype...");
    let gyro_fut = async {
        loop {
            log::info!("loopsy...");
            Timer::after_millis(20).await;

            let Ok(gyro) = mpu.get_gyro().await else {
                continue;
            };
            let (gx, gy, gz) = gyro;
            log::info!("gyro: {gx} {gy} {gz}");
            let vx = (gx*30.0) as i8;
            let vy = (-gz*20.0) as i8;
            // let vx = (gx/250.0) as i8;
            // let vy = (-gz/200.0) as i8;
            log::info!("mouse: {vx}\t{vy}");

            let m = { *mouse_enabled.lock().await };
            if m {
                log::info!("m enabled");
                let _ = usb_send_mouse_move(&mut mouse_writer, vx, vy).await;
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
            match outcome {
                Nothing => (),
                KeyPress(_) => (),
                KeyRelease(_) => (),
                KeyHit(key_with_flags) => {
                    if key_with_flags & HACK_MOUSE_MARKER == HACK_MOUSE_MARKER {
                        match key_with_flags {
                            HACK_MOUSE_ENABLE_TOGGLE => {
                                let mut m = mouse_enabled.lock().await;
                                *m = !*m;
                            }
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

async fn usb_send_mouse_move<'d, D, const N: usize>(writer: &mut hid::HidWriter<'d, D, N>, x: i8, y: i8)
where
      D: embassy_usb::driver::Driver<'d>,
{
    use usbd_hid::descriptor::MouseReport;
    let report = MouseReport {
        x,
        y,
        buttons: 0,
        wheel: 0,
        pan: 0,
    };
    let _ = writer.write_serialize(&report).await;
}
