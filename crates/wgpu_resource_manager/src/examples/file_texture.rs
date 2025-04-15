use crate::resources::{GpuContext, Resource};
use crate::surface_info::SurfaceInfo;
use crate::wgpu_label;
use wgpu::{Texture, TextureView};

pub struct FileTexture {
    pub path: String,
    pub texture: Texture,
    pub view: TextureView,
}

#[derive(Debug, thiserror::Error)]
pub enum FileTextureError {
    #[error("File not found: {0}")]
    FileNotFound(#[from] std::io::Error),
    #[error("Failed to load image: {0}")]
    InvalidImage(#[from] image::ImageError),
}

impl Resource for FileTexture {
    type State = ();
    type Error = FileTextureError;

    fn recreate(
        &mut self,
        ctx: &GpuContext,
        _info: &SurfaceInfo,
        _state: &(),
    ) -> Result<(), Self::Error> {
        let bytes = std::fs::read(&self.path)?;

        let image = image::load_from_memory(&bytes)?.to_rgba8();

        let dimensions = image.dimensions();
        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        self.texture = ctx.device.create_texture(&wgpu::TextureDescriptor {
            label: Some(wgpu_label!("file_texture")),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        ctx.queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                aspect: wgpu::TextureAspect::All,
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            &image,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * 1),
                rows_per_image: Some(1),
            },
            size,
        );

        self.view = self.texture.create_view(&Default::default());

        Ok(())
    }
}
