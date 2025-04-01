use embassy_usb::{self as eusb, class::hid};

pub mod buffers {
    use super::*;

    pub struct ForDevice {
        pub config_descriptor: [u8; 256],
        pub bos_descriptor: [u8; 256],
        pub msos_descriptor: [u8; 256],
        pub control_buf: [u8; 64],
    }

    impl ForDevice {
        pub fn new() -> Self {
            Self {
                // TODO: why below sizes in example? try to understand and adjust
                config_descriptor: [0; 256],
                bos_descriptor: [0; 256],
                msos_descriptor: [0; 256],
                control_buf: [0; 64],
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

pub fn new<'a>(manufacturer: &'a str, product: &'a str) -> ConfigBuilder<'a> {
    let mut config = eusb::Config::new(0xc0de, 0xcafe);
    config.manufacturer = Some(manufacturer);
    config.product = Some(product);
    ConfigBuilder { config }
}

pub struct ConfigBuilder<'a> {
    pub config: eusb::Config<'a>,
}

impl<'a> ConfigBuilder<'a> {
    pub fn into_device_builder<D>(
        self,
        driver: D,
        buf: &'a mut buffers::ForDevice,
    ) -> DeviceBuilder<'a, D>
    where
        D: eusb::driver::Driver<'a>,
    {
        DeviceBuilder {
            wrapped: eusb::Builder::new(
                driver,
                self.config,
                &mut buf.config_descriptor,
                &mut buf.bos_descriptor,
                &mut buf.msos_descriptor,
                &mut buf.control_buf,
            ),
        }
    }
}

pub struct DeviceBuilder<'a, D>
where
    D: eusb::driver::Driver<'a>,
{
    pub wrapped: eusb::Builder<'a, D>,
}

impl<'a, D> DeviceBuilder<'a, D>
where
    D: eusb::driver::Driver<'a>,
{
    pub fn add_hid_reader_writer<const READ_N: usize, const WRITE_N: usize>(
        &mut self,
        buf: &'a mut buffers::ForHid<'a>,
        cfg: hid::Config<'a>,
    ) -> hid::HidReaderWriter<'a, D, READ_N, WRITE_N> {
        hid::HidReaderWriter::new(&mut self.wrapped, &mut buf.state, cfg)
    }

    pub fn build(self) -> eusb::UsbDevice<'a, D> {
        self.wrapped.build()
    }
}
