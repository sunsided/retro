mod app_state;
mod logging;
mod widgets;

use crate::app_state::{AppState, InitializedAppState, UninitializedAppState};
use crate::logging::setup_logging;
use crate::widgets::shader_area::ShaderArea;
use egui::{FontDefinitions, Pos2, Rect};
use egui_wgpu_backend::{RenderPass, ScreenDescriptor};
use egui_winit_platform::{Platform, PlatformDescriptor};
use glam::{Mat4, Vec3};
use std::sync::Arc;
use std::time::Instant;
use tracing::{debug, instrument, warn};
use wgpu::util::DeviceExt;
use wgpu::{
    Backends, Instance, InstanceDescriptor, PolygonMode, PresentMode, SurfaceConfiguration,
    TextureUsages,
};
use winit::application::ApplicationHandler;
use winit::dpi::{LogicalSize, PhysicalPosition, PhysicalSize, Size};
use winit::event::{
    DeviceEvent, DeviceId, ElementState, KeyEvent, Modifiers, MouseButton, MouseScrollDelta,
    WindowEvent,
};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, ModifiersState, PhysicalKey};
use winit::window::{Fullscreen, Window, WindowId};

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    transform: [[f32; 4]; 4],
}

const EXAMPLE_SHADER: &str = // language=wgsl
    r#"
    // Vertex attributes
    struct VertexInput {
        @location(0) position: vec3<f32>,
        @location(1) color: vec3<f32>,
    };

    struct VertexOutput {
        @builtin(position) clip_position: vec4<f32>,
        @location(0) color: vec3<f32>,
    };

    struct Uniforms {
        transform: mat4x4<f32>
    };

    @group(0) @binding(0)
    var<uniform> uniforms: Uniforms;

    @vertex
    fn vs_main(input: VertexInput) -> VertexOutput {
        var out: VertexOutput;
        let pos = vec4<f32>(input.position, 1.0);
        out.clip_position = uniforms.transform * pos;
        out.color = input.color;
        return out;
    }

    @fragment
    fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
        return vec4<f32>(input.color, 1.0); // Pass-through color
    }
"#;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

const QUAD_VERTICES: &[Vertex] = &[
    Vertex {
        position: [-1.0, -1.0, 0.0],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [1.0, -1.0, 0.0],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [1.0, 1.0, 0.0],
        color: [0.0, 0.0, 1.0],
    },
    Vertex {
        position: [-1.0, 1.0, 0.0],
        color: [1.0, 1.0, 1.0],
    },
];
const QUAD_INDICES: &[u16] = &[0, 1, 2, 2, 3, 0];

const CUBE_VERTICES: &[Vertex] = &[
    // Front face
    Vertex {
        position: [-1.0, -1.0, 1.0],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [1.0, -1.0, 1.0],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [1.0, 1.0, 1.0],
        color: [0.0, 0.0, 1.0],
    },
    Vertex {
        position: [-1.0, 1.0, 1.0],
        color: [1.0, 1.0, 1.0],
    },
    // Back face
    Vertex {
        position: [-1.0, -1.0, -1.0],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [1.0, -1.0, -1.0],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [1.0, 1.0, -1.0],
        color: [0.0, 0.0, 1.0],
    },
    Vertex {
        position: [-1.0, 1.0, -1.0],
        color: [1.0, 1.0, 1.0],
    },
];
const CUBE_INDICES: &[u16] = &[
    // front
    0, 1, 2, 2, 3, 0, // right
    1, 5, 6, 6, 2, 1, // back
    5, 4, 7, 7, 6, 5, // left
    4, 0, 3, 3, 7, 4, // top
    3, 2, 6, 6, 7, 3, // bottom
    4, 5, 1, 1, 0, 4,
];

struct App {
    state: AppState,
}

impl App {
    pub fn new(instance: Instance) -> Self {
        Self {
            state: AppState::Bootstrapped(UninitializedAppState {
                instance,
                start_time: Instant::now(),
            }),
        }
    }

