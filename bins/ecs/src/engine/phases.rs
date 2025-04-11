//! This module contains various submodules responsible for handling
//! different aspects of the application lifecycle, including:
//!
//! - `fixed_update`: Manages fixed time-step updates.
//! - `update`: Handles variable time-step updates.
//! - `render`: Responsible for rendering-related functionality.
//! - `startup`: Performs actions needed during the application's startup.
//!
//! These submodules are designed to encapsulate specific functionality while
//! promoting modular and organized code structure for the application.

pub mod fixed_update;
pub mod render;
pub mod startup;
pub mod update;
