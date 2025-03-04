// chordite-rust is (a part of) firmware for chorded keyboards
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
use chordite_macros::chord;
use chordite_chords::{
    LayerOutcome::{self, *},
    UsbOutcome::KeyHit as Hit,
    keycodes::{self, *},
};

pub struct Layout {}

impl chordite_chords::Lookup for Layout {
    type KeyWithFlags = keycodes::KeyWithFlags;

    fn lookup(layer: i32, chord: u8) -> Option<LayerOutcome<Self::KeyWithFlags>> {
        match layer {
            1 => Self::lookup1(chord), // "SHIFT"
            2 => Self::lookup2(chord), // "Nav / Fn"
            _ => Self::lookup0(chord),
        }
    }
}

impl Layout {
    const_map!(
        LAYOUT0, lookup0(),
        (u8 => LayerOutcome<KeyWithFlags>) {
            chord!("__^_") => Emit(Hit(SPACE)),
            chord!("_^__") => Emit(Hit(BACKSPACE)),
            chord!("___^") => Emit(Hit(E)),
            chord!("___v") => Emit(Hit(T)),
            chord!("^___") => Emit(Hit(A)),
            chord!("v___") => Emit(Hit(O)),
            chord!("__v_") => Emit(Hit(I)),
            chord!("_v__") => Emit(Hit(N)),
            chord!("%___") => Emit(Hit(S)),
            chord!("_%__") => Emit(Hit(H)),
            chord!("_^^_") => Emit(Hit(R)),
            chord!("__^v") => Emit(Hit(D)),
            chord!("___%") => Emit(Hit(L)),
            chord!("__^%") => Emit(Hit(U)),
            chord!("_v_v") => Emit(Hit(C)),
            chord!("_^_v") => Emit(Hit(M)),
            chord!("_^^^") => Emit(Hit(W)),
            chord!("_^_%") => Emit(Hit(F)),
            chord!("^__^") => Emit(Hit(G)),
            chord!("v__v") => Emit(Hit(Y)),
            chord!("__^^") => Emit(Hit(P)),
            chord!("__vv") => Emit(Hit(B)),
            chord!("_^_^") => Emit(Hit(V)),
            chord!("^_^_") => Emit(Hit(K)),
            chord!("v_v_") => Emit(Hit(J)),
            chord!("_vv_") => Emit(Hit(X)),
            chord!("v__%") => Emit(Hit(Z)),
            chord!("__%_") => Emit(Hit(Q)),

            chord!("%__%") => TemporaryLayerSwitch { layer: 1 }, // SHIFT
            chord!("%_%_") => TemporaryPlusMask { mask: CTRL_FLAG }, // CTRL
            chord!("%%%_") => TemporaryPlusMask { mask: ALT_FLAG }, // ALT
            chord!("_%%%") => TemporaryPlusMask { mask: RIGHT_ALT_FLAG }, // R-ALT
            chord!("%%_%") => TemporaryPlusMask { mask: GUI_FLAG }, // GUI
            chord!("%_%%") => TemporaryPlusMask { mask: RIGHT_GUI_FLAG }, // R_GUI

            chord!("_%_%") => Emit(Hit(ENTER)),
            chord!("_%%_") => Emit(Hit(ESC)),
            chord!("_v_%") => Emit(Hit(TAB)),
            chord!("^^__") => Emit(Hit(DELETE)),
            chord!("v%%_") => Emit(Hit(INSERT)),

            chord!("__%%") => Emit(Hit(HOME)),
            chord!("%%__") => Emit(Hit(END)),
            chord!("__%^") => Emit(Hit(PAGE_UP)),
            chord!("__%v") => Emit(Hit(PAGE_DOWN)),

            chord!("^^^_") => Emit(Hit(PERIOD)), // .
            chord!("^^^^") => Emit(Hit(COMMA)), // ,
            chord!("^^_^") => Emit(Hit(SEMICOLON)), // ;
            chord!("vv_v") => Emit(Hit(SEMICOLON | SHIFT_FLAG)), // :
            chord!("^__%") => Emit(Hit(KEY_1 | SHIFT_FLAG)), // !
            chord!("^^_%") => Emit(Hit(SLASH | SHIFT_FLAG)), // ?

            chord!("^__v") => Emit(Hit(QUOTE)), // '
            chord!("v__^") => Emit(Hit(QUOTE | SHIFT_FLAG)), // "
            chord!("v_^_") => Emit(Hit(TILDE)), // `
            chord!("vv__") => Emit(Hit(SLASH)), // /
            chord!("vvv_") => Emit(Hit(BACKSLASH)), // \
            chord!("_%%v") => Emit(Hit(KEY_7 | SHIFT_FLAG)), // &
            chord!("_vvv") => Emit(Hit(KEY_8 | SHIFT_FLAG)), // *
            chord!("_^^v") => Emit(Hit(EQUAL)), // =
            chord!("_^^%") => Emit(Hit(EQUAL | SHIFT_FLAG)), // +
            chord!("%^^_") => Emit(Hit(MINUS)), // -
            chord!("%^^^") => Emit(Hit(MINUS | SHIFT_FLAG)), // _
            chord!("_v^_") => Emit(Hit(TILDE | SHIFT_FLAG)), // ~
            chord!("v^__") => Emit(Hit(KEY_4 | SHIFT_FLAG)), // $
            chord!("^^^%") => Emit(Hit(KEY_6 | SHIFT_FLAG)), // ^
            chord!("%_%v") => Emit(Hit(KEY_5 | SHIFT_FLAG)), // %
            chord!("vvvv") => Emit(Hit(BACKSLASH | SHIFT_FLAG)), // |
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
            chord!("%%%%") => ClearState,
            chord!("%_^^") => Emit(Hit(CAPS_LOCK)),

            // chord!("%%v_") => Emit(Hit(INSERT -- already defined)),

        }
    );

    // "SHIFT" layer
    const_map!(
        LAYOUT1, lookup1(),
        (u8 => LayerOutcome<KeyWithFlags>) {
            0 => FromOtherPlusMask { layer: 0, mask: SHIFT_FLAG },
        }
    );

    // "Nav / Fn" layer
    const_map!(
        LAYOUT2, lookup2(),
        (u8 => LayerOutcome<KeyWithFlags>) {
            0 => FromOtherPlusMask { layer: 0, mask: 0 }, // fallback

            chord!("%%_v") => ClearState, // quit to base layer
            chord!("v_^v") => ClearState, // quit to base layer
            chord!("%_^^") => TogglePlusMask { mask: ALT_FLAG }, // Fn-CAPSLOCK => sticky ALT

            chord!("_%__") => Emit(Hit(LEFT)), // Fn-H KEY_LEFT
            chord!("___%") => Emit(Hit(RIGHT)), // Fn-L KEY_RIGHT
            chord!("v_v_") => Emit(Hit(DOWN)), // Fn-J KEY_DOWN
            chord!("^_^_") => Emit(Hit(UP)), // Fn-K KEY_UP

            chord!("__^_") => Emit(Hit(RIGHT)), // Fn-Space KEY_RIGHT
            chord!("_^__") => Emit(Hit(LEFT)), // Fn-Bksp KEY_LEFT
            chord!("___^") => Emit(Hit(UP)), // Fn-E KEY_UP
            chord!("___v") => Emit(Hit(DOWN)), // Fn-T KEY_DOWN
            chord!("^___") => Emit(Hit(DOWN)), // Fn-A KEY_DOWN

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
        }
    );
}
