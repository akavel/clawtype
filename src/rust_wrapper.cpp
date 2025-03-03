// chordite-rust is (a part of) firmware for chorded keyboards
// Copyright (C) 2025  Mateusz Czapliński akavel.pl
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

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

