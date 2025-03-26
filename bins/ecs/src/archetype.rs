use crate::component::{Health, Position, Velocity};
use crate::entity::EntityId;

/// An archetype is a group of entities that share **exactly the same component types**.
/// Each archetype stores its **entities in rows**, and its **components in columns**,
/// like a table in a column-oriented DB.
pub trait Archetype {}

macro_rules! generate_archetype {
    ($name:ident; $($field_name:ident => $field_type:ty),*) => {
        pub struct $name {
            pub entities: Vec<EntityId>,
            $(pub $field_name: Vec<$field_type>,)*
        }

        impl Archetype for $name {}
    };
}

/// This archetype holds all entities with **Position AND Velocity** (and no other components).
generate_archetype!(ArchetypeA; positions => Position, velocities => Velocity);

/// These entities have **Position and Health, but no Velocity**.
generate_archetype!(ArchetypeB; positions => Position, healths => Health);

/// Entities with **only a Position**.
generate_archetype!(ArchetypeC; positions => Position);

/// Entities with **only Health**; they would be "invisible" in the game, but still exist.
generate_archetype!(ArchetypeD; healths => Health);
