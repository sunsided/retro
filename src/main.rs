use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 320;
const HEIGHT: usize = 200;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Retro Pixels - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: false,
            scale: minifb::Scale::X2,
            ..WindowOptions::default()
        },
    ).unwrap_or_else(|e| panic!("{e}"));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Draw a color gradient
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let r = (x as u32) & 0xFF;
                let g = (y as u32) & 0xFF;
                let b = ((x * y) as u32) & 0xFF;
                buffer[y * WIDTH + x] = (r << 16) | (g << 8) | b;
            }
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
