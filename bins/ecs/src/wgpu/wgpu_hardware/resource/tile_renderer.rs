use crate::wgpu::wgpu_hardware::resource::WgpuResource;
use crate::wgpu::wgpu_hardware::DeviceAndQueue;

pub struct TileRendererResource {}

impl WgpuResource for TileRendererResource {
    fn recreate(&mut self, _dq: &mut DeviceAndQueue) {}
}
