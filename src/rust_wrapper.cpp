extern "C" {
#include "wiring_private.h"
#include "usb_private.h"
}
#include "usb_api.h"
#include "rust_wrapper.h"

void usb_debug_putchar(uint8_t c) {
    Serial.write(c);
    // void send_now(void); ??
    // virtual void flush(); ??
}

void usb_try_init() {
    cli();
    usb_init();
    sei();
    //Serial.begin(0);
}

void usb_simple_send_key(uint16_t k) {
    Keyboard.press(k);
    Keyboard.release(k);
}

