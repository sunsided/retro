mod logging;
mod widgets;

use crate::logging::setup_logging;
use crate::widgets::shader_area::ShaderArea;
use egui::{FontDefinitions, Pos2, Rect, emath};
use egui_demo_lib::DemoWindows;
use egui_wgpu_backend::{RenderPass, ScreenDescriptor};
use egui_winit_platform::{Platform, PlatformDescriptor};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tracing::{debug, error, info, instrument, warn};
use wgpu::util::DeviceExt;
use wgpu::{
    Adapter, Backends, Buffer, Device, Instance, InstanceDescriptor, PolygonMode, PresentMode,
    Queue, RenderPipeline, Surface, SurfaceConfiguration, TextureUsages,
};
use winit::application::ApplicationHandler;
use winit::dpi::{LogicalSize, PhysicalPosition, PhysicalSize, Size};
use winit::event::{
    DeviceEvent, DeviceId, ElementState, KeyEvent, Modifiers, MouseButton, MouseScrollDelta,
    WindowEvent,
};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, ModifiersState, PhysicalKey};
use winit::monitor::MonitorHandle;
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
        @location(0) position: vec2<f32>,
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
        let pos = vec4<f32>(input.position, 0.0, 1.0);
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
    position: [f32; 2],
    color: [f32; 3],
}

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-1.0, -1.0],
        color: [1.0, 0.0, 0.0],
    }, // bottom left - red
    Vertex {
        position: [1.0, -1.0],
        color: [0.0, 1.0, 0.0],
    }, // bottom right - green
    Vertex {
        position: [1.0, 1.0],
        color: [0.0, 0.0, 1.0],
    }, // top right - blue
    Vertex {
        position: [-1.0, 1.0],
        color: [1.0, 1.0, 1.0],
    }, // top left - white
];

const INDICES: &[u16] = &[0, 1, 2, 2, 3, 0]; // two triangles

/// A custom event type for the winit app.
enum Event {
    RequestRedraw,
}

/// This is the repaint signal type that egui needs for requesting a repaint from another thread.
/// It sends the custom RequestRedraw event to the winit event loop.
struct ExampleRepaintSignal(std::sync::Mutex<winit::event_loop::EventLoopProxy<Event>>);

impl epi::backend::RepaintSignal for ExampleRepaintSignal {
    fn request_repaint(&self) {
        self.0.lock().unwrap().send_event(Event::RequestRedraw).ok();
    }
}

struct App {
    window: Option<Arc<Window>>,
    modifiers: ModifiersState,
    instance: Instance,
    surface: Option<Surface<'static>>,
    device: Option<Device>,
    adapter: Option<Adapter>,
    config: Option<SurfaceConfiguration>,
    queue: Option<Queue>,
    pipeline: Option<RenderPipeline>,
    vertex_buffer: Option<Buffer>,
    index_buffer: Option<Buffer>,
    egui_platform: Option<Platform>,
    egui_egui_rpass: Option<RenderPass>,
    egui_demo_app: Option<DemoWindows>,
    uniform_buffer: Option<Buffer>,
    uniform_bind_group: Option<wgpu::BindGroup>,
    start_time: Instant,
}

impl App {
    pub fn new(instance: Instance) -> Self {
        Self {
            window: None,
            modifiers: ModifiersState::default(),
            instance,
            device: None,
            surface: None,
            adapter: None,
            config: None,
            queue: None,
            pipeline: None,
            vertex_buffer: None,
            index_buffer: None,
            egui_demo_app: None,
            egui_platform: None,
            egui_egui_rpass: None,
            uniform_buffer: None,
            uniform_bind_group: None,
            start_time: Instant::now(),
        }
    }

    fn reinitialize_window(&mut self, active_event_loop: &ActiveEventLoop) {
        if !self.window.is_none() {
            return;
        }

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

        self.window = Some(window.clone());
        self.wgpu_reinitialize(window);
    }

