//! This module provides functionality related to handling keyboard input, including key bindings,
//! key debouncing, and input handling. It contains these submodules:
//!
//! - `bindings`: Defines [`InputIntent`] and [`KeyBindings`] for binding keys to specific actions or intents.
//! - `debounce`: Implements [`KeyDebounce`] for managing keypress debouncing functionality.
//! - `handling`: Provides [`KeyHandling`] for processing key inputs and executing corresponding actions.
//!
//! By re-exporting key components, this module simplifies access to keyboard handling utilities for use in applications.

mod bindings;
mod debounce;
mod handling;

pub use bindings::{InputIntent, KeyBindings};
pub use debounce::KeyDebounce;
pub use handling::KeyHandling;
