use crate::InputEvent;
use crate::input::KeyDebounce;
use minifb::{Key, Window};

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
    ) {
        if debouncer.is_fresh_press(self.toggle_crt, window.is_key_down(self.toggle_crt)) {
            let _ = input_event_tx.send(InputEvent::ToggleCRT);
        }

        if debouncer.is_fresh_press(self.escape, window.is_key_down(self.escape)) {
            let _ = input_event_tx.send(InputEvent::EscapePressed);
        }

        if debouncer.is_fresh_press(self.plus, window.is_key_down(self.plus)) {
            let _ = input_event_tx.send(InputEvent::IncreasePressed);
        }

        if debouncer.is_fresh_press(self.minus, window.is_key_down(self.minus)) {
            let _ = input_event_tx.send(InputEvent::DecreasePressed);
        }
    }
}
