/// The index of a glyph within an alphabet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct GlyphIndex(u8);

impl GlyphIndex {
    pub const fn new(index: u8) -> Self {
        Self(index)
    }

    pub const fn into_inner(self) -> u8 {
        self.0
    }
}

impl PartialEq<u8> for GlyphIndex {
    fn eq(&self, other: &u8) -> bool {
        self.0 == *other
    }
}
