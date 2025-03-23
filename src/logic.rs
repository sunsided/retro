use crate::{GameState, HEIGHT, InputState, LOGIC_DURATION, MainEvent, WIDTH};
use arc_swap::ArcSwapAny;
use crossbeam::channel::Sender;
use std::sync::Arc;
use std::thread;

pub fn start_logic_thread(
    tx_state: Sender<GameState>,
    input: Arc<ArcSwapAny<Arc<InputState>>>,
    tx_main: Sender<MainEvent>,
) {
    thread::spawn(move || {
        let mut state = GameState {
            tick: 0,
            x: 0,
            y: 0,
            plasma: 1.0,
        };

        loop {
            // TODO: Validate input state
            let input = *input.load().clone();
            if input.escape_pressed {
                tx_main.send(MainEvent::Shutdown).ok();
            }

            if input.minus_pressed {
                state.plasma = (state.plasma - 0.1).clamp(0.0, 1.0);
            } else if input.plus_pressed {
                state.plasma = (state.plasma + 0.1).clamp(0.0, 1.0);
            }

            state.tick += 1;
            state.x = (state.x + 1) % WIDTH;
            state.y = (state.y + 1) % HEIGHT;

            // TODO: Some state is only necessary for simulation and does not need to be passed to the renderer
            //       e.g. bounding boxes are relevant for sim, but only pose (and mesh and texture data) is needed by the renderer
            let _ = tx_state.send(state);

            thread::sleep(LOGIC_DURATION);
        }
    });
}
