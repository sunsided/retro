use crate::wgpu::wgpu_hardware::resource::WgpuResource;
use wgpu::Device;

pub struct TileRendererResource {}

impl WgpuResource for TileRendererResource {
    fn recreate(&mut self, _d: &Device) {}
}
