use crate::wgpu::texture_sampler::TextureSampler;
use tracing::instrument;

#[derive(Debug)]
pub struct TextureSamplers {
    /// Sampler for linear, wrapping texture samples.
    pub linear: TextureSampler,
    /// Sampler for depth textures.
    #[allow(dead_code)]
    pub depth: TextureSampler,
    /// A retro sampler with jaggy edges.
    pub retro_sampler: TextureSampler,
}

impl TextureSamplers {
    #[instrument(skip_all)]
    pub fn create_from(device: &wgpu::Device) -> Self {
        let linear = TextureSampler::with_clamp_to_edge_and_linear_filtering(device);
        let depth = TextureSampler::depth_sampler(device);
        let retro_sampler = Self::create_retro_sampler(device);
        Self {
            linear,
            depth,
            retro_sampler,
        }
    }

    #[instrument(skip_all)]
    pub fn create_retro_sampler(device: &wgpu::Device) -> TextureSampler {
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Retro Sampler"),
            mag_filter: wgpu::FilterMode::Nearest, // or .Linear, your choice
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        TextureSampler { sampler }
    }
}
