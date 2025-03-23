use crate::colors::palettes;
use crate::{FrameBuffer, HEIGHT, WIDTH};
use rand::{Rng, rng};

pub struct Fire {
    heat: Vec<u8>,
}

impl Fire {
    pub fn new() -> Self {
        Self {
            heat: vec![0u8; WIDTH * HEIGHT],
        }
    }

    pub fn update(&mut self) {
        for x in 0..WIDTH {
            self.heat[(HEIGHT - 1) * WIDTH + x] = if rng().random_range(0..10) > 1 {
                255
            } else {
                0
            };
        }
        for y in 1..HEIGHT {
            for x in 0..WIDTH {
                let src = y * WIDTH + x;
                let below = src + WIDTH;
                let decay = rng().random_range(1..20);
                let new_val = self
                    .heat
                    .get(below)
                    .copied()
                    .unwrap_or(0)
                    .saturating_sub(decay);
                let dx = rng().random_range(0..3);
                let dst_x = x.wrapping_add(dx).wrapping_sub(1).min(WIDTH - 1);
                let dst_y = y - 1;
                let dst = dst_y * WIDTH + dst_x;
                self.heat[dst] = new_val;
            }
        }
    }

    pub fn render(&self, buffer: &mut FrameBuffer) {
        for (i, &val) in self.heat.iter().enumerate() {
            buffer[i] = palettes::FIRE[val as usize];
        }
    }
}
