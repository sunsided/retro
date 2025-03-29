use crate::entity::EntityId;

include!(concat!(env!("OUT_DIR"), "/systems.gen.rs"));

#[derive(Debug, Clone)]
pub struct PhysicsSystemData;
