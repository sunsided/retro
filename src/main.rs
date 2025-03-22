mod font8x8;

use crate::font8x8::{FONT8X8_BASIC, FONT8X8_STANDARD_GALACTIC};
use minifb::{Key, Window, WindowOptions};
use std::time::Instant;

const WIDTH: usize = 320;
const HEIGHT: usize = 200;

// Simple fake 256-color palette: RGB 3-3-2 bit (like mode 13h)
fn generate_palette() -> Vec<u32> {
    (0..=255)
        .map(|i| {
            let r = ((i >> 5) & 0b111) * 36; // 3 bits
            let g = ((i >> 2) & 0b111) * 36;
            let b = (i & 0b11) * 85; // 2 bits
            (r << 16) | (g << 8) | b
        })
        .collect()
}

fn draw_char(buffer: &mut [u32], x: usize, y: usize, ch: u8, color: u32) {
    if let Some(rows) = FONT8X8_BASIC.get(ch as usize) {
        for (dy, byte) in rows.iter().enumerate() {
            for dx in 0..8 {
                if (byte >> (7 - dx)) & 1 != 0 {
                    let px = x + dx;
                    let py = y + dy;
                    if px < WIDTH && py < HEIGHT {
                        buffer[py * WIDTH + px] = color;
                    }
                }
            }
        }
    }
}

fn draw_text(buffer: &mut [u32], x: usize, y: usize, text: &str, color: u32) {
    for (i, ch) in text.bytes().enumerate() {
        draw_char(buffer, x + i * 8, y, ch, color);
    }
}

struct Sprite {
    width: usize,
    height: usize,
    pixels: Vec<u8>, // Index into a palette or 0x00 = transparent
}

fn make_sprite() -> Sprite {
    Sprite {
        width: 8,
        height: 8,
        #[rustfmt::skip]
        pixels: vec![
            1, 1, 1, 1, 1, 1, 1, 1,
            1, 0, 0, 0, 0, 0, 0, 1,
            1, 0, 0, 0, 0, 0, 0, 1,
            1, 0, 0, 0, 0, 0, 0, 1,
            1, 0, 0, 0, 0, 0, 0, 1,
            1, 0, 0, 0, 0, 0, 0, 1,
            1, 0, 0, 0, 0, 0, 0, 1,
            1, 1, 1, 1, 1, 1, 1, 1,
        ],
    }
}

fn draw_sprite(buffer: &mut [u32], sprite: &Sprite, x: usize, y: usize, palette: &[u32]) {
    for sy in 0..sprite.height {
        for sx in 0..sprite.width {
            let color_index = sprite.pixels[sy * sprite.width + sx];
            if color_index == 0 {
                continue;
            } // 0 = transparent
            let px = x + sx;
            let py = y + sy;
            if px < WIDTH && py < HEIGHT {
                buffer[py * WIDTH + px] = palette[color_index as usize];
            }
        }
    }
}

fn main() {
    let mut window = Window::new(
        "Plasma Animation - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: false,
            scale: minifb::Scale::X2,
            ..WindowOptions::default()
        },
    )
    .unwrap();

    let palette = generate_palette();
    let mut buffer = vec![0u32; WIDTH * HEIGHT];
    let start = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let t = start.elapsed().as_secs_f32() * 2.0;

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let v = ((128.0 + 128.0 * f32::sin(x as f32 / 16.0 + t))
                    + (128.0 + 128.0 * f32::sin(y as f32 / 8.0 + t))
                    + (128.0 + 128.0 * f32::sin((x as f32 + y as f32) / 16.0 + t))
                    + (128.0 + 128.0 * f32::sin(((x as f32).hypot(y as f32)) / 8.0 - t)))
                    / 4.0;

                buffer[y * WIDTH + x] = palette[v as usize % 256];
            }
        }

        draw_text(&mut buffer, 20, 20, "HELLO VGA WORLD", 0xFFFFFF);
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
