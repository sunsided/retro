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
    ).unwrap();

    let palette = generate_palette();
    let mut framebuffer = vec![0u32; WIDTH * HEIGHT];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let color_index = ((x / 4 + y / 4) % 256) as usize;
                framebuffer[y * WIDTH + x] = palette[color_index];
            }
        }

        window.update_with_buffer(&framebuffer, WIDTH, HEIGHT).unwrap();
    }
}
