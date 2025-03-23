//! This module provides utilities for text rendering on a frame buffer.
//!
//! It includes functionalities for:
//! - Rendering individual characters using `draw_char`.
//! - Rendering sentences or text using `draw_text`.
//!
//! Various text properties such as scaling, italicization, and color
//! customization are supported. It also defines and re-exports
//! types for working with glyphs, alphabets, and sentences.

mod alphabet;
pub mod font8x8;
mod glyphs;
pub mod sentence;

use crate::{Color, FrameBuffer, HEIGHT, WIDTH};
pub use alphabet::Alphabet;
pub use glyphs::GlyphIndex;
pub use sentence::Sentence;
pub(crate) use sentence::font8x8_sentence;
use std::borrow::Borrow;

/// Renders a single character on the frame buffer.
///
/// # Arguments
///
/// * `buffer` - A mutable reference to the frame buffer where the character
///   will be drawn.
/// * `x` - The x-coordinate of the top-left corner where the character will
///   be rendered.
/// * `y` - The y-coordinate of the top-left corner where the character will
///   be rendered.
/// * `alphabet` - The alphabet to use for rendering the character.
/// * `glyph_index` - The index of the glyph to render within the chosen alphabet.
/// * `color` - The color of the character in the frame buffer (RGBA format).
/// * `scale` - The scaling factor to enlarge the character (each pixel in the
///   glyph will be expanded by `scale` times in both dimensions).
/// * `italic` - Whether the character should be rendered with an italic slant.
///
/// # Notes
///
/// - The character is drawn using the glyph data from the specified `alphabet`.
/// - If `italic` is set to `true`, a slant effect is applied, shifting pixels
///   based on their vertical position within the glyph.
/// - The function ensures that rendered pixels do not go out of bounds of
///   the `buffer`.
#[allow(clippy::too_many_arguments)]
pub fn draw_char(
    buffer: &mut FrameBuffer,
    x: usize,
    y: usize,
    alphabet: Alphabet,
    glyph_index: GlyphIndex,
    color: Color,
    scale: usize,
    italic: bool,
) {
    let Some(rows) = alphabet.get(glyph_index) else {
        panic!("Glyph index out of bounds");
    };

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

/// Renders a sentence or text on the frame buffer.
///
/// # Arguments
///
/// * `buffer` - A mutable reference to the frame buffer where the text
///   will be drawn.
/// * `x` - The x-coordinate of the top-left corner where the text will
///   be rendered.
/// * `y` - The y-coordinate of the top-left corner where the text will
///   be rendered.
/// * `text` - The `Sentence` reference containing the text to render.
/// * `color` - The color of the text in the frame buffer (RGBA format).
/// * `scale` - The scaling factor to enlarge the text (each pixel in the
///   glyphs will be expanded by `scale` times in both dimensions).
/// * `italic` - Whether the text should be rendered with an italic slant.
///
/// # Notes
///
/// - This function draws each glyph from the specified `Sentence` on the
///   frame buffer. Characters are spaced apart by their scaled dimensions.
/// - Characters are rendered in the order defined by the `Sentence`.
/// - If `italic` is set to `true`, a slant effect is applied as defined
///   in [`draw_char`].
/// - Ensures that rendered glyphs do not exceed the dimensions of the
///   frame buffer.
pub fn draw_text<'a, S>(
    buffer: &mut FrameBuffer,
    x: usize,
    y: usize,
    text: S,
    color: Color,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draw_text() {
        let mut buffer = [0; WIDTH * HEIGHT];
        let sentence = font8x8_sentence!("Hello");

        draw_text(&mut buffer, 10, 20, &sentence, 0xFFFFFFFF, 2, false);

        // Verify that buffer contains the expected result.
        // Add assertions to check key pixels in the frame buffer, such as:
        // - Start of the text
        // - Middle character spacing
        // - End of the text
        // In this case, you may manually inspect the frame buffer here as needed.
        // Example assertion (requires precise pixel mapping):
        // assert_eq!(buffer[calculated_index], 0xFFFFFFFF);
    }
}
