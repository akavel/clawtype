// clawtype-chords is (a part of) firmware for chorded keyboards
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

/// Most significant byte is modifiers ("USB DV flags"), like Shift,
/// Alt, GUI, or Right Alt.
/// Least significant byte is keycode.
pub type KeyWithFlags = u16;

pub const CTRL_FLAG: KeyWithFlags = 0x0100;
pub const SHIFT_FLAG: KeyWithFlags = 0x0200;
pub const ALT_FLAG: KeyWithFlags = 0x0400;
pub const GUI_FLAG: KeyWithFlags = 0x0800;
pub const LEFT_CTRL_FLAG: KeyWithFlags = 0x0100;
pub const LEFT_SHIFT_FLAG: KeyWithFlags = 0x0200;
pub const LEFT_ALT_FLAG: KeyWithFlags = 0x0400;
pub const LEFT_GUI_FLAG: KeyWithFlags = 0x0800;
pub const RIGHT_CTRL_FLAG: KeyWithFlags = 0x1000;
pub const RIGHT_SHIFT_FLAG: KeyWithFlags = 0x2000;
pub const RIGHT_ALT_FLAG: KeyWithFlags = 0x4000;
pub const RIGHT_GUI_FLAG: KeyWithFlags = 0x8000;

pub const HACK_MOUSE_MARKER: KeyWithFlags = 0xF0;
pub const HACK_MOUSE_ENABLE_TOGGLE: KeyWithFlags = 0xF0;
// pub const HACK_MOUSE_ENABLE: KeyWithFlags = 0xF0;
// pub const HACK_MOUSE_DISABLE: KeyWithFlags = 0xF1;
// pub const HACK_MOUSE_LEFT_PRESS: KeyWithFlags = 0xF2;
// pub const HACK_MOUSE_LEFT_RELEASE: KeyWithFlags = 0xF3;
pub const HACK_MOUSE_LEFT_DRAG_TOGGLE: KeyWithFlags = 0xF3;
pub const HACK_MOUSE_LEFT_CLICK: KeyWithFlags = 0xF4;
pub const HACK_MOUSE_RIGHT_PRESS: KeyWithFlags = 0xF5;
pub const HACK_MOUSE_RIGHT_RELEASE: KeyWithFlags = 0xF6;
pub const HACK_MOUSE_RIGHT_CLICK: KeyWithFlags = 0xF7;
pub const HACK_MOUSE_MIDDLE_PRESS: KeyWithFlags = 0xF8;
pub const HACK_MOUSE_MIDDLE_RELEASE: KeyWithFlags = 0xF9;
pub const HACK_MOUSE_MIDDLE_CLICK: KeyWithFlags = 0xFA;
pub const HACK_MOUSE_WHEEL_DOWN: KeyWithFlags = 0xFB;
pub const HACK_MOUSE_WHEEL_UP: KeyWithFlags = 0xFC;

