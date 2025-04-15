use crate::wgpu::texture::Texture;
use crate::wgpu::wgpu_hardware::resource::camera_uniform::CameraUniformResource;
use crate::wgpu::wgpu_hardware::resource::frame_global_uniform::FrameGlobalUniformResource;
use std::borrow::Borrow;
use wgpu::{Device, RenderPipeline, ShaderModule, SurfaceConfiguration, TextureFormat};
use wgpu_resource_manager::wgpu_label;

#[derive(Debug)]
pub struct SkyRendererResource {
    pub pipeline: RenderPipeline,
    _shader: ShaderModule,
}

impl SkyRendererResource {
    pub fn new(
        device: &Device,
        config: &SurfaceConfiguration,
        frame_global: &FrameGlobalUniformResource,
        camera: &CameraUniformResource,
    ) -> Self {
        let targets = [
            // Render to a texture
            Some(wgpu::ColorTargetState {
                format: config.format,
                blend: Some(wgpu::BlendState {
                    alpha: wgpu::BlendComponent::REPLACE,
                    color: wgpu::BlendComponent::REPLACE,
                }),
                write_mask: wgpu::ColorWrites::ALL,
            }),
            // Render to an entity ID target
            Some(wgpu::ColorTargetState {
                format: TextureFormat::R32Uint, // entity ID target
                blend: None,
                write_mask: wgpu::ColorWrites::ALL,
            }),
        ];

        let shader = wgpu::ShaderModuleDescriptor {
            label: Some(wgpu_label!("sky.wgsl")),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../../shaders/sky.wgsl").into()),
        };
        let shader = device.create_shader_module(shader);

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some(wgpu_label!("sky_pipeline")),
            bind_group_layouts: &[&frame_global.layout, &camera.layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(wgpu_label!("sky_render")),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[], // We use a full-screen triangle—no vertex buffer needed
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &targets,
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: Some(wgpu::DepthStencilState {
                format: Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::LessEqual, // sic!
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None, // Allows wgpu to cache shader compilation data. Only really useful for Android build targets.
        });

        Self {
            pipeline,
            _shader: shader,
        }
    }
}

impl Borrow<RenderPipeline> for SkyRendererResource {
    fn borrow(&self) -> &RenderPipeline {
        &self.pipeline
    }
}

impl Borrow<RenderPipeline> for &SkyRendererResource {
    fn borrow(&self) -> &RenderPipeline {
        &self.pipeline
    }
}
