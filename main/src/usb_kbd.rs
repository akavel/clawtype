// clawtype-rust is (a part of) firmware for chorded keyboards
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

use thiserror::Error;
use usbd_hid::descriptor as hid_desc;

#[derive(Copy, Clone, Debug)]
pub struct StateReport {
    report: hid_desc::KeyboardReport,
}

impl Default for StateReport {
    fn default() -> Self {
        Self {
            report: hid_desc::KeyboardReport::default(),
        }
    }
}

impl hid_desc::generator_prelude::Serialize for StateReport {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: hid_desc::generator_prelude::Serializer,
    {
        self.report.serialize(serializer)
    }
}

impl hid_desc::AsInputReport for StateReport {}

impl hid_desc::SerializedDescriptor for StateReport {
    fn desc() -> &'static [u8] {
        hid_desc::KeyboardReport::desc()
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("key is already marked as pressed")]
    AlreadyPressed,
    #[error("key is already marked as released")]
    AlreadyReleased,
    #[error("too many keys are already marked as pressed")]
    TooManyKeysPressed,
}

impl StateReport {
    pub fn clear(&mut self) {
        self.report = hid_desc::KeyboardReport::default();
    }

    pub fn press(&mut self, key: impl Into<u8>) -> Result<(), Error> {
        let key = key.into();
        if let Some(mask) = as_modifier_mask(key) {
            if self.report.modifier & mask != 0 {
                return Err(Error::AlreadyPressed);
            }
            self.report.modifier |= mask;
            return Ok(());
        }
        let slots = &mut self.report.keycodes;
        for i in 0..slots.len() {
            if slots[i] == key {
                return Err(Error::AlreadyPressed);
            }
        }
        // Note: the loop below must not be merged with the one above,
        // unless self.release() logic is also adjusted appropriately.
        for i in 0..slots.len() {
            if slots[i] == 0 {
                slots[i] = key;
                return Ok(());
            }
        }
        Err(Error::TooManyKeysPressed)
    }

    pub fn release(&mut self, key: impl Into<u8>) -> Result<(), Error> {
        let key = key.into();
        if let Some(mask) = as_modifier_mask(key) {
            if self.report.modifier & mask == 0 {
                return Err(Error::AlreadyReleased);
            }
            self.report.modifier &= !mask;
            return Ok(());
        }
        let slots = &mut self.report.keycodes;
        for i in 0..slots.len() {
            if slots[i] == key {
                slots[i] = 0;
                return Ok(());
            }
        }
        Err(Error::AlreadyReleased)
    }
}

fn as_modifier_mask(key: u8) -> Option<u8> {
    if !matches!(key, MODIFIERS_START..=MODIFIERS_END) {
        return None;
    }
    let idx = key - MODIFIERS_START;
    Some(1u8 << idx) // TODO: saturating_shl or smth?
}

const MODIFIERS_START: u8 = 224;
const MODIFIERS_END: u8 = 231;
