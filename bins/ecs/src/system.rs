use tracing::trace;
use crate::archetype::*;
use crate::component::*;

include!(concat!(env!("OUT_DIR"), "/systems.gen.rs"));

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
    fn apply_single(&self, velocity: &VelocityComponent, position: &mut PositionComponent) {
        // TODO: Factor in time delta
        position.x += velocity.x;
        position.y += velocity.y;
        position.z += velocity.z;
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
    fn apply_single(&self, position: &PositionComponent) {
        trace!("Rendering position: {:?}", position);
    }
}
