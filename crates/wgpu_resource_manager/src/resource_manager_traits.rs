use crate::WgpuResourceManager;
use std::borrow::{Borrow, BorrowMut};
use wgpu::{Backends, Instance, InstanceDescriptor};

impl From<Instance> for WgpuResourceManager {
    #[inline]
    fn from(instance: Instance) -> Self {
        Self::from_instance(instance)
    }
}

impl From<Backends> for WgpuResourceManager {
    #[inline]
    fn from(backends: Backends) -> Self {
        Self::from_backends(backends)
    }
}

impl From<InstanceDescriptor> for WgpuResourceManager {
    #[inline]
    fn from(descriptor: InstanceDescriptor) -> Self {
        Self::from_descriptor(&descriptor)
    }
}

impl From<&InstanceDescriptor> for WgpuResourceManager {
    #[inline]
    fn from(descriptor: &InstanceDescriptor) -> Self {
        Self::from_descriptor(descriptor)
    }
}

impl AsRef<Instance> for WgpuResourceManager {
    #[inline]
    fn as_ref(&self) -> &Instance {
        self.instance()
    }
}

impl AsMut<Instance> for WgpuResourceManager {
    #[inline]
    fn as_mut(&mut self) -> &mut Instance {
        self.instance_mut()
    }
}

impl Borrow<Instance> for WgpuResourceManager {
    #[inline]
    fn borrow(&self) -> &Instance {
        self.instance()
    }
}

impl BorrowMut<Instance> for WgpuResourceManager {
    #[inline]
    fn borrow_mut(&mut self) -> &mut Instance {
        self.instance_mut()
    }
}
