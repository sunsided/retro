use crate::ResourceId;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BindGroupResourceId(ResourceId);


#[allow(dead_code)]
impl BindGroupResourceId {
    pub fn next() -> Self {
        Self(ResourceId::next())
    }

    #[allow(dead_code)]
    pub fn into_inner(self) -> ResourceId {
        self.0
    }
}

impl PartialEq<ResourceId> for BindGroupResourceId {
    fn eq(&self, other: &ResourceId) -> bool {
        self.0 == *other
    }
}

impl Hash for BindGroupResourceId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}
