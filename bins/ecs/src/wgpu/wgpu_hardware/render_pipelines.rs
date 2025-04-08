use tracing::instrument;
use wgpu::{Device, PipelineLayout, RenderPipeline, TextureFormat};
use crate::wgpu::wgpu_hardware::bind_group_layouts::BindGroupLayouts;
use crate::wgpu::wgpu_hardware::shaders::{FragmentShaderRef, Shaders, VertexShaderRef};
use crate::wgpu::instance::InstanceRaw;
use crate::wgpu::model::{ModelVertex, Vertex};
use crate::wgpu::texture::Texture;

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
    Entity
}

impl RenderPipelines {
    /// Returns a reference to the render pipeline.
    #[inline]
    pub fn render_pipeline(&self, which: RenderPipelineSelector) -> &RenderPipeline {
        match which {
            RenderPipelineSelector::Textured => &self.textured_render,
            RenderPipelineSelector::Boring => &self.flat_render,
            RenderPipelineSelector::Wireframe => &self.wireframe_render.as_ref().unwrap_or(&self.flat_render),
            RenderPipelineSelector::Entity => &self.flat_entity_render,
        }
    }
}

impl RenderPipelines {
    #[instrument(skip(device, bind_group_layouts, shaders))]
    pub fn create_from(device: &Device, bind_group_layouts: &BindGroupLayouts, surface_format: TextureFormat, shaders: &Shaders) -> Self {
        let layout = Self::create_standard_render_layout(&device, bind_group_layouts);
        let textured_render = Self::create_textured_render(&device, &layout, surface_format, shaders);
        let flat_render = Self::create_flat_render(&device, &layout, surface_format, shaders);
        let flat_entity_render = Self::create_flat_entity_render(&device, &layout, surface_format, shaders);
        let wireframe_render = Self::create_wireframe_render(&device, &layout, surface_format, shaders);
        let light = Self::create_light(&device, bind_group_layouts, surface_format, shaders);
        let vga_fullscreen = Self::create_vga_fullscreen(&device, bind_group_layouts, surface_format, shaders);
        Self {
            textured_render,
            flat_render,
            flat_entity_render,
            wireframe_render,
            light,
            vga_fullscreen
        }
    }

    #[instrument(skip(device, bind_group_layouts, shaders))]
    fn create_vga_fullscreen(device: &Device, bind_group_layouts: &BindGroupLayouts, surface_format: TextureFormat, shaders: &Shaders) -> RenderPipeline {
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("vga_fullscreen"),
            bind_group_layouts: &[&bind_group_layouts.vga_fullscreen],
            push_constant_ranges: &[],
        });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("vga_fullscreen"),
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
            shaders.light.vertex_shader(),
            shaders.light.fragment_shader(),
            &device,
            &layout,
            surface_format,
            true,
            Some(Texture::DEPTH_FORMAT),
            &[
                ModelVertex::desc()
            ],
            RenderMode::Filled,
            "light"
        )
    }

    #[instrument(skip_all)]
    fn create_standard_render_layout(device: &Device, bind_group_layouts: &BindGroupLayouts) -> PipelineLayout {
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("render_layout"),
            bind_group_layouts: &[
                &bind_group_layouts.diffuse_and_normal, // `@group(0)` in the shader
                &bind_group_layouts.camera_uniforms,    // `@group(1)` in the shader
                &bind_group_layouts.lighting_uniforms,  // `@group(2)` in the shader
            ],
            push_constant_ranges: &[],
        })
    }

    #[instrument(skip_all)]
    fn create_textured_render(device: &Device, layout: &PipelineLayout, surface_format: TextureFormat, shaders: &Shaders) -> RenderPipeline {
        Self::create_render_pipeline(
            shaders.render.vertex_shader(),
            shaders.render.textured_fragment_shader(),
            &device,
            &layout,
            surface_format,
            true,
            Some(Texture::DEPTH_FORMAT),
            &[
                ModelVertex::desc(),
                InstanceRaw::desc()
            ],
            RenderMode::Filled,
            "textured_render"
        )
    }

    #[instrument(skip_all)]
    fn create_wireframe_render(device: &Device, layout: &PipelineLayout, surface_format: TextureFormat, shaders: &Shaders) -> Option<RenderPipeline> {
        if !device.features().contains(wgpu::Features::POLYGON_MODE_LINE) {
            return None;
        }

        let pipeline = Self::create_render_pipeline(
            shaders.render.vertex_shader(),
            shaders.render.flat_fragment_shader(),
            &device,
            &layout,
            surface_format,
            false,
            Some(Texture::DEPTH_FORMAT),
            &[
                ModelVertex::desc(),
                InstanceRaw::desc()
            ],
            RenderMode::Wireframe,
            "wireframe_render"
        );

        Some(pipeline)
    }

    #[instrument(skip_all)]
    fn create_flat_render(device: &Device, layout: &PipelineLayout, surface_format: TextureFormat, shaders: &Shaders) -> RenderPipeline {
        Self::create_render_pipeline(
            shaders.render.vertex_shader(),
            shaders.render.flat_fragment_shader(),
            &device,
            &layout,
            surface_format,
            false,
            Some(Texture::DEPTH_FORMAT),
            &[
                ModelVertex::desc(),
                InstanceRaw::desc()
            ],
            RenderMode::Filled,
            "flat_render"
        )
    }

    #[instrument(skip_all)]
    fn create_flat_entity_render(device: &Device, layout: &PipelineLayout, surface_format: TextureFormat, shaders: &Shaders) -> RenderPipeline {
        Self::create_render_pipeline(
            shaders.render.vertex_shader(),
            shaders.render.instance_fragment_shader(),
            &device,
            &layout,
            surface_format,
            false,
            Some(Texture::DEPTH_FORMAT),
            &[
                ModelVertex::desc(),
                InstanceRaw::desc()
            ],
            RenderMode::Filled,
            "flat_entity_render"
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
                targets: &[
                    texture_target,
                    entity_id_target,
                ],
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
            RenderMode::Wireframe => Self::Line
        }
    }
}
