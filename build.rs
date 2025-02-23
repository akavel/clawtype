fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo::rerun-if-changed=build.rs");

    for basename in ["usb_debug_only", "print"] {
        let path = format!("src/{basename}.c");
        println!("cargo::rerun-if-changed={path}");
        // Use the `cc` crate to build a C file and statically link it.
        cc::Build::new()
            .pic(false)
            .warnings(false)
            .flag("-mmcu=atmega32u4")
            .compiler("avr-gcc")
            .archiver("avr-ar")
            .file(path)
            .compile(basename);
    }
}
