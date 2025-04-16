use std::sync::Arc;
use std::sync::atomic::AtomicU8;

#[derive(Debug)]
#[allow(dead_code)]
pub struct ResourceStateOwner(Arc<AtomicU8>);

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ResourceStateHandle(Arc<AtomicU8>);

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
#[allow(dead_code)]
pub enum ResourceState {
    #[default]
    Uninitialized = 0,
    Valid = 1,
    Stale = 2,
    Destroyed = 3
}

#[allow(dead_code)]
impl ResourceState {
    #[inline]
    pub const fn is_valid(self) -> bool {
        matches!(self, ResourceState::Valid)
    }

    #[inline]
    pub const fn is_stale(self) -> bool {
        matches!(self, ResourceState::Stale)
    }

    #[inline]
    pub const fn is_destroyed(self) -> bool {
        matches!(self, ResourceState::Destroyed)
    }

    #[inline]
    pub const fn is_uninitialized(self) -> bool {
        matches!(self, ResourceState::Uninitialized)
    }
}

#[allow(dead_code)]
impl ResourceStateOwner {
    pub fn new(state: ResourceState) -> Self {
        Self(Arc::new(AtomicU8::new(state as u8)))
    }

    pub fn store(&self, state: ResourceState) {
        self.0.store(state as u8, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn fork(&self) -> ResourceStateHandle {
        ResourceStateHandle(Arc::clone(&self.0))
    }
}

#[allow(dead_code)]
impl ResourceStateHandle {
    pub fn get(&self) -> ResourceState {
        match self.0.load(std::sync::atomic::Ordering::Relaxed) {
            0 => ResourceState::Uninitialized,
            1 => ResourceState::Valid,
            2 => ResourceState::Stale,
            3 => ResourceState::Destroyed,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        let flag = ResourceStateOwner::new(ResourceState::Valid);
        let clone_flag = flag.fork();
        assert_eq!(clone_flag.get(), ResourceState::Valid);

        flag.store(ResourceState::Destroyed);
        assert_eq!(clone_flag.get(), ResourceState::Destroyed);
    }
}
