use crate::component::{Health, Position, Velocity};
use crate::entity::EntityId;

/// An archetype is a group of entities that share **exactly the same component types**.
/// Each archetype stores its **entities in rows**, and its **components in columns**,
/// like a table in a column-oriented DB.
pub trait Archetype {
    const ID: ArchetypeId;
}

pub type ArchetypeId = u64;

/// Identifies an entity's location by specifying the archetype it is in ([`Archetype::ID`]),
/// as well as the index within the archetype.
pub type ArchetypeRef = (ArchetypeId, EntityId);

macro_rules! generate_archetype {
    ($id:literal => $name:ident; $($field_name:ident => $field_type:ty),*) => {
        pub struct $name {
            pub entities: Vec<EntityId>,
            $(pub $field_name: Vec<$field_type>,)*
        }

        impl Archetype for $name {
            const ID: ArchetypeId = $id;
        }
    };
}

/// This archetype holds all entities with **Position AND Velocity** (and no other components).
generate_archetype!(0 => ArchetypeA; positions => Position, velocities => Velocity);

/// These entities have **Position and Health, but no Velocity**.
generate_archetype!(1 => ArchetypeB; positions => Position, healths => Health);

/// Entities with **only a Position**.
generate_archetype!(2 => ArchetypeC; positions => Position);

/// Entities with **only Health**; they would be "invisible" in the game, but still exist.
generate_archetype!(3 => ArchetypeD; healths => Health);
