//! This module provides structures and utilities for working with font glyphs
//! and sentences composed of these glyphs. It includes:
//!
//! - The `Sentence` struct: Represents multiple [`Glyphs`] glued together,
//!   allowing efficient manipulation and iteration of text data.
//! - The `Glyphs` struct: Represents a slice of glyph references from the
//!   same alphabet, providing convenient access to the underlying data.
//! - The `font8x8_sentence` macro: Facilitates the creation of `Sentence`
//!   objects from literals, enabling a concise and readable syntax for
//!   embedding text data with specified alphabets.
//!
//! The main use case of this module is to manage and render text efficiently,
//! especially in graphics or text-intensive applications where control over
//! individual glyphs and alphabets is necessary.

use std::ops::Deref;

macro_rules! font8x8_sentence {
    (
        $(
            $( $alphabet:path : )? $text:literal
        )+
        $(,)?
    ) => {{
        const GLYPHS_ARR: &[$crate::text::sentence::Glyphs] = &[
            $(
                {
                    const ALPHABET: $crate::text::Alphabet = font8x8_sentence!(@resolve_alphabet $( $alphabet )?);

                    const GLYPHS: [$crate::text::GlyphIndex; $text.len()] = {
                        let mut out = [$crate::text::GlyphIndex::new(0); $text.len()];
                        let bytes = $text.as_bytes();
                        let mut i = 0;
                        while i < $text.len() {
                            let ch = bytes[i] as char;
                            out[i] = ALPHABET.map_char_index_or_panic(ch);
                            i += 1;
                        }
                        out
                    };
                    $crate::text::sentence::Glyphs::new(ALPHABET, &GLYPHS)
                }
            ),+
        ];
        $crate::text::Sentence::new(GLYPHS_ARR)
    }};

    (@resolve_alphabet) => {
        $crate::text::Alphabet::Basic
    };
    (@resolve_alphabet $alphabet:path) => {
        $alphabet
    };
}

use crate::text::GlyphIndex;
use crate::text::alphabet::Alphabet;
pub(crate) use font8x8_sentence;

/// Multiple [`Glyphs`] glued together.
#[derive(Debug, Clone)]
pub struct Sentence<'a>(&'a [Glyphs<'a>]);

impl<'a> Sentence<'a> {
    pub const fn new(glyphs: &'a [Glyphs<'a>]) -> Self {
        Self(glyphs)
    }

    pub fn iter(&self) -> std::slice::Iter<Glyphs<'a>> {
        self.0.iter()
    }
}

/// A slice of glyph references from the same alphabet.
#[derive(Debug, Clone)]
pub struct Glyphs<'a>(Alphabet, &'a [GlyphIndex]);

impl<'a> Glyphs<'a> {
    pub const fn new(alphabet: Alphabet, glyphs: &'a [GlyphIndex]) -> Self {
        Self(alphabet, glyphs)
    }

    pub const fn alphabet(&self) -> Alphabet {
        self.0
    }

    pub fn bytes(&'a self) -> std::slice::Iter<'a, GlyphIndex> {
        self.1.iter()
    }
}

impl PartialEq<&[u8]> for Glyphs<'_> {
    fn eq(&self, other: &&[u8]) -> bool {
        self.1.eq(*other)
    }
}

impl<const N: usize> PartialEq<[u8; N]> for Glyphs<'_> {
    fn eq(&self, other: &[u8; N]) -> bool {
        self.1.eq(other)
    }
}

impl Deref for Glyphs<'_> {
    type Target = [GlyphIndex];

    fn deref(&self) -> &Self::Target {
        self.1
    }
}

#[cfg(test)]
mod tests {
    use crate::text::{Alphabet, GlyphIndex, font8x8_sentence};

    #[test]
    fn test_font8x8_sentence() {
        let sentence = font8x8_sentence!(
            "Hello"
            Alphabet::Basic: "World",
        );

        let mut iter = sentence.iter();

        // Test first Glyphs: "Hello"
        let first_glyphs = iter.next().expect("Expected first glyphs");
        assert_eq!(first_glyphs.alphabet(), Alphabet::Basic);
        let hello_indices: Vec<GlyphIndex> = first_glyphs.bytes().cloned().collect();
        assert_eq!(hello_indices.len(), 5); // "Hello" has 5 characters

        // Test second Glyphs: "World"
        let second_glyphs = iter.next().expect("Expected second glyphs");
        assert_eq!(second_glyphs.alphabet(), Alphabet::Basic);
        let world_indices: Vec<GlyphIndex> = second_glyphs.bytes().cloned().collect();
        assert_eq!(world_indices.len(), 5); // "World" has 5 characters

        // Ensure there are no more Glyphs
        assert!(iter.next().is_none());
    }
}
