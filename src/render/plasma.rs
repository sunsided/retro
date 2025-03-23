use crate::colors::palettes::PLASMA;
use crate::lut::SIN_LUT;
use crate::render::dim_color;
use crate::{FrameBuffer, HEIGHT, WIDTH};
use std::time::Instant;

pub struct Plasma {
    start: Instant,
    speed: f32,
    scale_x: usize,
    scale_y: usize,
    scale_mix: usize,
    radial_scale: usize,
    offset: usize,
    intensity: f32,
}

impl Plasma {
    pub(crate) fn new() -> Self {
        Self {
            start: Instant::now(),
            speed: 60.0,
            scale_x: 3,
            scale_y: 2,
            scale_mix: 2,
            radial_scale: 4,
            offset: 1,
            intensity: 1.0,
        }
    }

    pub fn update(&mut self, intensity: f32) {
        self.intensity = intensity;
    }

    pub fn render(&self, buffer: &mut FrameBuffer) {
        let t = (self.start.elapsed().as_secs_f32() * self.speed) as usize;

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let a = SIN_LUT[((x * self.scale_x).wrapping_add(t * 1)) & 0xFF] as usize;
                let b = SIN_LUT[((y * self.scale_y).wrapping_add(t * 2)) & 0xFF] as usize;
                let c = SIN_LUT[(((x + y) * self.scale_mix).wrapping_add(t * 3)) & 0xFF] as usize;
                let dist =
                    (((x * x + y * y) as f32).sqrt() as usize).wrapping_add(t * self.radial_scale);
                let d = SIN_LUT[(dist * 2) & 0xFF] as usize;
                let v = ((a + b + c + d) >> 2) & 0xFF;
                let index = (v + self.offset) & 0xFF;
                buffer[y * WIDTH + x] = dim_color(PLASMA[index], self.intensity);
            }
        }
    }
}
