pub mod rendering;
mod vertex;
pub mod physics;

use crate::game::system::rendering::{background::RenderBackgroundSystemData, RenderSystemData};
use crate::game::system::physics::PhysicsSystemData;
use crate::game::archetype::*;
use crate::game::component::*;
use crate::game::world::{FrameContext, WorldCommandSender};
use crate::game::system::rendering::WgpuRenderState;

include!("../../generated/systems_gen.rs");

pub struct SystemFactory;

impl CreateSystems for SystemFactory {
    fn create_physics(&self) -> PhysicsSystem {
        PhysicsSystem::default()
    }

    fn create_render_background(&self) -> RenderBackgroundSystem {
        RenderBackgroundSystem(RenderBackgroundSystemData)
    }

    fn create_render(&self) -> RenderSystem {
        RenderSystem::default()
    }
}
