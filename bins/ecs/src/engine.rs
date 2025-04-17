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

mod command_queue;
pub mod components;
pub mod input_state;
mod phase_events;
pub mod phases;
pub mod resource_manager_state;
pub mod wgpu;

use crate::engine::components::*;
pub use crate::engine::input_state::InputState;
use crate::engine::phases::{
    fixed_update::{
        fog::FogSystemData, physics::PhysicsSystemData, stars::StarsSystemData, sun::SunSystemData,
        time_of_day::TimeOfDaySystemData,
    },
    render::{
        background::{BackgroundColorData, RenderBackgroundSystemData},
        camera_render::CameraRenderSystemData,
        dynamic_light::DynamicLightSystemData,
        frame_globals::FrameGlobalsSystemData,
        render::RenderSystemData,
        render_sky::RenderSkySystemData,
        tile_render::TileRenderSystemData,
        vga_blit::VgaBlitSystemData,
        WgpuRenderState,
    },
    update::{camera::CameraSystemData, color_cycle::BackgroundColorCycleSystemData},
};
pub use crate::engine::resource_manager_state::ResourceManagerState;
pub use crate::engine::wgpu::*;
pub use command_queue::CommandQueue;
pub use phase_events::PhaseEvents;

/// The hash map used for entity lookups.
type EntityLocationMap<K, V> = fxhash::FxHashMap<K, V>;

include!("../generated/components_gen.rs");
include!("../generated/world_gen.rs");
include!("../generated/systems_gen.rs");
include!("../generated/archetypes_gen.rs");

/// The factory of the different systems.
///
/// Implements [`CreateSystems`].
pub struct SystemFactory;
