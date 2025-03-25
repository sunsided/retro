use crate::Entity;
use crate::component_vec::ComponentVec;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::hash::Hash;

pub struct Archetype {
    entities: Vec<Entity>,
    components: HashMap<TypeId, ComponentVec>,
}

impl Archetype {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            components: HashMap::new(),
        }
    }

    pub fn push(&mut self, entity: Entity, components: HashMap<TypeId, Box<dyn Any>>) {
        self.entities.push(entity);
        for (type_id, component) in components {
            let entry = self
                .components
                .entry(type_id)
                .or_insert_with(ComponentVec::new);
            entry.data.push(component);
        }
    }
}
