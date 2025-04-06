//! This module serves as the entry point for system components and phases used in the engine.
//!
//! It includes various submodules that are responsible for different aspects of the engine's functionality:
//! - [`components`]: Defines the reusable components of the engine.
//! - [`phases`]: Contains logic for different engine execution phases (e.g., fixed updates, rendering).
//!
//! Additionally, this module pulls in generated code related to components, systems, world, and archetypes,
//! ensuring that the underlying engine structure is dynamically extensible.
//!
//! The main structure exported by this module is the `SystemFactory`, which implements
//! the [`CreateSystems`] trait to create and manage the systems used during the engine's execution.

#![allow(unused_imports)]

pub mod components;
pub mod phases;
mod main_world;
mod command_queue;
mod phase_events;

pub use main_world::InitializeMainWorld;
pub use command_queue::CommandQueue;
pub use phase_events::PhaseEvents;
use crate::engine::components::*;
use crate::engine::phases::fixed_update::physics::PhysicsSystemData;
use crate::engine::phases::update::color_cycle::BackgroundColorCycleSystemData;
use crate::engine::phases::render::{RenderSystemData, WgpuRenderState, background::{BackgroundColorData, RenderBackgroundSystemData}};

include!("../generated/components_gen.rs");
include!("../generated/world_gen.rs");
include!("../generated/systems_gen.rs");
include!("../generated/archetypes_gen.rs");

/// The factory of the different systems.
///
/// Implements [`CreateSystems`].
pub struct SystemFactory;
