use const_map::const_map;

use macros::chord;

use crate::LayerOutcome::{self, *};
use crate::UsbOutcome::KeyHit as Hit;
use crate::keycodes::{self, *};

pub struct SampleLayers {}

impl super::Lookup for SampleLayers {
    type KeyWithFlags = keycodes::KeyWithFlags;

    fn lookup(layer: i32, chord: u8) -> Option<LayerOutcome<Self::KeyWithFlags>> {
        match layer {
            1 => Self::lookup1(chord), // "SHIFT"
            _ => Self::lookup0(chord),
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
}
