use tracing::instrument;
use wgpu::{Device, PipelineLayout, RenderPipeline, TextureFormat};
use crate::wgpu::gpu_hardware::bind_group_layouts::BindGroupLayouts;
use crate::wgpu::gpu_hardware::shaders::{FragmentShaderRef, Shaders, VertexShaderRef};
use crate::wgpu::instance::InstanceRaw;
use crate::wgpu::model::{ModelVertex, Vertex};
use crate::wgpu::texture::Texture;

#[derive(Debug)]
pub struct RenderPipelines {
    pub textured_render: RenderPipeline,
    pub flat_render: RenderPipeline,
    pub light: RenderPipeline,
}


#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum RenderPipelineSelector {
    Boring,
    #[default]
    Colored
}

impl RenderPipelines {
    /// Returns a reference to the render pipeline.
    #[inline]
    pub const fn render_pipeline(&self, which: RenderPipelineSelector) -> &RenderPipeline {
        match which {
            RenderPipelineSelector::Boring => &self.flat_render,
            RenderPipelineSelector::Colored => &self.textured_render
        }
    }
}

impl RenderPipelines {
    #[instrument(skip(device, bind_group_layouts, shaders))]
    pub fn create_from(device: &Device, bind_group_layouts: &BindGroupLayouts, surface_format: TextureFormat, shaders: &Shaders) -> Self {
        let textured_render = Self::create_textured_render(&device, bind_group_layouts, surface_format, shaders);
        let flat_render = Self::create_flat_render(&device, bind_group_layouts, surface_format, shaders);
        let light = Self::create_light(&device, bind_group_layouts, surface_format, shaders);
        Self {
            textured_render,
            flat_render,
            light
        }
    }

    #[instrument(skip(device, bind_group_layouts, shaders))]
    fn create_light(device: &Device, bind_group_layouts: &BindGroupLayouts, surface_format: TextureFormat, shaders: &Shaders) -> RenderPipeline {
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("light"),
            bind_group_layouts: &[
                &bind_group_layouts.camera_uniforms,    // `@group(0)` in the shader
                &bind_group_layouts.lighting_uniforms,  // `@group(1)` in the shader
            ],
            push_constant_ranges: &[],
        });

        Self::create_render_pipeline(
            shaders.render.vertex_shader(),
            shaders.render.textured_fragment_shader(),
            &device,
            &layout,
            surface_format,
            Some(Texture::DEPTH_FORMAT),
            &[
                ModelVertex::desc(),
                InstanceRaw::desc()
            ],
            "light"
        )
    }

    #[instrument(skip(device, bind_group_layouts, shaders))]
    fn create_textured_render(device: &Device, bind_group_layouts: &BindGroupLayouts, surface_format: TextureFormat, shaders: &Shaders) -> RenderPipeline {
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("textured_render"),
            bind_group_layouts: &[
                &bind_group_layouts.diffuse_and_normal, // `@group(0)` in the shader
                &bind_group_layouts.camera_uniforms,    // `@group(1)` in the shader
                &bind_group_layouts.lighting_uniforms,  // `@group(2)` in the shader
            ],
            push_constant_ranges: &[],
        });

        Self::create_render_pipeline(
            shaders.render.vertex_shader(),
            shaders.render.textured_fragment_shader(),
            &device,
            &layout,
            surface_format,
            Some(Texture::DEPTH_FORMAT),
            &[
                ModelVertex::desc(),
                InstanceRaw::desc()
            ],
            "textured_render"
        )
    }

    #[instrument(skip(device, bind_group_layouts, shaders))]
    fn create_flat_render(device: &Device, bind_group_layouts: &BindGroupLayouts, surface_format: TextureFormat, shaders: &Shaders) -> RenderPipeline {
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("flat_render"),
            bind_group_layouts: &[
                &bind_group_layouts.diffuse_and_normal, // `@group(0)` in the shader
                &bind_group_layouts.camera_uniforms,    // `@group(1)` in the shader
                &bind_group_layouts.lighting_uniforms,  // `@group(2)` in the shader
            ],
            push_constant_ranges: &[],
        });

        Self::create_render_pipeline(
            shaders.render.vertex_shader(),
            shaders.render.flat_fragment_shader(),
            &device,
            &layout,
            surface_format,
            Some(Texture::DEPTH_FORMAT),
            &[
                ModelVertex::desc(),
                InstanceRaw::desc()
            ],
            "flat_render"
        )
    }

    fn create_render_pipeline(
        vertex: VertexShaderRef<'_>,
        fragment: FragmentShaderRef<'_>,
        device: &Device,
        layout: &PipelineLayout,
        color_format: TextureFormat,
        depth_format: Option<TextureFormat>,
        vertex_layouts: &[wgpu::VertexBufferLayout],
        label: &str,
    ) -> RenderPipeline {
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(label),
            layout: Some(layout),
            vertex: wgpu::VertexState {
                module: vertex.module(),
                entry_point: vertex.entry_point(),
                buffers: vertex_layouts,
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: fragment.module(),
                entry_point: fragment.entry_point(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: color_format,
                    blend: Some(wgpu::BlendState {
                        alpha: wgpu::BlendComponent::REPLACE,
                        color: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: depth_format.map(|format| wgpu::DepthStencilState {
                format,
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
        })
    }
}
