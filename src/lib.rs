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
        UsbOutcome::Nothing
    }

    const_map!(
        LAYOUT0, lookup0(),
        (u8 => LayerOutcome) {
            0b00_10_00_11 => Emit(Hit(UP)),
        }
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero() {
        let mut ch = Chordite::default();
        assert_eq!(ch.handle(SwitchSet(0)), UsbOutcome::Nothing);
    }
}
