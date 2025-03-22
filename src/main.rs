mod colors;
mod font8x8;

use crate::colors::COLORS;
use crate::font8x8::{FONT8X8_BASIC, FONT8X8_STANDARD_GALACTIC};
use minifb::{Key, Window, WindowOptions};
use std::thread::sleep;
use std::time::{Duration, Instant};

const WIDTH: usize = 320;
const HEIGHT: usize = 200;
const FRAME_DURATION: Duration = Duration::from_millis(20); // 50 FPS
const LUT_SIZE: usize = 256;

fn make_sin_lut() -> [u8; LUT_SIZE] {
    let mut lut = [0u8; LUT_SIZE];
    for i in 0..LUT_SIZE {
        let angle = i as f32 * std::f32::consts::TAU / LUT_SIZE as f32;
        lut[i] = ((f32::sin(angle) * 127.5) + 127.5) as u8;
    }
    lut
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

    let sin_lut = make_sin_lut();
    let mut buffer = vec![0u32; WIDTH * HEIGHT];
    let start = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let frame_start = Instant::now();

        let time = (start.elapsed().as_secs_f32() * 60.0) as usize;
        let palette_shift = time % 256;

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let a = sin_lut[((x << 3).wrapping_add(time)) & 0xFF] as usize;
                let b = sin_lut[((y << 2).wrapping_add(time * 2)) & 0xFF] as usize;
                let c = sin_lut[(((x + y) << 2).wrapping_add(time * 3)) & 0xFF] as usize;
                let dist = (((x * x + y * y) as f32).sqrt() as usize).wrapping_add(time * 4);
                let d = sin_lut[(dist << 1) & 0xFF] as usize;

                let v = ((a + b + c + d) >> 2) & 0xFF;
                let shifted_index = (v + palette_shift) & 0xFF;

                buffer[y * WIDTH + x] = COLORS[shifted_index];
            }
        }

        draw_text(&mut buffer, 20, 20, "HELLO VGA WORLD", 0xFFFFFF);
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

        // Frame limiter
        let frame_elapsed = frame_start.elapsed();
        if frame_elapsed < FRAME_DURATION {
            sleep(FRAME_DURATION - frame_elapsed);
        }
    }
}
