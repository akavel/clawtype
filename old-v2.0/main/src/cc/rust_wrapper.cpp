// clawtype-rust is (a part of) firmware for chorded keyboards
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
#include <avr/interrupt.h>
#include <stdlib.h>
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

void usb_set_mod_now(uint8_t mod) {
    Keyboard.set_modifier(mod);
    Keyboard.send_now();
}

void usb_set_key_now(uint8_t key) {
    Keyboard.set_key1(key);
    Keyboard.send_now();
}

void usb_send_key_with_mod(uint8_t key, uint8_t mod) {
    // Press modifier & key. In sequence, just in case it would matter to OS.
    usb_set_mod_now(mod);
    usb_set_key_now(key);
    // Release key & modifier.
    usb_set_key_now(0);
    usb_set_mod_now(0);
}

void usb_mouse_move(int8_t x, int8_t y) {
    Mouse.move(x, y);
}

void usb_mouse_press(uint8_t btn) {
    Mouse.press(btn);
}

void usb_mouse_release(uint8_t btn) {
    Mouse.release(btn);
}

void usb_mouse_wheel_scroll(int8_t amount) {
    Mouse.scroll(amount);
}

