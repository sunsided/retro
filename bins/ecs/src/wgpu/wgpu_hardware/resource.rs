use crate::wgpu::wgpu_hardware::DeviceAndQueue;

mod tile_renderer;

pub trait WgpuResource {
    fn recreate(&mut self, dq: &mut DeviceAndQueue);
}
