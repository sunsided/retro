use crate::text::Alphabet::StandardGalactic;
use crate::text::{Alphabet, GlyphIndex, draw_char};
use crate::{FrameBuffer, HEIGHT, WIDTH};
use rand::rng;

pub struct MatrixEffect {
    columns: Vec<usize>,
    alphabet: Alphabet,
}

impl MatrixEffect {
    pub fn new() -> Self {
        let cols = WIDTH / 8;
        Self {
            columns: vec![0; cols],
            alphabet: StandardGalactic,
        }
    }

    pub fn render(&mut self, buffer: &mut FrameBuffer, frame: usize) {
        use rand::Rng;
        const ROWS: usize = HEIGHT / 8;
        const SPEED: usize = 2;

        for col in 0..self.columns.len() {
            if frame % SPEED == 0 {
                if rng().random_bool(0.1) {
                    self.columns[col] = 0;
                } else {
                    self.columns[col] += 1;
                }
            }

            for row in 0..ROWS {
                let brightness =
                    255 - ((self.columns[col] as isize - row as isize).abs() * 32).min(255) as u8;
                if brightness > 0 {
                    let x = col * 8;
                    let y = row * 8;
                    let color = ((brightness as u32) << 8);
                    let ch = GlyphIndex::new(rng().random_range(0..self.alphabet.len()) as u8);
                    draw_char(buffer, x, y, self.alphabet, ch, color, 1, false);
                }
            }
        }
    }
}
