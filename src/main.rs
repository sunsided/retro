use minifb::{Key, Window, WindowOptions};

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

const FONT: [[u8; 8]; 2] = [
    // Only 2 characters for demo: 'A' and 'B'
    [
        0b00111100, 0b01000010, 0b10000001, 0b11111111, 0b10000001, 0b10000001, 0b10000001,
        0b00000000,
    ],
    [
        0b11111110, 0b10000001, 0b10000001, 0b11111110, 0b10000001, 0b10000001, 0b11111110,
        0b00000000,
    ],
];

fn draw_char(buffer: &mut [u32], x: usize, y: usize, char_index: usize, color: u32) {
    for (row, bits) in FONT[char_index].iter().enumerate() {
        for col in 0..8 {
            if bits & (1 << (7 - col)) != 0 {
                let px = x + col;
                let py = y + row;
                if px < WIDTH && py < HEIGHT {
                    buffer[py * WIDTH + px] = color;
                }
            }
        }
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
        "256-Color Mode Simulation",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: false,
            scale: minifb::Scale::X2,
            ..Default::default()
        },
    )
    .unwrap();

    let sprite = make_sprite();

    let palette = generate_palette();
    let mut framebuffer = vec![0u32; WIDTH * HEIGHT];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let color_index = ((x / 4 + y / 4) % 256) as usize;
                framebuffer[y * WIDTH + x] = palette[color_index];
            }
        }

        draw_sprite(&mut framebuffer, &sprite, 100, 100, &palette);

        draw_char(&mut framebuffer, 50, 50, 0, 0xFFFFFF); // Draw 'A'
        draw_char(&mut framebuffer, 60, 50, 1, 0xFF0000); // Draw 'B'

        window
            .update_with_buffer(&framebuffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
