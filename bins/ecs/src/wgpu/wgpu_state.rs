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
use crate::camera::Camera;
use crate::camera_controller::CameraController;
use crate::wgpu::camera_uniform::CameraUniform;
use crate::wgpu::instance::{Instance, InstanceRaw};
use crate::wgpu::model::{ModelVertex, Vertex};
use crate::wgpu::texture::Texture;
use crate::wgpu::TextureSampler;

const NUM_INSTANCES_PER_ROW: u32 = 10;
const INSTANCE_DISPLACEMENT: cgmath::Vector3<f32> = cgmath::Vector3::new(NUM_INSTANCES_PER_ROW as f32 * 0.5, 0.0, NUM_INSTANCES_PER_ROW as f32 * 0.5);

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

        // TODO: Move shader to meaningful place
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/shader.wgsl").into()),
        });

        // TODO: Refactor :)
        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    // Binding for the texture (`@binding(0)` in the shader)
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT, // NONE, VERTEX, FRAGMENT or COMPUTE
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    // Binding for the texture sampler (`@binding(1)` in the shader)
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        // This should match the filterable field of the
                        // corresponding Texture entry above.
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
                    visibility: wgpu::ShaderStages::VERTEX, // it's only needed for vertex mainpulation
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

        // TODO: Most definitely refactor this.
        // NOTE: Make sure that if you add new instances to the Vec, you recreate the
        //       instance_buffer as well as camera_bind_group. Otherwise, your new instances
        //       won't show up correctly.
        let instances = (0..NUM_INSTANCES_PER_ROW).flat_map(|z| {
            (0..NUM_INSTANCES_PER_ROW).map(move |x| {
                let position = cgmath::Vector3 { x: x as f32, y: 0.0, z: z as f32 } - INSTANCE_DISPLACEMENT;

                let rotation = if position.is_zero() {
                    // this is needed so an object at (0, 0, 0) won't get scaled to zero
                    // as Quaternions can affect scale if they're not created correctly
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

        // TODO: Refactor
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[
                    &texture_bind_group_layout, // `@group(0)` in the shader
                    &camera_bind_group_layout   // `@group(1)` in the shader
                ],
                push_constant_ranges: &[],
            });

        // TODO: Refactor
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[
                    ModelVertex::desc(),
                    InstanceRaw::desc(),
                ],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState { // Currently only one for the target surface.
                    format: surface_format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
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
            depth_stencil: Some(wgpu::DepthStencilState {
                format: Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less, // Less = front to back
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1, // Don't use multi-sampling.
                mask: !0, // Use all samples.
                alpha_to_coverage_enabled: false, // Anti-aliasing is currently not enabled.
            },
            multiview: None, // We're not using array textures.
            cache: None, // Allows wgpu to cache shader compilation data. Only really useful for Android build targets.
        });

        // TODO: Refactor ...
        let colored_render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Textured Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[
                    ModelVertex::desc(),
                    InstanceRaw::desc(),
                ],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_textured_main"),
                targets: &[Some(wgpu::ColorTargetState { // Currently only one for the target surface.
                    format: surface_format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
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
            depth_stencil: Some(wgpu::DepthStencilState {
                format: Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1, // Don't use multi-sampling.
                mask: !0, // Use all samples.
                alpha_to_coverage_enabled: false, // Anti-aliasing is currently not enabled.
            },
            multiview: None, // We're not using array textures.
            cache: None, // Allows wgpu to cache shader compilation data. Only really useful for Android build targets.
        });

        Self {
            instance,
            surface,
            surface_config,
            device,
            queue,
            render_pipeline_layout,
            render_pipeline,
            colored_render_pipeline,
            texture_bind_group_layout,
            linear_sampler: sampler,
            camera,
            camera_uniform,
            camera_buffer,
            camera_bind_group,
            instances,
            instance_buffer,
            depth_texture
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
    pub fn update_camera_uniform(&mut self, controller: &CameraController) {
        controller.update_camera(&mut self.camera);
        self.camera_uniform.update_view_proj(&self.camera);
        self.queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[self.camera_uniform]));
    }

    /// Polls the underlying device.
    #[inline]
    #[allow(dead_code)]
    pub fn poll_device(&mut self, maintain: wgpu::Maintain) -> MaintainResult {
        self.device.poll(maintain)
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
