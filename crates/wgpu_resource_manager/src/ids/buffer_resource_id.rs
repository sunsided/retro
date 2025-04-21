use crate::ResourceId;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BufferResourceId(ResourceId);


#[allow(dead_code)]
impl BufferResourceId {
    pub fn next() -> Self {
        Self(ResourceId::next())
    }

    #[allow(dead_code)]
    pub fn into_inner(self) -> ResourceId {
        self.0
    }
}

impl PartialEq<ResourceId> for BufferResourceId {
    fn eq(&self, other: &ResourceId) -> bool {
        self.0 == *other
    }
}

impl Hash for BufferResourceId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}