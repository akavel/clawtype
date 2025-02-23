#![no_std]
#![no_main]

use embedded_hal::delay::DelayNs;
use panic_halt as _;

extern "C" {
    fn usb_init();
    fn usb_debug_putchar(c: u8) -> i8;
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

    // let mut usb = UsbSerial::new(dp.USB_DEVICE);
    // usb.init(&dp.PLL);

    // ufmt::uwriteln!(usb, "Hello ATmega!\r").unwrap();
    // ufmt::uwriteln!(&mut serial, "Hello from ATmega!\r").unwrap();

    let mut led = pins.pd6.into_output();

    unsafe { usb_init(); }
    for c in "Hello chordy!\r\n".bytes() {
        unsafe { usb_debug_putchar(c); }
    }

    loop {
        led.toggle();
        Delay::new().delay_ms(1000u32);
    }
}
