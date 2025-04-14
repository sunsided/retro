use crate::wgpu::wgpu_hardware::DeviceAndQueue;
use crate::wgpu::wgpu_hardware::resource::WgpuResource;

pub struct TileRendererResource {}

impl WgpuResource for TileRendererResource {
    fn recreate(&mut self, dq: &mut DeviceAndQueue) {}
}
