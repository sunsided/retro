use tracing::{debug, instrument};
use wgpu::SurfaceConfiguration;
use crate::wgpu::texture::Texture;

#[derive(Debug)]
pub struct DepthBuffers {
    /// Depth buffer for the render surface.
    pub screen: Texture,
}

impl DepthBuffers {
    #[instrument(skip(device, surface_config))]
    pub fn create_from(device: &wgpu::Device, surface_config: &SurfaceConfiguration) -> Self {
        let screen = Self::create_depth_texture(device, surface_config);
        Self {
            screen,
        }
    }

    #[instrument(skip(self, device, surface_config))]
    pub fn recreate(&mut self, device: &wgpu::Device, surface_config: &SurfaceConfiguration) {
        debug!("Recreating depth buffers");
        self.screen = Self::create_depth_texture(device, surface_config);
    }

    #[instrument(skip(device, surface_config))]
    fn create_depth_texture(device: &wgpu::Device, surface_config: &SurfaceConfiguration) -> Texture {
        Texture::depth_texture(&device, &surface_config, "depth_texture")
    }
}
