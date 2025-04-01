
pub fn new<'a>(manufacturer: &'a str, product: &'a str) -> (Step1a<'a>, Step1b) {
    let mut config = embassy_usb::Config::new(0xc0de, 0xcafe);
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
    pub config: embassy_usb::Config<'a>,
}

pub struct Step1b {
    pub config_descriptor: [u8; 256],
    pub bos_descriptor: [u8; 256],
    pub msos_descriptor: [u8; 256],
    pub control_buf: [u8; 64],
}
