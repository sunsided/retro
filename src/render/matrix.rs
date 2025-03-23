use crate::{FrameBuffer, HEIGHT, WIDTH};

pub struct MatrixEffect {
    columns: Vec<usize>,
}

impl MatrixEffect {
    pub fn new() -> Self {
        let cols = WIDTH / 8;
        Self {
            columns: vec![0; cols],
        }
    }

    pub fn render(&mut self, buffer: &mut FrameBuffer, frame: usize) {
        use rand::{Rng, thread_rng};
        const ROWS: usize = HEIGHT / 8;
        const SPEED: usize = 2;

        for col in 0..self.columns.len() {
            if frame % SPEED == 0 {
                if thread_rng().gen_bool(0.1) {
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
                    for dy in 0..8 {
                        for dx in 0..8 {
                            let px = x + dx;
                            let py = y + dy;
                            if px < WIDTH && py < HEIGHT {
                                buffer[py * WIDTH + px] = color;
                            }
                        }
                    }
                }
            }
        }
    }
}
