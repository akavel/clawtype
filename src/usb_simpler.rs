use embassy_usb as eusb;

pub mod step1 {
    use super::*;

    pub fn new<'a>(manufacturer: &'a str, product: &'a str) -> Step1<'a> {
        let mut config = eusb::Config::new(0xc0de, 0xcafe);
        config.manufacturer = Some(manufacturer);
        config.product = Some(product);
        Step1 {
            config,
        }
    }

    pub struct Step1<'a> {
        pub config: eusb::Config<'a>,
    }
}

pub mod step2 {
    use super::*;

    pub struct Buffers {
        pub config_descriptor: [u8; 256],
        pub bos_descriptor: [u8; 256],
        pub msos_descriptor: [u8; 256],
        pub control_buf: [u8; 64],
    }

    impl Buffers {
        pub fn new() -> Buffers {
            Buffers {
                // TODO: why below sizes in example? try to understand and adjust
                config_descriptor: [0; 256],
                bos_descriptor: [0; 256],
                msos_descriptor: [0; 256],
                control_buf: [0; 64],
            }
        }
    }

    pub fn from<'a, D>(step1: step1::Step1<'a>, buf: &'a mut Buffers, driver: D, handler: &'a mut dyn eusb::Handler) -> Step2<'a, D>
    where D: eusb::driver::Driver<'a>
    {
        let mut builder = eusb::Builder::new(
            driver,
            step1.config,
            &mut buf.config_descriptor,
            &mut buf.bos_descriptor,
            &mut buf.msos_descriptor,
            &mut buf.control_buf,
        );
        builder.handler(handler);
        Step2 { builder }
    }

    pub struct Step2<'a, D: eusb::driver::Driver<'a>> {
        pub builder: eusb::Builder<'a, D>,
    }
}

