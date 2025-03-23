//! Provides text rendering functionality.

mod alphabet;
pub mod font8x8;
mod glyphs;
mod sentence;

use crate::{FrameBuffer, HEIGHT, WIDTH};
pub use alphabet::Alphabet;
pub use glyphs::GlyphIndex;
pub(crate) use sentence::font8x8_sentence;
pub use sentence::{Glyphs, Sentence};
use std::borrow::Borrow;

pub fn draw_char(
    buffer: &mut FrameBuffer,
    x: usize,
    y: usize,
    alphabet: Alphabet,
    glyph_index: GlyphIndex,
    color: u32,
    scale: usize,
    italic: bool,
) {
    if let Some(rows) = alphabet.get(glyph_index) {
        for (dy, byte) in rows.iter().enumerate() {
            let slant = if italic { (7 - dy) / 2 } else { 0 };
            for dx in 0..8 {
                if (byte >> (7 - dx)) & 1 != 0 {
                    for sy in 0..scale {
                        for sx in 0..scale {
                            let px = x + (dx + slant) * scale + sx;
                            let py = y + dy * scale + sy;
                            if px < WIDTH && py < HEIGHT {
                                buffer[py * WIDTH + px] = color;
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn draw_text<'a, S>(
    buffer: &mut FrameBuffer,
    x: usize,
    y: usize,
    text: S,
    color: u32,
    scale: usize,
    italic: bool,
) where
    S: Borrow<Sentence<'a>>,
{
    let text = text.borrow();
    let mut offset = 0;
    for sentence in text.iter() {
        let alphabet = sentence.alphabet();
        for ch in sentence.bytes() {
            offset += 1;
            draw_char(
                buffer,
                x + offset * 8 * scale,
                y,
                alphabet,
                *ch,
                color,
                scale,
                italic,
            );
        }
    }
}
