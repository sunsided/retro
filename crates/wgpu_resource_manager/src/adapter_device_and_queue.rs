use crate::DeviceAndQueue;
use std::ops::{Deref, DerefMut};
use wgpu::{Adapter, Device, Queue};

#[derive(Debug)]
#[allow(dead_code)]
pub struct AdapterDeviceAndQueue {
    adapter: Adapter,
    device_and_queue: DeviceAndQueue,
}

impl AdapterDeviceAndQueue {
    #[inline]
    pub const fn new(adapter: Adapter, device_and_queue: DeviceAndQueue) -> Self {
        Self {
            adapter,
            device_and_queue,
        }
    }

    #[inline]
    pub const fn new_from(adapter: Adapter, device: Device, queue: Queue) -> Self {
        Self::new(adapter, DeviceAndQueue::new(device, queue))
    }

    #[inline]
    pub const fn adapter(&self) -> &Adapter {
        &self.adapter
    }

    #[inline]
    pub const fn adapter_mut(&mut self) -> &mut Adapter {
        &mut self.adapter
    }

    #[inline]
    pub const fn device_and_queue(&self) -> &DeviceAndQueue {
        &self.device_and_queue
    }

    #[inline]
    pub const fn device_and_queue_mut(&mut self) -> &mut DeviceAndQueue {
        &mut self.device_and_queue
    }
}

impl Deref for AdapterDeviceAndQueue {
    type Target = DeviceAndQueue;
    #[inline]
    fn deref(&self) -> &Self::Target {
        self.device_and_queue()
    }
}

impl DerefMut for AdapterDeviceAndQueue {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.device_and_queue_mut()
    }
}
