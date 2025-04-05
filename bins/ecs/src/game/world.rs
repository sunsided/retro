use crate::game::archetype::*;
use crate::game::component::*;
use crate::game::system::rendering::RenderSystemData;
pub use crate::game::world::rendering::{WgpuRenderState, background::RenderBackgroundSystemData};
pub use crate::game::system::physics::PhysicsSystemData;
use crate::game::system::*;

include!("../../generated/world_gen.rs");
