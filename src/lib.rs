#![cfg_attr(not(test), no_std)]

use core::mem;

pub mod keycodes;
use keycodes::KeyWithFlags;
pub mod sample_layers;

/// Currently, the most significant bit is the pinky finger's tip switch,
/// then pinky finger's base switch. Subsequent bits represent tip & base
/// of ring finger, middle finger, and index finger.
///
/// E.g.: `0b10_00_00_01` is: pinky tip + index base pressed.
#[derive(Default)]
pub struct SwitchSet(u8);

#[derive(Copy, Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum UsbOutcome {
    Nothing,
    KeyHit(KeyWithFlags),
}

impl core::ops::BitOr<KeyWithFlags> for UsbOutcome {
    type Output = Self;
    fn bitor(self, mask: KeyWithFlags) -> Self {
        use UsbOutcome::*;
        match self {
            Nothing => Nothing,
            KeyHit(k) => KeyHit(k | mask),
        }
    }
}

#[derive(Copy, Clone)]
pub enum LayerOutcome {
    Emit(UsbOutcome),
    /// Intended for adding USB flag key, like Alt, Shift, GUI, RAlt, etc.
    TemporaryPlusMask {
        mask: KeyWithFlags,
    },
    TemporaryLayerSwitch {
        layer: i32,
    },
    /// Intended for adding USB flag key, like Alt, Shift, GUI, RAlt, etc.
    FromOtherPlusMask {
        layer: i32,
        mask: KeyWithFlags,
    },
}

#[derive(Default)]
pub struct Chordite {
    most: SwitchSet,
    temporary_layer: Option<i32>,
    temporary_plus_mask: KeyWithFlags,
}

use LayerOutcome::*;

pub trait Lookup {
    fn lookup(layer: i32, chord: u8) -> Option<LayerOutcome>;
}

impl Chordite {
    pub fn handle<L: Lookup>(&mut self, switches: SwitchSet) -> UsbOutcome {
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
        let layer = self.temporary_layer.take().unwrap_or(0);
        self.resolve::<L>(layer, most)
    }

