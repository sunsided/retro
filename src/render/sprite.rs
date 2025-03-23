use crate::colors::palettes;
use crate::{FrameBuffer, HEIGHT, PaletteIndex, WIDTH};

/// A constant array representing a 2D UFO sprite using color indices.
/// Each value corresponds to a color index in the `palettes::STANDARD` palette,
/// where `0` represents transparency.
#[rustfmt::skip]
pub const UFO_SPRITE: [PaletteIndex; 8 * 8] = [
        0, 0, 0, 78, 78, 0, 0, 0,    // light gray top
        0, 0, 78, 87, 87, 78, 0, 0,   // mid blue body
        0, 78, 87, 87, 87, 87, 78, 0,
        87, 87, 200, 87, 87, 200, 87, 87,
        87, 87, 87, 190, 190, 87, 87, 87,
        0, 78, 87, 87, 87, 87, 78, 0,
        0, 0, 0, 226, 226, 0, 0, 0,    // warm lights (yellow/orange)
        0, 0, 226, 0, 0, 226, 0, 0,    // spaced lights
    ];

/// Draws a sprite onto the framebuffer with optional scaling.
///
/// # Parameters
/// - `buffer`: A mutable reference to the framebuffer where the sprite will be drawn.
/// - `sprite`: A slice containing the sprite's color indices.
/// - `sprite_width`: The width of the sprite in pixels.
/// - `x`: The X-coordinate to begin drawing the sprite on the screen.
/// - `y`: The Y-coordinate to begin drawing the sprite on the screen.
/// - `scale`: The scaling factor to apply to the sprite. E.g., a factor of 2 doubles its size.
///
/// # Behavior
/// - Pixels with a color index of `0` in the sprite are treated as transparent and skipped.
/// - Colors are mapped from the sprite's indices to the `palettes::STANDARD` palette.
/// - The function automatically handles clipping if parts of the sprite extend beyond the screen boundaries.
pub fn draw_sprite(
    buffer: &mut FrameBuffer,
    sprite: &[PaletteIndex; 8 * 8],
    sprite_width: usize,
    x: usize,
    y: usize,
    scale: usize,
) {
    let sprite_height = sprite.len() / sprite_width;
    for sy in 0..sprite_height {
        for sx in 0..sprite_width {
            let color_index = sprite[sy * sprite_width + sx];
            if color_index == 0 {
                continue; // Transparent
            }
            for dy in 0..scale {
                for dx in 0..scale {
                    let px = x.wrapping_add(sx * scale + dx); // Compute pixel X position
                    let py = y.wrapping_add(sy * scale + dy); // Compute pixel Y position
                    if px < WIDTH && py < HEIGHT {
                        buffer[py * WIDTH + px] = palettes::STANDARD[color_index as usize];
                    }
                }
            }
        }
    }
}
