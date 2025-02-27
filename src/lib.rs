#![cfg_attr(not(test), no_std)]

use const_map::const_map;
use core::mem;
use macros::chord;

pub mod keycodes;
use keycodes::*;

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
use UsbOutcome::KeyHit as Hit;

impl Chordite {
    pub fn handle(&mut self, switches: SwitchSet) -> UsbOutcome {
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
        self.resolve(layer, most)
    }

    fn resolve(&mut self, layer: i32, chord: u8) -> UsbOutcome {
        let lookup = match self.lookup(layer, chord) {
            Some(v) => v,
            // As a fallback, try if we can find default action on an empty
            // chord 0 (this chord can't be ever selected as a combination
            // so we hackily reuse it as a "default" action for a layer)
            None => match self.lookup(layer, 0) {
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
                self.resolve(layer, chord) | mask
            }
        }
    }

    fn lookup(&self, layer: i32, chord: u8) -> Option<LayerOutcome> {
        match layer {
            1 => Self::lookup1(chord), // "SHIFT"
            _ => Self::lookup0(chord),
        }
    }

    const_map!(
        LAYOUT0, lookup0(),
        (u8 => LayerOutcome) {
            chord!("_^_%") => Emit(Hit(UP)),
            chord!("_v_%") => Emit(Hit(DOWN)),
            chord!("^__%") => Emit(Hit(LEFT)),
            chord!("v__%") => Emit(Hit(RIGHT)),
            chord!("^^_%") => Emit(Hit(PAGE_UP)),
            chord!("vv_v") => Emit(Hit(PAGE_DOWN)),

            chord!("__^_") => Emit(Hit(SPACE)),
            chord!("_^__") => Emit(Hit(BACKSPACE)),
            chord!("___^") => Emit(Hit(E)),
            chord!("___v") => Emit(Hit(T)),
            chord!("__v_") => Emit(Hit(A)),
            chord!("___%") => Emit(Hit(I)),
            chord!("__%_") => Emit(Hit(O)),
            chord!("_v__") => Emit(Hit(N)),
            chord!("^___") => Emit(Hit(S)),
            chord!("_%__") => Emit(Hit(H)),
            chord!("v___") => Emit(Hit(R)),
            chord!("%___") => Emit(Hit(L)),
            chord!("__^^") => Emit(Hit(D)),
            chord!("__vv") => Emit(Hit(C)),
            chord!("__^v") => Emit(Hit(U)),
            chord!("^^__") => Emit(Hit(M)),
            chord!("_vv_") => TemporaryLayerSwitch { layer: 1 }, // SHIFT
            chord!("_^^_") => TemporaryPlusMask { mask: CTRL_FLAG }, // CTRL
            chord!("%%__") => TemporaryPlusMask { mask: ALT_FLAG }, // ALT
            // chord!("%%_^") => TemporaryPlusMask { mask: RIGHT_ALT_FLAG }, // R-ALT
            chord!("_%%_") => TemporaryPlusMask { mask: GUI_FLAG }, // GUI
            chord!("_^_^") => Emit(Hit(TAB)),
            chord!("__^%") => Emit(Hit(W)),
            chord!("_^_v") => Emit(Hit(G)),
            chord!("__%v") => Emit(Hit(F)),
            chord!("__%%") => Emit(Hit(Y)),
            chord!("_v_v") => Emit(Hit(P)),
            chord!("v__v") => Emit(Hit(B)),
            chord!("^__^") => Emit(Hit(COMMA)),
            chord!("_^^^") => Emit(Hit(PERIOD)),
            chord!("_vvv") => Emit(Hit(V)),
            chord!("_%_%") => Emit(Hit(ENTER)),
            chord!("vvvv") => Emit(Hit(ESC)),
            chord!("^__v") => Emit(Hit(K)),
            chord!("%__%") => Emit(Hit(QUOTE)),
            chord!("%__v") => Emit(Hit(QUOTE | SHIFT_FLAG)), // "
            chord!("vvv_") => Emit(Hit(MINUS)),
            chord!("__v%") => Emit(Hit(X)),
            chord!("_%%%") => Emit(Hit(J)),
            chord!("_%_v") => Emit(Hit(SEMICOLON)),
            chord!("^^^_") => Emit(Hit(KEY_9 | SHIFT_FLAG)), // (
            chord!("^_^_") => Emit(Hit(KEY_0 | SHIFT_FLAG)), // )
            chord!("^^^^") => Emit(Hit(Q)),
            chord!("_^^v") => Emit(Hit(SLASH)),
            chord!("_^^%") => Emit(Hit(Z)),
            chord!("^^_v") => Emit(Hit(SEMICOLON | SHIFT_FLAG)), // :
            chord!("_^%_") => Emit(Hit(KEY_0)),
            chord!("v_v_") => Emit(Hit(KEY_1)),
            chord!("%_%_") => Emit(Hit(KEY_2)),
            chord!("%%%_") => Emit(Hit(KEY_3)),
            chord!("^^^%") => Emit(Hit(KEY_4)),
            chord!("_vv%") => Emit(Hit(EQUAL)),
            chord!("%^__") => Emit(Hit(KEY_4 | SHIFT_FLAG)), // $
            chord!("^^_%") => Emit(Hit(KEY_8 | SHIFT_FLAG)), // *
            chord!("^_%_") => Emit(Hit(LEFT_BRACE | SHIFT_FLAG)), // {
            chord!("v_%_") => Emit(Hit(RIGHT_BRACE | SHIFT_FLAG)), // }
        }
    );

