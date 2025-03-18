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

#![cfg_attr(not(test), no_std)]

use core::mem;
use core::ops::{BitAndAssign, BitOr, BitOrAssign, Not};

pub mod keycodes;
pub mod sample_layers;

/// Currently, the most significant bit is the pinky finger's tip switch,
/// then pinky finger's base switch. Subsequent bits represent tip & base
/// of ring finger, middle finger, and index finger.
///
/// E.g.: `0b10_00_00_01` is: pinky tip + index base pressed.
#[derive(Default)]
pub struct SwitchSet(pub u8);

#[derive(Copy, Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum UsbOutcome<KeyWithFlags> {
    Nothing,
    KeyHit(KeyWithFlags),
    KeyPress(KeyWithFlags),
    KeyRelease(KeyWithFlags),
}

impl<K: BitOr<Output = K>> BitOr<K> for UsbOutcome<K> {
    type Output = Self;
    fn bitor(self, mask: K) -> Self {
        self.map(|k| k | mask)
    }
}

impl<K> UsbOutcome<K> {
    fn map<U, F>(self, f: F) -> UsbOutcome<U>
    where F: FnOnce(K) -> U,
    {
        use UsbOutcome::*;
        match self {
            Self::Nothing => Nothing,
            Self::KeyHit(k) => KeyHit(f(k)),
            Self::KeyPress(k) => KeyPress(f(k)),
            Self::KeyRelease(k) => KeyRelease(f(k)),
        }
    }
}

pub struct LayerInfo {
    pub unchorded_mask: SwitchSet,
}

#[derive(Copy, Clone)]
pub enum LayerOutcome<KeyWithFlags> {
    ClearState,
    Emit(UsbOutcome<KeyWithFlags>),
    LayerSwitch {
        layer: i32,
    },
    TemporaryLayerSwitch {
        layer: i32,
    },
    /// Intended for adding USB flag key, like Alt, Shift, GUI, RAlt, etc.
    TogglePlusMask {
        mask: KeyWithFlags,
    },
    /// Intended for adding USB flag key, like Alt, Shift, GUI, RAlt, etc.
    TemporaryPlusMask {
        mask: KeyWithFlags,
    },
    /// Intended for adding USB flag key, like Alt, Shift, GUI, RAlt, etc.
    FromOtherPlusMask {
        layer: i32,
        mask: KeyWithFlags,
    },
}

pub struct Engine<L: Lookup> {
    most: SwitchSet,
    layer: i32,
    temporary_layer: Option<i32>,
    plus_mask: L::KeyWithFlags,
    temporary_plus_mask: L::KeyWithFlags,
}

impl<L> Default for Engine<L>
where
    L: Lookup,
    L::KeyWithFlags: Default,
{
    fn default() -> Self {
        Self {
            most: SwitchSet::default(),
            layer: 0,
            temporary_layer: None,
            plus_mask: L::KeyWithFlags::default(),
            temporary_plus_mask: L::KeyWithFlags::default(),
        }
    }
}

pub trait Lookup {
    type KeyWithFlags;
    fn lookup(layer: i32, chord: u8) -> Option<LayerOutcome<Self::KeyWithFlags>>;
    fn info(_layer: i32) -> LayerInfo {
        LayerInfo { unchorded_mask: Default::default() }
    }

    fn unchorded_key(_layer: i32, _switch: SwitchSet) -> Option<Self::KeyWithFlags> { None }
}

pub fn lookup_in_slice<K>(chord: u8, layout: &[(u8, LayerOutcome<K>)]) -> Option<&LayerOutcome<K>> {
    layout.iter().find(|x| x.0 == chord).map(|x| &x.1)
}


