use crate::wgpu::texture_sampler::TextureSampler;
use tracing::instrument;

#[derive(Debug)]
pub struct TextureSamplers {
    /// Sampler for linear, wrapping texture samples.
    pub linear: TextureSampler,
    /// Sampler for depth textures.
    #[allow(dead_code)]
    pub depth: TextureSampler,
    pub tile: TextureSampler,
    pub normals: TextureSampler,
}

impl TextureSamplers {
    #[instrument(skip_all)]
    pub fn create_from(device: &wgpu::Device) -> Self {
        let linear = TextureSampler::with_clamp_to_edge_and_linear_filtering(device);
        let depth = TextureSampler::depth_sampler(device);
        let tile = Self::create_tile(device);
        let normals = Self::create_normals(device);
        Self {
            linear,
            depth,
            tile,
            normals,
        }
    }

    #[instrument(skip_all)]
    pub fn create_tile(device: &wgpu::Device) -> TextureSampler {
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("tile"),
            mag_filter: wgpu::FilterMode::Nearest, // or .Linear, your choice
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            lod_max_clamp: 4.0,
            ..Default::default()
        });

        TextureSampler { sampler }
    }

    #[instrument(skip_all)]
    pub fn create_normals(device: &wgpu::Device) -> TextureSampler {
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("normals"),
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            lod_min_clamp: 0.0,
            lod_max_clamp: 4.0,
            ..Default::default()
        });

        TextureSampler { sampler }
    }
}
