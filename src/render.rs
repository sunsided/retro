mod crt_effect;
mod plasma;
mod sprite;

use crate::colors::palettes;
use crate::logic::GameState;
use crate::logic::scene_background::SceneBackground;
use crate::render::crt_effect::apply_crt_effect;
use crate::render::plasma::Plasma;
use crate::render::sprite::{UFO_SPRITE, draw_sprite};
use crate::text::{Alphabet, draw_text, font8x8_sentence};
use crate::{FRAME_DURATION, FrameBuffer, HEIGHT, WIDTH};
use crossbeam::channel;
use rand::Rng;
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

struct Fire {
    heat: Vec<u8>,
    rng: rand::rngs::ThreadRng,
}

impl Fire {
    fn new() -> Self {
        Self {
            heat: vec![0u8; WIDTH * HEIGHT],
            rng: rand::thread_rng(),
        }
    }

    fn update(&mut self) {
        for x in 0..WIDTH {
            self.heat[(HEIGHT - 1) * WIDTH + x] = if self.rng.gen_range(0..10) > 1 {
                255
            } else {
                0
            };
        }
        for y in 1..HEIGHT {
            for x in 0..WIDTH {
                let src = y * WIDTH + x;
                let below = src + WIDTH;
                let decay = self.rng.gen_range(1..20);
                let new_val = self
                    .heat
                    .get(below)
                    .copied()
                    .unwrap_or(0)
                    .saturating_sub(decay);
                let dx = self.rng.gen_range(0..3);
                let dst_x = x.wrapping_add(dx).wrapping_sub(1).min(WIDTH - 1);
                let dst_y = y - 1;
                let dst = dst_y * WIDTH + dst_x;
                self.heat[dst] = new_val;
            }
        }
    }

    fn render(&self, buffer: &mut FrameBuffer) {
        for (i, &val) in self.heat.iter().enumerate() {
            buffer[i] = palettes::FIRE[val as usize];
        }
    }
}

pub fn start_render_thread(
    rx_state: channel::Receiver<GameState>,
    ready_tx: channel::Sender<Box<FrameBuffer>>,
    free_rx: channel::Receiver<Box<FrameBuffer>>,
) {
    thread::spawn(move || {
        let start = Instant::now();
        let mut state = GameState::default();

        let mut plasma = Plasma::new();
        let mut fire = Fire::new();

        let mut frame = 0;
        loop {
            let mut buffer = free_rx.recv().unwrap();

            while let Ok(new_state) = rx_state.try_recv() {
                state = new_state;
            }

            match state.background {
                SceneBackground::None => {
                    for pixel in buffer.iter_mut() {
                        *pixel = 0x000000;
                    }
                }
                SceneBackground::Plasma(ref config) => {
                    plasma.update(config.intensity);
                    plasma.render(&mut buffer);
                }
                SceneBackground::Fire => {
                    fire.update();
                    fire.render(&mut buffer);
                }
            }

            if state.x < WIDTH && state.y < HEIGHT {
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
