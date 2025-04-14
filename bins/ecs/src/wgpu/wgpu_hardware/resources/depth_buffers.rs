use crate::wgpu::texture::Texture;
use tracing::{debug, instrument};
use wgpu::SurfaceConfiguration;

#[derive(Debug)]
pub struct DepthBuffers {
    /// Depth buffer for the render surface.
    pub screen: Texture,
    /// Depth buffer for the VGA offscreen buffer.
    pub vga_offscreen: Texture,
}

impl DepthBuffers {
    #[instrument(skip_all)]
    pub fn create_from(device: &wgpu::Device, surface_config: &SurfaceConfiguration) -> Self {
        let screen = Self::create_depth_texture(device, surface_config);
        let vga_offscreen = Self::create_vga_offscreen(device);
        Self {
            screen,
            vga_offscreen,
        }
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

    #[instrument(skip_all)]
    fn create_vga_offscreen(device: &wgpu::Device) -> Texture {
        let size = wgpu::Extent3d {
            width: Texture::VGA_WIDTH,
            height: Texture::VGA_HEIGHT,
            depth_or_array_layers: 1,
        };
        let desc = wgpu::TextureDescriptor {
            label: Some("vga_offscreen_depth_texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: Texture::DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        };
        let texture = device.create_texture(&desc);

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        Texture { texture, view }
    }
}
