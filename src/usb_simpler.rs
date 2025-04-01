
pub fn new<'a>(manufacturer: &'a str, product: &'a str) -> Step1<'a> {
    let mut config = embassy_usb::Config::new(0xc0de, 0xcafe);
    config.manufacturer = Some(manufacturer);
    config.product = Some(product);
    Step1 {
        config,
        // TODO: why below sizes in example? try to understand and adjust
        config_descriptor: [0; 256],
        bos_descriptor: [0; 256],
        msos_descriptor: [0; 256],
        control_buf: [0; 64],
    }
}

pub struct Step1<'a> {
    pub config: embassy_usb::Config<'a>,
    pub config_descriptor: [u8; 256],
    pub bos_descriptor: [u8; 256],
    pub msos_descriptor: [u8; 256],
    pub control_buf: [u8; 64],
}
