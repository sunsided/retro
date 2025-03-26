use crate::components::{Health, Position, Velocity};
use crate::entity::EntityId;

/// An archetype is a group of entities that share **exactly the same component types**.
/// Each archetype stores its **entities in rows**, and its **components in columns**,
/// like a table in a column-oriented DB.
pub struct Archetype {}

/// This archetype holds all entities with **Position AND Velocity** (and no other components).
pub struct ArchetypeA {
    pub entities: Vec<EntityId>,
    pub position: Vec<Position>,
    pub velocity: Vec<Velocity>,
}

/// These entities have **Position and Health, but no Velocity**.
pub struct ArchetypeB {
    pub entities: Vec<EntityId>,
    pub position: Vec<Position>,
    pub healths: Vec<Health>,
}

/// Entities with **only a Position**.
pub struct ArchetypeC {
    pub entities: Vec<EntityId>,
    pub position: Vec<Position>,
}

/// Entities with **only Health**; they would be "invisible" in the game, but still exist.
pub struct ArchetypeD {
    pub entities: Vec<EntityId>,
    pub position: Vec<Health>,
}
