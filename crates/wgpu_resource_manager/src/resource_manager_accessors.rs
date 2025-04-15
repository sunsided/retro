use crate::{AdapterDeviceAndQueue, SurfaceWrapper, WgpuResourceManagerId};
use wgpu::{Adapter, Instance};

pub trait WgpuResourceManagerAccessors {
    fn id(&self) -> WgpuResourceManagerId;

    fn instance(&self) -> &Instance;

    fn instance_mut(&mut self) -> &mut Instance;

    fn adapter(&self) -> &Adapter;

    fn adapter_device_and_queue(&self) -> &AdapterDeviceAndQueue;

    fn surface(&self) -> &SurfaceWrapper;

    fn is_ready(&self) -> bool;
}

pub trait WgpuResourceManagerRef {
    fn id(&self) -> WgpuResourceManagerId;

    fn instance(&self) -> Option<&Instance>;

    fn instance_mut(&mut self) -> Option<&mut Instance>;

    fn adapter(&self) -> Option<&Adapter>;

    fn adapter_device_and_queue(&self) -> Option<&AdapterDeviceAndQueue>;

    fn surface(&self) -> Option<&SurfaceWrapper>;

    fn is_ready(&self) -> bool;
}

impl<T> WgpuResourceManagerRef for T
where
    T: WgpuResourceManagerAccessors,
{
    fn id(&self) -> WgpuResourceManagerId {
        self.id()
    }

    fn instance(&self) -> Option<&Instance> {
        Some(self.instance())
    }

    fn instance_mut(&mut self) -> Option<&mut Instance> {
        Some(self.instance_mut())
    }

    fn adapter(&self) -> Option<&Adapter> {
        Some(self.adapter())
    }

    fn adapter_device_and_queue(&self) -> Option<&AdapterDeviceAndQueue> {
        Some(self.adapter_device_and_queue())
    }

    fn surface(&self) -> Option<&SurfaceWrapper> {
        Some(self.surface())
    }

    fn is_ready(&self) -> bool {
        self.is_ready()
    }
}
