mod scene_background;

use crate::{GameState, HEIGHT, InputEvent, LOGIC_DURATION, MainEvent, WIDTH};
use crossbeam::channel::{Receiver, Sender};
use std::thread;
use std::time::{Duration, Instant};

pub fn start_logic_thread(
    tx_state: Sender<GameState>,
    tx_main: Sender<MainEvent>,
    input_event_rx: Receiver<InputEvent>,
) {
    thread::spawn(move || {
        let mut state = GameState {
            tick: 0,
            x: 0,
            y: 0,
            plasma: 1.0,
            crt: true,
        };

        let mut last_logic = Instant::now();

        loop {
            while let Ok(event) = input_event_rx.try_recv() {
                match event {
                    InputEvent::ToggleCRT => state.crt = !state.crt,
                    InputEvent::EscapePressed => {
                        tx_main.send(MainEvent::Shutdown).ok();
                    }
                    InputEvent::IncreasePressed => {
                        state.plasma = (state.plasma - 0.1).clamp(0.0, 1.0);
                    }
                    InputEvent::DecreasePressed => {
                        state.plasma = (state.plasma + 0.1).clamp(0.0, 1.0);
                    }
                }
            }

            if last_logic.elapsed() >= LOGIC_DURATION {
                state.tick += 1;
                state.x = (state.x + 1) % WIDTH;
                state.y = (state.y + 1) % HEIGHT;
                let _ = tx_state.send(state);
                last_logic = Instant::now();
            }

            thread::sleep(Duration::from_millis(1));
        }
    });
}
