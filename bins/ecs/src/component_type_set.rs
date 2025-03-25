use std::any::TypeId;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ComponentTypeSet {
    pub types: HashSet<TypeId>,
}

impl Hash for ComponentTypeSet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // To get a deterministic hash, hash sorted TypeIds
        let mut ids: Vec<_> = self.types.iter().collect();
        ids.sort_unstable();
        for id in ids {
            id.hash(state);
        }
    }
}

impl ComponentTypeSet {
    pub fn new() -> Self {
        Self {
            types: HashSet::new(),
        }
    }

    pub fn with(mut self, t: TypeId) -> Self {
        self.types.insert(t);
        self
    }
}
