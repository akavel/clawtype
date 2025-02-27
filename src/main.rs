#![no_std]
#![no_main]

use embedded_hal::delay::DelayNs;
use panic_halt as _;
use chordite_chords::keycodes as new_keys;

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

    let mut led = pins.pd6.into_output();

    let switch0 = pins.pb0.into_pull_up_input();
    let mut switch0_last = false;

    let switch1 = pins.pb1.into_pull_up_input();
    let mut switch1_last = false;

    unsafe { usb_try_init(); }

    loop {
        led.toggle();

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

        let switch1_low = switch1.is_low();
        if switch1_low != switch1_last {
            switch1_last = switch1_low;
            if switch1_low {
                println("KEY PRESS 1");
                usb_send_new_key(new_keys::C | new_keys::SHIFT_FLAG);
            } else {
                println("Key release 1");
                usb_send_new_key(new_keys::C);
            }
        }

        if switch0.is_low() {
            Delay::new().delay_ms(100u32);
            continue;
        }

        Delay::new().delay_ms(1000u32);
        println("Hello cpp :)");
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
    let key = bytes[0];
    let modifier = bytes[1];
    unsafe { usb_send_key_with_mod(key, modifier) };
}