impl<L> Engine<L>
where
    L: Lookup,
    L::KeyWithFlags: Copy + Default + BitAndAssign + BitOr<Output = L::KeyWithFlags> + BitOrAssign + Not<Output = L::KeyWithFlags>,
{
    pub fn handle(&mut self, switches: SwitchSet) -> UsbOutcome<L::KeyWithFlags> {
        // some switches are pressed?
        if switches.0 != 0 {
            self.most.0 |= switches.0;
            return UsbOutcome::Nothing;
        }

        // all switches released
        let most = self.most.0;
        self.most = SwitchSet::default();
        if most == 0 {
            return UsbOutcome::Nothing;
        }
        let layer = self.temporary_layer.take().unwrap_or(self.layer);
        self.resolve(layer, most)
    }

    fn resolve(&mut self, layer: i32, chord: u8) -> UsbOutcome<L::KeyWithFlags> {
        let lookup = match L::lookup(layer, chord) {
            Some(v) => v,
            // As a fallback, try if we can find default action on an empty
            // chord 0 (this chord can't be ever selected as a combination
            // so we hackily reuse it as a "default" action for a layer)
            None => match L::lookup(layer, 0) {
                Some(v) => v,
                None => return UsbOutcome::Nothing,
            },
        };
        use LayerOutcome::*;
        use core::mem::take;
        match lookup {
            ClearState => {
                take(&mut self.layer);
                take(&mut self.temporary_layer);
                take(&mut self.plus_mask);
                take(&mut self.temporary_plus_mask);
                UsbOutcome::Nothing
            }
            Emit(v) => v | mem::take(&mut self.temporary_plus_mask) | self.plus_mask,
            LayerSwitch { layer } => {
                self.layer = layer;
                UsbOutcome::Nothing
            }
            TemporaryLayerSwitch { layer } => {
                self.temporary_layer = Some(layer);
                UsbOutcome::Nothing
            }
            TogglePlusMask { mask } => {
                self.temporary_plus_mask &= !mask;
                self.plus_mask |= mask;
                UsbOutcome::Nothing
            }
            TemporaryPlusMask { mask } => {
                self.temporary_plus_mask |= mask;
                UsbOutcome::Nothing
            }
            FromOtherPlusMask { layer, mask } => {
                self.temporary_plus_mask |= mask;
                // FIXME: protect against infinite recursion
                self.resolve(layer, chord)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use SwitchSet as S;
    use UsbOutcome::{
        KeyHit as Hit, KeyPress as Press, KeyRelease as Release,
        Nothing,
    };
    use keycodes::*;
    use clawtype_macros::chord;
    use sample_layers::SampleLayers as L;

    #[test]
    fn zero() {
        let mut eng = Engine::<L>::default();
        assert_eq!(eng.handle(S(0)), Nothing);
    }

    #[test]
    fn key_up_incremental_then_decremental_then_esc_instant() {
        let mut eng = Engine::<L>::default();
        assert_eq!(eng.handle(S(0b00_10_00_00)), Nothing);
        assert_eq!(eng.handle(S(0b00_10_00_10)), Nothing);
        assert_eq!(eng.handle(S(0b00_10_00_11)), Nothing);
        assert_eq!(eng.handle(S(0b00_10_00_01)), Nothing);
        assert_eq!(eng.handle(S(0b00_00_00_01)), Nothing);
        assert_eq!(eng.handle(S(0)), Hit(UP));

        assert_eq!(eng.handle(S(chord!("vvvv"))), Nothing);
        assert_eq!(eng.handle(S(0)), Hit(ESC));
    }

    #[test]
    fn key_from_shift_layer() {
        let mut eng = Engine::<L>::default();
        assert_eq!(eng.handle(S(chord!("_v__"))), Nothing);
        assert_eq!(eng.handle(S(chord!("_vv_"))), Nothing); // "shift"
        assert_eq!(eng.handle(S(0)), Nothing);
        // "shifted" key
        assert_eq!(eng.handle(S(chord!("_^__"))), Nothing);
        assert_eq!(eng.handle(S(0)), Hit(DELETE));
        // back to "unshifted" key
        assert_eq!(eng.handle(S(chord!("_^__"))), Nothing);
        assert_eq!(eng.handle(S(0)), Hit(BACKSPACE));
    }

    #[test]
    fn upper_case_letter_from_shift_layer() {
        let mut eng = Engine::<L>::default();
        assert_eq!(eng.handle(S(chord!("_v__"))), Nothing);
        assert_eq!(eng.handle(S(chord!("_vv_"))), Nothing); // "shift"
        assert_eq!(eng.handle(S(0)), Nothing);
        // "shifted" key
        assert_eq!(eng.handle(S(chord!("___^"))), Nothing);
        assert_eq!(eng.handle(S(0)), Hit(E | SHIFT_FLAG));
        // back to "unshifted" key
        assert_eq!(eng.handle(S(chord!("___^"))), Nothing);
        assert_eq!(eng.handle(S(0)), Hit(E));

        // another try

        assert_eq!(eng.handle(S(chord!("_v__"))), Nothing);
        assert_eq!(eng.handle(S(chord!("_vv_"))), Nothing); // "shift"
        assert_eq!(eng.handle(S(0)), Nothing);
        // "shifted" key
        assert_eq!(eng.handle(S(chord!("__vv"))), Nothing);
        assert_eq!(eng.handle(S(0)), Hit(C | SHIFT_FLAG));
        // back to "unshifted" key
        assert_eq!(eng.handle(S(chord!("__vv"))), Nothing);
        assert_eq!(eng.handle(S(0)), Hit(C));
    }

    #[test]
    fn masking_keys() {
        let mut eng = Engine::<L>::default();

        // ctrl-alt-del
        assert_eq!(eng.handle(S(chord!("_^^_"))), Nothing); // Ctrl
        assert_eq!(eng.handle(S(0)), Nothing);
        assert_eq!(eng.handle(S(chord!("%%__"))), Nothing); // Alt
        assert_eq!(eng.handle(S(0)), Nothing);
        assert_eq!(eng.handle(S(chord!("_vv_"))), Nothing); // SHIFT layer
        assert_eq!(eng.handle(S(0)), Nothing);
        assert_eq!(eng.handle(S(chord!("_^__"))), Nothing); // DEL
        assert_eq!(eng.handle(S(0)), Hit(DELETE | CTRL_FLAG | ALT_FLAG));

        // Win-shift-s = Snippet tool on Windows
        assert_eq!(eng.handle(S(chord!("_%%_"))), Nothing); // Gui
        assert_eq!(eng.handle(S(0)), Nothing);
        assert_eq!(eng.handle(S(chord!("_vv_"))), Nothing); // SHIFT layer
        assert_eq!(eng.handle(S(0)), Nothing);
        assert_eq!(eng.handle(S(chord!("^___"))), Nothing); // S
        assert_eq!(eng.handle(S(0)), Hit(keycodes::S | SHIFT_FLAG | GUI_FLAG));
    }

    #[test]
    fn shift_with_other_modifier_and_letter() {
        let mut eng = Engine::<L>::default();

        // shift-r_alt-e => Ę
        assert_eq!(eng.handle(S(chord!("_vv_"))), Nothing);
        assert_eq!(eng.handle(S(0)), Nothing);
        assert_eq!(eng.handle(S(chord!("%%_^"))), Nothing);
        assert_eq!(eng.handle(S(0)), Nothing);
        assert_eq!(eng.handle(S(chord!("___^"))), Nothing);
        assert_eq!(eng.handle(S(0)), Hit(keycodes::E | SHIFT_FLAG | RIGHT_ALT_FLAG));

        // r_alt-shift-e => also Ę
        assert_eq!(eng.handle(S(chord!("%%_^"))), Nothing);
        assert_eq!(eng.handle(S(0)), Nothing);
        assert_eq!(eng.handle(S(chord!("_vv_"))), Nothing);
        assert_eq!(eng.handle(S(0)), Nothing);
        assert_eq!(eng.handle(S(chord!("___^"))), Nothing);
        assert_eq!(eng.handle(S(0)), Hit(keycodes::E | SHIFT_FLAG | RIGHT_ALT_FLAG));
    }

    #[test]
    fn unchorded() {
        let mut eng = Engine::<L>::default();

        // enter TEST layer with some unchorded keys
        assert_eq!(eng.handle(S(chord!("v^_v"))), Nothing);
        // immediate press of mouse button, then release (a click)
        assert_eq!(eng.handle(S(chord!("___^"))), Press(HACK_MOUSE_LEFT_BTN));
        assert_eq!(eng.handle(S(chord!("____"))), Release(HACK_MOUSE_LEFT_BTN));
        // ctrl-press, then release
        assert_eq!(eng.handle(S(chord!("^^__"))), Nothing);
        assert_eq!(eng.handle(S(chord!("___^"))), Press(HACK_MOUSE_LEFT_BTN | CTRL_FLAG));
        assert_eq!(eng.handle(S(chord!("____"))), Release(HACK_MOUSE_LEFT_BTN));
        // ctrl-press, then ctrl-release
        assert_eq!(eng.handle(S(chord!("^^__"))), Nothing);
        assert_eq!(eng.handle(S(chord!("___^"))), Press(HACK_MOUSE_LEFT_BTN | CTRL_FLAG));
        assert_eq!(eng.handle(S(chord!("^^_^"))), Nothing);
        assert_eq!(eng.handle(S(chord!("____"))), Release(HACK_MOUSE_LEFT_BTN | CTRL_FLAG));
    }
}
