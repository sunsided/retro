//! This module handles keyboard input processing, including key intents, debouncing, and event transmission.
//!
//! The main structure provided by this module is [`KeyHandling`], which facilitates:
//! - Mapping input intents to specific key bindings using [`KeyBindings`].
//! - Preventing repeated keypresses being processed in quick succession through `KeyDebounce`.
//! - Transmitting detected input events via a [`crossbeam::channel::Sender`].
//!
//! The `KeyHandling` struct exposes methods to process user input, emit events for detected key intents,
//! and manage key debouncing.

use crate::InputEvent;
use crate::input::{InputIntent, KeyBindings, KeyDebounce};
use crossbeam::channel::Sender;
use minifb::Window;

/// Manages key handling, including key mapping, debouncing, and event transmission.
pub struct KeyHandling {
    /// Holds the key bindings for input intents.
    bindings: KeyBindings,
    /// Manages the debouncing logic for key presses.
    debouncer: KeyDebounce,
    /// Transmits input events to the respective receivers.
    pub input_event_tx: Sender<InputEvent>,
}

impl KeyHandling {
    /// Constructs a new instance of `KeyHandling`.
    ///
    /// # Parameters
    ///
    /// - `key_bindings`: The key bindings that map intents to specific keys.
    /// - `input_event_tx`: The channel sender for transmitting input events.
    ///
    /// # Returns
    ///
    /// A new instance of `KeyHandling`.
    pub fn new(key_bindings: KeyBindings, input_event_tx: Sender<InputEvent>) -> Self {
        Self {
            bindings: key_bindings,
            debouncer: KeyDebounce::new(),
            input_event_tx,
        }
    }

    /// Processes user input from the given window and emits events for detected input intents.
    ///
    /// # Parameters
    ///
    /// - `window`: The window instance to check for key presses.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success (`Ok(())`) or an error if the event transmission fails.
    pub fn process_input(
        &mut self,
        window: &Window,
    ) -> Result<(), crossbeam::channel::SendError<InputEvent>> {
        self.try_emit_event(window, InputIntent::ToggleCrt, InputEvent::ToggleCRT)?;
        self.try_emit_event(window, InputIntent::Escape, InputEvent::EscapePressed)?;
        self.try_emit_event(window, InputIntent::Increase, InputEvent::IncreasePressed)?;
        self.try_emit_event(window, InputIntent::Decrease, InputEvent::DecreasePressed)?;
        Ok(())
    }

    /// Attempts to emit an event if the corresponding input intent is detected.
    ///
    /// # Parameters
    ///
    /// - `window`: The window instance to check for key presses.
    /// - `intent`: The input intent being checked.
    /// - `event`: The event to emit if the intent is detected.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or an error if the event transmission fails.
    fn try_emit_event(
        &mut self,
        window: &Window,
        intent: InputIntent,
        event: InputEvent,
    ) -> Result<(), crossbeam::channel::SendError<InputEvent>> {
        if self.event_detected(window, intent) {
            self.input_event_tx.send(event)
        } else {
            Ok(())
        }
    }

    /// Detects if a specific input intent has been triggered.
    ///
    /// # Parameters
    ///
    /// - `window`: The window instance to check for key presses.
    /// - `intent`: The input intent being checked.
    ///
    /// # Returns
    ///
    /// `true` if the intent has been triggered, otherwise `false`.
    fn event_detected(&mut self, window: &Window, intent: InputIntent) -> bool {
        let key = intent.get_key(&self.bindings);
        self.debouncer
            .fresh_press_detected(intent, window.is_key_down(key))
    }
}
