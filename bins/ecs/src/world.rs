use crate::Entity;
use crate::archetypes::Archetype;
use crate::component_type_set::ComponentTypeSet;
use std::any::Any;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};

pub struct World {
    entity_counter: AtomicU32,
    archetypes: HashMap<ComponentTypeSet, Archetype>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entity_counter: AtomicU32::new(0),
            archetypes: HashMap::new(),
        }
    }

    pub fn spawn<T: 'static>(&mut self, components: Vec<Box<dyn Any>>) -> Entity {
        let entity = self.entity_counter.fetch_add(1, Ordering::Relaxed);
        let mut type_set = ComponentTypeSet::new();
        let mut component_map = HashMap::new();

        for c in components {
            let type_id = c.type_id();
            type_set.types.insert(type_id);
            component_map.insert(type_id, c);
        }

        let archetype = self
            .archetypes
            .entry(type_set)
            .or_insert_with(crate::archetypes::Archetype::new);
        archetype.push(entity, component_map);

        entity
    }
}
