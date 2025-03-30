use crate::archetype::*;
use crate::component::*;

include!(concat!(env!("OUT_DIR"), "/systems.gen.rs"));

#[derive(Debug, Clone)]
pub struct PhysicsSystemData;

impl ApplyPhysicsSystem for PhysicsSystem {
    fn apply(&self, velocity: &VelocityComponent, position: &mut PositionComponent) {
        todo!()
    }
}
