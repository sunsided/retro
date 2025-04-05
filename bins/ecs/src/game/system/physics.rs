use std::convert::Infallible;
use crate::game::archetype::EntityId;
use crate::game::component::{PositionComponent, VelocityComponent};
use crate::game::system::{ApplyPhysicsSystem, PhysicsSystem};
use crate::game::world::{FrameContext, WorldCommandSender};

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
