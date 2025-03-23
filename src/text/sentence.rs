use std::ops::Deref;

macro_rules! font8x8_sentence {
    (
        $(
            $( $alphabet:path , )? $text:literal
        )+
        $(,)?
    ) => {{
        const GLYPHS_ARR: &[$crate::text::Glyphs] = &[
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
                    $crate::text::Glyphs::new(ALPHABET, &GLYPHS)
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
        &self.1
    }
}
