//! Provides various `8x8` byte fonts.

use std::ops::Index;

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
macro_rules! reverse_font_8x8 {
    ($wrapper:ident, $name:ident, $src:expr) => {
        pub struct $wrapper(Alphabet<8, { $src.len() }>);

        #[allow(dead_code)]
        pub const $name: $wrapper = $wrapper($crate::text::Alphabet::new(
            $crate::text::font8x8::reverse_all($src),
        ));

        impl $wrapper {
            const LEN: usize = { $src.len() };

            /// Look up a glyph.
            pub const fn get(&self, index: usize) -> Option<&[u8; 8]> {
                if index >= Self::LEN {
                    None
                } else {
                    self.0.get(index)
                }
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
    use crate::text::Alphabet;

    #[test]
    fn test_reverse_font_macro() {
        const SAMPLE_SOURCE: [[u8; 8]; 2] = [[0b00000001; 8], [0b10000000; 8]];
        reverse_font_8x8!(ReversedFont, REVERSED_FONT, SAMPLE_SOURCE);
        assert_eq!(
            *REVERSED_FONT.0.as_ref(),
            [[0b10000000; 8], [0b00000001; 8]]
        );
    }
}
