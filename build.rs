fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo::rerun-if-changed=build.rs");

    // FIXME: refactor to merge the blocks below

    for basename in ["usb", "keylayouts", "wiring"] {
        let path = format!("src/{basename}.c");
        println!("cargo::rerun-if-changed={path}");
        // Use the `cc` crate to build a C file and statically link it.
        cc::Build::new()
            .pic(false)
            .warnings(false) // ??
            .flag("-mmcu=atmega32u4")
            .flag("-Os")
.flag("-funsigned-char")
.flag("-funsigned-bitfields")
.flag("-ffunction-sections")
.flag("-fpack-struct")
.flag("-fshort-enums")
            .define("LAYOUT_US_INTERNATIONAL", None)
            // .define("LAYOUT_US_ENGLISH", None)
            .define("ARDUINO", "100")
            .define("F_CPU", "8000000UL")
            // .define("USB_SERIAL_HID", None)
            // .define("MCU", "atmega32u4")
            // .opt_level_str("s")
            .compiler("avr-gcc")
            .archiver("avr-ar")
            .file(path)
            .compile(basename);
    }

    for basename in ["rust_wrapper", "usb_api", "Print", "Stream", "WString", "new"] {
        let path = format!("src/{basename}.cpp");
        println!("cargo::rerun-if-changed={path}");
        // Use the `cc` crate to build a C file and statically link it.
        cc::Build::new()
            .cpp(true)
            .pic(false)
            .warnings(false) // ??
            .flag("-mmcu=atmega32u4")
            .flag("-Os")
.flag("-funsigned-char")
.flag("-funsigned-bitfields")
.flag("-fpack-struct")
.flag("-fshort-enums")
.flag("-fno-exceptions")
            .define("LAYOUT_US_INTERNATIONAL", None)
            // .define("LAYOUT_US_ENGLISH", None)
            .define("ARDUINO", "100")
            .define("F_CPU", "8000000UL")
            // .define("USB_SERIAL_HID", None)
            // .define("MCU", "atmega32u4")
            // .opt_level_str("s")
            .compiler("avr-gcc")
            .archiver("avr-ar")
            .file(path)
            .compile(basename);
    }
}
