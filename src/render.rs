mod crt_effect;
mod fire;
mod plasma;
mod sprite;

use crate::colors::palettes;
use crate::logic::GameState;
use crate::logic::scene_background::SceneBackground;
use crate::render::crt_effect::apply_crt_effect;
use crate::render::fire::Fire;
use crate::render::plasma::Plasma;
use crate::render::sprite::{UFO_SPRITE, draw_sprite};
use crate::text::{Alphabet, draw_text, font8x8_sentence};
use crate::{FRAME_DURATION, FrameBuffer, HEIGHT, WIDTH};
use crossbeam::channel;
use rand::Rng;
use std::thread;
use std::time::{Duration, Instant};
use tracing::{debug, info, warn};

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
    rx_state: channel::Receiver<GameState>,
    ready_tx: channel::Sender<Box<FrameBuffer>>,
    free_rx: channel::Receiver<Box<FrameBuffer>>,
) {
    thread::spawn(move || {
        let mut state = GameState::default();

        let mut plasma = Plasma::new();
        let mut fire = Fire::new();

        let mut frame = 0;
        let mut fps_counter = 0;
        let mut fps_display = 0;
        let mut last_fps_at = Instant::now();
        let mut last_frame_at = Instant::now();

        loop {
            let mut buffer = match free_rx.try_recv() {
                Ok(buffer) => buffer,
                Err(_) => {
                    warn!("Stalling: waiting for a free frame buffer...");
                    free_rx.recv().unwrap()
                }
            };

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

            let now = Instant::now();
            let elapsed_since_last_frame = now.duration_since(last_frame_at);
            if elapsed_since_last_frame <= FRAME_DURATION {
                let sleep_until_next_frame = FRAME_DURATION - elapsed_since_last_frame;
                thread::sleep(sleep_until_next_frame);
            } else {
                warn!(
                    "Warning: frame took longer than expected: {elapsed_since_last_frame:.2?} / {FRAME_DURATION:.2?} (current FPS: {fps_display})"
                );
            }
            last_frame_at = now;

            frame += 1;
            fps_counter += 1;
            if last_fps_at.elapsed().as_secs() >= 1 {
                fps_display = fps_counter;
                fps_counter = 0;
                last_fps_at = Instant::now();

                debug!("FPS: {}", fps_display);
            }
        }
    });
}
