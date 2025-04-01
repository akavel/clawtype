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

use defmt::warn;
use embassy_executor::Spawner;
use embassy_futures::join::join;
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Input, Pull};
use embassy_rp::peripherals::USB;
use embassy_rp::usb as rp_usb;
use embassy_usb::class::hid;
use usbd_hid::descriptor::{self as hid_desc, SerializedDescriptor as _};
use {defmt_rtt as _, panic_probe as _};

use clawtype_chords::{
    self as chords,
    keycodes as new_keys,
    SwitchSet,
    UsbOutcome::*
};

pub mod usb_kbd;
pub mod usb_simpler;
mod layout;

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => rp_usb::InterruptHandler<USB>;
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
    let mut usb_buf_hid = usb_buffers::ForHid::new();
    let mut usb_dev_builder =
        usb_simpler::new("akavel", "clawtype").into_device_builder(driver, &mut usb_buf_dev);
    let hid = usb_dev_builder.add_hid_reader_writer::<1, 8>(
        &mut usb_buf_hid,
        hid::Config {
            report_descriptor: hid_desc::KeyboardReport::desc(),
            request_handler: None,
            poll_ms: 60,
            max_packet_size: 64,
        },
    );
    let mut usb = usb_dev_builder.build();
    let usb_fut = usb.run();
    let (reader, mut writer) = hid.split();

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
    //// OTHER
    ////

    let mut kbd_state = usb_kbd::StateReport::default();

    // Do stuff with the class!
    let in_fut = async {
        loop {
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
                    usb_send_key_with_flags(&mut writer, key_with_flags).await;
                }
            }

            /*
            p0.wait_for_low().await;
            // Create a report with the D key pressed. (no shift modifier)
            let _ = kbd_state.press(usbd_hut::Keyboard::D);
            // Send the report.
            match writer.write_serialize(&kbd_state).await {
                Ok(()) => {}
                Err(e) => warn!("Failed to send report: {:?}", e),
            };
            p0.wait_for_high().await;
            kbd_state.clear();
            match writer.write_serialize(&kbd_state).await {
                Ok(()) => {}
                Err(e) => warn!("Failed to send report: {:?}", e),
            };
            */
        }
    };

    let mut request_handler = MyRequestHandler {};
    let out_fut = async {
        reader.run(false, &mut request_handler).await;
    };

    // Run everything concurrently.
    // If we had made everything `'static` above instead, we could do this using separate tasks instead.
    join(usb_fut, join(in_fut, out_fut)).await;
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
