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

use const_map::const_map;
use clawtype_macros::chord;
use clawtype_chords::{
    LayerOutcome::{self, *},
    UsbOutcome::KeyHit as Hit,
    keycodes::{self, *},
};

pub struct Layout {}

impl clawtype_chords::Lookup for Layout {
    type KeyWithFlags = keycodes::KeyWithFlags;

    fn lookup(layer: i32, chord: u8) -> Option<LayerOutcome<Self::KeyWithFlags>> {
        clawtype_chords::lookup_in_slice(chord, match layer {
            1 => &Self::LAYOUT1, // "SHIFT"
            2 => &Self::LAYOUT2, // "Nav / Fn"
            _ => &Self::LAYOUT0,
        }).copied()
    }
}

impl Layout {
    const_map!(
        LAYOUT0, lookup0(),
        (u8 => LayerOutcome<KeyWithFlags>) {

            chord!("%%%%") => ClearState,

            // TODO: put mouse on cheatsheet
            chord!("^_^%") => Emit(Hit(HACK_MOUSE_ENABLE_TOGGLE)),
            chord!("_v_^") => Emit(Hit(HACK_MOUSE_LEFT_CLICK)),
            chord!("^_v_") => Emit(Hit(HACK_MOUSE_RIGHT_CLICK)),
            chord!("%%_^") => Emit(Hit(HACK_MOUSE_LEFT_DRAG_TOGGLE)),
            chord!("v_^^") => Emit(Hit(HACK_MOUSE_WHEEL_DOWN)),
            chord!("v^^_") => Emit(Hit(HACK_MOUSE_WHEEL_UP)),

            chord!("__^_") => Emit(Hit(RIGHT)),
            chord!("_^__") => Emit(Hit(LEFT)),
            chord!("___^") => Emit(Hit(UP)),
            chord!("___v") => Emit(Hit(DOWN)),
            chord!("^___") => Emit(Hit(SPACE)),
            chord!("v___") => Emit(Hit(BACKSPACE)),

            chord!("__v_") => Emit(Hit(E)),
            chord!("_v__") => Emit(Hit(T)),
            chord!("%___") => Emit(Hit(A)),
            chord!("_%__") => Emit(Hit(O)),
            chord!("_^^_") => Emit(Hit(I)),
            chord!("__^v") => Emit(Hit(N)),
            chord!("___%") => Emit(Hit(S)),
            chord!("__^%") => Emit(Hit(H)),
            chord!("_v_v") => Emit(Hit(R)),
            chord!("_^_v") => Emit(Hit(D)),
            chord!("_^^^") => Emit(Hit(L)),
            chord!("_^_%") => Emit(Hit(U)),
            chord!("^__^") => Emit(Hit(C)),
            chord!("v__v") => Emit(Hit(M)),
            chord!("__^^") => Emit(Hit(W)),
            chord!("__vv") => Emit(Hit(F)),
            chord!("_^_^") => Emit(Hit(G)),
            chord!("^_^_") => Emit(Hit(Y)),
            chord!("v_v_") => Emit(Hit(P)),
            chord!("_vv_") => Emit(Hit(B)),
            chord!("v__%") => Emit(Hit(V)),
            chord!("__%_") => Emit(Hit(K)),
            chord!("^__v") => Emit(Hit(J)),
            chord!("v__^") => Emit(Hit(X)),
            chord!("v_^_") => Emit(Hit(Z)),
            chord!("vv__") => Emit(Hit(Q)),

            chord!("%__%") => TemporaryLayerSwitch { layer: 1 }, // SHIFT
            chord!("%_%_") => TemporaryPlusMask { mask: CTRL_FLAG }, // CTRL
            chord!("^^__") => TemporaryPlusMask { mask: CTRL_FLAG }, // CTRL (new easier version)
            chord!("%%%_") => TemporaryPlusMask { mask: ALT_FLAG }, // ALT
            chord!("_%%%") => TemporaryPlusMask { mask: RIGHT_ALT_FLAG }, // R-ALT
            chord!("^_^v") => TemporaryPlusMask { mask: GUI_FLAG }, // GUI
            // candidate "mouse layer"
            chord!("%%_%") => TemporaryPlusMask { mask: GUI_FLAG }, // GUI (old!)
            // candidate "mouse layer"
            chord!("v%_%") => TemporaryPlusMask { mask: GUI_FLAG }, // GUI (old!)
            // candidate "mouse layer"
            chord!("vv_%") => TemporaryPlusMask { mask: GUI_FLAG }, // GUI (old!)
            chord!("%_%%") => TemporaryPlusMask { mask: RIGHT_GUI_FLAG }, // R_GUI

            chord!("_%_%") => Emit(Hit(ENTER)),
            chord!("_%%_") => Emit(Hit(ESC)),
            chord!("_v_%") => Emit(Hit(TAB)),
            // chord!("^^__") => Emit(Hit(DELETE)),
            chord!("v%%_") => Emit(Hit(INSERT)),

            chord!("%%__") => Emit(Hit(HOME)),
            chord!("__%%") => Emit(Hit(END)),
            chord!("__%^") => Emit(Hit(PAGE_UP)),
            chord!("__%v") => Emit(Hit(PAGE_DOWN)),

            chord!("^^^_") => Emit(Hit(PERIOD)), // .
            chord!("^^^^") => Emit(Hit(COMMA)), // ,
            chord!("^^_^") => Emit(Hit(SEMICOLON)), // ;
            chord!("vv_v") => Emit(Hit(SEMICOLON | SHIFT_FLAG)), // :
            chord!("^__%") => Emit(Hit(KEY_1 | SHIFT_FLAG)), // !
            chord!("^^_%") => Emit(Hit(SLASH | SHIFT_FLAG)), // ?

            chord!("vvv_") => Emit(Hit(SLASH)), // /
            chord!("_%%v") => Emit(Hit(KEY_7 | SHIFT_FLAG)), // &
            chord!("_vvv") => Emit(Hit(KEY_8 | SHIFT_FLAG)), // *
            chord!("_^^v") => Emit(Hit(EQUAL)), // =
            chord!("_^^%") => Emit(Hit(EQUAL | SHIFT_FLAG)), // +
            chord!("%%v_") => Emit(Hit(TILDE)), // `
            chord!("%^^_") => Emit(Hit(MINUS)), // -
            chord!("%^^^") => Emit(Hit(MINUS | SHIFT_FLAG)), // _
            chord!("_v^_") => Emit(Hit(QUOTE)), // '
            chord!("__v%") => Emit(Hit(QUOTE | SHIFT_FLAG)), // "
            chord!("v^__") => Emit(Hit(KEY_4 | SHIFT_FLAG)), // $
            chord!("^^^%") => Emit(Hit(KEY_6 | SHIFT_FLAG)), // ^
            chord!("%_%v") => Emit(Hit(KEY_5 | SHIFT_FLAG)), // %
            chord!("vvvv") => Emit(Hit(BACKSLASH | SHIFT_FLAG)), // |
            // candidate "mouse layer"
            chord!("v^_v") => Emit(Hit(KEY_2 | SHIFT_FLAG)), // @
            chord!("%^^%") => Emit(Hit(KEY_3 | SHIFT_FLAG)), // #

            chord!("v_vv") => Emit(Hit(KEY_9 | SHIFT_FLAG)), // (
            chord!("^_^^") => Emit(Hit(KEY_0 | SHIFT_FLAG)), // )
            chord!("_v%_") => Emit(Hit(LEFT_BRACE)), // [
            chord!("_^%_") => Emit(Hit(RIGHT_BRACE)), // ]
            chord!("v%__") => Emit(Hit(LEFT_BRACE | SHIFT_FLAG)), // {
            chord!("^%__") => Emit(Hit(RIGHT_BRACE | SHIFT_FLAG)), // }
            chord!("^^_v") => Emit(Hit(COMMA | SHIFT_FLAG)), // <
            chord!("^^^v") => Emit(Hit(PERIOD | SHIFT_FLAG)), // >

            chord!("%__v") => Emit(Hit(KEY_0)),
            chord!("%__^") => Emit(Hit(KEY_1)),
            chord!("%_v_") => Emit(Hit(KEY_2)),
            chord!("%_^_") => Emit(Hit(KEY_3)),
            chord!("%v__") => Emit(Hit(KEY_4)),
            chord!("%^__") => Emit(Hit(KEY_5)),
            chord!("_%v_") => Emit(Hit(KEY_6)),
            chord!("_%^_") => Emit(Hit(KEY_7)),
            chord!("_%_v") => Emit(Hit(KEY_8)),
            chord!("_%_^") => Emit(Hit(KEY_9)),

            chord!("%%_v") => LayerSwitch { layer: 2 }, // "Nav / Fn" layer
            chord!("v_^v") => LayerSwitch { layer: 2 }, // "Nav / Fn" layer
            chord!("%_^^") => Emit(Hit(CAPS_LOCK)),

            // chord!("%%v_") => Emit(Hit(INSERT -- already defined)),

        }
    );

