mod crt_effect;
mod fire;
mod plasma;
mod sprite;

use crate::colors::palettes;
use crate::logic::GameState;
use crate::logic::scene_background::SceneBackground;
pub use crate::render::crt_effect::apply_crt_effect;
pub use crate::render::fire::Fire;
pub use crate::render::plasma::Plasma;
use crate::render::sprite::{UFO_SPRITE, draw_sprite};
use crate::text::{Alphabet, draw_text, font8x8_sentence};
use crate::{FRAME_DURATION, FrameBuffer, HEIGHT, WIDTH};
use crossbeam::channel;
use rand::Rng;
use std::thread;
use std::time::{Duration, Instant};
use tracing::{debug, error, info, trace, warn};

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

        let sleeper = spin_sleep::SpinSleeper::new(0)
            .with_spin_strategy(spin_sleep::SpinStrategy::YieldThread);

        loop {
            let t0 = Instant::now();
            let mut buffer = match free_rx.try_recv() {
                Ok(buffer) => buffer,
                Err(_) => {
                    error!("Stalling: waiting for a free frame buffer...");
                    free_rx.recv().unwrap()
                }
            };

            while let Ok(new_state) = rx_state.try_recv() {
                state = new_state;
            }

            let t1 = Instant::now();
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

            let t2 = Instant::now();
            if state.x < WIDTH && state.y < HEIGHT {
                draw_sprite(&mut buffer, &UFO_SPRITE, 8, state.x, state.y, 4);
            }

            let t3 = Instant::now();
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

            let t4 = Instant::now();
            if state.crt {
                apply_crt_effect(&mut buffer, frame / 10);
            }

            let t5 = Instant::now();
            if let Err(err) = ready_tx.try_send(buffer) {
                error!(
                    "Contention detected: unable to send frame buffer to the channel. Error: {}",
                    err
                );
            }

            let now = Instant::now();
            let elapsed_since_last_frame = now.duration_since(last_frame_at);
            if elapsed_since_last_frame <= FRAME_DURATION {
                trace!(
                    "fetch buffer: {:.3} ms, background.render: {:.3} ms, sprites: {:.3} ms, text: {:.3} ms, crt: {:.3} ms: send: {:.3} ms",
                    (t1 - t0).as_secs_f64() * 1000.0,
                    (t2 - t1).as_secs_f64() * 1000.0,
                    (t3 - t2).as_secs_f64() * 1000.0,
                    (t4 - t3).as_secs_f64() * 1000.0,
                    (t5 - t4).as_secs_f64() * 1000.0,
                    (now - t5).as_secs_f64() * 1000.0,
                );

                let sleep_until_next_frame = FRAME_DURATION - elapsed_since_last_frame;
                sleeper.sleep(sleep_until_next_frame);
            } else {
                warn!(
                    "Warning: frame took longer than expected: {elapsed_since_last_frame:.2?} / {FRAME_DURATION:.2?} (current FPS: {fps_display})"
                );

                warn!(
                    "fetch buffer: {:.3} ms, background.render: {:.3} ms, sprites: {:.3} ms, text: {:.3} ms, crt: {:.3} ms: send: {:.3} ms",
                    (t1 - t0).as_secs_f64() * 1000.0,
                    (t2 - t1).as_secs_f64() * 1000.0,
                    (t3 - t2).as_secs_f64() * 1000.0,
                    (t4 - t3).as_secs_f64() * 1000.0,
                    (t5 - t4).as_secs_f64() * 1000.0,
                    (now - t5).as_secs_f64() * 1000.0,
                );
            }

            frame += 1;
            fps_counter += 1;
            if last_fps_at.elapsed().as_secs() >= 1 {
                fps_display = fps_counter;
                fps_counter = 0;
                last_fps_at = Instant::now();

                debug!("FPS: {}", fps_display);
            }

            last_frame_at = now;
        }
    });
}
