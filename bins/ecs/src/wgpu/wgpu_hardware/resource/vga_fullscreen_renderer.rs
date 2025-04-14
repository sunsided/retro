use crate::engine::BackgroundColorComponent;
use crate::wgpu::texture::Texture;
use crate::wgpu::wgpu_hardware::resource::camera_uniform::CameraUniformResource;
use crate::wgpu::wgpu_hardware::resource::frame_global_uniform::FrameGlobalUniformResource;
use crate::wgpu::wgpu_hardware::resource::light_uniform::LightUniformResource;
use std::borrow::Borrow;
use wgpu::{
    BindGroup, BindGroupLayout, CommandEncoder, Device, PipelineLayout, RenderPass, RenderPipeline,
    Sampler, ShaderModule, SurfaceConfiguration, TextureFormat, TextureView,
};

#[derive(Debug)]
pub struct VgaFullscreenRendererResource {
    /// Bind group for the VGA fullscreen shader.
    ///
    /// ## Bindings
    /// - `@binding(0)`: The texture
    /// - `@binding(1)`: The texture sampler
    /// - `@binding(3)`: The entity ID
    _bind_group_layout: BindGroupLayout,
    _shader: ShaderModule,
    bind_group: BindGroup,
    frame_texture: Texture,
    entity_id_texture: Texture,
    depth: Texture,
    _sampler: Sampler,
    _pipeline_layout: PipelineLayout,
    pipeline: RenderPipeline,
}

