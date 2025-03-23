use crate::{Color, FrameBuffer, HEIGHT, WIDTH};
use rand::Rng;
use rayon::prelude::*;

pub struct CrtEffect {
    scratch: FrameBuffer,
}

impl CrtEffect {
    pub fn new() -> Self {
        Self {
            scratch: [Color::default(); WIDTH * HEIGHT],
        }
    }

    pub fn render(&mut self, pixels: &mut FrameBuffer, frame: usize) {
        let wobble_offset =
            ((frame as f32 * 0.3).sin() * rand::rng().random_range(1.5..3.0)) as usize; // horizontal wobble
        let roll_line = (frame * 2 + fastrand::usize(0..10)) % HEIGHT; // rolling distortion line
        let analog_glitch = fastrand::f32() < 0.02;

        pixels.par_iter_mut().enumerate().for_each(|(i, px)| {
            let y = i / WIDTH;
            let brightness = if y % 2 == 0 { 255 } else { 220 };

            let noise = if analog_glitch && rand::random::<f32>() < 0.2 {
                rand::random::<u8>() % 70 + 30
            } else {
                rand::random::<u8>() % 11
            };

            let r = (((*px >> 16) & 0xFF).saturating_add(noise as _)) * brightness / 255;
            let g = (((*px >> 8) & 0xFF).saturating_add(noise as _)) * brightness / 255;
            let b = ((*px) & 0xFF).saturating_add(noise as _) * brightness / 255;
            // *px = (r << 16) | (g << 8) | b;

            // RGB shadow mask simulation
            let x = i % WIDTH;
            let modulated = match x % 3 {
                0 => (r / 2) << 16,
                1 => (g / 2) << 8,
                _ => b / 2,
            };

            *px = (*px & 0xFF_FF_FF) & !0x7F_7F_7F | modulated;
        });

        // Chromatic aberration
        pixels.par_chunks_mut(WIDTH).for_each(|row| {
            for x in 1..(WIDTH - 1) {
                let prev = row[x - 1];
                let next = row[x + 1];

                let r = (next >> 16) & 0xFF;
                let g = (row[x] >> 8) & 0xFF;
                let b = prev & 0xFF;

                row[x] = (r << 16) | (g << 8) | b;
            }
        });

        // Vignette
        pixels.par_iter_mut().enumerate().for_each(|(i, px)| {
            let x = i % WIDTH;
            let y = i / WIDTH;

            let xf = (x as f32 - WIDTH as f32 / 2.0) / (WIDTH as f32 / 2.0);
            let yf = (y as f32 - HEIGHT as f32 / 2.0) / (HEIGHT as f32 / 2.0);
            let vignette = (1.0 - (xf * xf + yf * yf) * 0.2).clamp(0.0, 1.0);

            let r = (((*px >> 16) & 0xFF) as f32 * vignette) as u8;
            let g = (((*px >> 8) & 0xFF) as f32 * vignette) as u8;
            let b = ((*px & 0xFF) as f32 * vignette) as u8;

            *px = ((r as u32) << 16) | ((g as u32) << 8) | b as u32;
        });

        // Horizontal wobble + rolling distortion
        let wobble_per_row: Vec<usize> = (0..HEIGHT)
            .map(|y| {
                if y == roll_line {
                    fastrand::usize(2..6)
                } else {
                    wobble_offset
                }
            })
            .collect();

        self.scratch.copy_from_slice(pixels);
        let scratch = &self.scratch;
        pixels.par_iter_mut().enumerate().for_each(|(i, dst)| {
            let x = i % WIDTH;
            let y = i / WIDTH;

            let wobble = wobble_per_row[y];
            let src_x = (x + wobble).min(WIDTH - 1);
            let src_idx = y * WIDTH + src_x;

            *dst = scratch[src_idx];
        });
    }
}
