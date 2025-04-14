use crate::wgpu::wgpu_hardware::DeviceAndQueue;
use wgpu::Device;

pub mod frame_global_uniform;
mod sky_renderer;
pub mod tile_renderer;

#[allow(dead_code)]
pub trait WgpuResource {
    fn recreate(&mut self, device: &Device);
}
