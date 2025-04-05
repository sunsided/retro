use crate::archetype::*;
use crate::component::*;
use crate::world::{FrameContext, RenderState, WorldState, WorldCommandSender};
use tracing::trace;

include!("../generated/systems_gen.rs");

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
        &self,
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

#[derive(Debug, Clone)]
pub struct RenderSystemData;

impl ApplyRenderSystem for RenderSystem {
    fn apply_single(
        &self,
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
