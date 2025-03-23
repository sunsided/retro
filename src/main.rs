#![forbid(unsafe_code)]

mod colors;
mod input;
mod logic;
mod lut;
mod render;
mod text;

use crate::input::{InputEvent, KeyBindings, KeyHandling};
use crate::logic::{GameState, start_logic_thread};
use crate::render::start_render_thread;
use crossbeam::{channel, select};
use dotenvy::dotenv;
use minifb::{Window, WindowOptions};
use std::time::{Duration, SystemTime};
use tracing_appender::non_blocking;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{EnvFilter, fmt};

const WIDTH: usize = 320;
const HEIGHT: usize = 200;
const FRAME_DURATION: Duration = Duration::from_millis(20); // 50 Hz
const LOGIC_DURATION: Duration = Duration::from_millis(10); // 100 Hz
const INPUT_DURATION: Duration = Duration::from_millis(10); // 100 Hz
const LUT_SIZE: usize = 256;
const NUM_BUFFERS: usize = 3;

type Color = u32;
type FrameBuffer = [Color; WIDTH * HEIGHT];

enum MainEvent {
    Shutdown,
}

fn main() {
    dotenv().ok();
    let _guard = setup_logging();

    let (input_event_tx, input_event_rx) = channel::unbounded::<InputEvent>();
    let (tx_state, rx_state) = channel::bounded::<GameState>(1);
    let (tx_main, rx_main) = channel::unbounded::<MainEvent>();
    let (ready_tx, ready_rx) = channel::bounded::<Box<FrameBuffer>>(NUM_BUFFERS);
    let (free_tx, free_rx) = channel::bounded::<Box<FrameBuffer>>(NUM_BUFFERS);

    for _ in 0..NUM_BUFFERS {
        let buffer = Box::new([Color::default(); WIDTH * HEIGHT]);
        free_tx.send(buffer).unwrap();
    }

    start_render_thread(rx_state, ready_tx, free_rx);
    start_logic_thread(tx_state, tx_main, input_event_rx);

    let mut window = Window::new(
        "Plasma Animation - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: false,
            scale: minifb::Scale::X4,
            ..WindowOptions::default()
        },
    )
    .unwrap();

    let bindings = KeyBindings::default();
    let mut input_handling = KeyHandling::new(bindings, input_event_tx.clone());

    while window.is_open() {
        input_handling.process_input(&window).ok();

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
    }
}

fn setup_logging() -> WorkerGuard {
    // Write logs to stdout with a non-blocking writer.
    let (non_blocking_writer, guard) = non_blocking(std::io::stdout());

    // Use an environment filter for dynamic log levels.
    // e.g., set RUST_LOG=debug before running.
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    fmt()
        .with_writer(non_blocking_writer)
        .with_env_filter(env_filter)
        .init();
    guard
}
