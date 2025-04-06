use std::convert::Infallible;
use crate::engine::{ApplyPhysicsSystem, FrameContext, PhysicsSystem, PositionComponent, VelocityComponent};

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
        velocity: &VelocityComponent,
        position: &mut PositionComponent,
    ) {
        let dt = context.fixed_time_secs;
        position.x += velocity.x * dt;
        position.y += velocity.y * dt;
        position.z += velocity.z * dt;
    }
}
