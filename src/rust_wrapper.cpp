#include "rust_wrapper.h"

#include "usb_api.h"

void usb_debug_putchar(uint8_t c) {
    Serial.write(c+0);
    // void send_now(void); ??
    // virtual void flush(); ??
}

void usb_try_init() {
    Serial.begin(0);

}
