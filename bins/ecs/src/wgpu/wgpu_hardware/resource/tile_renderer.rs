use crate::map::tile_instance::{TileInstance, TileVertex};
use crate::wgpu::texture::Texture;
use crate::wgpu::wgpu_hardware::resources::render_pipelines::RenderMode;
use crate::wgpu::wgpu_hardware::resources::standard_render_layout::StandardRenderLayout;
use std::borrow::Borrow;
use wgpu::{Device, RenderPipeline, ShaderModule, SurfaceConfiguration, TextureFormat};

#[derive(Debug)]
pub struct TileRendererResource {
    pub pipeline: RenderPipeline,
    _shader: ShaderModule,
}

impl TileRendererResource {
    pub fn new(
        device: &Device,
        config: &SurfaceConfiguration,
        standard_render: &StandardRenderLayout,
    ) -> Self {
        let vertex_buffers = &[TileVertex::desc(), TileInstance::desc()];

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

        let render_mode = RenderMode::Filled; // TODO: Support wireframe

        let shader = wgpu::ShaderModuleDescriptor {
            label: Some(wgpu_label!("tile.wgsl")),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../../shaders/tile.wgsl").into()),
        };
        let shader = device.create_shader_module(shader);

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(wgpu_label!("tile_render")),
            layout: Some(&standard_render.pipeline),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: vertex_buffers,
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"), // TODO: Use main again!
                targets: &targets,
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // These need to be requested explicitly on device request.
                polygon_mode: render_mode.into(),
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
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

impl Borrow<RenderPipeline> for TileRendererResource {
    fn borrow(&self) -> &RenderPipeline {
        &self.pipeline
    }
}

impl Borrow<RenderPipeline> for &TileRendererResource {
    fn borrow(&self) -> &RenderPipeline {
        &self.pipeline
    }
}
