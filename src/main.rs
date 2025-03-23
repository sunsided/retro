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
use arc_swap::ArcSwap;
use crossbeam::{channel, select};
use minifb::{Key, KeyRepeat, Window, WindowOptions};
use std::sync::Arc;
use std::time::Duration;

const WIDTH: usize = 320;
const HEIGHT: usize = 200;
const FRAME_DURATION: Duration = Duration::from_millis(20); // 50 FPS
const LOGIC_DURATION: Duration = Duration::from_millis(40); // 25 Hz
const INPUT_DURATION: Duration = Duration::from_millis(20); // 50 Hz
const LUT_SIZE: usize = 256;
const NUM_BUFFERS: usize = 3;

type FrameBuffer = [u32; WIDTH * HEIGHT];

#[derive(Clone, Copy)]
struct GameState {
    tick: u64,
    x: usize,
    y: usize,
    plasma: f32,
}

#[derive(Clone, Copy, Default)]
struct InputState {
    escape_pressed: bool,
    // later: movement keys, mouse, etc.
    plus_pressed: bool,
    minus_pressed: bool,
}

enum MainEvent {
    Shutdown,
}

fn main() {
    let input_state = Arc::new(ArcSwap::from_pointee(InputState::default()));

    let (tx_state, rx_state) = channel::bounded::<GameState>(1);
    let (tx_main, rx_main) = channel::unbounded::<MainEvent>();
    let (ready_tx, ready_rx) = channel::bounded::<Box<FrameBuffer>>(NUM_BUFFERS);
    let (free_tx, free_rx) = channel::bounded::<Box<FrameBuffer>>(NUM_BUFFERS);

    for _ in 0..NUM_BUFFERS {
        let buffer = Box::new([0u32; WIDTH * HEIGHT]);
        free_tx.send(buffer).unwrap();
    }

    start_render_thread(make_sin_lut(), rx_state, ready_tx, free_rx);
    start_logic_thread(tx_state, input_state.clone(), tx_main);

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

    while window.is_open() {
        select! {
            recv(ready_rx) -> msg => match msg {
                Ok(buffer_arc) => {
                    window
                        .update_with_buffer(buffer_arc.as_slice(), WIDTH, HEIGHT)
                        .unwrap();
                    free_tx.send(buffer_arc).unwrap();
                }
                Err(_) => {
                    eprintln!("Error receiving buffer event");
                    break;
                },
            },

            recv(rx_main) -> msg => match msg {
                Ok(event) => {
                    match event {
                        MainEvent::Shutdown => break,
                    }
                }
                Err(_) => {
                    eprintln!("Error receiving output event");
                    break;
                },
            },

            default(INPUT_DURATION) => {
                // Neither frame nor event received in INPUT_DURATION
            }
        }

        let input = InputState {
            escape_pressed: window.is_key_down(Key::Escape),
            plus_pressed: window.is_key_pressed(Key::Up, KeyRepeat::Yes),
            minus_pressed: window.is_key_pressed(Key::Down, KeyRepeat::Yes),
        };
        input_state.store(Arc::new(input));
    }
}
