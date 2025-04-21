use crate::DeviceId;

pub struct GpuContext<'device> {
    pub device_id: DeviceId,
    pub device: &'device wgpu::Device,
    pub queue: &'device wgpu::Queue,
}
