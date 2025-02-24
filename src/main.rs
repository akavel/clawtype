#![no_std]
#![no_main]

use embedded_hal::delay::DelayNs;
use panic_halt as _;

/*
extern "C" {
    fn usb_init();
    fn usb_debug_putchar(c: u8) -> i8;
}
*/

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

    // unsafe { usb_init(); }

    loop {
        led.toggle();

        let switch0_low = switch0.is_low();
        if switch0_low != switch0_last {
            switch0_last = switch0_low;
            // println(if switch0_low {
            //     "KEY PRESS"
            // } else {
            //     "Key release"
            // });
        }

        if switch0.is_low() {
            Delay::new().delay_ms(100u32);
            continue;
        }

        Delay::new().delay_ms(1000u32);
        // println("Hello keys :)");
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

