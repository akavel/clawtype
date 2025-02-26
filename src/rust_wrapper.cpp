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

void usb_send_key_with_mod(uint8_t key, uint8_t mod) {
    // Press modifier & key. In sequence, just in case it would matter to OS.
    Keyboard.set_modifier(mod);
    Keyboard.send_now();
    Keyboard.set_key1(key);
    Keyboard.send_now();
    // Release key & modifier.
    Keyboard.set_key1(0);
    Keyboard.send_now();
    Keyboard.set_modifier(0);
    Keyboard.send_now();
}

