mod game_state;
pub mod scene_background;

use crate::input::InputEvent;
use crate::logic::scene_background::SceneBackground;
use crate::lut::SIN_LUT;
use crate::{HEIGHT, LOGIC_DURATION, MainEvent, WIDTH};
use crossbeam::channel::{Receiver, Sender};
pub use game_state::GameState;
use std::thread;
use std::time::Instant;
use tracing::warn;

pub fn start_logic_thread(
    tx_state: Sender<GameState>,
    tx_main: Sender<MainEvent>,
    input_event_rx: Receiver<InputEvent>,
) {
    thread::spawn(move || {
        let mut state = GameState::default();

        let mut next_frame_due = Instant::now();

        let sleeper = spin_sleep::SpinSleeper::new(0)
            .with_spin_strategy(spin_sleep::SpinStrategy::YieldThread);

        loop {
            while let Ok(event) = input_event_rx.try_recv() {
                match event {
                    InputEvent::ToggleCRT => state.crt = !state.crt,
                    InputEvent::EscapePressed => {
                        tx_main.send(MainEvent::Shutdown).ok();
                    }
                    InputEvent::IncreasePressed => {
                        if let SceneBackground::Plasma(plasma) = &mut state.background {
                            plasma.intensity = (plasma.intensity - 0.1).clamp(0.0, 1.0);
                        }
                    }
                    InputEvent::DecreasePressed => {
                        if let SceneBackground::Plasma(plasma) = &mut state.background {
                            plasma.intensity = (plasma.intensity + 0.1).clamp(0.0, 1.0);
                        }
                    }
                    InputEvent::NumberSelected(num) => match num {
                        0 => state.background = SceneBackground::None,
                        1 => state.background = SceneBackground::Plasma(Default::default()),
                        2 => state.background = SceneBackground::Fire,
                        _ => (),
                    },
                    InputEvent::ToggleItalic => {
                        state.italic = !state.italic;
                    }
                }
            }

            state.tick += 1;
            state.x = (state.x + 1) % WIDTH;
            // state.y = (state.y + 1) % HEIGHT;
            // state.y = HEIGHT / 2;
            state.y = ((SIN_LUT[(state.tick & 0xFF) as usize] as usize * HEIGHT / 768)
                + HEIGHT / 2)
                .clamp(0, HEIGHT);

            if tx_state.send(state.clone()).is_err() {
                warn!("Failed to send game state update");
            }

            // Schedule next frame time exactly `LOGIC_DURATION` after previous
            next_frame_due += LOGIC_DURATION;

            let now = Instant::now();
            if now < next_frame_due {
                sleeper.sleep_until(next_frame_due);
            } else {
                warn!(
                    "Logic frame missed deadline by {:?}",
                    now.duration_since(next_frame_due)
                );
                next_frame_due = now;
            }
        }
    });
}
