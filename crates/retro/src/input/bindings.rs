//! # Key Bindings Module
//!
//! This module provides a macro, `define_key_bindings`, to define mappings between high-level input
//! intents and specific key bindings in a structured and efficient manner. It generates a struct
//! for key bindings and an enum for intents, ensuring readability and maintainability in handling
//! key-related actions in applications.
//!
//! ## Overview
//!
//! - **`KeyBindings` struct**: Stores key mappings for each input intent.
//! - **`InputIntent` enum**: Represents high-level input intents or actions.
//! - Implements the `Default` trait to provide default key mappings.
//! - Provides a method to retrieve the key for a specific intent.
//!
//! ## Usage Design
//!
//! The macro `define_key_bindings!` allows developers to define key mappings and associated
//! high-level intents in one concise place. This minimizes boilerplate and keeps intent-to-key
//! management centralized and consistent across the codebase.

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

pub(crate) use define_key_bindings;

#[cfg(test)]
mod tests {
    use super::*;

    define_key_bindings! {
        /// Intent for moving up in the game.
        Up, up_key => Key::UpArrow,
        /// Intent for moving down in the game.
        Down, down_key => Key::DownArrow,
        /// Intent for moving left in the game.
        Left, left_key => Key::LeftArrow,
        /// Intent for moving right in the game.
        Right, right_key => Key::RightArrow,
    }

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
    pub enum Key {
        UpArrow,
        DownArrow,
        LeftArrow,
        RightArrow,
    }

    #[test]
    fn test_key_bindings_default() {
        let bindings = KeyBindings::default();

        assert_eq!(bindings.up_key, Key::UpArrow);
        assert_eq!(bindings.down_key, Key::DownArrow);
        assert_eq!(bindings.left_key, Key::LeftArrow);
        assert_eq!(bindings.right_key, Key::RightArrow);
    }

    #[test]
    fn test_get_key_for_intent() {
        let bindings = KeyBindings::default();

        let up_intent = InputIntent::Up;
        let down_intent = InputIntent::Down;
        let left_intent = InputIntent::Left;
        let right_intent = InputIntent::Right;

        assert_eq!(up_intent.get_key(&bindings), Key::UpArrow);
        assert_eq!(down_intent.get_key(&bindings), Key::DownArrow);
        assert_eq!(left_intent.get_key(&bindings), Key::LeftArrow);
        assert_eq!(right_intent.get_key(&bindings), Key::RightArrow);
    }
}
