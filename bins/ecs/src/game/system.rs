pub mod render_system;
mod vertex;

use std::convert::Infallible;
use crate::game::system::render_system::RenderSystemData;
use crate::game::archetype::*;
use crate::game::component::*;
use crate::game::world::{FrameContext, RenderState, WorldCommandSender};

include!("../../generated/systems_gen.rs");

pub struct SystemFactory;

impl CreateSystems for SystemFactory {
    fn create_physics(&self) -> PhysicsSystem {
        PhysicsSystem::default()
    }

    fn create_render(&self) -> RenderSystem {
        RenderSystem::default()
    }
}

impl Default for PhysicsSystem {
    fn default() -> Self {
        Self(PhysicsSystemData)
    }
}

#[derive(Debug, Clone)]
pub struct PhysicsSystemData;

impl ApplyPhysicsSystem for PhysicsSystem {
    type Error = Infallible;

    fn apply_single(
        &mut self,
        context: &FrameContext,
        _entity: EntityId,
        velocity: &VelocityComponent,
        position: &mut PositionComponent,
        _commands: &impl WorldCommandSender,
    ) {
        let dt = context.fixed_time_secs;
        position.x += velocity.x * dt;
        position.y += velocity.y * dt;
        position.z += velocity.z * dt;
    }
}
