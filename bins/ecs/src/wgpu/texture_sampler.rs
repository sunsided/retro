use derive_more::{AsRef, Deref, DerefMut};

/// A struct representing a GPU texture sampler.
///
/// This structure encapsulates a `wgpu::Sampler` object, which defines how a texture
/// is sampled when applied to a surface. It includes settings for filtering,
/// addressing modes, and mipmap behavior.
///
/// ## See also
///
/// - [`wgpu::Sampler`](https://docs.rs/wgpu/latest/wgpu/struct.Sampler.html)
/// - [`Texture`](crate::texture::Texture)

#[derive(Debug, Deref, DerefMut, AsRef)]
pub struct TextureSampler {
    /// The underlying GPU texture sampler resource.
    pub sampler: wgpu::Sampler,
}

impl TextureSampler {
    /// Creates a new `TextureSampler` with predefined properties.
    ///
    /// This sampler uses the following settings:
    /// - Address modes for U, V, and W are set to `ClampToEdge`.
    /// - The magnification filter is set to `Linear`.
    /// - The minification filter and mipmap filter are set to `Nearest`.
    ///
    /// # Parameters
    /// - `device`: The `wgpu::Device` used to create the sampler.
    ///
    /// # Returns
    /// A new instance of `TextureSampler`.
    pub fn with_clamp_to_edge_and_linear_filtering(device: &wgpu::Device) -> Self {
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Self { sampler }
    }

    /// Creates a sampler optimized for depth textures.
    ///
    /// This sampler uses the following settings:
    /// - Address modes for U, V, and W are set to `ClampToEdge`.
    /// - Both magnification and minification filters are set to `Linear`.
    /// - Mipmap filter is set to `Nearest`.
    /// - Comparison function is set to `LessEqual`.
    /// - LOD (Level of Detail) clamping is set with a minimum of 0.0 and a maximum of 100.0.
    ///
    /// # Parameters
    /// - `device`: The `wgpu::Device` used to create the sampler.
    ///
    /// # Returns
    /// A new `TextureSampler` configured for depth textures.
    ///
    /// # See also
    /// - [`Texture::depth_texture`](crate::wgpu::texture::Texture::depth_texture)
    pub fn depth_sampler(device: &wgpu::Device) -> Self {
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
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
        });
        Self { sampler }
    }
}
