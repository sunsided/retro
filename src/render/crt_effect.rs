use crate::{FrameBuffer, HEIGHT, WIDTH};

pub fn apply_crt_effect(pixels: &mut FrameBuffer, frame: usize) {
    use rand::Rng;
    let mut rng = rand::rng();
    let wobble_offset = ((frame as f32 * 0.3).sin() * rng.random_range(1.5..3.0)) as usize; // horizontal wobble
    let roll_line = (frame * 2 + rng.random_range(0..10)) % HEIGHT; // rolling distortion line
    let analog_glitch = rng.random_bool(0.02);

    for y in 0..HEIGHT {
        let brightness = if y % 2 == 0 { 255 } else { 220 };
        for x in 0..WIDTH {
            let idx = y * WIDTH + x;
            let px = pixels[idx];
            let noise = if analog_glitch && rng.random_bool(0.2) {
                rng.random_range(30..100)
            } else {
                rng.random_range(0..=10)
            };

            let r = (((px >> 16) & 0xFF).saturating_add(noise)) * brightness / 255;
            let g = (((px >> 8) & 0xFF).saturating_add(noise)) * brightness / 255;
            let b = ((px & 0xFF).saturating_add(noise)) * brightness / 255;

            pixels[idx] = (r << 16) | (g << 8) | b;
        }
    }

    // RGB shadow mask simulation
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let idx = y * WIDTH + x;
            let px = pixels[idx];
            let r = (px >> 16) & 0xFF;
            let g = (px >> 8) & 0xFF;
            let b = px & 0xFF;

            let modulated = match x % 3 {
                0 => (r / 2) << 16,
                1 => (g / 2) << 8,
                _ => b / 2,
            };

            pixels[idx] = px & (0xFF_FF_FF - 0x7F_7F_7F + modulated);
        }
    }

    // Chromatic aberration
    for y in 0..HEIGHT {
        for x in 1..(WIDTH - 1) {
            let idx = y * WIDTH + x;
            let prev = pixels[idx - 1];
            let next = pixels[idx + 1];

            let r = (next >> 16) & 0xFF;
            let g = (pixels[idx] >> 8) & 0xFF;
            let b = prev & 0xFF;

            pixels[idx] = (r << 16) | (g << 8) | b;
        }
    }

    // Vignette
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let idx = y * WIDTH + x;
            let xf = (x as f32 - WIDTH as f32 / 2.0) / (WIDTH as f32 / 2.0);
            let yf = (y as f32 - HEIGHT as f32 / 2.0) / (HEIGHT as f32 / 2.0);
            let vignette = 1.0 - (xf * xf + yf * yf) * 0.2;
            let vignette = vignette.clamp(0.0, 1.0);

            let px = pixels[idx];
            let r = (((px >> 16) & 0xFF) as f32 * vignette) as u8;
            let g = (((px >> 8) & 0xFF) as f32 * vignette) as u8;
            let b = ((px & 0xFF) as f32 * vignette) as u8;

            pixels[idx] = ((r as u32) << 16) | ((g as u32) << 8) | b as u32;
        }
    }

    // Horizontal wobble + rolling distortion
    let temp = pixels.to_vec();
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let wobble = if y == roll_line {
                rng.random_range(2..6)
            } else {
                wobble_offset
            };
            let src_x = (x + wobble).clamp(0, WIDTH - 1);
            let src_idx = y * WIDTH + src_x;
            let dst_idx = y * WIDTH + x;
            pixels[dst_idx] = temp[src_idx];
        }
    }
}
