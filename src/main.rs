mod colors;
mod font8x8;

use crate::colors::COLORS;
use crate::font8x8::{FONT8X8_BASIC, FONT8X8_STANDARD_GALACTIC};
use crossbeam::channel;
use minifb::{Key, Window, WindowOptions};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::{Duration, Instant};

const WIDTH: usize = 320;
const HEIGHT: usize = 200;
const FRAME_DURATION: Duration = Duration::from_millis(20); // 50 FPS
const LOGIC_DURATION: Duration = Duration::from_millis(40); // 25 Hz
const LUT_SIZE: usize = 256;
const NUM_BUFFERS: usize = 3;

static mut BUFFERS: [[u32; WIDTH * HEIGHT]; NUM_BUFFERS] = [[0; WIDTH * HEIGHT]; NUM_BUFFERS];

#[derive(Clone, Copy)]
struct GameState {
    tick: u64,
    x: usize,
    y: usize,
}

fn make_sin_lut() -> [u8; LUT_SIZE] {
    let mut lut = [0u8; LUT_SIZE];
    for i in 0..LUT_SIZE {
        let angle = i as f32 * std::f32::consts::TAU / LUT_SIZE as f32;
        lut[i] = ((f32::sin(angle) * 127.5) + 127.5) as u8;
    }
    lut
}

fn draw_char(buffer: &mut [u32], x: usize, y: usize, ch: u8, color: u32) {
    if let Some(rows) = FONT8X8_BASIC.get(ch as usize) {
        for (dy, byte) in rows.iter().enumerate() {
            for dx in 0..8 {
                if (byte >> (7 - dx)) & 1 != 0 {
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

fn draw_text(buffer: &mut [u32], x: usize, y: usize, text: &str, color: u32) {
    for (i, ch) in text.bytes().enumerate() {
        draw_char(buffer, x + i * 8, y, ch, color);
    }
}

fn start_render_thread(
    sin_lut: [u8; LUT_SIZE],
    rx_state: channel::Receiver<GameState>,
    read_index: Arc<AtomicUsize>,
    write_index: Arc<AtomicUsize>,
) {
    thread::spawn(move || {
        let start = Instant::now();
        let mut state = GameState {
            tick: 0,
            x: 0,
            y: 0,
        };

        loop {
            while let Ok(new_state) = rx_state.try_recv() {
                state = new_state;
            }

            let t = (start.elapsed().as_secs_f32() * 60.0) as usize;
            let palette_shift = state.tick as usize % 256;

            let ri = read_index.load(Ordering::Acquire);
            let wi = write_index.load(Ordering::Acquire);
            let bi = (0..NUM_BUFFERS).find(|&i| i != ri && i != wi).unwrap();
            let buffer = unsafe { &mut BUFFERS[bi] };

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

            write_index.store(bi, Ordering::Release);
            read_index.store(bi, Ordering::Release);

            thread::sleep(FRAME_DURATION);
        }
    });
}

fn main() {
    let (tx_state, rx_state) = channel::bounded::<GameState>(1);

    let read_index = Arc::new(AtomicUsize::new(0));
    let write_index = Arc::new(AtomicUsize::new(1));

    start_render_thread(
        make_sin_lut(),
        rx_state,
        read_index.clone(),
        write_index.clone(),
    );

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

    let mut last_frame = read_index.load(Ordering::Acquire);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let idx = read_index.load(Ordering::Acquire);
        if idx != last_frame {
            last_frame = idx;
            let buffer = unsafe { &BUFFERS[idx] };
            window.update_with_buffer(buffer, WIDTH, HEIGHT).unwrap();
        }
        thread::sleep(Duration::from_millis(1));
    }
}
