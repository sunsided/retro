use crate::colors::palettes;
use crate::{FrameBuffer, HEIGHT, WIDTH};

#[rustfmt::skip]
pub const UFO_SPRITE: [u8; 64] = [
        0, 0, 0, 78, 78, 0, 0, 0,    // light gray top
        0, 0, 78, 87, 87, 78, 0, 0,   // mid blue body
        0, 78, 87, 87, 87, 87, 78, 0,
        87, 87, 200, 87, 87, 200, 87, 87,
        87, 87, 87, 190, 190, 87, 87, 87,
        0, 78, 87, 87, 87, 87, 78, 0,
        0, 0, 0, 226, 226, 0, 0, 0,    // warm lights (yellow/orange)
        0, 0, 226, 0, 0, 226, 0, 0,    // spaced lights
    ];

pub fn draw_sprite(
    buffer: &mut FrameBuffer,
    sprite: &[u8],
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
                    let px = x.wrapping_add(sx * scale + dx);
                    let py = y.wrapping_add(sy * scale + dy);
                    if px < WIDTH && py < HEIGHT {
                        buffer[py * WIDTH + px] = palettes::STANDARD[color_index as usize];
                    }
                }
            }
        }
    }
}