    // "SHIFT" layer
    const_map!(
        LAYOUT1, lookup1(),
        (u8 => LayerOutcome<KeyWithFlags>) {
            0 => FromOtherPlusMask { layer: 0, mask: SHIFT_FLAG },

            chord!("v___") => Emit(Hit(DELETE)), // S-Bksp Del
            chord!("vvv_") => Emit(Hit(BACKSLASH)), // S-/ \
            chord!("%%v_") => Emit(Hit(TILDE | SHIFT_FLAG)), // S-` ~
            chord!("_v^_") => Emit(Hit(QUOTE | SHIFT_FLAG)), // S-' "
        }
    );

    // "Nav / Fn" layer
    const_map!(
        LAYOUT2, lookup2(),
        (u8 => LayerOutcome<KeyWithFlags>) {
            // 0 => FromOtherPlusMask { layer: 0, mask: 0 }, // fallback

            chord!("%%%%") => ClearState,

            chord!("%%_v") => ClearState, // quit to base layer
            chord!("v_^v") => ClearState, // quit to base layer
            chord!("%_^^") => TogglePlusMask { mask: ALT_FLAG }, // Fn-CAPSLOCK => sticky ALT

            // F1-F12
            chord!("%__v") => Emit(Hit(F10)),
            chord!("%__^") => Emit(Hit(F1)),
            chord!("%_v_") => Emit(Hit(F2)),
            chord!("%_^_") => Emit(Hit(F3)),
            chord!("%v__") => Emit(Hit(F4)),
            chord!("%^__") => Emit(Hit(F5)),
            chord!("_%v_") => Emit(Hit(F6)),
            chord!("_%^_") => Emit(Hit(F7)),
            chord!("_%_v") => Emit(Hit(F8)),
            chord!("_%_^") => Emit(Hit(F9)),
            chord!("__%v") => Emit(Hit(F11)),
            chord!("__%^") => Emit(Hit(F12)),

            // Keypad - for mouse navigation on Windows / Mac
            chord!("__^_") => Emit(Hit(KEYPAD_6)), // Right
            chord!("_^__") => Emit(Hit(KEYPAD_4)), // Left
            chord!("___^") => Emit(Hit(KEYPAD_8)), // Up
            chord!("___v") => Emit(Hit(KEYPAD_2)), // Down
            chord!("^___") => Emit(Hit(KEYPAD_7)), // Home / up-left
            chord!("v___") => Emit(Hit(KEYPAD_1)), // End / down-left
            chord!("_v__") => Emit(Hit(KEYPAD_3)), // PgDn / down-right
            chord!("__v_") => Emit(Hit(KEYPAD_9)), // PgUp / up-right
            chord!("__^%") => Emit(Hit(KEYPAD_5)), // 5 / click
            chord!("^^^%") => Emit(Hit(KEYPAD_0)), // 0 / press&lock
            // chord!("__v%") => Emit(Hit(KEYPAD_0)), // 0 / press&lock
            // chord!("^^^^") => Emit(Hit(KEYPAD_0)), // 0 / press&lock
            chord!("^^^_") => Emit(Hit(KEYPAD_PERIOD)), // . / drag release
            chord!("___%") => Emit(Hit(KEYPAD_SLASH)), // / / left-click
            chord!("__%_") => Emit(Hit(KEYPAD_ASTERIX)), // * / mid-click
            chord!("_%__") => Emit(Hit(KEYPAD_MINUS)), // - / right-click

            // transparent to layer 0:

            chord!("%__%") => FromOtherPlusMask { layer: 0, mask: 0 },
            chord!("%_%_") => FromOtherPlusMask { layer: 0, mask: 0 },
            chord!("%%%_") => FromOtherPlusMask { layer: 0, mask: 0 },
            chord!("_%%%") => FromOtherPlusMask { layer: 0, mask: 0 },
            chord!("%%_%") => FromOtherPlusMask { layer: 0, mask: 0 },
            chord!("v%_%") => FromOtherPlusMask { layer: 0, mask: 0 },
            chord!("vv_%") => FromOtherPlusMask { layer: 0, mask: 0 },
            chord!("%_%%") => FromOtherPlusMask { layer: 0, mask: 0 },

            chord!("_%_%") => FromOtherPlusMask { layer: 0, mask: 0 },
            chord!("_%%_") => FromOtherPlusMask { layer: 0, mask: 0 },
            chord!("_v_%") => FromOtherPlusMask { layer: 0, mask: 0 },
            chord!("^^__") => FromOtherPlusMask { layer: 0, mask: 0 },
            chord!("v%%_") => FromOtherPlusMask { layer: 0, mask: 0 },

            chord!("__%%") => FromOtherPlusMask { layer: 0, mask: 0 },
            chord!("%%__") => FromOtherPlusMask { layer: 0, mask: 0 },
            chord!("__%^") => FromOtherPlusMask { layer: 0, mask: 0 },
            chord!("__%v") => FromOtherPlusMask { layer: 0, mask: 0 },
        }
    );
}
