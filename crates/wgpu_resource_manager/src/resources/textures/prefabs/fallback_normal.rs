use crate::surface_info::SurfaceInfo;
use crate::{wgpu_label, DependentResource, GpuContext, RecreateReason, Resource, ResourceId, SurfaceSensitive, TextureWithView};
use std::convert::Infallible;

#[derive(Debug)]
pub struct FallbackNormalTexture;

impl Resource for FallbackNormalTexture {
    type Output = TextureWithView;

    type Error = Infallible;

    fn recreate<'device>(
        &mut self,
        ctx: &GpuContext<'device>,
        _surface_info: &SurfaceInfo,
        _reason: RecreateReason,
    ) -> Result<Self::Output, Self::Error> {
        // TODO: Don't recreate if not necessary

        // Flat normal: (0.5, 0.5, 1.0, 1.0) encoded as [128, 128, 255, 255]
        let normal_pixel = [128u8, 128u8, 255u8, 255u8];

        let size = wgpu::Extent3d {
            width: 1,
            height: 1,
            depth_or_array_layers: 1,
        };

        let texture = ctx.device.create_texture(&wgpu::TextureDescriptor {
            label: Some(wgpu_label!("fallback_normal_map")),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            // Use linear format for normal maps
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        ctx.queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            &normal_pixel,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * 1),
                rows_per_image: Some(1),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        Ok(Self::Output::new(texture, view))
    }
}

impl SurfaceSensitive for FallbackNormalTexture {
    fn is_surface_dependent(&self) -> bool {
        false
    }

    fn must_recreate(&self, _current: &SurfaceInfo, _previous: &SurfaceInfo) -> bool {
        false
    }
}

impl DependentResource for FallbackNormalTexture {
    fn get_dependencies(&self) -> &[ResourceId] {
        &[]
    }
}
