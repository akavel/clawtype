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

use const_map::const_map;

use clawtype_macros::chord;

use crate::LayerOutcome::{self, *};
use crate::UsbOutcome::KeyHit as Hit;
use crate::keycodes::{self, *};
use crate::{LayerInfo, SwitchSet};

pub struct SampleLayers {}

impl super::Lookup for SampleLayers {
    type KeyWithFlags = keycodes::KeyWithFlags;

    fn lookup(layer: u8, chord: u8) -> Option<LayerOutcome<Self::KeyWithFlags>> {
        let layout: &[_] = match layer {
            1 => &Self::LAYOUT1, // "SHIFT"
            2 => &Self::LAYOUT2, // "TEST"
            _ => &Self::LAYOUT0,
        };
        crate::lookup_in_slice(chord, layout).copied()
    }

    fn info(layer: u8) -> LayerInfo {
        let unchorded_mask = SwitchSet(match layer {
            2 => chord!("__^^"),
            _ => 0,
        });
        LayerInfo { unchorded_mask }
    }

    fn unchorded_key(layer: u8, switch: SwitchSet) -> Option<Self::KeyWithFlags> {
        match (layer, switch.0) {
            (2, chord!("___^")) => Some(HACK_MOUSE_LEFT_BTN),
            (2, chord!("__^_")) => Some(HACK_MOUSE_RIGHT_BTN),
            _ => None,
        }
    }
}

impl SampleLayers {
    const_map!(
        LAYOUT0, lookup0(),
        (u8 => LayerOutcome<KeyWithFlags>) {
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
            chord!("%%_^") => TemporaryPlusMask { mask: RIGHT_ALT_FLAG }, // R-ALT
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

            chord!("v^_v") => LayerSwitchAndEmit {
                layer: 2,
                emit: Hit(HACK_MOUSE_ENABLE_TOGGLE),
            },
        }
    );

    // "SHIFT" layer
    const_map!(
        LAYOUT1, lookup1(),
        (u8 => LayerOutcome<KeyWithFlags>) {
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

    //== TODO: ==
    // '%'
    // '#'
    // F1-F12
    // CapsLock
    // Insert
    // PrintScreen
    // Alt-Tab & Alt-Shift-Tab

    // TEST layer
    const_map!(
        LAYOUT2, lookup2(),
        (u8 => LayerOutcome<KeyWithFlags>) {
            // 0 => FromOtherPlusMask { layer: 0, mask: SHIFT_FLAG },
            chord!("^^__") => TemporaryPlusMask { mask: CTRL_FLAG }, // CTRL

            chord!("v^_v") => LayerSwitchAndEmit {
                layer: 0,
                emit: Hit(HACK_MOUSE_ENABLE_TOGGLE),
            },
        }
    );
}
