//! Provides the [`Alphabet`] type and various fonts.

use std::ops::Index;

/// An alphabet.
pub struct Alphabet<const B: usize, const N: usize>([[u8; B]; N]);

impl<const B: usize, const N: usize> Index<usize> for Alphabet<B, N> {
    type Output = [u8; B];

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl<const B: usize, const N: usize> Alphabet<B, N> {
    pub const fn new(glyphs: [[u8; B]; N]) -> Self {
        Self(glyphs)
    }

    /// Look up a glyph.
    pub const fn get(&self, index: usize) -> Option<&[u8; B]> {
        if index >= N {
            None
        } else {
            Some(&self.0[index])
        }
    }
}
