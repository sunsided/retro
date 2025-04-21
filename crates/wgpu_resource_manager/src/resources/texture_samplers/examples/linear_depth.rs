use std::convert::Infallible;
use std::fmt::Debug;
use crate::{wgpu_label, GpuContext, RecreateReason, Resource, SurfaceInfo};
use crate::resources::texture_samplers::texture_sampler::TextureSampler;

/// A sampler optimized for depth textures.
///
/// This sampler uses the following settings:
/// - Address modes for U, V, and W are set to `ClampToEdge`.
/// - Both magnification and minification filters are set to `Linear`.
/// - Mipmap filter is set to `Nearest`.
/// - Comparison function is set to `LessEqual`.
/// - LOD (Level of Detail) clamping is set with a minimum of 0.0 and a maximum of 100.0.
#[derive(Debug)]
pub struct LinearDepthTextureSampler;

impl Resource for LinearDepthTextureSampler
{
    type Output = TextureSampler;
    type Error = Infallible;

    fn recreate<'device>(&mut self, ctx: &GpuContext<'device>, _surface_info: &SurfaceInfo, _reason: RecreateReason) -> Result<Self::Output, Self::Error> {
        // TODO: Don't recreate if not necessary

        let label = wgpu_label!("linear_depth");

        let desc = wgpu::SamplerDescriptor {
            label: Some(&label),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            compare: Some(wgpu::CompareFunction::LessEqual),
            lod_min_clamp: 0.0,
            lod_max_clamp: 100.0,
            ..Default::default()
        };
        let sampler = ctx.device.create_sampler(&desc);

        Ok(TextureSampler::new(sampler))
    }
}
