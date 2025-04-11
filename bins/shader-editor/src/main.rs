mod logging;

use crate::logging::setup_logging;
use egui::{FontDefinitions, Pos2};
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

    @vertex
    fn vs_main(input: VertexInput) -> VertexOutput {
        var out: VertexOutput;
        out.clip_position = vec4<f32>(input.position, 0.0, 1.0);
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

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline"),
            bind_group_layouts: &[],
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

        rpass.set_pipeline(self.pipeline.as_ref().unwrap());
        rpass.set_vertex_buffer(0, self.vertex_buffer.as_ref().unwrap().slice(..));
        rpass.set_index_buffer(
            self.index_buffer.as_ref().unwrap().slice(..),
            wgpu::IndexFormat::Uint16,
        );
        rpass.draw_indexed(0..INDICES.len() as u32, 0, 0..1);

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
        egui::Window::new("My Draggable Frame")
            .default_pos(Pos2::new(100.0, 100.0))
            .collapsible(true)
            .show(&ctx, |ui| {
                egui::CollapsingHeader::new("Panel Title")
                    .default_open(true)
                    .show(ui, |ui| {
                        ui.label("Basic text content inside the panel.");
                    });
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
