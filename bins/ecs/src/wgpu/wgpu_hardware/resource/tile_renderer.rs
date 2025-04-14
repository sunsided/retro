use crate::map::tile_instance::{TileInstance, TileVertex};
use crate::wgpu::texture::Texture;
use crate::wgpu::wgpu_hardware::resource::WgpuResource;
use crate::wgpu::wgpu_hardware::resources::render_pipelines::RenderMode;
use crate::wgpu::wgpu_hardware::resources::WgpuResources;
use crate::wgpu::wgpu_hardware::DeviceAndQueue;
use wgpu::{Device, PipelineLayout, RenderPipeline, TextureFormat};

pub struct TileRendererResource {
    pub pipeline: RenderPipeline,
}

impl WgpuResource for TileRendererResource {
    fn recreate(&mut self, device: &DeviceAndQueue, resources: &WgpuResources) {
        *self = Self::new(device, resources);
    }
}

impl TileRendererResource {
    pub fn new(device: &DeviceAndQueue, resources: &WgpuResources) -> Self {
        let vertex_buffers = &[TileVertex::desc(), TileInstance::desc()];

        // Render to a texture.
        let texture_target = Some(wgpu::ColorTargetState {
            format: device.config().format,
            blend: Some(wgpu::BlendState {
                alpha: wgpu::BlendComponent::REPLACE,
                color: wgpu::BlendComponent::REPLACE,
            }),
            write_mask: wgpu::ColorWrites::ALL,
        });

        // Render to an entity ID target
        let entity_id_target = Some(wgpu::ColorTargetState {
            format: TextureFormat::R32Uint, // entity ID target
            blend: None,
            write_mask: wgpu::ColorWrites::ALL,
        });

        let render_mode = RenderMode::Filled; // TODO: Support wireframe

        let vertex = resources.shaders().tile.vertex_shader();
        let fragment = resources.shaders().tile.fragment_shader();

        let diffuse_and_normal =
            device
                .device()
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
                });

        let pipeline = device
            .device()
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some(wgpu_label!("tile_render")),
                layout: Some(&resources.standard_render().pipeline),
                vertex: wgpu::VertexState {
                    module: vertex.module(),
                    entry_point: vertex.entry_point(),
                    buffers: vertex_buffers,
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

        Self { pipeline }
    }
}
