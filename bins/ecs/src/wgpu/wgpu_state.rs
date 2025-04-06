//! This module provides the [`WgpuState`] struct and its associated implementation for managing
//! WebGPU state within a Rust application. It abstracts over the low-level WebGPU API to
//! simplify tasks such as creating surfaces, configuring devices, handling window resizing,
//! and submitting command buffers to the GPU.
//!
//! # Overview
//!
//! The [`WgpuState`] struct is designed to handle core WebGPU initialization and management tasks,
//! including creating a suitable adapter, device, and queue, as well as configuring surfaces for
//! rendering. This abstraction makes it easier to manage the GPU rendering lifecycle in modern
//! applications that use the `wgpu` and `winit` crates.
//!
//! # Features
//!
//! - Creates and manages a WebGPU instance, adapter, device, queue, and surface.
//! - Configures the surface with optimal settings for rendering.
//! - Handles window resizing by reconfiguring the surface appropriately.
//! - Provides utility methods to poll the device and submit command buffers for rendering.
//!
//! # Structs
//!
//! - `WgpuState`: The main struct of this module. It wraps and manages WebGPU objects
//!   such as the `Instance`, `Surface`, `Adapter`, `Device`, and `Queue`.
//!
//! # References
//!
//! For more details about the underlying WebGPU API, see the [wgpu documentation](https://docs.rs/wgpu).

use cgmath::prelude::*;
use std::sync::Arc;
use tracing::debug;
use wgpu::{BindGroup, BindGroupLayout, Buffer, Device, InstanceDescriptor, MaintainResult, PipelineLayout, PresentMode, Queue, RenderPipeline, Sampler, Surface, TextureUsages};
use wgpu::util::DeviceExt;
use winit::dpi::PhysicalSize;
use winit::window::Window;
use crate::camera::{Camera, CameraController};
use crate::resources;
use crate::wgpu::camera_uniform::CameraUniform;
use crate::wgpu::instance::{Instance, InstanceRaw};
use crate::wgpu::light::LightUniform;
use crate::wgpu::model::{Model, ModelVertex, Vertex};
use crate::wgpu::texture::Texture;
use crate::wgpu::TextureSampler;

#[allow(dead_code)]
pub struct WgpuState {
    instance: wgpu::Instance,
    // https://github.com/rust-windowing/winit/discussions/3667#discussioncomment-9329312
    surface: Surface<'static>,
    surface_config: wgpu::SurfaceConfiguration,
    device: Device,
    queue: Queue,
    linear_sampler: TextureSampler,
    render_pipeline_layout: PipelineLayout,
    render_pipeline: RenderPipeline,
    colored_render_pipeline: RenderPipeline,
    texture_bind_group_layout: BindGroupLayout,
    camera: Camera,
    camera_uniform: CameraUniform,
    camera_buffer: Buffer,
    camera_bind_group: BindGroup,
    pub(crate) instances: Vec<Instance>,
    pub(crate) instance_buffer: Buffer,
    pub depth_texture: Texture,
    pub obj_model: Model,
    pub light_uniform: LightUniform,
    pub light_buffer: Buffer,
    pub light_bind_group: BindGroup,
    pub light_render_pipeline: RenderPipeline,
}

impl WgpuState {
    pub async fn new(window: Arc<Window>) -> WgpuState {
        // There's trouble with the Vulkan backend on NVIDIA + Wayland.
        // If the application hangs after a couple of seconds, use `WGPU_BACKEND=gl`
        let descriptor = InstanceDescriptor {
            backends: wgpu::Backends::GL,
            flags: Default::default(),
            backend_options: Default::default(),
        };
        let instance = wgpu::Instance::new(&descriptor);

        // TODO: See if we need to drop existing surfaces first?
        let surface = instance.create_surface(Arc::clone(&window)).unwrap();

        let adapter = pollster::block_on(instance
            .request_adapter(
                &wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::HighPerformance, // or LowPower (e.g., iGPU)
                    compatible_surface: Some(&surface),
                    force_fallback_adapter: false,
                },
            ))
            .unwrap();

