mod bindings;

pub use bindings::KeyBindings;
use minifb::Key;

// Struct to debounce keypresses
pub struct KeyDebounce {
    last: std::collections::HashMap<Key, bool>,
}

impl KeyDebounce {
    pub(crate) fn new() -> Self {
        Self {
            last: std::collections::HashMap::new(),
        }
    }

    pub(crate) fn is_fresh_press(&mut self, key: Key, is_down: bool) -> bool {
        let was_down = self.last.get(&key).copied().unwrap_or(false);
        self.last.insert(key, is_down);
        is_down && !was_down
    }
}
