mod crt_effect;
mod sprite;

use crate::colors::palettes;
use crate::render::crt_effect::apply_crt_effect;
use crate::render::sprite::{UFO_SPRITE, draw_sprite};
use crate::text::{Alphabet, draw_text, font8x8_sentence};
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
            crt: true,
        };

        let mut fire = vec![0u8; WIDTH * HEIGHT];

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
                buffer[i] = palettes::FIRE[val as usize];
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

            if state.x < WIDTH && state.y < HEIGHT {
                // buffer[state.y * WIDTH + state.x] = 0xFFFFFF;
                draw_sprite(&mut buffer, &UFO_SPRITE, 8, state.x, state.y, 4);
            }

            draw_text(
                &mut buffer,
                20,
                20,
                font8x8_sentence!("HELLO VGA WORLD"),
                0xFFFFFF,
                2,
                true,
            );
            draw_text(
                &mut buffer,
                20,
                40,
                font8x8_sentence!(
                    "Happy "
                    Alphabet::StandardGalactic: "beep boop robot"
                    " y'all"
                ),
                0xFFFFFF,
                1,
                false,
            );

            if state.crt {
                apply_crt_effect(&mut buffer, frame / 10);
            }

            ready_tx.send(buffer).unwrap();
            thread::sleep(FRAME_DURATION);
            frame += 1;
        }
    });
}
