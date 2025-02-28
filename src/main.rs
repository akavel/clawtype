#![no_std]
#![no_main]

use debouncr::debounce_8 as debouncer;
use embedded_hal::delay::DelayNs;
use panic_halt as _;
use chordite_chords::{
    keycodes as new_keys, sample_layers,
    Chordite, SwitchSet,
    UsbOutcome::*
};

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

    let p0 = pins.pb0.into_pull_up_input();
    let p1 = pins.pb1.into_pull_up_input();
    let p2 = pins.pb2.into_pull_up_input();
    let p3 = pins.pb3.into_pull_up_input();
    let p4 = pins.pb7.into_pull_up_input();
    let p5 = pins.pd0.into_pull_up_input();
    let p6 = pins.pd1.into_pull_up_input();
    let p7 = pins.pd2.into_pull_up_input();

    unsafe { usb_try_init(); }

    let mut cho = Chordite::<sample_layers::SampleLayers>::default();

    let mut i0 = debouncer(false);
    let mut i1 = debouncer(false);
    let mut i2 = debouncer(false);
    let mut i3 = debouncer(false);
    let mut i4 = debouncer(false);
    let mut i5 = debouncer(false);
    let mut i6 = debouncer(false);
    let mut i7 = debouncer(false);

    led.toggle();

    loop {
        // led.toggle();

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
