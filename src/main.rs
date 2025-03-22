#![forbid(unsafe_code)]

mod colors;
mod font8x8;
mod logic;
mod lut;
mod render;
mod text;

use crate::logic::start_logic_thread;
use crate::lut::make_sin_lut;
use crate::render::start_render_thread;
use crossbeam::channel;
use minifb::{Key, Window, WindowOptions};
use std::thread;
use std::time::Duration;

const WIDTH: usize = 320;
const HEIGHT: usize = 200;
const FRAME_DURATION: Duration = Duration::from_millis(20); // 50 FPS
const LOGIC_DURATION: Duration = Duration::from_millis(40); // 25 Hz
const LUT_SIZE: usize = 256;
const NUM_BUFFERS: usize = 3;

type FrameBuffer = [u32; WIDTH * HEIGHT];

#[derive(Clone, Copy)]
struct GameState {
    tick: u64,
    x: usize,
    y: usize,
}

fn main() {
    let (tx_state, rx_state) = channel::bounded::<GameState>(1);
    let (ready_tx, ready_rx) = channel::bounded::<Box<[u32; WIDTH * HEIGHT]>>(NUM_BUFFERS);
    let (free_tx, free_rx) = channel::bounded::<Box<[u32; WIDTH * HEIGHT]>>(NUM_BUFFERS);

    for _ in 0..NUM_BUFFERS {
        let buffer = Box::new([0u32; WIDTH * HEIGHT]);
        free_tx.send(buffer).unwrap();
    }

    start_render_thread(make_sin_lut(), rx_state, ready_tx, free_rx);
    start_logic_thread(tx_state);

    let mut window = Window::new(
        "Plasma Animation - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: false,
            scale: minifb::Scale::X2,
            ..WindowOptions::default()
        },
    )
    .unwrap();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if let Ok(buffer_arc) = ready_rx.recv() {
            window
                .update_with_buffer(buffer_arc.as_slice(), WIDTH, HEIGHT)
                .unwrap();
            free_tx.send(buffer_arc).unwrap();
        }
        thread::sleep(Duration::from_millis(1));
    }
}
