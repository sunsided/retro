
use crate::surface_info::SurfaceInfo;
use crate::{wgpu_label, DependentResource, GpuContext, RecreateReason, Resource, ResourceId, SurfaceSensitive, TextureWithView};
use std::convert::Infallible;

#[derive(Debug)]
pub struct SurfaceDepthTexture;

impl Resource for SurfaceDepthTexture {
    type Output = TextureWithView;
    type Error = Infallible;

    fn recreate<'device>(
        &mut self,
        ctx: &GpuContext<'device>,
        surface_info: &SurfaceInfo,
        _reason: RecreateReason,
    ) -> Result<Self::Output, Self::Error> {
        // TODO: Don't recreate if not necessary

        let size = wgpu::Extent3d {
            width: surface_info.width.max(1),
            height: surface_info.height.max(1),
            depth_or_array_layers: 1,
        };

        let texture = ctx.device.create_texture(&wgpu::TextureDescriptor {
            label: Some(wgpu_label!("surface_depth_map")),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        Ok(Self::Output::new(texture, view))
    }
}

impl SurfaceSensitive for SurfaceDepthTexture {
    fn is_surface_dependent(&self) -> bool {
        true
    }

    fn must_recreate(&self, current: &SurfaceInfo, previous: &SurfaceInfo) -> bool {
        current.size_changed(&previous)
    }
}

impl DependentResource for SurfaceDepthTexture {
    fn get_dependencies(&self) -> &[ResourceId] {
        &[]
    }
}
