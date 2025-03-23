//! A utility module for managing key debounce logic.
//!
//! This module provides the `KeyDebounce` struct, which facilitates detecting
//! freshly pressed keys in a key input system. It tracks the state of keys
//! and determines if a key has been transitioned to a pressed state (`true`)
//! from a non-pressed state (`false`).

use crate::input::InputIntent;

/// A utility struct for managing key debounce logic.
///
/// `KeyDebounce` tracks the press state of keys and provides a method to check
/// if a key has been freshly pressed (transitioned from not being pressed to pressed).
pub struct KeyDebounce {
    /// A hashmap that stores the last known state (`true` for pressed, `false` for not pressed)
    /// of each key.
    last: std::collections::HashMap<InputIntent, bool>,
}

impl KeyDebounce {
    /// Creates a new instance of `KeyDebounce`.
    ///
    /// # Returns
    ///
    /// A new `KeyDebounce` instance with an empty key state map.
    pub fn new() -> Self {
        Self {
            last: std::collections::HashMap::new(),
        }
    }

    /// Checks if a key has been freshly pressed.
    ///
    /// A keypress is considered fresh if it transitioned from a non-pressed
    /// state (`false`) to a pressed state (`true`). This method also updates
    /// the internal state of the key.
    ///
    /// # Parameters
    ///
    /// - `key`: The key to check.
    /// - `is_down`: The current state of the key (`true` if pressed, `false` otherwise).
    ///
    /// # Returns
    ///
    /// Returns `true` if the key was freshly pressed, otherwise `false`.
    pub fn fresh_press_detected(&mut self, intent: InputIntent, is_down: bool) -> bool {
        let was_down = self.last.get(&intent).copied().unwrap_or(false);
        self.last.insert(intent, is_down);
        is_down && !was_down
    }
}
