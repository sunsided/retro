use tracing::instrument;
use crate::wgpu::texture_sampler::TextureSampler;

#[derive(Debug)]
pub struct TextureSamplers {
    /// Sampler for linear, wrapping texture samples.
    pub linear: TextureSampler,
    /// Sampler for depth textures.
    #[allow(dead_code)]
    pub depth: TextureSampler,
}

impl TextureSamplers {
    #[instrument(skip(device))]
    pub fn create_from(device: &wgpu::Device) -> Self {
        let linear = TextureSampler::with_clamp_to_edge_and_linear_filtering(&device);
        let depth = TextureSampler::depth_sampler(&device);
        Self {
            linear,
            depth
        }
    }
}
