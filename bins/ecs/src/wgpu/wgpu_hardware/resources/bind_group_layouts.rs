use tracing::instrument;
use wgpu::BindGroupLayout;

#[derive(Debug)]
pub struct BindGroupLayouts {
    /// Bind group for the VGA fullscreen shader.
    ///
    /// ## Bindings
    /// - `@binding(0)`: The texture
    /// - `@binding(1)`: The texture sampler
    /// - `@binding(3)`: The entity ID
    pub vga_fullscreen: BindGroupLayout,
}

impl BindGroupLayouts {
    #[instrument(skip_all)]
    pub fn create_from(device: &wgpu::Device) -> Self {
        let vga_fullscreen = Self::create_vga_fullscreen(&device);
        Self { vga_fullscreen }
    }

    #[instrument(skip_all)]
    fn create_vga_fullscreen(device: &wgpu::Device) -> BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some(wgpu_label!("vga_fullscreen")),
            entries: &[
                // Binding 0 = texture
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                // Binding 1 = sampler
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                // Binding 2 = (per-frame) entity ID
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Uint,
                    },
                    count: None,
                },
                // Binding 3 = depth buffer
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Depth,
                    },
                    count: None,
                },
            ],
        })
    }
}
