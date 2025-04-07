use winit::dpi::PhysicalSize;
use crate::wgpu::texture::Texture;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RetroUniform {
    pub window_aspect: f32,
    pub retro_aspect: f32,
}

impl Default for RetroUniform {
    fn default() -> Self {
        Self {
            window_aspect: 1.0,
            retro_aspect: Texture::VGA_WIDTH as f32 / Texture::VGA_HEIGHT as f32,
        }
    }
}

impl RetroUniform {
    pub fn resize_surface(&mut self, new_size: PhysicalSize<u32>) {
        self.update(new_size.width as f32 / new_size.height as f32);
    }

    pub fn update(&mut self, window_aspect: f32) {
        self.window_aspect = window_aspect;
    }
}
