#![cfg_attr(not(test), no_std)]

/// Assumes Teensy keycode with modifiers.
pub type KeyWithModifiers = u16;

const UP: KeyWithModifiers =                  (  82  | 0xF000 );