impl VgaFullscreenRendererResource {
    pub fn new(
        device: &Device,
        config: &SurfaceConfiguration,
        frame_global: &FrameGlobalUniformResource,
        camera: &CameraUniformResource,
        light: &LightUniformResource,
    ) -> Self {
        let size = wgpu::Extent3d {
            width: Texture::VGA_WIDTH,
            height: Texture::VGA_HEIGHT,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some(wgpu_label!("vga_offscreen")),
            size,
            mip_level_count: 1,
            sample_count: 1,
            format: config.format, // TODO: Careful, that's BGRA or RGBA depending on platform; use sampler magic to convert
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            // | wgpu::TextureUsages::COPY_DST, // TODO: Only needed if we write_texture; not needed if it's just a render target
            dimension: wgpu::TextureDimension::D2,
            view_formats: &[],
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let frame_texture = Texture { texture, view };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some(wgpu_label!("vga_entity_id")),
            size,
            mip_level_count: 1,
            sample_count: 1,
            format: TextureFormat::R32Uint,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            dimension: wgpu::TextureDimension::D2,
            view_formats: &[],
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let entity_id_texture = Texture { texture, view };

        let desc = wgpu::TextureDescriptor {
            label: Some(wgpu_label!("vga_depth")),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: Texture::DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        };
        let texture = device.create_texture(&desc);
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let depth = Texture { texture, view };

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
        });

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some(wgpu_label!("retro")),
            mag_filter: wgpu::FilterMode::Nearest, // or .Linear, your choice
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(wgpu_label!("vga_fullscreen")),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&frame_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(&entity_id_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::TextureView(&depth.view),
                },
            ],
        });

        let shader = wgpu::ShaderModuleDescriptor {
            label: Some(wgpu_label!("vga.wgsl")),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../../shaders/vga.wgsl").into()),
        };
        let shader = device.create_shader_module(shader);

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some(wgpu_label!("vga_fullscreen")),
            bind_group_layouts: &[
                &frame_global.layout, // `@group(0)` in the shader
                &camera.layout,       // `@group(1)` in the shader
                &light.layout,        // `@group(2)` in the shader
                &bind_group_layout,   // `@group(3)` in the shader
            ],
            push_constant_ranges: &[],
        });

        let targets = [Some(wgpu::ColorTargetState {
            format: config.format,
            blend: Some(wgpu::BlendState {
                alpha: wgpu::BlendComponent::REPLACE,
                color: wgpu::BlendComponent::REPLACE,
            }),
            write_mask: wgpu::ColorWrites::ALL,
        })];

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(wgpu_label!("vga_fullscreen")),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[], // No vertex buffer needed
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &targets,
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
        });

        Self {
            _bind_group_layout: bind_group_layout,
            bind_group,
            _shader: shader,
            frame_texture,
            entity_id_texture,
            depth,
            _sampler: sampler,
            _pipeline_layout: pipeline_layout,
            pipeline,
        }
    }

    #[inline]
    pub const fn pipeline(&self) -> &RenderPipeline {
        &self.pipeline
    }

    #[inline]
    pub const fn bind_group(&self) -> &BindGroup {
        &self.bind_group
    }

    pub fn create_retro_clear_pass<'encoder>(
        &self,
        encoder: &'encoder mut CommandEncoder,
        background_color: &BackgroundColorComponent,
    ) -> RenderPass<'encoder> {
        Self::create_render_pass(
            encoder,
            Some(background_color),
            &self.frame_texture.view,
            Some(&self.entity_id_texture.view),
            &self.depth.view,
            wgpu_label!("vga_first_pass"),
        )
    }

    pub fn create_retro_render_pass<'encoder>(
        &self,
        encoder: &'encoder mut CommandEncoder,
    ) -> RenderPass<'encoder> {
        Self::create_render_pass(
            encoder,
            None,
            &self.frame_texture.view,
            Some(&self.entity_id_texture.view),
            &self.depth.view,
            wgpu_label!("vga_render"),
        )
    }

    pub fn create_final_pass<'encoder>(
        &self,
        encoder: &'encoder mut CommandEncoder,
        surface_view: &TextureView,
        background_color: &BackgroundColorComponent,
    ) -> RenderPass<'encoder> {
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some(wgpu_label!("vga_second_pass")),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &surface_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(background_color.as_wgpu_color()),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        })
    }

    fn create_render_pass<'encoder>(
        encoder: &'encoder mut CommandEncoder,
        background_color: Option<&BackgroundColorComponent>,
        tex_view: &TextureView,
        entity_view: Option<&TextureView>,
        depth_view: &TextureView,
        label: &'static str,
    ) -> RenderPass<'encoder> {
        let tex_load = if let Some(background_color) = background_color {
            wgpu::LoadOp::Clear(background_color.as_wgpu_color())
        } else {
            wgpu::LoadOp::Load
        };

        let depth_load = if background_color.is_some() {
            wgpu::LoadOp::Clear(1.0)
        } else {
            wgpu::LoadOp::Load
        };

        let entity_id_attachment = entity_view.map(|view| wgpu::RenderPassColorAttachment {
            view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: tex_load,
                store: wgpu::StoreOp::Store,
            },
        });

        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some(label),
            color_attachments: &[
                Some(wgpu::RenderPassColorAttachment {
                    view: tex_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: tex_load,
                        store: wgpu::StoreOp::Store,
                    },
                }),
                entity_id_attachment,
            ],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: depth_view,
                depth_ops: Some(wgpu::Operations {
                    load: depth_load,
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            occlusion_query_set: None,
            timestamp_writes: None,
        })
    }
}

impl Borrow<RenderPipeline> for VgaFullscreenRendererResource {
    fn borrow(&self) -> &RenderPipeline {
        &self.pipeline
    }
}

impl Borrow<RenderPipeline> for &VgaFullscreenRendererResource {
    fn borrow(&self) -> &RenderPipeline {
        &self.pipeline
    }
}

impl Borrow<BindGroup> for VgaFullscreenRendererResource {
    fn borrow(&self) -> &BindGroup {
        &self.bind_group
    }
}

impl Borrow<BindGroup> for &VgaFullscreenRendererResource {
    fn borrow(&self) -> &BindGroup {
        &self.bind_group
    }
}
