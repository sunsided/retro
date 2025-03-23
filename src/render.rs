mod crt_effect;
pub mod cube;
mod fire;
pub mod line;
mod matrix;
mod plasma;
mod sphere;
mod sprite;

use crate::colors::palettes::STANDARD;
use crate::logic::GameState;
use crate::logic::scene_background::SceneBackground;
pub use crate::render::crt_effect::CrtEffect;
use crate::render::cube::draw_wireframe_cube;
pub use crate::render::fire::Fire;
use crate::render::matrix::MatrixEffect;
pub use crate::render::plasma::Plasma;
use crate::render::sphere::draw_wireframe_sphere;
use crate::render::sprite::{UFO_SPRITE, draw_sprite};
use crate::text::{Alphabet, draw_text, font8x8_sentence, font8x8_sentence_runtime};
use crate::{FRAME_DURATION, FrameBuffer, HEIGHT, WIDTH};
use crossbeam::channel;
use std::thread;
use std::time::Instant;
use tracing::{debug, error, trace, warn};

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
        let mut matrix = MatrixEffect::new();
        let mut crt = CrtEffect::new();

        let mut frame = 0;
        let mut fps_counter = 0;
        #[allow(dead_code, unused_assignments)]
        let mut fps_display = 0;
        let mut last_fps_at = Instant::now();
        let mut next_frame_due = Instant::now();

        let sleeper = spin_sleep::SpinSleeper::new(0)
            .with_spin_strategy(spin_sleep::SpinStrategy::YieldThread);

        let mut current_buffer: Option<Box<FrameBuffer>> = None;

        loop {
            let t0 = Instant::now();

            if current_buffer.is_none() {
                current_buffer = match free_rx.try_recv() {
                    Ok(buffer) => Some(buffer),
                    Err(_) => {
                        error!("Stalling: waiting for a free frame buffer...");
                        Some(free_rx.recv().unwrap())
                    }
                };
            }

            let mut buffer = current_buffer.as_mut().unwrap();

            for pixel in buffer.iter_mut() {
                *pixel = 0x000000;
            }

            while let Ok(new_state) = rx_state.try_recv() {
                state = new_state;
            }

            let t1 = Instant::now();
            match state.background {
                SceneBackground::None => {
                    // done.
                }
                SceneBackground::Plasma(ref config) => {
                    plasma.update(config.intensity);
                    plasma.render(&mut buffer);
                }
                SceneBackground::Fire => {
                    fire.update();
                    fire.render(&mut buffer);
                }
                SceneBackground::Matrix => {
                    matrix.render(&mut buffer, frame);
                }
            }

            draw_wireframe_cube(&mut buffer, frame, STANDARD[255]);
            draw_wireframe_sphere(&mut buffer, frame, STANDARD[255]);

            let t2 = Instant::now();
            if state.x < WIDTH && state.y < HEIGHT {
                draw_sprite(&mut buffer, &UFO_SPRITE, 8, state.x, state.y, 4);
            }

            let t3 = Instant::now();
            if state.crt {
                crt.render(&mut buffer, frame / 10);
            }

            let t4 = Instant::now();
            draw_text(
                &mut buffer,
                20,
                20,
                font8x8_sentence!("HELLO VGA WORLD"),
                0xFFFFFF,
                2,
                state.italic,
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

            draw_text(
                &mut buffer,
                20,
                60,
                font8x8_sentence_runtime!(format!("FPS: {}", fps_display)),
                0xFFFFFF,
                1,
                false,
            );

            let t5 = Instant::now();

            if ready_tx.try_send(current_buffer.take().unwrap()).is_err() {
                // Couldn't send this frame — keep the buffer and try again next loop
                error!("Contention detected: unable to send frame buffer to the channel");
            }

            let t6 = Instant::now();
            let elapsed_since_frame_start = t6.duration_since(t0);
            if elapsed_since_frame_start <= FRAME_DURATION {
                trace!(
                    "frame time: {:.3} ms, fetch buffer: {:.3} ms, background.render: {:.3} ms, sprites: {:.3} ms, crt: {:.3} ms, text: {:.3} ms, send: {:.3} ms",
                    elapsed_since_frame_start.as_secs_f32() * 1000.0,
                    (t1 - t0).as_secs_f32() * 1000.0,
                    (t2 - t1).as_secs_f32() * 1000.0,
                    (t3 - t2).as_secs_f32() * 1000.0,
                    (t4 - t3).as_secs_f32() * 1000.0,
                    (t5 - t4).as_secs_f32() * 1000.0,
                    (t6 - t5).as_secs_f32() * 1000.0,
                );
            } else {
                warn!(
                    "frame time: {:.3} ms, fetch buffer: {:.3} ms, background.render: {:.3} ms, sprites: {:.3} ms, crt: {:.3} ms, text: {:.3} ms, send: {:.3} ms",
                    elapsed_since_frame_start.as_secs_f32() * 1000.0,
                    (t1 - t0).as_secs_f64() * 1000.0,
                    (t2 - t1).as_secs_f64() * 1000.0,
                    (t3 - t2).as_secs_f64() * 1000.0,
                    (t4 - t3).as_secs_f64() * 1000.0,
                    (t5 - t4).as_secs_f64() * 1000.0,
                    (t6 - t5).as_secs_f64() * 1000.0,
                );
            }

            // Schedule next frame time exactly `FRAME_DURATION` after previous
            next_frame_due += FRAME_DURATION;

            let now = Instant::now();
            if now < next_frame_due {
                sleeper.sleep_until(next_frame_due);
            } else {
                warn!(
                    "Render frame missed deadline by {:?}",
                    now.duration_since(next_frame_due)
                );
                next_frame_due = now;
            }

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
