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

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2, TokenTree as TT2};

/// Convert string like: `"v^_%"` to byte, by translating
/// each of the 4 characters to a crumb (i.e. 2 bits) as follows:
/// - '^' -> 0b10
/// - 'v' -> 0b01
/// - '_' -> 0b00
/// - '%' -> 0b11
#[proc_macro]
pub fn chord(input: TokenStream) -> TokenStream {
    let mut input = TokenStream2::from(input).into_iter();

    let Some(tree) = input.next() else {
        panic!("required exactly 1 token, none found");
    };
    let TT2::Literal(lit) = tree else {
        panic!("required a string, found: {:?}", tree);
    };
    let s = lit.to_string();
    let s = s.trim_matches('"');
    let bits = match s.as_bytes() {
        [a, b, c, d] => b2c(*a, 6) | b2c(*b, 4) | b2c(*c, 2) | b2c(*d, 0),
        _ => panic!("wanted 4 chars in string, got: {:?}", s),
    };

    let tree: TT2 = proc_macro2::Literal::u8_suffixed(bits).into();
    let output: TokenStream2 = Some(tree).into_iter().collect();
    TokenStream::from(output)
}

fn b2c(b: u8, shift: usize) -> u8 {
    let c = match b {
        b'^' => 0b10u8,
        b'v' => 0b01u8,
        b'_' | b'.' => 0b00u8,
        b'%' => 0b11u8,
        _ => panic!("unknown crumb pattern: {b:?}"),
    };
    c << shift
}
