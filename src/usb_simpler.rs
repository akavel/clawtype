use embassy_usb as eusb;

pub fn new<'a>(manufacturer: &'a str, product: &'a str) -> (Step1a<'a>, Step1b) {
    let mut config = eusb::Config::new(0xc0de, 0xcafe);
    config.manufacturer = Some(manufacturer);
    config.product = Some(product);
    let s1a = Step1a {
        config,
    };
    let s1b = Step1b {
        // TODO: why below sizes in example? try to understand and adjust
        config_descriptor: [0; 256],
        bos_descriptor: [0; 256],
        msos_descriptor: [0; 256],
        control_buf: [0; 64],
    };
    (s1a, s1b)
}

pub struct Step1a<'a> {
    pub config: eusb::Config<'a>,
}

pub struct Step1b {
    pub config_descriptor: [u8; 256],
    pub bos_descriptor: [u8; 256],
    pub msos_descriptor: [u8; 256],
    pub control_buf: [u8; 64],
}

impl<'a> Step1a<'a> {
    pub fn next<D>(self, step1b: &'a mut Step1b, driver: D, handler: &'a mut dyn eusb::Handler) -> Step2<'a, D>
    where D: eusb::driver::Driver<'a>
    {
        let mut builder = eusb::Builder::new(
            driver,
            self.config,
            &mut step1b.config_descriptor,
            &mut step1b.bos_descriptor,
            &mut step1b.msos_descriptor,
            &mut step1b.control_buf,
        );
        builder.handler(handler);
        Step2 { builder }
    }
}

pub struct Step2<'a, D: eusb::driver::Driver<'a>> {
    pub builder: eusb::Builder<'a, D>,
}

