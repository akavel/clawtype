extern "C" {
#include "wiring_private.h"
#include "usb_private.h"
}
#include "usb_api.h"
#include "rust_wrapper.h"

void usb_debug_putchar(uint8_t c) {
    Serial.write(c+0);
    // void send_now(void); ??
    // virtual void flush(); ??
}

void usb_try_init() {
    cli();
    usb_init();
    sei();
    //Serial.begin(0);
}
