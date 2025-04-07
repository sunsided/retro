use tracing::instrument;
use wgpu::{Device, Queue, TextureFormat};
use crate::wgpu::texture::{Texture, TextureType};

#[derive(Debug)]
pub struct Textures {
    /// My silly little VGA offscreen buffer (whopping 320x200 pixels!)
    pub vga_offscreen: Texture,
    /// A 1x1 normal map.
    pub fallback_normals: Texture,
    /// An example cover texture
    pub cover: Texture,
}

impl Textures {
    #[instrument(skip_all)]
    pub fn create_from(device: &Device, queue: &Queue, surface_format: TextureFormat) -> Self {
        let vga_offscreen = Self::create_vga_offscreen(&device, surface_format);
        let fallback_normals = Self::create_fallback_normals(&device, queue);
        let cover = Self::create_cover(&device, queue);
        Self {
            vga_offscreen,
            fallback_normals,
            cover
        }
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
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING,
                // | wgpu::TextureUsages::COPY_DST, // TODO: Only needed if we write_texture; not needed if it's just a render target
            label: Some("vga_offscreen"),
            dimension: wgpu::TextureDimension::D2,
            view_formats: &[],
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        Texture {
            texture,
            view
        }
    }

    #[instrument(skip_all)]
    fn create_fallback_normals(device: &Device, queue: &Queue) -> Texture {
        Texture::fallback_normal_map(device, queue)
    }

    #[instrument(skip(device, queue))]
    fn create_cover(device: &Device, queue: &Queue) -> Texture {
        Texture::from_file_with("textures/cover.webp", device, queue, TextureType::Diffuse, Some("cover.webp"))
            .expect("Failed to load cover texture")
    }
}
