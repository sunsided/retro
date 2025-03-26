use crate::Entity;
use crate::archetypes::Archetype;
use crate::component_type_set::ComponentTypeSet;
use crate::utils::get_two_mut;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};

pub type SystemFn = Box<dyn FnMut(&mut Archetype) + Send + Sync>;

pub struct World {
    entity_counter: AtomicU32,
    archetypes: HashMap<ComponentTypeSet, Archetype>,
    systems: Vec<(ComponentTypeSet, SystemFn)>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entity_counter: AtomicU32::new(0),
            archetypes: HashMap::new(),
            systems: Vec::new(),
        }
    }

    pub fn spawn(&mut self, components: Vec<Box<dyn Any + Send + Sync>>) -> Entity {
        let entity = self.entity_counter.fetch_add(1, Ordering::Relaxed);
        let mut type_set = ComponentTypeSet::new();
        let mut component_map = HashMap::new();

        for c in components {
            let type_id = c.type_id();
            type_set.insert(type_id);
            component_map.insert(type_id, c);
        }

        let archetype = self
            .archetypes
            .entry(type_set)
            .or_insert_with(Archetype::new);
        archetype.push(entity, component_map);

        entity
    }

    pub fn register_system<
        T: 'static,
        U: 'static,
        F: 'static + Fn(&mut T, &mut U) + Send + Sync,
    >(
        &mut self,
        f: F,
    ) {
        let mut required = ComponentTypeSet::new();
        let a_id = TypeId::of::<T>();
        let b_id = TypeId::of::<U>();
        required.insert(a_id);
        required.insert(b_id);

        let wrapper: SystemFn = Box::new(move |arch: &mut Archetype| {
            let (comp_a, comp_b) = get_two_mut(&mut arch.components, a_id, b_id).unwrap();

            for i in 0..arch.entities.len() {
                let a = comp_a.get_mut::<T>(i).unwrap();
                let b = comp_b.get_mut::<U>(i).unwrap();
                f(a, b);
            }
        });

        self.systems.push((required, wrapper));
    }

    pub fn run_systems(&mut self) {
        for (req, sys) in &mut self.systems {
            for (arch_types, arch) in &mut self.archetypes {
                if req.is_subset_of(arch_types) {
                    sys(arch);
                }
            }
        }
    }

    pub fn run_systems_parallel(&mut self) {
        for (req, sys) in &mut self.systems {
            for (arch_types, arch) in &mut self.archetypes {
                if req.is_subset_of(arch_types) {
                    sys(arch);
                }
            }
        }
    }
}
