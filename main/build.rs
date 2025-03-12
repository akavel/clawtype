fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo::rerun-if-changed=build.rs");

    // FIXME: refactor to merge the blocks below

    /*
     * TODO: try to mimic:
  "arguments": [
   "C:\\Users\\Mateusz\\AppData\\Local\\Arduino15\\packages\\teensy\\tools\\teensy-compile\\11.3.1/avr/bin/avr-g++",
   "-c",
   "-Os",
   "-g",
   "-Wall",
   "-ffunction-sections",
   "-fdata-sections",
   "-MMD",
   "-fno-exceptions",
   "-fpermissive",
   "-felide-constructors",
   "-std=gnu++11",
   "-mmcu=atmega32u4",
   "-DTEENSYDUINO=159",
   "-DARDUINO_ARCH_AVR",
   "-DARDUINO=10607",
   "-DARDUINO_TEENSY2",
   "-DF_CPU=16000000L",
   "-DUSB_SERIAL_HID",
   "-DLAYOUT_US_ENGLISH",
   "-IC:\\Users\\Mateusz\\AppData\\Local\\arduino\\sketches\\55DEF8E976DC8F46FAE695DEF032A07E/pch",
   "-IC:\\Users\\Mateusz\\AppData\\Local\\Arduino15\\packages\\teensy\\hardware\\avr\\1.59.0\\cores\\teensy",
   "C:\\Users\\Mateusz\\AppData\\Local\\Arduino15\\packages\\teensy\\hardware\\avr\\1.59.0\\cores\\teensy\\usb_api.cpp",
   "-o",
   "C:\\Users\\Mateusz\\AppData\\Local\\arduino\\sketches\\55DEF8E976DC8F46FAE695DEF032A07E\\core\\usb_api.cpp.o"
  ],
     */

    for basename in [
        "rust_wrapper", "usb_api",
        "Print", "Stream", "WString", "new", "HardwareSerial",
        "CrashReport",
    ] {
        let path = format!("src/cc/{basename}.cpp");
        println!("cargo::rerun-if-changed={path}");
        // Use the `cc` crate to build a C file and statically link it.
        cc::Build::new()
            // .cpp(true)
            .includes(&["src/cc"])
            // .no_default_flags(true)
            .force_frame_pointer(false)
            .pic(false)
            .warnings(false) // ??
            .flag("-mmcu=atmega32u4")
            .flag("-Os")
// .flag("-funsigned-char")
// .flag("-funsigned-bitfields")
// .flag("-fpack-struct")
// .flag("-fshort-enums")
            .flag("-fno-exceptions")
.flag("-felide-constructors")
.flag("-fpermissive")
.flag("-std=gnu++11")
            .define("LAYOUT_US_INTERNATIONAL", None)
            // .define("LAYOUT_US_ENGLISH", None)
            .define("ARDUINO", "100")
            .define("F_CPU", "8000000UL")
            // .define("USB_SERIAL_HID", None)
            // .define("MCU", "atmega32u4")
            // .opt_level_str("s")
            .compiler("avr-g++")
            .archiver("avr-ar")
            .file(path)
            .compile(basename);
    }

    for basename in ["usb", "keylayouts", "wiring", "pins_teensy"] {
        let path = format!("src/cc/{basename}.c");
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
}
