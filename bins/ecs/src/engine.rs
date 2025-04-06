#![allow(unused_imports)]

pub mod component;
pub mod system;

use crate::engine::component::*;
use crate::engine::system::physics::PhysicsSystemData;
use crate::engine::system::rendering::background::{BackgroundColorData, RenderBackgroundSystemData};
use crate::engine::system::color_cycle::BackgroundColorCycleSystemData;
use crate::engine::system::rendering::{RenderSystemData, WgpuRenderState};

include!("../generated/components_gen.rs");
include!("../generated/world_gen.rs");
include!("../generated/systems_gen.rs");
include!("../generated/archetypes_gen.rs");
