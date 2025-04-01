use embassy_usb::{self as eusb, class::hid};

pub mod buffers {
    use super::*;

    pub struct ForDevice {
        pub config_descriptor: [u8; 256],
        pub bos_descriptor: [u8; 256],
        pub msos_descriptor: [u8; 256],
        pub control_buf: [u8; 64],
        pub dumb_handler: handler::Trivial,
    }

    impl ForDevice {
        pub fn new() -> Self {
            Self {
                // TODO: why below sizes in example? try to understand and adjust
                config_descriptor: [0; 256],
                bos_descriptor: [0; 256],
                msos_descriptor: [0; 256],
                control_buf: [0; 64],
                dumb_handler: handler::Trivial { },
            }
        }
    }

    pub struct ForHid<'a> {
        pub state: hid::State<'a>,
    }

    impl<'a> ForHid<'a> {
        pub fn new() -> Self {
            Self {
                state: hid::State::new(),
            }
        }
    }

}

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

impl<'a> Step1<'a> {
    pub fn into_device_builder<D>(self, driver: D, buf: &'a mut buffers::ForDevice) -> eusb::Builder<'a, D>
    where D: eusb::driver::Driver<'a>
    {
        let mut builder = eusb::Builder::new(
            driver,
            self.config,
            &mut buf.config_descriptor,
            &mut buf.bos_descriptor,
            &mut buf.msos_descriptor,
            &mut buf.control_buf,
        );
        builder.handler(&mut buf.dumb_handler);
        builder
    }
}

pub mod handler {
    use super::*;

    pub struct Trivial { }
    impl eusb::Handler for Trivial { }
}
