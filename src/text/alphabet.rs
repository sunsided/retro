//! Provides the [`Alphabet`] type and various fonts.

use crate::text::font8x8::{FONT8X8_BASIC, FONT8X8_STANDARD_GALACTIC};
use crate::text::glyphs::GlyphIndex;
use std::ops::Index;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
pub enum Alphabets {
    Basic = 0,
    StandardGalactic = 1,
}

impl Alphabets {
    /// Look up a glyph.
    pub const fn get(&self, index: GlyphIndex) -> Option<&[u8; 8]> {
        match self {
            Self::Basic => FONT8X8_BASIC.get(index),
            Self::StandardGalactic => FONT8X8_STANDARD_GALACTIC.get(index),
        }
    }

    /// Look up a glyph index or panic.
    pub const fn map_char_index_or_panic(&self, ch: char) -> GlyphIndex {
        match self {
            Alphabets::Basic => FONT8X8_BASIC.map_char_index_or_panic(ch),
            Alphabets::StandardGalactic => FONT8X8_STANDARD_GALACTIC.map_char_index_or_panic(ch),
        }
    }
}

/// An alphabet.
pub struct Alphabet<const B: usize, const N: usize>([[u8; B]; N]);

impl<const B: usize, const N: usize> Index<usize> for Alphabet<B, N> {
    type Output = [u8; B];

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl<const B: usize, const N: usize> Index<GlyphIndex> for Alphabet<B, N> {
    type Output = [u8; B];

    #[inline(always)]
    fn index(&self, index: GlyphIndex) -> &Self::Output {
        self.0.index(index.into_inner() as usize)
    }
}

impl<const B: usize, const N: usize> Alphabet<B, N> {
    pub const fn new(glyphs: [[u8; B]; N]) -> Self {
        Self(glyphs)
    }

    /// Look up a glyph.
    pub const fn get(&self, index: GlyphIndex) -> Option<&[u8; B]> {
        let index = index.into_inner() as usize;
        if index >= N {
            None
        } else {
            Some(&self.0[index])
        }
    }

    #[cfg(test)]
    pub const fn as_ref(&self) -> &[[u8; B]; N] {
        &self.0
    }
}
