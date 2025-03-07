// chordite-rust is (a part of) firmware for chorded keyboards
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

use debouncr::debounce_8 as debouncer;
use embedded_hal::delay::DelayNs;
use mpu6050_dmp::sensor::Mpu6050;
use panic_halt as _;
use chordite_chords::{
    keycodes as new_keys,
    Chordite, SwitchSet,
    UsbOutcome::*
};

mod layout;

extern "C" {
    fn usb_try_init();
    fn usb_debug_putchar(c: u8);
    fn usb_simple_send_key(k: u16);
    fn usb_send_key_with_mod(key: u8, modifier: u8);
}

// Define core clock. This can be used in the rest of the project.
type CoreClock = atmega_hal::clock::MHz8;
type Delay = atmega_hal::delay::Delay<crate::CoreClock>;

#[avr_device::entry]
fn main() -> ! {
    let dp = atmega_hal::Peripherals::take().unwrap();
    let pins = atmega_hal::pins!(dp);

    // Set clock speed at 8MHz
    // FIXME: disable interrupts & enable back afterwards
    let clkpr = &dp.CPU.clkpr;
    use atmega_hal::pac::cpu::clkpr::CLKPS_A;
    clkpr.write(|w| w.clkpce().set_bit().clkps().variant(CLKPS_A::VAL_0X00));
    clkpr.write(|w| w.clkps().variant(CLKPS_A::VAL_0X01));

    let mut i2c = atmega_hal::I2c::new(
        dp.TWI,
        pins.pd1.into_pull_up_input(),
        pins.pd0.into_pull_up_input(),
        400_000, // TODO: double-check if ok
    );
    let gy521 = Mpu6050::new(i2c, mpu6050_dmp::address::Address::default());

    let mut led = pins.pd6.into_output();

    let p0 = pins.pb0.into_pull_up_input();
    let p1 = pins.pb1.into_pull_up_input();
    let p2 = pins.pb2.into_pull_up_input();
    let p3 = pins.pb3.into_pull_up_input();
    let p4 = pins.pb7.into_pull_up_input();
    let p5 = pins.pc6.into_pull_up_input();
    let p6 = pins.pc7.into_pull_up_input();
    let p7 = pins.pd2.into_pull_up_input();

    unsafe { usb_try_init(); }

    let mut cho = Chordite::<layout::Layout>::default();

    let mut i0 = debouncer(false);
    let mut i1 = debouncer(false);
    let mut i2 = debouncer(false);
    let mut i3 = debouncer(false);
    let mut i4 = debouncer(false);
    let mut i5 = debouncer(false);
    let mut i6 = debouncer(false);
    let mut i7 = debouncer(false);

    led.toggle();

    let mut prnt = PrinterWrapper{};

    loop {
        // led.toggle();

        match gy521 {
            Ok(sensor) => 'sensor: {
                let Ok(gyro) = sensor.gyro() else {
                    println("gyro error :(");
                    break 'sensor;
                };
                ufmt::uwrite!(prnt, "gx:{}, gy:{}, gz:{}", gyro.x(), gyro.y(), gyro.z());
            }
            Err(err) => {
                println("mpu6050 error :(");
            }
        }

        let switches =
            debit(0b01_00_00_00, &mut i0, p0.is_low()) | // pinky base
            debit(0b10_00_00_00, &mut i1, p1.is_low()) | // pinky tip
            debit(0b00_01_00_00, &mut i2, p2.is_low()) | // ring base
            debit(0b00_10_00_00, &mut i3, p3.is_low()) | // ring tip
            debit(0b00_00_01_00, &mut i4, p4.is_low()) | // middle base
            debit(0b00_00_10_00, &mut i5, p5.is_low()) | // middle tip
            debit(0b00_00_00_01, &mut i6, p6.is_low()) | // index base
            debit(0b00_00_00_10, &mut i7, p7.is_low());  // index tip

        let outcome = cho.handle(SwitchSet(switches));
        match outcome {
            Nothing => (),
            KeyHit(key_with_flags) => {
                println("Sending!");
                led.toggle();
                usb_send_new_key(key_with_flags);
            }
        }
        Delay::new().delay_ms(2u32);


        /*
        let switch0_low = switch0.is_low();
        if switch0_low != switch0_last {
            switch0_last = switch0_low;
            if switch0_low {
                println("KEY PRESS 0");
                const KEY_A: u16 = 4 | 0xF000;
                unsafe { usb_simple_send_key(KEY_A); }
            } else {
                println("Key release 0");
                const KEY_B: u16 = 5 | 0xF000;
                unsafe { usb_simple_send_key(KEY_B); }
            }
        }

        if switch0.is_low() {
            Delay::new().delay_ms(100u32);
            continue;
        }

        Delay::new().delay_ms(1000u32);
        println("Hello cpp :)");
        */
    }
}

fn println(s: &str) {
    print(s);
    print("\r\n");
}

fn print(s: &str) {
    for c in s.bytes() {
        unsafe { usb_debug_putchar(c); }
    }
}

fn usb_send_new_key(k: new_keys::KeyWithFlags) {
    let bytes = k.to_be_bytes();
    let modifier = bytes[0];
    let key = bytes[1];
    unsafe { usb_send_key_with_mod(key, modifier) };
}

// debit means a debounced bit
fn debit(mask: u8, debouncer: &mut debouncr::Debouncer<u8, debouncr::Repeat8>, switch_pressed: bool) -> u8 {
    debouncer.update(switch_pressed);
    bit(mask, debouncer.is_high())
}

fn bit(mask: u8, apply: bool) -> u8 {
    if apply { mask } else { 0 }
}

struct PrinterWrapper {};
impl ufmt_write::uWrite for PrinterWrapper {
    type Error = Infallible;

    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        print(s);
        Ok(())
    }
}
