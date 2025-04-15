use crate::ConfiguredWgpuResourceManager;
use std::borrow::{Borrow, BorrowMut};
use wgpu::Instance;

impl AsRef<Instance> for ConfiguredWgpuResourceManager {
    #[inline]
    fn as_ref(&self) -> &Instance {
        self.instance()
    }
}

impl AsMut<Instance> for ConfiguredWgpuResourceManager {
    #[inline]
    fn as_mut(&mut self) -> &mut Instance {
        self.instance_mut()
    }
}

impl Borrow<Instance> for ConfiguredWgpuResourceManager {
    #[inline]
    fn borrow(&self) -> &Instance {
        self.instance()
    }
}

impl BorrowMut<Instance> for ConfiguredWgpuResourceManager {
    #[inline]
    fn borrow_mut(&mut self) -> &mut Instance {
        self.instance_mut()
    }
}
