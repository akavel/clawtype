#![cfg_attr(not(test), no_std)]

use const_map::const_map;
use macros::chord;

pub mod keycodes;
use keycodes::*;

#[derive(Default)]
/// Currently, the most significant bit is the pinky finger's tip switch,
/// then pinky finger's base switch. Subsequent bits represent tip & base
/// of ring finger, middle finger, and index finger.
///
/// E.g.: `0b10_00_00_01` is: pinky tip + index base pressed.
pub struct SwitchSet(u8);

#[derive(Copy, Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum UsbOutcome {
    Nothing,
    KeyHit(KeyWithModifiers),
}

#[derive(Copy, Clone)]
pub enum LayerOutcome {
    Emit(UsbOutcome),
}

#[derive(Default)]
pub struct Chordite {
    most: SwitchSet,
}

use LayerOutcome::Emit;
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
        match Self::lookup0(most) {
            Some(Emit(v)) => return v,
            None => return UsbOutcome::Nothing,
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
            // _vv_ SHIFT
            // _^^_ CTRL
            // _%%_ WIN
            // %%__ ALT
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
            chord!("%__v") => Emit(Hit(QUOTE | SHIFT_MASK)),
            chord!("vvv_") => Emit(Hit(MINUS)),
            chord!("__v%") => Emit(Hit(X)),
            chord!("_%%%") => Emit(Hit(J)),
            chord!("_%_v") => Emit(Hit(SEMICOLON)),
            chord!("^^^_") => Emit(Hit(KEY_9 | SHIFT_MASK)), // (
            chord!("^_^_") => Emit(Hit(KEY_0 | SHIFT_MASK)), // )
            chord!("^^^^") => Emit(Hit(Q)),
            chord!("_^^v") => Emit(Hit(SLASH)),
            chord!("_^^%") => Emit(Hit(Z)),
            chord!("^^_v") => Emit(Hit(SEMICOLON | SHIFT_MASK)), // :
            chord!("_^%_") => Emit(Hit(KEY_0)),
            chord!("v_v_") => Emit(Hit(KEY_1)),
            chord!("%_%_") => Emit(Hit(KEY_2)),
            chord!("%%%_") => Emit(Hit(KEY_3)),
            chord!("^^^%") => Emit(Hit(KEY_4)),
            chord!("_vv%") => Emit(Hit(EQUAL)),
            chord!("%^__") => Emit(Hit(KEY_4 | SHIFT_MASK)), // $
            chord!("^^_%") => Emit(Hit(KEY_8 | SHIFT_MASK)), // *
            chord!("^_%_") => Emit(Hit(LEFT_BRACE | SHIFT_MASK)), // {
            chord!("v_%_") => Emit(Hit(RIGHT_BRACE | SHIFT_MASK)), // }
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
    fn key_up_incremental_then_decremental() {
        let mut ch = Chordite::default();
        use SwitchSet as S;
        assert_eq!(ch.handle(S(0b00_10_00_00)), Nothing);
        assert_eq!(ch.handle(S(0b00_10_00_10)), Nothing);
        assert_eq!(ch.handle(S(0b00_10_00_11)), Nothing);
        assert_eq!(ch.handle(S(0b00_10_00_01)), Nothing);
        assert_eq!(ch.handle(S(0b00_00_00_01)), Nothing);
        assert_eq!(ch.handle(S(0)), Hit(UP));
    }
}
