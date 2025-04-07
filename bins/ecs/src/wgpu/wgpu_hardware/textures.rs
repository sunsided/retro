use tracing::instrument;
use wgpu::{Device, Queue};
use crate::wgpu::texture::{Texture, TextureType};

#[derive(Debug)]
pub struct Textures {
    /// A 1x1 normal map.
    pub fallback_normals: Texture,
    /// An example cover texture
    pub cover: Texture,
}

impl Textures {
    #[instrument(skip(device, queue))]
    pub fn create_from(device: &Device, queue: &Queue) -> Self {
        let fallback_normals = Self::create_fallback_normals(&device, queue);
        let cover = Self::create_cover(&device, queue);
        Self {
            fallback_normals,
            cover
        }
    }

    #[instrument(skip(device, queue))]
    fn create_fallback_normals(device: &Device, queue: &Queue) -> Texture {
        Texture::fallback_normal_map(device, queue)
    }

    #[instrument(skip(device, queue))]
    fn create_cover(device: &Device, queue: &Queue) -> Texture {
        Texture::from_file_with("textures/cover.webp", device, queue, TextureType::Diffuse, Some("cover.webp"))
            .expect("Failed to load cover texture")
    }
}
