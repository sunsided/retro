#![allow(unused_imports)]

pub mod components;
pub mod phases;

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