    fn reinitialize_window(&mut self, active_event_loop: &ActiveEventLoop) {
        self.state = match std::mem::take(&mut self.state) {
            AppState::Bootstrapped(UninitializedAppState {
                instance,
                start_time,
            }) => {
                // Initialize if missing.
                let attributes = Window::default_attributes()
                    .with_title("Shader Editor")
                    .with_inner_size(LogicalSize::new(1024.0, 640.0))
                    .with_resizable(false);

                let window = Arc::new(
                    active_event_loop
                        .create_window(attributes)
                        .expect("Failed to create window"),
                );

                AppState::Initialized(self.wgpu_reinitialize(instance, window, start_time))
            }
            state => state,
        };
    }

    fn handle_resize_event(&mut self, new_size: PhysicalSize<u32>) {
        debug!("Window resized: {:?}", new_size);
        self.wgpu_resize(new_size);
    }

    fn handle_key_modifiers_event(&mut self, new_modifiers: Modifiers) {
        let AppState::Initialized(InitializedAppState { modifiers, .. }) = &mut self.state else {
            return;
        };

        *modifiers = new_modifiers.state();
    }

    fn handle_mouse_input_event(&mut self, _state: ElementState, button: MouseButton) {
        match button {
            MouseButton::Left => {}
            MouseButton::Right => {}
            MouseButton::Middle => {}
            MouseButton::Back => {}
            MouseButton::Forward => {}
            MouseButton::Other(_) => {}
        }
    }

    fn handle_cursor_moved_to_event(&mut self, _position: PhysicalPosition<f64>) {
        // nothing here.
    }

    fn handle_mouse_wheel_event(&mut self, _delta: MouseScrollDelta) {}

    fn handle_cursor_moved_by_event(&mut self, _position: (f64, f64)) {}

    fn handle_keypress_event(&mut self, active_event_loop: &ActiveEventLoop, event: KeyEvent) {
        let AppState::Initialized(InitializedAppState {
            window, modifiers, ..
        }) = &mut self.state
        else {
            return;
        };

        match event.physical_key {
            PhysicalKey::Code(KeyCode::Escape) if event.state == ElementState::Pressed => {
                active_event_loop.exit();
            }
            PhysicalKey::Code(KeyCode::Enter)
                if event.state == ElementState::Pressed && modifiers.alt_key() =>
            {
                // Toggle fullscreen mode.
                if window.fullscreen().is_some() {
                    window.set_max_inner_size(Some(Size::Logical(LogicalSize::new(1024.0, 640.0))));
                    window.set_fullscreen(None);
                } else {
                    window.set_max_inner_size::<Size>(None);
                    window.set_fullscreen(Some(Fullscreen::Borderless(None)));
                }
            }

            _ => {}
        }
    }

