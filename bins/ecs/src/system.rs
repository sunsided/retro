use crate::archetype::*;
use crate::component::*;

include!(concat!(env!("OUT_DIR"), "/systems.gen.rs"));

impl Default for PhysicsSystem {
    fn default() -> Self {
        Self(PhysicsSystemData)
    }
}

#[derive(Debug, Clone)]
pub struct PhysicsSystemData;

impl ApplyPhysicsSystem for PhysicsSystem {
    fn apply_single(&self, velocity: &VelocityComponent, position: &mut PositionComponent) {
        todo!()
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
        todo!()
    }
}
