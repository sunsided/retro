use std::convert::Infallible;
use std::fmt::Debug;
use crate::{wgpu_label, GpuContext, RecreateReason, Resource, SurfaceInfo};
use crate::resources::texture_samplers::texture_sampler::TextureSampler;

/// A linear texture sampler.
///
/// This sampler uses the following settings:
/// - Address modes for U, V, and W are set to `ClampToEdge`.
/// - The magnification filter is set to `Linear`.
/// - The minification filter and mipmap filter are set to `Nearest`.
#[derive(Debug)]
pub struct LinearTextureSampler;

impl Resource for LinearTextureSampler
{
    type Output = TextureSampler;
    type Error = Infallible;

    fn recreate<'device>(&mut self, ctx: &GpuContext<'device>, _surface_info: &SurfaceInfo, _reason: RecreateReason) -> Result<Self::Output, Self::Error> {
        // TODO: Don't recreate if not necessary

        let label = wgpu_label!("linear");

        let desc = wgpu::SamplerDescriptor {
            label: Some(&label),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        };
        let sampler = ctx.device.create_sampler(&desc);

        Ok(TextureSampler::new(sampler))
    }
}
