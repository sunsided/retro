use crate::colors::palettes::PLASMA;
use crate::lut::SIN_LUT;
use crate::render::dim_color;
use crate::{FrameBuffer, HEIGHT, WIDTH};
use std::time::Instant;

/// Represents a plasma effect renderer with customizable parameters.
pub struct Plasma {
    /// The starting time of the plasma effect.
    start: Instant,
    /// Speed at which the plasma effect evolves over time.
    speed: f32,
    /// Horizontal scaling factor for the plasma waves.
    scale_x: usize,
    /// Vertical scaling factor for the plasma waves.
    scale_y: usize,
    /// Scaling factor for mix-dependent plasma waves.
    scale_mix: usize,
    /// Scaling factor for the radial plasma waves.
    radial_scale: usize,
    /// Offset applied to the computed color index.
    offset: usize,
    /// Intensity multiplier for dimming the colors.
    intensity: f32,
}

impl Plasma {
    /// Creates a new `Plasma` instance with default parameters.
    pub fn new() -> Self {
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

    /// Updates the plasma effect with a new intensity value.
    ///
    /// # Parameters
    /// - `intensity`: The new intensity value to set for the plasma effect.
    pub fn update(&mut self, intensity: f32) {
        self.intensity = intensity;
    }

    /// Renders the plasma effect onto the provided framebuffer.
    ///
    /// # Parameters
    /// - `buffer`: A mutable reference to the framebuffer where the plasma effect will be rendered.
    pub fn render(&self, buffer: &mut FrameBuffer) {
        let t = (self.start.elapsed().as_secs_f32() * self.speed) as usize;

        let lut = SIN_LUT.as_ref();
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                // Calculate the contribution from the horizontal sine wave
                let a = lut[((x * self.scale_x).wrapping_add(t * 1)) & 0xFF] as usize;
                // Calculate the contribution from the vertical sine wave
                let b = lut[((y * self.scale_y).wrapping_add(t * 2)) & 0xFF] as usize;
                // Calculate the contribution from the mixed sine wave
                let c = lut[(((x + y) * self.scale_mix).wrapping_add(t * 3)) & 0xFF] as usize;
                // Calculate the contribution based on a radial pattern
                let dist =
                    (((x * x + y * y) as f32).sqrt() as usize).wrapping_add(t * self.radial_scale);
                let d = lut[(dist * 2) & 0xFF] as usize;

                let v = ((a + b + c + d) >> 2) & 0xFF;

                let index = (v + self.offset) & 0xFF;
                buffer[y * WIDTH + x] = dim_color(PLASMA[index], self.intensity);
            }
        }
    }
}
