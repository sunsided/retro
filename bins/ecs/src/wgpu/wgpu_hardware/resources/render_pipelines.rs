use crate::wgpu::instance::InstanceRaw;
use crate::wgpu::model::{ModelVertex, Vertex};
use crate::wgpu::texture::Texture;
use crate::wgpu::wgpu_hardware::resource::camera_uniform::CameraUniformResource;
use crate::wgpu::wgpu_hardware::resource::frame_global_uniform::FrameGlobalUniformResource;
use crate::wgpu::wgpu_hardware::resource::light_uniform::LightUniformResource;
use crate::wgpu::wgpu_hardware::resources::bind_group_layouts::BindGroupLayouts;
use crate::wgpu::wgpu_hardware::resources::shaders::{FragmentShaderRef, Shaders, VertexShaderRef};
use crate::wgpu::wgpu_hardware::resources::standard_render_layout::StandardRenderLayout;
use tracing::instrument;
use wgpu::{Device, PipelineLayout, RenderPipeline, TextureFormat};

#[derive(Debug)]
pub struct RenderPipelines {
    pub textured_render: RenderPipeline,
    pub flat_render: RenderPipeline,
    pub flat_entity_render: RenderPipeline,
    pub light: RenderPipeline,
    pub wireframe_render: Option<RenderPipeline>,
    pub vga_fullscreen: RenderPipeline,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum RenderPipelineSelector {
    /// The standard textured rendering pipeline.
    #[default]
    Textured,
    /// An untextured, solid-color rendering pipeline.
    Boring,
    /// Uses a wireframe rendering pipeline, if supported.
    /// Falls back to [`Boring`](Self::Boring) otherwise.
    Wireframe,
    /// Flat-shade each distinct entity.
    Entity,
}

impl RenderPipelines {
    /// Returns a reference to the render pipeline.
    #[inline]
    pub fn render_pipeline(&self, which: RenderPipelineSelector) -> &RenderPipeline {
        match which {
            RenderPipelineSelector::Textured => &self.textured_render,
            RenderPipelineSelector::Boring => &self.flat_render,
            RenderPipelineSelector::Wireframe => {
                &self.wireframe_render.as_ref().unwrap_or(&self.flat_render)
            }
            RenderPipelineSelector::Entity => &self.flat_entity_render,
        }
    }
}

impl RenderPipelines {
    #[instrument(skip_all)]
    pub fn create_from(
        device: &Device,
        bind_group_layouts: &BindGroupLayouts,
        render_pipeline_layouts: &StandardRenderLayout,
        surface_format: TextureFormat,
        shaders: &Shaders,
        frame_global: &FrameGlobalUniformResource,
        camera: &CameraUniformResource,
        light: &LightUniformResource,
    ) -> Self {
        let textured_render = Self::create_textured_render(
            &device,
            &render_pipeline_layouts.pipeline,
            surface_format,
            shaders,
        );
        let flat_render = Self::create_flat_render(
            &device,
            &render_pipeline_layouts.pipeline,
            surface_format,
            shaders,
        );
        let flat_entity_render = Self::create_flat_entity_render(
            &device,
            &render_pipeline_layouts.pipeline,
            surface_format,
            shaders,
        );
        let wireframe_render = Self::create_wireframe_render(
            &device,
            &render_pipeline_layouts.pipeline,
            surface_format,
            shaders,
        );
        let light_rp = Self::create_light(
            &device,
            surface_format,
            shaders,
            frame_global,
            camera,
            light,
        );
        let vga_fullscreen = Self::create_vga_fullscreen(
            &device,
            frame_global,
            camera,
            light,
            bind_group_layouts,
            surface_format,
            shaders,
        );
        Self {
            textured_render,
            flat_render,
            flat_entity_render,
            wireframe_render,
            light: light_rp,
            vga_fullscreen,
        }
    }

