use crate::font8x8::FONT8X8_BASIC;
use crate::{FrameBuffer, HEIGHT, WIDTH};

pub fn draw_char(
    buffer: &mut FrameBuffer,
    x: usize,
    y: usize,
    ch: u8,
    color: u32,
    scale: usize,
    italic: bool,
) {
    if let Some(rows) = FONT8X8_BASIC.get(ch as usize) {
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

pub fn draw_text(
    buffer: &mut FrameBuffer,
    x: usize,
    y: usize,
    text: &str,
    color: u32,
    scale: usize,
    italic: bool,
) {
    for (i, ch) in text.bytes().enumerate() {
        draw_char(buffer, x + i * 8 * scale, y, ch, color, scale, italic);
    }
}
