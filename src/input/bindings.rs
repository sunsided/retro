use crate::input::KeyDebounce;
use crate::{InputEvent, InputState};
use minifb::{Key, KeyRepeat, Window};

// Struct for configurable key bindings
pub struct KeyBindings {
    toggle_crt: Key,
    plus: Key,
    minus: Key,
    escape: Key,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            toggle_crt: Key::Space,
            plus: Key::Up,
            minus: Key::Down,
            escape: Key::Escape,
        }
    }
}

// KeyBindings manages key mapping and debouncing
impl KeyBindings {
    pub(crate) fn process_input(
        &self,
        window: &Window,
        debouncer: &mut KeyDebounce,
        input_event_tx: &crossbeam::channel::Sender<InputEvent>,
    ) -> InputState {
        let state = InputState {
            escape_pressed: window.is_key_down(self.escape),
            plus_pressed: window.is_key_pressed(self.plus, KeyRepeat::Yes),
            minus_pressed: window.is_key_pressed(self.minus, KeyRepeat::Yes),
        };

        if debouncer.is_fresh_press(self.toggle_crt, window.is_key_down(self.toggle_crt)) {
            let _ = input_event_tx.send(InputEvent::ToggleCRT);
        }

        state
    }
}
