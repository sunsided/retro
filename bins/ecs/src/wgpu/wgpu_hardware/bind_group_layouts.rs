use crate::wgpu::uniforms::FrameGlobalUniform;
use tracing::instrument;
use wgpu::BindGroupLayout;

#[derive(Debug)]
pub struct BindGroupLayouts {
    /// Frame-global uniforms.
    ///
    /// ## Bindings
    /// - `@binding(0)`: The frame-global uniforms
    pub frame_global_uniform: BindGroupLayout,

    /// A texture bind group layout with four bindings:
    ///
    /// ## Bindings
    /// - `@binding(0)`: The diffuse texture
    /// - `@binding(1)`: The diffuse texture sampler
    /// - `@binding(2)`: The normal map texture
    /// - `@binding(3)`: The normal map sampler
    pub diffuse_and_normal: BindGroupLayout,

    /// A bind group for the camera uniforms.
    ///
    /// ## Bindings
    /// - `@binding(0)`: The camera uniforms
    pub camera_uniforms: BindGroupLayout,

    /// A bind group for the lighting uniforms.
    ///
    /// ## Bindings
    /// - `@binding(0)`: The lighting uniforms
    pub lighting_uniforms: BindGroupLayout,

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
        let frame_global_uniform = Self::create_frame_global(device);
        let diffuse_and_normal = Self::create_diffuse_and_normal(device);
        let camera_uniforms = Self::create_camera_buffer_uniforms(device);
        let lighting_uniforms = Self::create_lighting_uniforms(device);
        let vga_fullscreen = Self::create_vga_fullscreen(device);
        Self {
            frame_global_uniform,
            diffuse_and_normal,
            camera_uniforms,
            lighting_uniforms,
            vga_fullscreen,
        }
    }

    fn create_frame_global(device: &wgpu::Device) -> BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("frame_global"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(size_of::<FrameGlobalUniform>() as _),
                },
                count: None,
            }],
        })
    }

    #[instrument(skip_all)]
    fn create_diffuse_and_normal(device: &wgpu::Device) -> BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                // Diffuse texture
                wgpu::BindGroupLayoutEntry {
                    binding: 0,                               // @binding(0)
                    visibility: wgpu::ShaderStages::FRAGMENT, // NONE, VERTEX, FRAGMENT or COMPUTE
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1, // @binding(1)
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    // This should match the filterable field of the
                    // corresponding Texture entry above.
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                // normal map
                wgpu::BindGroupLayoutEntry {
                    binding: 2, // @binding(2)
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3, // @binding(3)
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("diffuse_and_normal"),
        })
    }

    #[instrument(skip_all)]
    fn create_camera_buffer_uniforms(device: &wgpu::Device) -> BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None, // TODO: Maybe set this later
                },
                count: None,
            }],
            label: Some("camera_buffer"),
        })
    }

    #[instrument(skip_all)]
    fn create_lighting_uniforms(device: &wgpu::Device) -> BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("lighting_uniforms"),
        })
    }

    #[instrument(skip_all)]
    fn create_vga_fullscreen(device: &wgpu::Device) -> BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("vga_fullscreen"),
            entries: &[
                // Binding 0: the uniform
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                // Binding 1 = texture
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                // Binding 2 = sampler
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                // Binding 3 = (per-frame) entity ID
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Uint,
                    },
                    count: None,
                },
                // Binding 4 = depth buffer
                wgpu::BindGroupLayoutEntry {
                    binding: 4,
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