    fn handle_resize_event(&mut self, new_size: PhysicalSize<u32>) {
        debug!("Window resized: {:?}", new_size);
        self.wgpu_resize(new_size);
    }

    fn handle_key_modifiers_event(&mut self, new_modifiers: Modifiers) {
        self.modifiers = new_modifiers.state();
    }

    fn handle_mouse_input_event(&mut self, state: ElementState, button: MouseButton) {
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

    fn handle_mouse_wheel_event(&mut self, delta: MouseScrollDelta) {}

    fn handle_cursor_moved_by_event(&mut self, position: (f64, f64)) {}

    fn handle_keypress_event(&mut self, active_event_loop: &ActiveEventLoop, event: KeyEvent) {
        match event.physical_key {
            PhysicalKey::Code(KeyCode::Escape) if event.state == ElementState::Pressed => {
                active_event_loop.exit();
            }
            PhysicalKey::Code(KeyCode::Enter)
                if event.state == ElementState::Pressed && self.modifiers.alt_key() =>
            {
                // Toggle fullscreen mode.
                if let Some(window) = &self.window {
                    if window.fullscreen().is_some() {
                        window.set_max_inner_size(Some(Size::Logical(LogicalSize::new(
                            1024.0, 640.0,
                        ))));
                        window.set_fullscreen(None);
                    } else {
                        window.set_max_inner_size::<Size>(None);
                        window.set_fullscreen(Some(Fullscreen::Borderless(None)));
                    }
                }
            }

            _ => {}
        }
    }

    fn wgpu_reinitialize(&mut self, window: Arc<Window>) {
        self.adapter.take();
        self.device.take();
        self.surface.take();

        let size = window.inner_size();
        debug!(width = %size.width, height = %size.height, "Creating WGPU surface");
        let surface = self
            .instance
            .create_surface(window.clone())
            .expect("Failed to create surface");

        let adapter =
            pollster::block_on(self.instance.request_adapter(&wgpu::RequestAdapterOptions {
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
        let mut platform = Platform::new(PlatformDescriptor {
            physical_width: size.width,
            physical_height: size.height,
            scale_factor: window.scale_factor(),
            font_definitions: FontDefinitions::default(),
            style: Default::default(),
        });

        // We use the egui_wgpu_backend crate as the render backend.
        let mut egui_rpass = RenderPass::new(&device, surface_format, 1);

        // Display the demo application that ships with egui.
        let mut demo_app = egui_demo_lib::DemoWindows::default();

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
            attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x3],
        };

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
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
            depth_stencil: None,
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

        self.adapter = Some(adapter);
        self.device = Some(device);
        self.surface = Some(surface);
        self.queue = Some(queue);
        self.config = Some(config);
        self.pipeline = Some(pipeline);
        self.vertex_buffer = Some(vertex_buffer);
        self.index_buffer = Some(index_buffer);
        self.egui_platform = Some(platform);
        self.egui_egui_rpass = Some(egui_rpass);
        self.egui_demo_app = Some(demo_app);
        self.uniform_buffer = Some(uniform_buffer);
        self.uniform_bind_group = Some(uniform_bind_group);
    }

    #[instrument(skip(self))]
    fn wgpu_resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width <= 0 || new_size.height <= 0 {
            warn!("Invalid window size");
            return;
        }

        let Some(surface) = &mut self.surface else {
            warn!("No surface available");
            return;
        };

        let Some(device) = &mut self.device else {
            warn!("No device available");
            return;
        };

        let Some(config) = &mut self.config else {
            warn!("No surface configuration available");
            return;
        };

        config.width = new_size.width;
        config.height = new_size.height;
        surface.configure(&device, &config);
    }

    fn render(&mut self) {
        let surface = self.surface.as_ref().unwrap();
        let device = self.device.as_ref().unwrap();
        let queue = self.queue.as_ref().unwrap();
        let config = self.config.as_ref().unwrap();

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

        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        /*
        rpass.set_pipeline(self.pipeline.as_ref().unwrap());
        rpass.set_vertex_buffer(0, self.vertex_buffer.as_ref().unwrap().slice(..));
        rpass.set_index_buffer(
            self.index_buffer.as_ref().unwrap().slice(..),
            wgpu::IndexFormat::Uint16,
        );
        rpass.draw_indexed(0..INDICES.len() as u32, 0, 0..1);
        */

        drop(rpass);

        let window = self.window.as_ref().unwrap();
        let platform = self.egui_platform.as_mut().unwrap();

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
        let surface_config = self.config.as_ref().unwrap();
        let screen_descriptor = ScreenDescriptor {
            physical_width: surface_config.width,
            physical_height: surface_config.height,
            scale_factor: window.scale_factor() as f32,
        };
        let tdelta: egui::TexturesDelta = full_output.textures_delta;
        let egui_rpass = self.egui_egui_rpass.as_mut().unwrap();
        egui_rpass
            .add_textures(&device, &queue, &tdelta)
            .expect("add texture ok");
        egui_rpass.update_buffers(&device, &queue, &paint_jobs, &screen_descriptor);

        // Record all render passes.
        egui_rpass
            .execute(&mut encoder, &view, &paint_jobs, &screen_descriptor, None)
            .unwrap();

        // Assume that the ShaderArea widget is the one we just added.
        // For simplicity, we fetch its coordinates via egui (you might store them in App state).
        // Here we assume the last ui call produced the shader_response (and you can store shader_area_rect globally).
        // For this example, we'll use a dummy rectangle if needed.
        // (In production, store the rect from the widget callback.)
        let scale = window.scale_factor() as f32;

        // Convert from egui points to physical pixels:
        let area_min_x = shader_area_rect.min.x * scale;
        let area_min_y = shader_area_rect.min.y * scale;
        let area_width = shader_area_rect.width() * scale;
        let area_height = shader_area_rect.height() * scale;
        let screen_w = config.width as f32;
        let screen_h = config.height as f32;
        let center_x = area_min_x + area_width * 0.5;
        let center_y = area_min_y + area_height * 0.5;

        // Convert window center to clip space:
        let center_clip_x = center_x / screen_w * 2.0 - 1.0;
        let center_clip_y = 1.0 - center_y / screen_h * 2.0;

        // Compute scale factors so that quad width in clip space is (area_width/screen_w)*2, etc.
        let scale_x = area_width / screen_w;
        let scale_y = area_height / screen_h;
        let transform: [[f32; 4]; 4] = [
            [scale_x, 0.0, 0.0, center_clip_x],
            [0.0, scale_y, 0.0, center_clip_y],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        // Transpose to match WGSL’s column-major expectation:
        let transform = transpose(transform);
        let uniform_data = Uniforms { transform };

        // Update the uniform buffer with the new transform
        queue.write_buffer(
            self.uniform_buffer.as_ref().unwrap(),
            0,
            bytemuck::cast_slice(&[uniform_data]),
        );

        // ----- NEW: Shader Overlay Pass -----
        {
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
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            shader_rpass.set_pipeline(self.pipeline.as_ref().unwrap());
            // Bind our uniform so the vertex shader gets the transformation matrix.
            shader_rpass.set_bind_group(0, self.uniform_bind_group.as_ref().unwrap(), &[]);
            shader_rpass.set_vertex_buffer(0, self.vertex_buffer.as_ref().unwrap().slice(..));
            shader_rpass.set_index_buffer(
                self.index_buffer.as_ref().unwrap().slice(..),
                wgpu::IndexFormat::Uint16,
            );
            shader_rpass.draw_indexed(0..INDICES.len() as u32, 0, 0..1);
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
        // Pass the winit events to the platform integration.
        if let Some(platform) = &mut self.egui_platform {
            platform.handle_event(&event);
        }

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
                if let Some(platform) = &mut self.egui_platform {
                    platform.update_time(self.start_time.elapsed().as_secs_f64());
                }

                if let Some(window) = &self.window {
                    window.request_redraw();
                    self.render();
                }
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
