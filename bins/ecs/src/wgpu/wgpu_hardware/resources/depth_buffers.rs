use crate::wgpu::texture::Texture;
use tracing::{debug, instrument};
use wgpu::SurfaceConfiguration;

#[derive(Debug)]
pub struct DepthBuffers {
    /// Depth buffer for the render surface.
    pub screen: Texture,
}

impl DepthBuffers {
    #[instrument(skip_all)]
    pub fn create_from(device: &wgpu::Device, surface_config: &SurfaceConfiguration) -> Self {
        let screen = Self::create_depth_texture(device, surface_config);
        Self { screen }
    }

    #[instrument(skip_all)]
    pub fn recreate(&mut self, device: &wgpu::Device, surface_config: &SurfaceConfiguration) {
        debug!("Recreating depth buffers");
        self.screen = Self::create_depth_texture(device, surface_config);
    }

    #[instrument(skip_all)]
    fn create_depth_texture(
        device: &wgpu::Device,
        surface_config: &SurfaceConfiguration,
    ) -> Texture {
        Texture::depth_texture(&device, &surface_config, "depth_texture")
    }
}
