//! Provides various `8x8` byte fonts.

use std::io::Read;
use std::ops::{Deref, Index};

#[derive(Debug, Clone)]
pub struct Sentence<'a>(&'a [u8]);

impl<'a> Sentence<'a> {
    pub const fn new(glyphs: &'a [u8]) -> Self {
        Self(glyphs)
    }

    pub const fn as_slice(&'a self) -> &'a [u8] {
        self.0
    }

    pub fn bytes(&'a self) -> std::slice::Iter<'a, u8> {
        self.0.iter()
    }
}

impl PartialEq<&[u8]> for Sentence<'_> {
    fn eq(&self, other: &&[u8]) -> bool {
        self.0.eq(*other)
    }
}

impl<const N: usize> PartialEq<[u8; N]> for Sentence<'_> {
    fn eq(&self, other: &[u8; N]) -> bool {
        self.0.eq(other)
    }
}

impl Deref for Sentence<'_> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

macro_rules! font8x8_sentence {
    ($text:literal) => {{
        use crate::text::font8x8::FONT8X8_BASIC;
        $crate::text::font8x8_sentence!(FONT8X8_BASIC, $text)
    }};
    ($alphabet:tt, $text:literal) => {{
        const GLYPHS: [u8; $text.len()] = {
            let mut out = [0_u8; $text.len()];
            let bytes = $text.as_bytes();
            let mut i = 0;
            while i < $text.len() {
                out[i] = $alphabet.map_char_index_or_panic(bytes[i] as char);
                i += 1;
            }
            out
        };
        $crate::text::Sentence::new(&GLYPHS)
    }};
}

pub(crate) use font8x8_sentence;

/// A macro to reverse the bits of each byte in a 2D array and create a public constant.
///
/// # Arguments
///
/// * `$name` - The identifier for the resulting public constant.
/// * `$src` - The 2D array of `u8` values to have their bits reversed.
///
/// # Description
///
/// This macro generates a public constant with the specified name.
/// The constant is initialized by reversing the bits of each byte in the provided 2D array using the `reverse_all` function.
macro_rules! impl_font_8x8 {
    ($wrapper:ident, $name:ident, $src:expr) => {
        pub struct $wrapper($crate::text::alphabet::Alphabet<8, { $src.len() }>);

        #[allow(dead_code)]
        pub const $name: $wrapper = $wrapper($crate::text::Alphabet::new(
            $crate::text::font8x8::reverse_all($src),
        ));

        impl $wrapper {
            const LEN: usize = { $src.len() };

            /// Look up a glyph or panic.
            pub const fn map_char(&self, ch: char) -> Option<[u8; 8]> {
                match self.map_char_index(ch) {
                    None => None,
                    Some(index) => match self.0.get(index) {
                        None => None,
                        Some(glyph) => Some(*glyph),
                    },
                }
            }

            /// Look up a glyph index or panic.
            pub const fn map_char_index_or_panic(&self, ch: char) -> u8 {
                match self.map_char_index(ch) {
                    None => panic!("A character could not be looked up"),
                    Some(glyph) => glyph,
                }
            }

            /// Look up a glyph.
            pub const fn get(&self, index: u8) -> Option<&[u8; 8]> {
                if index as usize >= Self::LEN {
                    None
                } else {
                    self.0.get(index)
                }
            }

            /// Returns the number of glyphs in this alphabet.
            pub const fn len(&self) -> usize {
                Self::LEN
            }
        }
    };
}

mod basic;
mod block;
mod r#box;
mod ext_latin;
mod standard_galactic;

#[allow(unused_imports)]
pub use basic::FONT8X8_BASIC;

#[allow(unused_imports)]
pub use block::FONT8X8_BLOCK;

#[allow(unused_imports)]
pub use ext_latin::FONT8X8_EXT_LATIN;

#[allow(unused_imports)]
pub use r#box::FONT8X8_BOX;

#[allow(unused_imports)]
pub use standard_galactic::*;

/// Reverses the order of bits in a `u8`.
///
/// # Arguments
///
/// * `b` - An 8-bit unsigned integer.
///
/// # Returns
///
/// A `u8` value with its bits reversed.
const fn reverse_bits(mut b: u8) -> u8 {
    b = (b >> 4) | (b << 4);
    b = ((b & 0xCC) >> 2) | ((b & 0x33) << 2);
    b = ((b & 0xAA) >> 1) | ((b & 0x55) << 1);
    b
}

/// Reverses the bits of each byte in a 2D array of size `N x 8`.
///
/// # Arguments
///
/// * `src` - A 2D array of `u8` values where each byte's bits are to be reversed.
///
/// # Returns
///
/// A new 2D array of `u8` values with each byte's bits reversed.
///
/// # Examples
///
/// ```
/// const INPUT: [[u8; 8]; 2] = [[0b00000001; 8], [0b10000000; 8]];
/// const OUTPUT: [[u8; 8]; 2] = reverse_all(INPUT);
/// assert_eq!(OUTPUT, [[0b10000000; 8], [0b00000001; 8]]);
/// ```
const fn reverse_all<const N: usize>(src: [[u8; 8]; N]) -> [[u8; 8]; N] {
    let mut out = [[0u8; 8]; N];
    let mut i = 0;
    while i < N {
        let mut j = 0;
        while j < 8 {
            out[i][j] = reverse_bits(src[i][j]);
            j += 1;
        }
        i += 1;
    }
    out
}

#[cfg(test)]
mod tests {
    const SAMPLE_SOURCE: [[u8; 8]; 2] = [[0b00000001; 8], [0b10000000; 8]];
    impl_font_8x8!(ReversedFont, REVERSED_FONT, SAMPLE_SOURCE);

    impl ReversedFont {
        pub const fn map_char_index(&self, ch: char) -> Option<u8> {
            unimplemented!()
        }
    }

    #[test]
    fn test_reverse_font_macro() {
        assert_eq!(
            *REVERSED_FONT.0.as_ref(),
            [[0b10000000; 8], [0b00000001; 8]]
        );
    }
}
