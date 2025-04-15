use crate::wgpu::wgpu_hardware::resource::camera_uniform::CameraUniformResource;
use crate::wgpu::wgpu_hardware::resource::frame_global_uniform::FrameGlobalUniformResource;
use crate::wgpu::wgpu_hardware::resource::light_uniform::LightUniformResource;
use wgpu::{BindGroupLayout, Device, PipelineLayout};
use wgpu_resource_manager::wgpu_label;

#[derive(Debug)]
pub struct StandardRenderLayout {
    /// A texture bind group layout with four bindings:
    ///
    /// ## Bindings
    /// - `@binding(0)`: The albedo texture
    /// - `@binding(1)`: The albedo texture sampler
    /// - `@binding(2)`: The normal map texture
    /// - `@binding(3)`: The normal map sampler
    pub texture_bind_group: BindGroupLayout,
    pub pipeline: PipelineLayout,
}

impl StandardRenderLayout {
    pub fn create_from(
        device: &Device,
        frame_global: &FrameGlobalUniformResource,
        camera: &CameraUniformResource,
        light: &LightUniformResource,
    ) -> Self {
        let bind_group = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some(wgpu_label!("albedo_and_normal")),
            entries: &[
                // Binding 0 = albedo texture
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
                // Binding 1 = albedo sampler
                wgpu::BindGroupLayoutEntry {
                    binding: 1, // @binding(1)
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                // Binding 2 = normal map
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
                // Binding 2 = normal map sampler
                wgpu::BindGroupLayoutEntry {
                    binding: 3, // @binding(3)
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let pipeline = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some(wgpu_label!("standard_render")),
            bind_group_layouts: &[
                &frame_global.layout, // `@group(0)` in the shader
                &camera.layout,       // `@group(1)` in the shader
                &light.layout,        // `@group(2)` in the shader
                &bind_group,          // `@group(3)` in the shader
            ],
            push_constant_ranges: &[],
        });

        Self {
            texture_bind_group: bind_group,
            pipeline,
        }
    }
}