pub const A: KeyWithFlags = 4;
pub const B: KeyWithFlags = 5;
pub const C: KeyWithFlags = 6;
pub const D: KeyWithFlags = 7;
pub const E: KeyWithFlags = 8;
pub const F: KeyWithFlags = 9;
pub const G: KeyWithFlags = 10;
pub const H: KeyWithFlags = 11;
pub const I: KeyWithFlags = 12;
pub const J: KeyWithFlags = 13;
pub const K: KeyWithFlags = 14;
pub const L: KeyWithFlags = 15;
pub const M: KeyWithFlags = 16;
pub const N: KeyWithFlags = 17;
pub const O: KeyWithFlags = 18;
pub const P: KeyWithFlags = 19;
pub const Q: KeyWithFlags = 20;
pub const R: KeyWithFlags = 21;
pub const S: KeyWithFlags = 22;
pub const T: KeyWithFlags = 23;
pub const U: KeyWithFlags = 24;
pub const V: KeyWithFlags = 25;
pub const W: KeyWithFlags = 26;
pub const X: KeyWithFlags = 27;
pub const Y: KeyWithFlags = 28;
pub const Z: KeyWithFlags = 29;
pub const KEY_1: KeyWithFlags = 30;
pub const KEY_2: KeyWithFlags = 31;
pub const KEY_3: KeyWithFlags = 32;
pub const KEY_4: KeyWithFlags = 33;
pub const KEY_5: KeyWithFlags = 34;
pub const KEY_6: KeyWithFlags = 35;
pub const KEY_7: KeyWithFlags = 36;
pub const KEY_8: KeyWithFlags = 37;
pub const KEY_9: KeyWithFlags = 38;
pub const KEY_0: KeyWithFlags = 39;
pub const ENTER: KeyWithFlags = 40;
pub const ESC: KeyWithFlags = 41;
pub const BACKSPACE: KeyWithFlags = 42;
pub const TAB: KeyWithFlags = 43;
pub const SPACE: KeyWithFlags = 44;
pub const MINUS: KeyWithFlags = 45;
pub const EQUAL: KeyWithFlags = 46;
pub const LEFT_BRACE: KeyWithFlags = 47;
pub const RIGHT_BRACE: KeyWithFlags = 48;
pub const BACKSLASH: KeyWithFlags = 49;
pub const NON_US_NUM: KeyWithFlags = 50;
pub const SEMICOLON: KeyWithFlags = 51;
pub const QUOTE: KeyWithFlags = 52;
pub const TILDE: KeyWithFlags = 53;
pub const COMMA: KeyWithFlags = 54;
pub const PERIOD: KeyWithFlags = 55;
pub const SLASH: KeyWithFlags = 56;
pub const CAPS_LOCK: KeyWithFlags = 57;
pub const F1: KeyWithFlags = 58;
pub const F2: KeyWithFlags = 59;
pub const F3: KeyWithFlags = 60;
pub const F4: KeyWithFlags = 61;
pub const F5: KeyWithFlags = 62;
pub const F6: KeyWithFlags = 63;
pub const F7: KeyWithFlags = 64;
pub const F8: KeyWithFlags = 65;
pub const F9: KeyWithFlags = 66;
pub const F10: KeyWithFlags = 67;
pub const F11: KeyWithFlags = 68;
pub const F12: KeyWithFlags = 69;
pub const PRINTSCREEN: KeyWithFlags = 70;
pub const SCROLL_LOCK: KeyWithFlags = 71;
pub const PAUSE: KeyWithFlags = 72;
pub const INSERT: KeyWithFlags = 73;
pub const HOME: KeyWithFlags = 74;
pub const PAGE_UP: KeyWithFlags = 75;
pub const DELETE: KeyWithFlags = 76;
pub const END: KeyWithFlags = 77;
pub const PAGE_DOWN: KeyWithFlags = 78;
pub const RIGHT: KeyWithFlags = 79;
pub const LEFT: KeyWithFlags = 80;
pub const DOWN: KeyWithFlags = 81;
pub const UP: KeyWithFlags = 82;
pub const NUM_LOCK: KeyWithFlags = 83;

pub const KEYPAD_SLASH: KeyWithFlags = 84;
pub const KEYPAD_ASTERIX: KeyWithFlags = 85;
pub const KEYPAD_MINUS: KeyWithFlags = 86;
pub const KEYPAD_PLUS: KeyWithFlags = 87;
pub const KEYPAD_ENTER: KeyWithFlags = 88;
pub const KEYPAD_1: KeyWithFlags = 89;
pub const KEYPAD_2: KeyWithFlags = 90;
pub const KEYPAD_3: KeyWithFlags = 91;
pub const KEYPAD_4: KeyWithFlags = 92;
pub const KEYPAD_5: KeyWithFlags = 93;
pub const KEYPAD_6: KeyWithFlags = 94;
pub const KEYPAD_7: KeyWithFlags = 95;
pub const KEYPAD_8: KeyWithFlags = 96;
pub const KEYPAD_9: KeyWithFlags = 97;
pub const KEYPAD_0: KeyWithFlags = 98;
pub const KEYPAD_PERIOD: KeyWithFlags = 99;

pub const NON_US_BS: KeyWithFlags = 100;
pub const MENU: KeyWithFlags = 101;
pub const F13: KeyWithFlags = 104;
pub const F14: KeyWithFlags = 105;
pub const F15: KeyWithFlags = 106;
pub const F16: KeyWithFlags = 107;
pub const F17: KeyWithFlags = 108;
pub const F18: KeyWithFlags = 109;
pub const F19: KeyWithFlags = 110;
pub const F20: KeyWithFlags = 111;
pub const F21: KeyWithFlags = 112;
pub const F22: KeyWithFlags = 113;
pub const F23: KeyWithFlags = 114;
pub const F24: KeyWithFlags = 115;
