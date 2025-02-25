#![cfg_attr(not(test), no_std)]

#[derive(Default)]
pub struct SwitchSet(u8);

#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum UsbOutcome {
    Nothing,
    KeyHit { teensy_keycode: u16 },
}

#[derive(Default)]
pub struct Chordite {
    most: SwitchSet,
}

impl Chordite {
    pub fn handle(&mut self, switches: SwitchSet) -> UsbOutcome {
        UsbOutcome::Nothing
    }
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

