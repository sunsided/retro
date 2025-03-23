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
        };

        loop {
            // TODO: Validate input state
            let input = *input.load().clone();
            if input.escape_pressed {
                tx_main.send(MainEvent::Shutdown).ok();
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
