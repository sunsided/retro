use crate::wgpu::texture::{Texture, TextureType};
use tracing::{debug, instrument};
use wgpu::{Device, Queue, SurfaceConfiguration, TextureFormat};

#[derive(Debug)]
pub struct Textures {
    /// My silly little VGA offscreen buffer (whopping 320x200 pixels!)
    pub vga_offscreen: Texture,
    /// Frame entity ID buffer (surface-sized)
    pub surface_frame_entity_id: Texture,
    /// Frame entity ID buffer (VGA sized)
    pub vga_frame_entity_id: Texture,
    /// A 1x1 normal map.
    pub fallback_normals: Texture,
    /// An example cover texture
    pub cover: Texture,
    pub bricks_diffuse: Texture,
    pub bricks_normal: Texture,
}

impl Textures {
    #[instrument(skip_all)]
    pub fn create_from(
        device: &Device,
        queue: &Queue,
        surface_config: &SurfaceConfiguration,
    ) -> Self {
        let vga_offscreen = Self::create_vga_offscreen(&device, surface_config.format);
        let surface_frame_entity_id = Self::create_surface_frame_entity_id(&device, surface_config);
        let vga_frame_entity_id = Self::create_vga_frame_entity_id(&device);
        let fallback_normals = Self::create_fallback_normals(&device, queue);
        let cover = Self::create_cover(&device, queue);
        let bricks_diffuse = Self::create_bricks_diffuse(&device, queue);
        let bricks_normal = Self::create_bricks_normal(&device, queue);
        Self {
            vga_offscreen,
            surface_frame_entity_id,
            vga_frame_entity_id,
            fallback_normals,
            cover,
            bricks_diffuse,
            bricks_normal,
        }
    }

    #[instrument(skip_all)]
    pub fn recreate(&mut self, device: &Device, surface_config: &SurfaceConfiguration) {
        debug!("Recreating texture buffers");
        self.surface_frame_entity_id =
            Self::create_surface_frame_entity_id(&device, surface_config);
    }

    #[instrument(skip_all)]
    fn create_surface_frame_entity_id(
        device: &Device,
        surface_config: &SurfaceConfiguration,
    ) -> Texture {
        let size = wgpu::Extent3d {
            width: surface_config.width,
            height: surface_config.height,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            size,
            mip_level_count: 1,
            sample_count: 1,
            format: TextureFormat::R32Uint,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            label: Some("frame_entity_id"),
            dimension: wgpu::TextureDimension::D2,
            view_formats: &[],
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        Texture { texture, view }
    }

    #[instrument(skip_all)]
    fn create_vga_frame_entity_id(device: &Device) -> Texture {
        let size = wgpu::Extent3d {
            width: Texture::VGA_WIDTH,
            height: Texture::VGA_HEIGHT,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            size,
            mip_level_count: 1,
            sample_count: 1,
            format: TextureFormat::R32Uint,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            label: Some("frame_entity_id"),
            dimension: wgpu::TextureDimension::D2,
            view_formats: &[],
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        Texture { texture, view }
    }

    #[instrument(skip_all)]
    fn create_vga_offscreen(device: &Device, surface_format: TextureFormat) -> Texture {
        let size = wgpu::Extent3d {
            width: Texture::VGA_WIDTH,
            height: Texture::VGA_HEIGHT,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            size,
            mip_level_count: 1,
            sample_count: 1,
            format: surface_format, // TODO: Careful, that's BGRA or RGBA depending on platform; use sampler magic to convert
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            // | wgpu::TextureUsages::COPY_DST, // TODO: Only needed if we write_texture; not needed if it's just a render target
            label: Some("vga_offscreen"),
            dimension: wgpu::TextureDimension::D2,
            view_formats: &[],
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        Texture { texture, view }
    }

    #[instrument(skip_all)]
    fn create_fallback_normals(device: &Device, queue: &Queue) -> Texture {
        Texture::fallback_normal_map(device, queue)
    }

    #[instrument(skip(device, queue))]
    fn create_cover(device: &Device, queue: &Queue) -> Texture {
        Texture::from_file_with(
            "textures/cover.webp",
            device,
            queue,
            TextureType::Diffuse,
            Some("cover.webp"),
        )
        .expect("Failed to load cover texture")
    }

    #[instrument(skip(device, queue))]
    fn create_bricks_diffuse(device: &Device, queue: &Queue) -> Texture {
        Texture::from_file_with(
            "models/cube/cube-diffuse.jpg",
            device,
            queue,
            TextureType::Diffuse,
            Some("cube-diffuse.jpg"),
        )
        .expect("Failed to load cover texture")
    }

    #[instrument(skip(device, queue))]
    fn create_bricks_normal(device: &Device, queue: &Queue) -> Texture {
        Texture::from_file_with(
            "models/cube/cube-normal.png",
            device,
            queue,
            TextureType::Normal,
            Some("cube-normal.png"),
        )
        .expect("Failed to load cover texture")
    }
}
