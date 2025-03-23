//! This module defines the [`StaticAlphabet`] type, which represents a statically-known and sized set of glyphs,
//! and provides utilities for working with alphabets and fonts, including the [`Alphabet`] enum.

use crate::text::font8x8::{BasicAlphabet, StandardGalacticAlphabet};
use crate::text::glyphs::GlyphIndex;
use std::ops::Index;

/// A set of alphabets / fonts to choose from.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
pub enum Alphabet {
    Basic = 0,
    StandardGalactic = 1,
}

impl Alphabet {
    /// Look up a glyph.
    pub const fn get(&self, index: GlyphIndex) -> Option<&[u8; 8]> {
        match self {
            Self::Basic => BasicAlphabet.get(index),
            Self::StandardGalactic => StandardGalacticAlphabet.get(index),
        }
    }

    /// Look up a glyph index or panic.
    pub const fn map_char_index_or_panic(&self, ch: char) -> GlyphIndex {
        match self {
            Alphabet::Basic => BasicAlphabet.map_char_index_or_panic(ch),
            Alphabet::StandardGalactic => StandardGalacticAlphabet.map_char_index_or_panic(ch),
        }
    }
}

/// A statically-known and sized alphabet, i.e. set of glyphs.
pub struct StaticAlphabet<const B: usize, const N: usize>([[u8; B]; N]);

impl<const B: usize, const N: usize> Index<usize> for StaticAlphabet<B, N> {
    type Output = [u8; B];

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl<const B: usize, const N: usize> Index<GlyphIndex> for StaticAlphabet<B, N> {
    type Output = [u8; B];

    #[inline(always)]
    fn index(&self, index: GlyphIndex) -> &Self::Output {
        self.0.index(index.into_inner() as usize)
    }
}

impl<const B: usize, const N: usize> StaticAlphabet<B, N> {
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
