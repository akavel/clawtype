
pub fn new<'a>(manufacturer: &'a str, product: &'a str) -> Step1<'a> {
    let mut config = embassy_usb::Config::new(0xc0de, 0xcafe);
    config.manufacturer = Some(manufacturer);
    config.product = Some(product);
    Step1 { config }
}

pub struct Step1<'a> {
    pub config: embassy_usb::Config<'a>,
}
