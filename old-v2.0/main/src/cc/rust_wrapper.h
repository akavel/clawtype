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

// Inspired by:
// - https://friendlyuser.github.io/posts/tech/rust/rust_ffi_with_c_and_cplusplus

#include <inttypes.h>

//#ifdef __cplusplus
extern "C" {
//#endif

void usb_debug_putchar(uint8_t c);
void usb_try_init();
void usb_send_key_with_mod(uint8_t key, uint8_t mod);
void usb_set_mod_now(uint8_t mod);
void usb_set_key_now(uint8_t key);
void usb_mouse_move(int8_t x, int8_t y);
void usb_mouse_press(uint8_t btn);
void usb_mouse_release(uint8_t btn);
void usb_mouse_wheel_scroll(int8_t amount);

//#ifdef __cplusplus
}
//#endif