    // "SHIFT" layer
    const_map!(
        LAYOUT1, lookup1(),
        (u8 => LayerOutcome) {
            0 => FromOtherPlusMask { layer: 0, mask: SHIFT_FLAG },

            chord!("_^%_") => Emit(Hit(KEY_5)), // S-0 5
            chord!("v_v_") => Emit(Hit(KEY_6)), // S-1 6
            chord!("%_%_") => Emit(Hit(KEY_7)), // S-2 7
            chord!("%%%_") => Emit(Hit(KEY_8)), // S-3 8
            chord!("^^^%") => Emit(Hit(KEY_9)), // S-4 9
            chord!("^__^") => Emit(Hit(SLASH | SHIFT_FLAG)), // S-, ?
            chord!("_^^^") => Emit(Hit(KEY_1 | SHIFT_FLAG)), // S-. !
            chord!("vvv_") => Emit(Hit(MINUS | SHIFT_FLAG)), // S-- _
            chord!("%__%") => Emit(Hit(TILDE)), // S-' `
            chord!("^^^_") => Emit(Hit(LEFT_BRACE)), // S-( [
            chord!("^_^_") => Emit(Hit(RIGHT_BRACE)), // S-) ]
            chord!("_vv%") => Emit(Hit(EQUAL | SHIFT_FLAG)), // S-= +
            chord!("^_%_") => Emit(Hit(COMMA | SHIFT_FLAG)), // S-{ <
            chord!("v_%_") => Emit(Hit(PERIOD | SHIFT_FLAG)), // S-} >
            chord!("%__v") => Emit(Hit(KEY_7 | SHIFT_FLAG)), // S-" &
            chord!("_%_v") => Emit(Hit(KEY_2 | SHIFT_FLAG)), // S-; @
            chord!("_^^v") => Emit(Hit(BACKSLASH)), // S-/ \
            chord!("^^_v") => Emit(Hit(BACKSLASH | SHIFT_FLAG)), // S-: |
            chord!("%^__") => Emit(Hit(TILDE | SHIFT_FLAG)), // S-$ ~
            chord!("^^_%") => Emit(Hit(KEY_6 | SHIFT_FLAG)), // S-* ^

            chord!("_^__") => Emit(Hit(DELETE)), // S-Backspace KEY_DELETE
            chord!("_^_%") => Emit(Hit(HOME)), // S-Up KEY_HOME
            chord!("_v_%") => Emit(Hit(END)), // S-Down KEY_END
        }
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    use UsbOutcome::Nothing;

    #[test]
    fn zero() {
        let mut ch = Chordite::default();
        assert_eq!(ch.handle(SwitchSet(0)), Nothing);
    }

    #[test]
    fn key_up_incremental_then_decremental_then_esc_instant() {
        let mut ch = Chordite::default();
        use SwitchSet as S;
        assert_eq!(ch.handle(S(0b00_10_00_00)), Nothing);
        assert_eq!(ch.handle(S(0b00_10_00_10)), Nothing);
        assert_eq!(ch.handle(S(0b00_10_00_11)), Nothing);
        assert_eq!(ch.handle(S(0b00_10_00_01)), Nothing);
        assert_eq!(ch.handle(S(0b00_00_00_01)), Nothing);
        assert_eq!(ch.handle(S(0)), Hit(UP));

        assert_eq!(ch.handle(S(chord!("vvvv"))), Nothing);
        assert_eq!(ch.handle(S(0)), Hit(ESC));
    }

    #[test]
    fn key_from_shift_layer() {
        let mut ch = Chordite::default();
        use SwitchSet as S;
        assert_eq!(ch.handle(S(chord!("_v__"))), Nothing);
        assert_eq!(ch.handle(S(chord!("_vv_"))), Nothing); // "shift"
        assert_eq!(ch.handle(S(0)), Nothing);
        // "shifted" key
        assert_eq!(ch.handle(S(chord!("_^__"))), Nothing);
        assert_eq!(ch.handle(S(0)), Hit(DELETE));
        // back to "unshifted" key
        assert_eq!(ch.handle(S(chord!("_^__"))), Nothing);
        assert_eq!(ch.handle(S(0)), Hit(BACKSPACE));
    }

    #[test]
    fn upper_case_letter_from_shift_layer() {
        let mut ch = Chordite::default();
        use SwitchSet as S;
        assert_eq!(ch.handle(S(chord!("_v__"))), Nothing);
        assert_eq!(ch.handle(S(chord!("_vv_"))), Nothing); // "shift"
        assert_eq!(ch.handle(S(0)), Nothing);
        // "shifted" key
        assert_eq!(ch.handle(S(chord!("___^"))), Nothing);
        assert_eq!(ch.handle(S(0)), Hit(E | SHIFT_FLAG));
        // back to "unshifted" key
        assert_eq!(ch.handle(S(chord!("___^"))), Nothing);
        assert_eq!(ch.handle(S(0)), Hit(E));

        // another try

        assert_eq!(ch.handle(S(chord!("_v__"))), Nothing);
        assert_eq!(ch.handle(S(chord!("_vv_"))), Nothing); // "shift"
        assert_eq!(ch.handle(S(0)), Nothing);
        // "shifted" key
        assert_eq!(ch.handle(S(chord!("__vv"))), Nothing);
        assert_eq!(ch.handle(S(0)), Hit(C | SHIFT_FLAG));
        // back to "unshifted" key
        assert_eq!(ch.handle(S(chord!("__vv"))), Nothing);
        assert_eq!(ch.handle(S(0)), Hit(C));
    }

    #[test]
    fn masking_keys() {
        let mut ch = Chordite::default();
        use SwitchSet as S;

        // ctrl-alt-del
        assert_eq!(ch.handle(S(chord!("_^^_"))), Nothing); // Ctrl
        assert_eq!(ch.handle(S(0)), Nothing);
        assert_eq!(ch.handle(S(chord!("%%__"))), Nothing); // Alt
        assert_eq!(ch.handle(S(0)), Nothing);
        assert_eq!(ch.handle(S(chord!("_vv_"))), Nothing); // SHIFT layer
        assert_eq!(ch.handle(S(0)), Nothing);
        assert_eq!(ch.handle(S(chord!("_^__"))), Nothing); // DEL
        assert_eq!(ch.handle(S(0)), Hit(DELETE | CTRL_FLAG | ALT_FLAG));

        // Win-shift-s = Snippet tool on Windows
        assert_eq!(ch.handle(S(chord!("_%%_"))), Nothing); // Gui
        assert_eq!(ch.handle(S(0)), Nothing);
        assert_eq!(ch.handle(S(chord!("_vv_"))), Nothing); // SHIFT layer
        assert_eq!(ch.handle(S(0)), Nothing);
        assert_eq!(ch.handle(S(chord!("^___"))), Nothing); // S
        assert_eq!(ch.handle(S(0)), Hit(keycodes::S | SHIFT_FLAG | GUI_FLAG));
    }
}