        let (device, queue) = pollster::block_on(adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Device Descriptor"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    memory_hints: Default::default(),
                },
                None,
            ))
            .unwrap();

        // Shader code in this tutorial assumes an sRGB surface texture. Using a different
        // one will result in all the colors coming out darker. If you want to support non
        // sRGB surfaces, you'll need to account for that when drawing to the frame.
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);
        debug!("Using surface format {surface_format:?}");

        let size = window.inner_size();
        let surface_config = wgpu::SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: PresentMode::Fifo, // surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        let sampler = TextureSampler::with_clamp_to_edge_and_linear_filtering(&device);

        // Create Z-buffers.
        let depth_texture = Texture::depth_texture(&device, &surface_config, "depth_texture");
        let _depth_sampler = TextureSampler::depth_sampler(&device);

        // TODO: Refactor :)
        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    // Diffuse texture
                    wgpu::BindGroupLayoutEntry {
                        binding: 0, // @binding(0)
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
                label: Some("texture_bind_group_layout"),
            });

        // TODO: Refactor
        let camera = Camera::new(size.width as f32, size.height as f32);
        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_proj(&camera);

        // TODO: Refactor
        let camera_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&[camera_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        // TODO: Refactor
        let camera_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None, // TODO: Maybe set this later
                    },
                    count: None,
                }
            ],
            label: Some("camera_bind_group_layout"),
        });

        // TODO: Reeeefactor
        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding(),
                }
            ],
            label: Some("camera_bind_group"),
        });

        // TODO: Totally refactor
        let obj_model =
            resources::load_model("models/cube/cube.obj", &device, &queue, &texture_bind_group_layout, &sampler)
                .await
                .unwrap();

        // TODO: Most definitely refactor this.
        // NOTE: Make sure that if you add new instances to the Vec, you recreate the
        //       instance_buffer as well as camera_bind_group. Otherwise, your new instances
        //       won't show up correctly.
        const SPACE_BETWEEN: f32 = 3.0;
        const NUM_INSTANCES_PER_ROW: u32 = 10;
        let instances = (0..NUM_INSTANCES_PER_ROW).flat_map(|z| {
            (0..NUM_INSTANCES_PER_ROW).map(move |x| {
                let x = SPACE_BETWEEN * (x as f32 - NUM_INSTANCES_PER_ROW as f32 / 2.0);
                let z = SPACE_BETWEEN * (z as f32 - NUM_INSTANCES_PER_ROW as f32 / 2.0);

                let position = cgmath::Vector3 { x, y: 0.0, z };

                let rotation = if position.is_zero() {
                    cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.0))
                } else {
                    cgmath::Quaternion::from_axis_angle(position.normalize(), cgmath::Deg(45.0))
                };

                Instance {
                    position, rotation,
                }
            })
        }).collect::<Vec<_>>();

        // TODO: This needs a refactor ...
        let instance_data = instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
        let instance_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&instance_data),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        // TODO: Brutally refactor this
        let light_uniform = LightUniform::new(
            [2.0, 2.0, 2.0],
            [1.0, 1.0, 1.0]);

        // We'll want to update our lights position, so we use COPY_DST
        let light_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Light VB"),
                contents: bytemuck::cast_slice(&[light_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        let light_bind_group_layout =
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
                label: None,
            });

        let light_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &light_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: light_buffer.as_entire_binding(),
            }],
            label: None,
        });

        // TODO: Refactor
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[
                    &texture_bind_group_layout, // `@group(0)` in the shader
                    &camera_bind_group_layout,  // `@group(1)` in the shader
                    &light_bind_group_layout,   // `@group(2)` in the shader
                ],
                push_constant_ranges: &[],
            });

        let shader = wgpu::ShaderModuleDescriptor {
            label: Some("Normal Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/shader.wgsl").into()),
        };

        // TODO: Refactor
        let render_pipeline = {
            Self::create_render_pipeline(
                "vs_main",
                "fs_main",
                &device,
                &render_pipeline_layout,
                surface_format,
                Some(Texture::DEPTH_FORMAT),
                &[
                    ModelVertex::desc(),
                    InstanceRaw::desc()
                ],
                shader.clone(),
                "Render Pipeline"
            )
        };

        let textured_render_pipeline = {
            Self::create_render_pipeline(
                "vs_main",
                "fs_textured_main",
                &device,
                &render_pipeline_layout,
                surface_format,
                Some(Texture::DEPTH_FORMAT),
                &[
                    ModelVertex::desc(),
                    InstanceRaw::desc()
                ],
                shader,
                "Textured Render Pipeline"
            )
        };

        let light_render_pipeline = {
            let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Light Pipeline Layout"),
                bind_group_layouts: &[&camera_bind_group_layout, &light_bind_group_layout],
                push_constant_ranges: &[],
            });
            let shader = wgpu::ShaderModuleDescriptor {
                label: Some("Light Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/light.wgsl").into()),
            };
            Self::create_render_pipeline(
                "vs_main",
                "fs_main",
                &device,
                &layout,
                surface_format,
                Some(Texture::DEPTH_FORMAT),
                &[ModelVertex::desc()],
                shader,
                "Light Render Pipeline"
            )
        };

        Self {
            instance,
            surface,
            surface_config,
            device,
            queue,
            render_pipeline_layout,
            render_pipeline,
            colored_render_pipeline: textured_render_pipeline,
            light_render_pipeline,
            texture_bind_group_layout,
            linear_sampler: sampler,
            camera,
            camera_uniform,
            camera_buffer,
            camera_bind_group,
            instances,
            instance_buffer,
            depth_texture,
            obj_model,
            light_uniform,
            light_buffer,
            light_bind_group
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            debug!("Resizing window to {:?}", new_size);
            self.surface_config.width = new_size.width;
            self.surface_config.height = new_size.height;
            self.surface.configure(&self.device, &self.surface_config);

            // Update depth buffer.
            self.depth_texture = Texture::depth_texture(&self.device, &self.surface_config, "depth_texture");

        }
    }

    /// Returns a reference to the WGPU surface.
    #[inline]
    pub const fn surface(&self) -> &Surface<'static> {
        &self.surface
    }

    /// Returns a reference to the WGPU device.
    #[inline]
    pub const fn device(&self) -> &Device {
        &self.device
    }

    /// Returns a reference to the WGPU queue.
    #[inline]
    #[allow(dead_code)]
    pub const fn queue(&self) -> &Queue {
        &self.queue
    }

    /// Returns a reference to the render pipeline.
    #[inline]
    pub const fn render_pipeline(&self, which: RenderPipelineSelector) -> &RenderPipeline {
        match which {
            RenderPipelineSelector::Boring => &self.render_pipeline,
            RenderPipelineSelector::Colored => &self.colored_render_pipeline
        }
    }

    /// Accesses the render pipeline layout.
    #[inline]
    pub const fn bind_group_layout(&self) -> &BindGroupLayout {
        &self.texture_bind_group_layout
    }

    /// Accesses the linear texture sampler.
    #[inline]
    pub const fn linear_sampler(&self) -> &Sampler {
        &self.linear_sampler.sampler
    }

    /// Accesses the camera bind group.
    #[inline]
    pub const fn camera_bind_group(&self) -> &BindGroup {
        &self.camera_bind_group
    }

    /// Accesses the light bind group.
    #[inline]
    pub const fn light_bind_group(&self) -> &BindGroup {
        &self.light_bind_group
    }

    /// Submits a command buffer to the WGPU queue for execution.
    ///
    /// This function is used to send GPU commands for processing.
    /// Make sure that the command buffer is properly recorded and finalized
    /// before submitting it.
    ///
    /// # Parameters
    ///
    /// - `command_buffer`: A finalized WGPU command buffer containing the commands to be executed.
    #[inline]
    pub fn submit_to_queue(&self, command_buffer: wgpu::CommandBuffer) {
        self.queue.submit(Some(command_buffer));
    }

    /// Update the camera uniform buffer.
    // TODO: Refactor
    pub fn update_camera_uniform(&mut self, controller: &CameraController) {
        controller.update_camera(&mut self.camera);
        self.camera_uniform.update_view_proj(&self.camera);
        self.queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[self.camera_uniform]));
    }

    // TODO: Refactor
    pub fn update_light(&mut self) {
        let old_position: cgmath::Vector3<_> = self.light_uniform.position.into();
        self.light_uniform.position =
            (cgmath::Quaternion::from_axis_angle((0.0, 1.0, 0.0).into(), cgmath::Deg(1.0))
                * old_position)
                .into();
        self.queue.write_buffer(&self.light_buffer, 0, bytemuck::cast_slice(&[self.light_uniform]));
    }

    /// Polls the underlying device.
    #[inline]
    #[allow(dead_code)]
    pub fn poll_device(&mut self, maintain: wgpu::Maintain) -> MaintainResult {
        self.device.poll(maintain)
    }

    fn create_render_pipeline(
        vs_main: &str,
        fs_main: &str,
        device: &Device,
        layout: &PipelineLayout,
        color_format: wgpu::TextureFormat,
        depth_format: Option<wgpu::TextureFormat>,
        vertex_layouts: &[wgpu::VertexBufferLayout],
        shader: wgpu::ShaderModuleDescriptor,
        label: &str,
    ) -> RenderPipeline {
        let shader = device.create_shader_module(shader);

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(label),
            layout: Some(layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some(vs_main),
                buffers: vertex_layouts,
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some(fs_main),
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum RenderPipelineSelector {
    Boring,
    #[default]
    Colored
}

impl AsRef<Surface<'static>> for WgpuState {
    #[inline]
    fn as_ref(&self) -> &Surface<'static> {
        &self.surface
    }
}

impl AsRef<Device> for WgpuState {
    #[inline]
    fn as_ref(&self) -> &Device {
        &self.device
    }
}

impl AsRef<Queue> for WgpuState {
    #[inline]
    fn as_ref(&self) -> &Queue {
        &self.queue
    }
}
