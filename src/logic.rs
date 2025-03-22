use crate::{GameState, HEIGHT, LOGIC_DURATION, WIDTH};
use crossbeam::channel::Sender;
use std::thread;
use std::time::{Duration, Instant};

pub fn start_logic_thread(tx_state: Sender<GameState>) {
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

                // TODO: Some state is only necessary for simulation and does not need to be passed to the renderer
                //       e.g. bounding boxes are relevant for sim, but only pose (and mesh and texture data) is needed by the renderer
                let _ = tx_state.send(state);
                last_logic += LOGIC_DURATION;
            }
            thread::sleep(Duration::from_millis(1));
        }
    });
}
