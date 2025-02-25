#![cfg_attr(not(test), no_std)]

pub mod keycodes;
use keycodes::*;

#[derive(Default)]
/// Currently, the most significant bit is the pinky finger's tip switch,
/// then pinky finger's base switch. Subsequent bits represent tip & base
/// of ring finger, middle finger, and index finger.
///
/// E.g.: `0b10_00_00_01` is: pinky tip + index base pressed.
pub struct SwitchSet(u8);

#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum UsbOutcome {
    Nothing,
    KeyHit(KeyWithModifiers),
}

pub enum LayerOutcome {
    Emit(UsbOutcome),
}

#[derive(Default)]
pub struct Chordite {
    most: SwitchSet,
}

// type S = SwitchSet;
// type L = LayerOutcome;
// type U = UsbOutcome;

impl Chordite {
    pub fn handle(&mut self, switches: SwitchSet) -> UsbOutcome {
        UsbOutcome::Nothing
    }

    // type S = SwitchSet;
    // type L = LayerOutcome<KeyWithModifiers>;
    // type U = UsbOutcome<KeyWithModifiers>;
    // use L::Emit as E;
    // use U::KeyHit as K;

    const_map!(
        Layout0, lookup0,
        (SwitchSet => LayerOutcome<KeyWithModifiers>) {
            S(0b00_10_00_11), 
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