    fn resolve<L: Lookup>(&mut self, layer: i32, chord: u8) -> UsbOutcome {
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
        match lookup {
            Emit(v) => v | mem::take(&mut self.temporary_plus_mask),
            TemporaryPlusMask { mask } => {
                self.temporary_plus_mask |= mask;
                UsbOutcome::Nothing
            }
            TemporaryLayerSwitch { layer } => {
                self.temporary_layer = Some(layer);
                UsbOutcome::Nothing
            }
            FromOtherPlusMask { layer, mask } => {
                // FIXME: protect against infinite recursion
                self.resolve::<L>(layer, chord) | mask
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use UsbOutcome::KeyHit as Hit;
    use UsbOutcome::Nothing;
    use keycodes::*;
    use macros::chord;
    use sample_layers::SampleLayers as L;

    #[test]
    fn zero() {
        let mut ch = Chordite::default();
        assert_eq!(ch.handle::<L>(SwitchSet(0)), Nothing);
    }

    #[test]
    fn key_up_incremental_then_decremental_then_esc_instant() {
        let mut ch = Chordite::default();
        use SwitchSet as S;
        assert_eq!(ch.handle::<L>(S(0b00_10_00_00)), Nothing);
        assert_eq!(ch.handle::<L>(S(0b00_10_00_10)), Nothing);
        assert_eq!(ch.handle::<L>(S(0b00_10_00_11)), Nothing);
        assert_eq!(ch.handle::<L>(S(0b00_10_00_01)), Nothing);
        assert_eq!(ch.handle::<L>(S(0b00_00_00_01)), Nothing);
        assert_eq!(ch.handle::<L>(S(0)), Hit(UP));

        assert_eq!(ch.handle::<L>(S(chord!("vvvv"))), Nothing);
        assert_eq!(ch.handle::<L>(S(0)), Hit(ESC));
    }

    #[test]
    fn key_from_shift_layer() {
        let mut ch = Chordite::default();
        use SwitchSet as S;
        assert_eq!(ch.handle::<L>(S(chord!("_v__"))), Nothing);
        assert_eq!(ch.handle::<L>(S(chord!("_vv_"))), Nothing); // "shift"
        assert_eq!(ch.handle::<L>(S(0)), Nothing);
        // "shifted" key
        assert_eq!(ch.handle::<L>(S(chord!("_^__"))), Nothing);
        assert_eq!(ch.handle::<L>(S(0)), Hit(DELETE));
        // back to "unshifted" key
        assert_eq!(ch.handle::<L>(S(chord!("_^__"))), Nothing);
        assert_eq!(ch.handle::<L>(S(0)), Hit(BACKSPACE));
    }

    #[test]
    fn upper_case_letter_from_shift_layer() {
        let mut ch = Chordite::default();
        use SwitchSet as S;
        assert_eq!(ch.handle::<L>(S(chord!("_v__"))), Nothing);
        assert_eq!(ch.handle::<L>(S(chord!("_vv_"))), Nothing); // "shift"
        assert_eq!(ch.handle::<L>(S(0)), Nothing);
        // "shifted" key
        assert_eq!(ch.handle::<L>(S(chord!("___^"))), Nothing);
        assert_eq!(ch.handle::<L>(S(0)), Hit(E | SHIFT_FLAG));
        // back to "unshifted" key
        assert_eq!(ch.handle::<L>(S(chord!("___^"))), Nothing);
        assert_eq!(ch.handle::<L>(S(0)), Hit(E));

        // another try

        assert_eq!(ch.handle::<L>(S(chord!("_v__"))), Nothing);
        assert_eq!(ch.handle::<L>(S(chord!("_vv_"))), Nothing); // "shift"
        assert_eq!(ch.handle::<L>(S(0)), Nothing);
        // "shifted" key
        assert_eq!(ch.handle::<L>(S(chord!("__vv"))), Nothing);
        assert_eq!(ch.handle::<L>(S(0)), Hit(C | SHIFT_FLAG));
        // back to "unshifted" key
        assert_eq!(ch.handle::<L>(S(chord!("__vv"))), Nothing);
        assert_eq!(ch.handle::<L>(S(0)), Hit(C));
    }

    #[test]
    fn masking_keys() {
        let mut ch = Chordite::default();
        use SwitchSet as S;

        // ctrl-alt-del
        assert_eq!(ch.handle::<L>(S(chord!("_^^_"))), Nothing); // Ctrl
        assert_eq!(ch.handle::<L>(S(0)), Nothing);
        assert_eq!(ch.handle::<L>(S(chord!("%%__"))), Nothing); // Alt
        assert_eq!(ch.handle::<L>(S(0)), Nothing);
        assert_eq!(ch.handle::<L>(S(chord!("_vv_"))), Nothing); // SHIFT layer
        assert_eq!(ch.handle::<L>(S(0)), Nothing);
        assert_eq!(ch.handle::<L>(S(chord!("_^__"))), Nothing); // DEL
        assert_eq!(ch.handle::<L>(S(0)), Hit(DELETE | CTRL_FLAG | ALT_FLAG));

        // Win-shift-s = Snippet tool on Windows
        assert_eq!(ch.handle::<L>(S(chord!("_%%_"))), Nothing); // Gui
        assert_eq!(ch.handle::<L>(S(0)), Nothing);
        assert_eq!(ch.handle::<L>(S(chord!("_vv_"))), Nothing); // SHIFT layer
        assert_eq!(ch.handle::<L>(S(0)), Nothing);
        assert_eq!(ch.handle::<L>(S(chord!("^___"))), Nothing); // S
        assert_eq!(
            ch.handle::<L>(S(0)),
            Hit(keycodes::S | SHIFT_FLAG | GUI_FLAG)
        );
    }
}
