use wgpu::Device;

pub mod camera_uniform;
pub mod frame_global_uniform;
pub mod light_uniform;
pub mod retro_uniform;
mod sky_renderer;
pub mod tile_renderer;

#[allow(dead_code)]
pub trait WgpuResource {
    fn recreate(&mut self, device: &Device);
}
