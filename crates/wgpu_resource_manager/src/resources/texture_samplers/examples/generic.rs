use std::convert::Infallible;
use std::fmt::Debug;
use wgpu::SamplerDescriptor;
use crate::{GpuContext, RecreateReason, Resource, SurfaceInfo};
use crate::resources::texture_samplers::texture_sampler::TextureSampler;

#[derive(Debug)]
pub struct GenericTextureSampler {
    desc: SamplerDescriptor<'static>
}

impl Resource for GenericTextureSampler
{
    type Output = TextureSampler;
    type Error = Infallible;

    fn recreate<'device>(&mut self, ctx: &GpuContext<'device>, _surface_info: &SurfaceInfo, _reason: RecreateReason) -> Result<Self::Output, Self::Error> {
        // TODO: Don't recreate if not necessary

        let sampler = ctx.device.create_sampler(&self.desc);
        Ok(TextureSampler::new(sampler))
    }
}
