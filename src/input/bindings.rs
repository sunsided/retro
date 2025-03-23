//! This module provides functionality to define and manage key bindings and intents for input handling.
//!
//! The primary feature of this module is the `define_key_bindings!` macro, which facilitates mapping
//! between high-level input intents (represented as an enum) and specific key bindings (represented as a struct).
//!
//! # Features
//!
//! - **Macro for Key Bindings**: The `define_key_bindings!` macro allows for concise definition of key
//!   intents and their corresponding key bindings in a structured manner. It generates:
//!   - A `KeyBindings` struct to hold the key-to-intent mapping.
//!   - An `InputIntent` enum to represent high-level input actions.
//!   - Implementations for the `Default` trait to provide default key mappings.
//!   - A method on the `InputIntent` enum to retrieve the key associated with each intent.
//!
//! # Generated Structure
//!
//! ## `KeyBindings` Struct
//! Holds the key mappings for each defined intent.
//!
//! ## `InputIntent` Enum
//! Represents high-level abstractions for user input intents.

use minifb::Key;

/// Macro to define key bindings and intents.
///
/// This macro allows defining the mapping between high-level key intents
/// and specific key bindings in a structured way. It generates:
/// - A `KeyBindings` struct holding the associated keys.
/// - A `KeyIntent` enum for high-level key intentions.
/// - Implementations for the `Default` trait and a method to get the key for a specific intent.
macro_rules! define_key_bindings {
    (
        $(
            $(#[doc = $doc:expr])*
            $variant:ident, $field:ident => $key:expr
        ),* $(,)?
    ) => {
        /// Struct to hold the key bindings for each key intent.
        #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
        pub struct KeyBindings {
            $(
                $(#[doc = $doc])*
                pub $field: Key,
            )*
        }

        /// Enum representing high-level input intents.
        #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
        pub enum InputIntent {
            $(
                $(#[doc = $doc])*
                $variant,
            )*
        }

        impl Default for KeyBindings {
            /// Provides default key bindings for each key intent.
            fn default() -> Self {
                Self {
                    $(
                        $field: $key,
                    )*
                }
            }
        }

        impl InputIntent {
            /// Retrieves the key associated with this intent from the given bindings.
            ///
            /// # Arguments
            ///
            /// * `bindings` - A reference to the `KeyBindings` struct holding the key mapping.
            ///
            /// # Returns
            ///
            /// The `Key` assigned to this specific intent.
            pub fn get_key(&self, bindings: &KeyBindings) -> Key {
                match self {
                    $(
                        Self::$variant => bindings.$field,
                    )*
                }
            }
        }
    };
}

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
