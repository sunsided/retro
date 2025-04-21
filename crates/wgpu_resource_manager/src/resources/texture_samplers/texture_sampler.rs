use crate::{ResourceData};
use crate::resources::texture_samplers::TextureSamplerResourceData;

#[derive(Debug)]
pub struct TextureSampler {
    pub sampler: wgpu::Sampler,
}

impl ResourceData for TextureSampler {}

impl TextureSamplerResourceData for TextureSampler {
    fn texture_sampler(&self) -> &wgpu::Sampler {
        &self.sampler
    }
}

impl TextureSampler {
    pub(crate) const fn new(sampler: wgpu::Sampler) -> Self {
        Self {
            sampler
        }
    }
}
