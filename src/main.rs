mod colors;
mod font8x8;
mod lut;
mod text;

use crate::colors::COLORS;
use crate::lut::make_sin_lut;
use crate::text::draw_text;
use crossbeam::channel;
use minifb::{Key, Window, WindowOptions};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

const WIDTH: usize = 320;
const HEIGHT: usize = 200;
const FRAME_DURATION: Duration = Duration::from_millis(20); // 50 FPS
const LOGIC_DURATION: Duration = Duration::from_millis(40); // 25 Hz
const LUT_SIZE: usize = 256;
const NUM_BUFFERS: usize = 3;

#[derive(Clone, Copy)]
struct GameState {
    tick: u64,
    x: usize,
    y: usize,
}

fn start_render_thread(
    sin_lut: [u8; LUT_SIZE],
    rx_state: channel::Receiver<GameState>,
    ready_tx: channel::Sender<Arc<Box<[u32; WIDTH * HEIGHT]>>>,
    free_rx: channel::Receiver<Arc<Box<[u32; WIDTH * HEIGHT]>>>,
) {
    thread::spawn(move || {
        let start = Instant::now();
        let mut state = GameState {
            tick: 0,
            x: 0,
            y: 0,
        };

        loop {
            let buffer = free_rx.recv().unwrap();
            let mut_buffer = Arc::try_unwrap(buffer).ok().unwrap();
            let mut_buffer = Box::into_raw(mut_buffer);
            let buffer = unsafe { &mut *mut_buffer };

            while let Ok(new_state) = rx_state.try_recv() {
                state = new_state;
            }

            let t = (start.elapsed().as_secs_f32() * 60.0) as usize;
            let palette_shift = state.tick as usize % 256;

            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    let a = sin_lut[((x << 3).wrapping_add(t)) & 0xFF] as usize;
                    let b = sin_lut[((y << 2).wrapping_add(t * 2)) & 0xFF] as usize;
                    let c = sin_lut[(((x + y) << 2).wrapping_add(t * 3)) & 0xFF] as usize;
                    let dist = (((x * x + y * y) as f32).sqrt() as usize).wrapping_add(t * 4);
                    let d = sin_lut[(dist << 1) & 0xFF] as usize;

                    let v = ((a + b + c + d) >> 2) & 0xFF;
                    let shifted_index = (v + palette_shift) & 0xFF;

                    buffer[y * WIDTH + x] = COLORS[shifted_index];
                }
            }

            draw_text(buffer, 20, 20, "HELLO VGA WORLD", 0xFFFFFF);

            if state.x < WIDTH && state.y < HEIGHT {
                buffer[state.y * WIDTH + state.x] = 0xFFFFFF;
            }

            let boxed = unsafe { Box::from_raw(buffer) };
            ready_tx.send(Arc::new(boxed)).unwrap();
            thread::sleep(FRAME_DURATION);
        }
    });
}

fn main() {
    let (tx_state, rx_state) = channel::bounded::<GameState>(1);
    let (ready_tx, ready_rx) = channel::bounded::<Arc<Box<[u32; WIDTH * HEIGHT]>>>(NUM_BUFFERS);
    let (free_tx, free_rx) = channel::bounded::<Arc<Box<[u32; WIDTH * HEIGHT]>>>(NUM_BUFFERS);

    for _ in 0..NUM_BUFFERS {
        let buffer = Box::new([0u32; WIDTH * HEIGHT]);
        free_tx.send(Arc::new(buffer)).unwrap();
    }

    start_render_thread(make_sin_lut(), rx_state, ready_tx, free_rx);

    // Game logic thread
    thread::spawn(move || {
        let mut state = GameState {
            tick: 0,
            x: 0,
            y: 0,
        };
        let mut last_logic = Instant::now();

        loop {
            if last_logic.elapsed() >= LOGIC_DURATION {
                state.tick += 1;
                state.x = (state.x + 1) % WIDTH;
                state.y = (state.y + 1) % HEIGHT;
                let _ = tx_state.send(state);
                last_logic += LOGIC_DURATION;
            }
            thread::sleep(Duration::from_millis(1));
        }
    });

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
