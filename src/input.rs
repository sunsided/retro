//! This module provides key handling utilities for defining and managing key bindings, debouncing key inputs,
//! and processing high-level key intents.
//!
//! It contains the following submodules:
//! - `bindings`: Used to define specific key mappings and their associated high-level intents.
//! - `debounce`: Facilitates key debounce handling to avoid unintended repeated input.
//! - `handling`: Provides mechanisms to handle key input events and map them to high-level actions.
//!
//! The module makes use of the `minifb` crate for key input handling and incorporates pre-defined
//! key bindings for common actions, such as toggling a CRT effect, modifying settings, or exiting a context.

mod bindings;
mod debounce;
mod handling;

use bindings::define_key_bindings;
pub use debounce::KeyDebounce;
pub use handling::KeyHandling;
use minifb::Key;

// Defines specific key bindings and their corresponding high-level intents.
define_key_bindings! {
    /// Toggle the CRT effect.
    ToggleCrt, toggle_crt => Key::Space,
    /// Increase a specific setting.
    Increase, increase => Key::Up,
    /// Decrease a specific setting.
    Decrease, decrease => Key::Down,
    /// Exit or escape the current context.
    Escape, escape => Key::Escape,
}
