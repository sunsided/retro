use crate::colors::{COLORS, make_fire_palette};
use crate::text::draw_text;
use crate::{FRAME_DURATION, FrameBuffer, GameState, HEIGHT, LUT_SIZE, WIDTH};
use crossbeam::channel;
use rand::{Rng, rng};
use std::thread;
use std::time::Instant;

pub const fn dim_color(color: u32, factor: f32) -> u32 {
    let r = ((color >> 16) & 0xFF) as f32 * factor;
    let g = ((color >> 8) & 0xFF) as f32 * factor;
    let b = (color & 0xFF) as f32 * factor;

    let r = r.min(255.0).max(0.0) as u32;
    let g = g.min(255.0).max(0.0) as u32;
    let b = b.min(255.0).max(0.0) as u32;

    (color & 0xFF000000) | (r << 16) | (g << 8) | b
}

fn apply_crt_effect(pixels: &mut FrameBuffer, frame: usize) {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let wobble_offset = ((frame as f32 * 0.3).sin() * 2.0) as isize; // horizontal wobble
    let roll_line = frame % HEIGHT; // rolling distortion line

    for y in 0..HEIGHT {
        let brightness = if y % 2 == 0 { 255 } else { 220 }; // scanlines
        for x in 0..WIDTH {
            let idx = y * WIDTH + x;
            let mut px = pixels[idx];
            let noise = rng.gen_range(0..=10);

            let r = (((px >> 16) & 0xFF).saturating_add(noise)) * brightness / 255;
            let g = (((px >> 8) & 0xFF).saturating_add(noise)) * brightness / 255;
            let b = ((px & 0xFF).saturating_add(noise)) * brightness / 255;

            pixels[idx] = (r << 16) | (g << 8) | b;
        }
    }

    // Chromatic aberration
    for y in 0..HEIGHT {
        for x in 1..(WIDTH - 1) {
            let idx = y * WIDTH + x;
            let prev = pixels[idx - 1];
            let next = pixels[idx + 1];

            let r = ((next >> 16) & 0xFF);
            let g = ((pixels[idx] >> 8) & 0xFF);
            let b = ((prev) & 0xFF);

            pixels[idx] = (r << 16) | (g << 8) | b;
        }
    }

    // Curvature/vignette
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

    // Horizontal wobble and rolling distortion
    let mut temp = pixels.to_vec();
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let wobble = if y == roll_line { 4 } else { wobble_offset }; // stronger wobble on roll line
            let src_x = (x as isize + wobble).clamp(0, (WIDTH - 1) as isize) as usize;
            let src_idx = y * WIDTH + src_x;
            let dst_idx = y * WIDTH + x;
            pixels[dst_idx] = temp[src_idx];
        }
    }
}

pub fn start_render_thread(
    sin_lut: [u8; LUT_SIZE],
    rx_state: channel::Receiver<GameState>,
    ready_tx: channel::Sender<Box<FrameBuffer>>,
    free_rx: channel::Receiver<Box<FrameBuffer>>,
) {
    thread::spawn(move || {
        let start = Instant::now();
        let mut state = GameState {
            tick: 0,
            x: 0,
            y: 0,
            plasma: 1.0,
        };

        let mut fire = vec![0u8; WIDTH * HEIGHT];
        let fire_palette = make_fire_palette();

        let mut frame = 0;
        loop {
            let mut buffer = free_rx.recv().unwrap();

            while let Ok(new_state) = rx_state.try_recv() {
                state = new_state;
            }

            let t = (start.elapsed().as_secs_f32() * 60.0) as usize;
            let palette_shift = state.tick as usize % 256;

            // Add random heat at the bottom row
            for x in 0..WIDTH {
                fire[(HEIGHT - 1) * WIDTH + x] = if rng().random_range(0..10) > 1 {
                    255
                } else {
                    0
                };
            }

            // Propagate fire upward
            for y in 1..HEIGHT {
                for x in 0..WIDTH {
                    let src = y * WIDTH + x;
                    let below = src + WIDTH;
                    let decay = rng().random_range(1..20);
                    let new_val = fire.get(below).copied().unwrap_or(0).saturating_sub(decay);

                    let dx = rng().random_range(0..3); // 0 = left, 1 = center, 2 = right
                    let dst_x = x.wrapping_add(dx).wrapping_sub(1).min(WIDTH - 1);
                    let dst_y = y - 1;
                    let dst = dst_y * WIDTH + dst_x;
                    fire[dst] = new_val;
                }
            }

            // Convert indices to colors
            for (i, &val) in fire.iter().enumerate() {
                buffer[i] = fire_palette[val as usize];
            }

            /*
            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    let a = sin_lut[((x << 3).wrapping_add(t * 1)) & 0xFF] as usize;
                    let b = sin_lut[((y << 2).wrapping_add(t * 2)) & 0xFF] as usize;
                    let c = sin_lut[(((x + y) << 2).wrapping_add(t * 3)) & 0xFF] as usize;
                    let dist = (((x * x + y * y) as f32).sqrt() as usize).wrapping_add(t * 4);
                    let d = sin_lut[(dist << 1) & 0xFF] as usize;

                    let v = ((a + b + c + d) >> 2) & 0xFF;
                    let shifted_index = (v + palette_shift) & 0xFF;

                    buffer[y * WIDTH + x] = dim_color(COLORS[shifted_index], state.plasma);
                }
            }
            */

            draw_text(&mut buffer, 20, 20, "HELLO VGA WORLD", 0xFFFFFF);

            if state.x < WIDTH && state.y < HEIGHT {
                buffer[state.y * WIDTH + state.x] = 0xFFFFFF;
            }

            apply_crt_effect(&mut buffer, frame);

            ready_tx.send(buffer).unwrap();
            thread::sleep(FRAME_DURATION);
            frame += 1;
        }
    });
}
