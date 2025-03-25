use crate::Entity;
use crate::archetypes::Archetype;
use crate::component_type_set::ComponentTypeSet;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};

type SystemFn = Box<dyn Fn(&mut Archetype)>;

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

    pub fn spawn(&mut self, components: Vec<Box<dyn Any>>) -> Entity {
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

    pub fn register_system<T: 'static, U: 'static, F: 'static + Fn(&mut T, &mut U)>(
        &mut self,
        f: F,
    ) {
        let mut required = ComponentTypeSet::new();
        required.insert(TypeId::of::<T>());
        required.insert(TypeId::of::<U>());

        let wrapper: SystemFn = Box::new(move |arch: &mut Archetype| {
            let comp_a = arch.components.get(&TypeId::of::<T>()).unwrap();
            let comp_b = arch.components.get(&TypeId::of::<U>()).unwrap();

            for i in 0..arch.entities.len() {
                let mut a = comp_a.get_mut::<T>(i).unwrap();
                let mut b = comp_b.get_mut::<U>(i).unwrap();
                f(&mut a, &mut b);
            }
        });

        self.systems.push((required, wrapper));
    }

    pub fn run_systems(&mut self) {
        for (req, sys) in &self.systems {
            for (arch_types, arch) in &mut self.archetypes {
                if req.is_subset_of(arch_types) {
                    sys(arch);
                }
            }
        }
    }
}
