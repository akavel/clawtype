#![cfg_attr(not(test), no_std)]

use const_map::const_map;

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
            0b__00_10_00_11 => Emit(Hit(UP)),
            0b__00_01_00_11 => Emit(Hit(DOWN)),
            0b__10_00_00_11 => Emit(Hit(LEFT)),
            0b__01_00_00_11 => Emit(Hit(RIGHT)),
            0b__10_10_00_10 => Emit(Hit(PAGE_UP)),
            0b__01_01_00_01 => Emit(Hit(PAGE_DOWN)),

            // 0b__00_00_00_00 => Emit(Hit()),
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
