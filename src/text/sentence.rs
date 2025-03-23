use std::ops::Deref;

macro_rules! font8x8_sentence {
    ($text:literal) => {{
        use $crate::text::Alphabets;
        $crate::text::font8x8_sentence!(Alphabets::Basic, $text)
    }};
    ($alphabet:path, $text:literal) => {{
        const GLYPHS: [$crate::text::GlyphIndex; $text.len()] = {
            let mut out = [$crate::text::GlyphIndex::new(0); $text.len()];
            let bytes = $text.as_bytes();
            let mut i = 0;
            while i < $text.len() {
                out[i] = $alphabet.map_char_index_or_panic(bytes[i] as char);
                i += 1;
            }
            out
        };
        $crate::text::Sentence::new($alphabet, &GLYPHS)
    }};
}

use crate::text::GlyphIndex;
use crate::text::alphabet::Alphabets;
pub(crate) use font8x8_sentence;

#[derive(Debug, Clone)]
pub struct Sentence<'a>(Alphabets, &'a [GlyphIndex]);

impl<'a> Sentence<'a> {
    pub const fn new(alphabet: Alphabets, glyphs: &'a [GlyphIndex]) -> Self {
        Self(alphabet, glyphs)
    }

    pub const fn alphabet(&self) -> Alphabets {
        self.0
    }

    pub fn bytes(&'a self) -> std::slice::Iter<'a, GlyphIndex> {
        self.1.iter()
    }
}

impl PartialEq<&[u8]> for Sentence<'_> {
    fn eq(&self, other: &&[u8]) -> bool {
        self.1.eq(*other)
    }
}

impl<const N: usize> PartialEq<[u8; N]> for Sentence<'_> {
    fn eq(&self, other: &[u8; N]) -> bool {
        self.1.eq(other)
    }
}

impl Deref for Sentence<'_> {
    type Target = [GlyphIndex];

    fn deref(&self) -> &Self::Target {
        &self.1
    }
}
