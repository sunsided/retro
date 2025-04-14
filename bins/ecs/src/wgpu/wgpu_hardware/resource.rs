use crate::wgpu::wgpu_hardware::DeviceAndQueue;

pub mod tile_renderer;

#[allow(dead_code)]
pub trait WgpuResource {
    fn recreate(&mut self, dq: &mut DeviceAndQueue);
}