    fn wgpu_reinitialize(
        &self,
        instance: Instance,
        window: Arc<Window>,
        start_time: Instant,
    ) -> InitializedAppState {
        let size = window.inner_size();
        debug!(width = %size.width, height = %size.height, "Creating WGPU surface");
        let surface = instance
            .create_surface(window.clone())
            .expect("Failed to create surface");

        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .expect("Failed to create adapter");

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("Device Descriptor"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: Default::default(),
            },
            None,
        ))
        .expect("Failed to create device and queue");

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps.formats[0];

        // We use the egui_winit_platform crate as the platform.
        let platform = Platform::new(PlatformDescriptor {
            physical_width: size.width,
            physical_height: size.height,
            scale_factor: window.scale_factor(),
            font_definitions: FontDefinitions::default(),
            style: Default::default(),
        });

        // We use the egui_wgpu_backend crate as the render backend.
        let egui_rpass = RenderPass::new(&device, surface_format, 1);

        // Display the demo application that ships with egui.
        let demo_app = egui_demo_lib::DemoWindows::default();

        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: PresentMode::Fifo, // surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        let vertex_buffer_layout = wgpu::VertexBufferLayout {
            array_stride: size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3],
        };

        let depth_format = wgpu::TextureFormat::Depth24PlusStencil8;
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size: wgpu::Extent3d {
                width: size.width,
                height: size.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: depth_format,
            usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

        // Create flat (quad) buffers
        let flat_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Flat Vertex Buffer"),
            contents: bytemuck::cast_slice(QUAD_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let flat_index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Flat Index Buffer"),
            contents: bytemuck::cast_slice(QUAD_INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        // Create cube buffers
        let cube_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Cube Vertex Buffer"),
            contents: bytemuck::cast_slice(CUBE_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let cube_index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Cube Index Buffer"),
            contents: bytemuck::cast_slice(CUBE_INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        let shader = wgpu::ShaderModuleDescriptor {
            label: Some("Example Shader"),
            source: wgpu::ShaderSource::Wgsl(EXAMPLE_SHADER.into()),
        };

        let shader = device.create_shader_module(shader);

        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Uniform Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(64),
                    },
                    count: None,
                }],
            });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline"),
            bind_group_layouts: &[&uniform_bind_group_layout],
            push_constant_ranges: &[],
        });

        let texture_target = Some(wgpu::ColorTargetState {
            format: config.format,
            blend: Some(wgpu::BlendState {
                alpha: wgpu::BlendComponent::REPLACE,
                color: wgpu::BlendComponent::REPLACE,
            }),
            write_mask: wgpu::ColorWrites::ALL,
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[vertex_buffer_layout],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[texture_target],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: depth_format,
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
            cache: None,
        });

        // Create uniform buffer and bind group (start with identity matrix) -----
        let identity: [[f32; 4]; 4] = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        let uniform_data = Uniforms {
            transform: identity,
        };
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&[uniform_data]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Uniform Bind Group"),
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        InitializedAppState {
            window,
            modifiers: ModifiersState::default(),
            instance,
            surface,
            device,
            queue,
            config,
            pipeline,
            flat_vertex_buffer,
            flat_index_buffer,
            cube_vertex_buffer,
            cube_index_buffer,
            egui_platform: platform,
            egui_egui_rpass: egui_rpass,
            egui_demo_app: demo_app,
            uniform_buffer,
            uniform_bind_group,
            adapter,
            start_time,
            render_cube: true,
            depth_texture,
            depth_view,
        }
    }

    #[instrument(skip(self))]
    fn wgpu_resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width <= 0 || new_size.height <= 0 {
            warn!("Invalid window size");
            return;
        }

        let AppState::Initialized(InitializedAppState {
            surface,
            device,
            config,
            ..
        }) = &mut self.state
        else {
            return;
        };

        config.width = new_size.width;
        config.height = new_size.height;
        surface.configure(&device, &config);
    }

    fn render(&mut self) {
        let AppState::Initialized(InitializedAppState {
            surface,
            device,
            config: surface_config,
            queue,
            window,
            egui_platform: platform,
            egui_egui_rpass: egui_rpass,
            uniform_buffer,
            pipeline,
            uniform_bind_group,
            flat_vertex_buffer,
            flat_index_buffer,
            cube_vertex_buffer,
            cube_index_buffer,
            render_cube,
            start_time,
            depth_view,
            ..
        }) = &mut self.state
        else {
            return;
        };

        let frame = match surface.get_current_texture() {
            Ok(frame) => frame,
            Err(e) => {
                warn!("Failed to acquire next swap chain texture: {e}");
                return;
            }
        };

        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        let rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &depth_view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        drop(rpass);

        // Begin to draw the UI frame.
        platform.context().set_zoom_factor(2.0);
        platform.begin_pass();

        // Draw the demo application.
        /*
        let demo_app = self.egui_demo_app.as_mut().unwrap();
        demo_app.ui(&platform.context());
        */

        let ctx = platform.context();

        let mut shader_area_rect = Rect::from_pos(Pos2::default());
        egui::Window::new("Shader Preview")
            .default_pos(Pos2::new(100.0, 100.0))
            .collapsible(true)
            .show(&ctx, |ui| {
                // Toggle render mode: flat (false) vs cube (true)
                ui.checkbox(render_cube, "Render Cube");

                // Insert the custom shader widget, e.g., 300x200 points.
                let shader_response = ui.add(ShaderArea::new(300.0, 200.0));

                // Obtain the rectangle (in egui points) where the shader area was drawn.
                shader_area_rect = shader_response.rect;

                // (For demonstration) Show the coordinates as text.
                ui.label(format!("Shader area coordinates: {:?}", shader_area_rect));
            });

        // End the UI frame. We could now handle the output and draw the UI with the backend.
        let full_output = platform.end_pass(Some(&window));

        let paint_jobs = platform
            .context()
            .tessellate(full_output.shapes, full_output.pixels_per_point);

        // Upload all resources for the GPU.
        let screen_descriptor = ScreenDescriptor {
            physical_width: surface_config.width,
            physical_height: surface_config.height,
            scale_factor: window.scale_factor() as f32,
        };
        let tdelta: egui::TexturesDelta = full_output.textures_delta;
        egui_rpass
            .add_textures(&device, &queue, &tdelta)
            .expect("add texture ok");
        egui_rpass.update_buffers(&device, &queue, &paint_jobs, &screen_descriptor);

        // Record all render passes.
        egui_rpass
            .execute(&mut encoder, &view, &paint_jobs, &screen_descriptor, None)
            .unwrap();

        // Shader Overlay Pass
        {
            // Compute physical coordinates from the UI widget rectangle:
            let scale = window.scale_factor() as f32;
            let scissor_x = (shader_area_rect.min.x * scale) as u32;
            let scissor_y = (shader_area_rect.min.y * scale) as u32;
            let scissor_width = (shader_area_rect.width() * scale) as u32;
            let scissor_height = (shader_area_rect.height() * scale) as u32;

            let mut shader_rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Shader Overlay Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        // Set load to LoadOp::Load to keep the UI already rendered.
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            shader_rpass.set_viewport(
                shader_area_rect.min.x * scale,
                shader_area_rect.min.y * scale,
                shader_area_rect.width() * scale,
                shader_area_rect.height() * scale,
                0.0,
                1.0,
            );

            shader_rpass.set_scissor_rect(scissor_x, scissor_y, scissor_width, scissor_height);

            shader_rpass.set_pipeline(pipeline);

            // Bind our uniform so the vertex shader gets the transformation matrix.
            shader_rpass.set_bind_group(0, &*uniform_bind_group, &[]);

            let uniform_data = if *render_cube {
                // Compute a perspective projection and a view matrix for 3D.
                let fov: f32 = 45.0_f32.to_radians();
                let aspect = surface_config.width as f32 / surface_config.height as f32;
                let near = 0.1;
                let far = 100.0;
                let perspective = Mat4::perspective_rh(fov, aspect, near, far);

                // Place the camera at (0,0,5) looking at the origin.
                let eye = Vec3::new(0.0, 2.5, 4.0);
                let center = Vec3::ZERO - Vec3::new(0.0, 0.25, 0.0);
                let up = Vec3::Y;
                let view = Mat4::look_at_rh(eye, center, up);

                // Apply a model rotation (for example, animate around Y axis).
                let angle = start_time.elapsed().as_secs_f32();
                let model = Mat4::from_rotation_y(angle);

                // Combine to get the final MVP matrix.
                let mvp = perspective * view * model;

                // Convert to a 4x4 array.
                Uniforms {
                    transform: mvp.to_cols_array_2d(),
                }
            } else {
                Uniforms {
                    transform: [
                        [1.0, 0.0, 0.0, 0.0],
                        [0.0, 1.0, 0.0, 0.0],
                        [0.0, 0.0, 1.0, 0.0],
                        [0.0, 0.0, 0.0, 1.0],
                    ],
                }
            };

            // Update the uniform buffer with the new transforms.
            queue.write_buffer(uniform_buffer, 0, bytemuck::cast_slice(&[uniform_data]));

            if *render_cube {
                shader_rpass.set_vertex_buffer(0, cube_vertex_buffer.slice(..));
                shader_rpass
                    .set_index_buffer(cube_index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                shader_rpass.draw_indexed(0..(CUBE_INDICES.len() as u32), 0, 0..1);
            } else {
                shader_rpass.set_vertex_buffer(0, flat_vertex_buffer.slice(..));
                shader_rpass
                    .set_index_buffer(flat_index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                shader_rpass.draw_indexed(0..(QUAD_INDICES.len() as u32), 0, 0..1);
            }
        }

        queue.submit(Some(encoder.finish()));
        frame.present();

        egui_rpass
            .remove_textures(tdelta)
            .expect("remove texture ok");
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, active_event_loop: &ActiveEventLoop) {
        self.reinitialize_window(active_event_loop);
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: DeviceId,
        event: DeviceEvent,
    ) {
        match event {
            DeviceEvent::Added => {}
            DeviceEvent::Removed => {}
            DeviceEvent::MouseMotion { delta } => {
                self.handle_cursor_moved_by_event(delta);
            }
            DeviceEvent::MouseWheel { delta } => {
                self.handle_mouse_wheel_event(delta);
            }
            DeviceEvent::Motion { .. } => {}
            DeviceEvent::Button { .. } => {}
            DeviceEvent::Key(_) => {}
        }
    }

    fn window_event(
        &mut self,
        active_event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        let AppState::Initialized(InitializedAppState {
            egui_platform: platform,
            start_time,
            window,
            ..
        }) = &mut self.state
        else {
            return;
        };

        // Pass the winit events to the platform integration.
        platform.handle_event(&event);

        match event {
            WindowEvent::CloseRequested => {
                active_event_loop.exit();
            }
            WindowEvent::ModifiersChanged(new_modifiers) => {
                self.handle_key_modifiers_event(new_modifiers);
            }
            WindowEvent::KeyboardInput { event, .. } => {
                self.handle_keypress_event(active_event_loop, event);
            }
            WindowEvent::MouseInput { state, button, .. } => {
                self.handle_mouse_input_event(state, button);
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.handle_cursor_moved_to_event(position);
            }
            WindowEvent::RedrawRequested => {
                // Pass the winit events to the platform integration.
                platform.update_time(start_time.elapsed().as_secs_f64());

                window.request_redraw();
                self.render();
            }
            WindowEvent::Resized(size) => self.handle_resize_event(size),
            _ => (),
        }
    }

    fn about_to_wait(&mut self, _active_event_loop: &ActiveEventLoop) {
        // intentionally left blank
    }
}

fn main() -> eyre::Result<()> {
    dotenvy::dotenv().ok();
    let _logging = setup_logging();

    let instance = wgpu_instance();

    let mut app = App::new(instance);
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut app)?;
    Ok(())
}

fn wgpu_instance() -> Instance {
    let descriptor = InstanceDescriptor {
        backends: Backends::default(),
        flags: Default::default(),
        backend_options: Default::default(),
    };
    Instance::new(&descriptor)
}

const fn transpose(mat: [[f32; 4]; 4]) -> [[f32; 4]; 4] {
    [
        [mat[0][0], mat[1][0], mat[2][0], mat[3][0]],
        [mat[0][1], mat[1][1], mat[2][1], mat[3][1]],
        [mat[0][2], mat[1][2], mat[2][2], mat[3][2]],
        [mat[0][3], mat[1][3], mat[2][3], mat[3][3]],
    ]
}