    #[instrument(skip_all)]
    fn create_vga_fullscreen(
        device: &Device,
        frame_global: &FrameGlobalUniformResource,
        camera: &CameraUniformResource,
        light: &LightUniformResource,
        bind_group_layouts: &BindGroupLayouts,
        surface_format: TextureFormat,
        shaders: &Shaders,
    ) -> RenderPipeline {
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some(wgpu_label!("vga_fullscreen")),
            bind_group_layouts: &[
                &frame_global.layout,               // `@group(0)` in the shader
                &camera.layout,                     // `@group(1)` in the shader
                &light.layout,                      // `@group(2)` in the shader
                &bind_group_layouts.vga_fullscreen, // `@group(3)` in the shader
            ],
            push_constant_ranges: &[],
        });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(wgpu_label!("vga_fullscreen")),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: shaders.vga.vertex_shader().module(),
                entry_point: shaders.vga.vertex_shader().entry_point(),
                buffers: &[], // No vertex buffer needed
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: shaders.vga.fragment_shader().module(),
                entry_point: shaders.vga.fragment_shader().entry_point(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_format,
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
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        })
    }

    #[instrument(skip_all)]
    fn create_light(
        device: &Device,
        surface_format: TextureFormat,
        shaders: &Shaders,
        frame_global: &FrameGlobalUniformResource,
        camera: &CameraUniformResource,
        light: &LightUniformResource,
    ) -> RenderPipeline {
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some(wgpu_label!("light")),
            bind_group_layouts: &[
                &frame_global.layout, // `@group(0)` in the shader
                &camera.layout,       // `@group(1)` in the shader
                &light.layout,        // `@group(2)` in the shader
            ],
            push_constant_ranges: &[],
        });

        Self::create_render_pipeline(
            shaders.light.vertex_shader(),
            shaders.light.fragment_shader(),
            &device,
            &layout,
            surface_format,
            true,
            Some(Texture::DEPTH_FORMAT),
            &[ModelVertex::desc()],
            RenderMode::Filled,
            wgpu_label!("light"),
        )
    }

    #[instrument(skip_all)]
    fn create_textured_render(
        device: &Device,
        layout: &PipelineLayout,
        surface_format: TextureFormat,
        shaders: &Shaders,
    ) -> RenderPipeline {
        Self::create_render_pipeline(
            shaders.render.vertex_shader(),
            shaders.render.textured_fragment_shader(),
            &device,
            &layout,
            surface_format,
            true,
            Some(Texture::DEPTH_FORMAT),
            &[ModelVertex::desc(), InstanceRaw::desc()],
            RenderMode::Filled,
            wgpu_label!("textured_render"),
        )
    }

    #[instrument(skip_all)]
    fn create_wireframe_render(
        device: &Device,
        layout: &PipelineLayout,
        surface_format: TextureFormat,
        shaders: &Shaders,
    ) -> Option<RenderPipeline> {
        if !device
            .features()
            .contains(wgpu::Features::POLYGON_MODE_LINE)
        {
            return None;
        }

        let pipeline = Self::create_render_pipeline(
            shaders.render.vertex_shader(),
            shaders.render.flat_fragment_shader(),
            &device,
            &layout,
            surface_format,
            true,
            Some(Texture::DEPTH_FORMAT),
            &[ModelVertex::desc(), InstanceRaw::desc()],
            RenderMode::Wireframe,
            wgpu_label!("wireframe_render"),
        );

        Some(pipeline)
    }

    #[instrument(skip_all)]
    fn create_flat_render(
        device: &Device,
        layout: &PipelineLayout,
        surface_format: TextureFormat,
        shaders: &Shaders,
    ) -> RenderPipeline {
        Self::create_render_pipeline(
            shaders.render.vertex_shader(),
            shaders.render.flat_fragment_shader(),
            &device,
            &layout,
            surface_format,
            true,
            Some(Texture::DEPTH_FORMAT),
            &[ModelVertex::desc(), InstanceRaw::desc()],
            RenderMode::Filled,
            wgpu_label!("flat_render"),
        )
    }

    #[instrument(skip_all)]
    fn create_flat_entity_render(
        device: &Device,
        layout: &PipelineLayout,
        surface_format: TextureFormat,
        shaders: &Shaders,
    ) -> RenderPipeline {
        Self::create_render_pipeline(
            shaders.render.vertex_shader(),
            shaders.render.instance_fragment_shader(),
            &device,
            &layout,
            surface_format,
            true,
            Some(Texture::DEPTH_FORMAT),
            &[ModelVertex::desc(), InstanceRaw::desc()],
            RenderMode::Filled,
            wgpu_label!("flat_entity_render"),
        )
    }

    fn create_render_pipeline(
        vertex: VertexShaderRef<'_>,
        fragment: FragmentShaderRef<'_>,
        device: &Device,
        layout: &PipelineLayout,
        color_format: TextureFormat,
        write_entity_ids: bool,
        depth_format: Option<TextureFormat>,
        vertex_layouts: &[wgpu::VertexBufferLayout],
        render_mode: RenderMode,
        label: &str,
    ) -> RenderPipeline {
        // Render to a texture.
        let texture_target = Some(wgpu::ColorTargetState {
            format: color_format,
            blend: Some(wgpu::BlendState {
                alpha: wgpu::BlendComponent::REPLACE,
                color: wgpu::BlendComponent::REPLACE,
            }),
            write_mask: wgpu::ColorWrites::ALL,
        });

        // Render to an entity ID target
        let entity_id_target = if write_entity_ids {
            Some(wgpu::ColorTargetState {
                format: TextureFormat::R32Uint, // entity ID target
                blend: None,
                write_mask: wgpu::ColorWrites::ALL,
            })
        } else {
            None
        };

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
                targets: &[texture_target, entity_id_target],
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderMode {
    Filled,
    Wireframe,
}

impl From<RenderMode> for wgpu::PolygonMode {
    fn from(value: RenderMode) -> Self {
        match value {
            RenderMode::Filled => Self::Fill,
            RenderMode::Wireframe => Self::Line,
        }
    }
}
