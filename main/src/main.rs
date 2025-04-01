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

use defmt::{info, warn};
use embassy_executor::Spawner;
use embassy_futures::join::join;
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Input, Pull};
use embassy_rp::peripherals::USB;
use embassy_rp::usb as rp_usb;
use embassy_usb::class::hid;
use embassy_usb::control::OutResponse;
use usbd_hid::descriptor::{self as hid_desc, SerializedDescriptor as _};
use {defmt_rtt as _, panic_probe as _};

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

    ////
    //// OTHER
    ////

    let mut kbd_state = usb_kbd::StateReport::default();

    // Do stuff with the class!
    let in_fut = async {
        loop {
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

impl hid::RequestHandler for MyRequestHandler {
    fn get_report(&mut self, id: hid::ReportId, _buf: &mut [u8]) -> Option<usize> {
        info!("Get report for {:?}", id);
        None
    }

    fn set_report(&mut self, id: hid::ReportId, data: &[u8]) -> OutResponse {
        info!("Set report for {:?}: {=[u8]}", id, data);
        OutResponse::Accepted
    }

    fn set_idle_ms(&mut self, id: Option<hid::ReportId>, dur: u32) {
        info!("Set idle rate for {:?} to {:?}", id, dur);
    }

    fn get_idle_ms(&mut self, id: Option<hid::ReportId>) -> Option<u32> {
        info!("Get idle rate for {:?}", id);
        None
    }
}
