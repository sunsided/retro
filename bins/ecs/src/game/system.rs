pub mod render_system;
mod vertex;

use crate::game::archetype::*;
use crate::game::component::*;
use crate::game::world::{FrameContext, RenderState, WorldState, WorldCommandSender};
use tracing::trace;
use crate::game::system::render_system::RenderSystemData;

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
    fn apply_single(
        &mut self,
        context: &FrameContext,
        _entity: EntityId,
        velocity: &VelocityComponent,
        position: &mut PositionComponent,
        _commands: &impl WorldCommandSender
    ) {
        let dt = context.fixed_time_secs;
        position.x += velocity.x * dt;
        position.y += velocity.y * dt;
        position.z += velocity.z * dt;
    }
}

impl Default for RenderSystem {
    fn default() -> Self {
        Self(RenderSystemData)
    }
}

impl ApplyRenderSystem for RenderSystem {
    fn apply_single(
        &mut self,
        context: &FrameContext,
        _state: &RenderState,
        position: &PositionComponent,
    ) {
        trace!(
            "Rendering position: {:?} in frame {}",
            position,
            context.frame_number
        );
    }
}
