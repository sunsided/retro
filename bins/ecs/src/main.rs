use crate::logging::setup_logging;
use parking_lot::Mutex;
use std::sync::Arc;
use std::time::Instant;
use ::wgpu::{Backends, InstanceDescriptor};
use tracing::{debug, trace};
use winit::application::ApplicationHandler;
use winit::dpi::{LogicalSize, PhysicalPosition, PhysicalSize, Size};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{KeyCode, ModifiersState, PhysicalKey};
use winit::window::{Fullscreen, Window, WindowId};
use winit::{
    event::{ElementState, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};
use winit::event::{DeviceEvent, DeviceId, KeyEvent, Modifiers, MouseButton, MouseScrollDelta};
use crate::camera::CameraController;
use crate::cli::parse_arguments;
use crate::engine::{CommandQueue, MainWorld, PhaseEvents, SystemFactory};
use crate::engine::phases::render::WgpuRenderState;
use crate::engine::InitializeMainWorld;
use crate::wgpu::{RenderPipelineSelector, WgpuState};
use crate::wgpu::gpu_hardware::WgpuHardware;
use crate::wgpu::texture::{CreateTexture, Texture, TextureType};

mod engine;
mod logging;
mod wgpu;
mod camera;
mod resources;
mod cli;

struct App {
    window: Option<Arc<Window>>,
    state: Option<Arc<Mutex<WgpuState>>>,
    modifiers: ModifiersState,
    ecs_world: MainWorld<PhaseEvents, CommandQueue>,
    last_update: Instant,
    camera_controller: CameraController,
    mouse_pressed: bool,
}

impl App {
    pub fn new(wgpu_hardware: WgpuHardware) -> Self {
        let events = PhaseEvents::default();
        let state = WgpuRenderState::default();
        let mut world = MainWorld::new_with_events(&SystemFactory, state, CommandQueue::new(), events);
        world.states.wgpu_render.set_hardware(wgpu_hardware);
        world.initialize_entities();

        Self {
            window: None,
            state: None,
            modifiers: Default::default(),
            last_update: Instant::now(),
            ecs_world: world,
            camera_controller: CameraController::new(4.0, 0.4),
            mouse_pressed: false
        }
    }

    fn reinitialize_window(&mut self, active_event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let attributes = Window::default_attributes()
                .with_title("Retro 2000")
                .with_inner_size(LogicalSize::new(1024.0, 640.0))
                .with_resizable(false);

            let window = Arc::new(
                active_event_loop
                    .create_window(attributes)
                    .expect("Failed to create window"),
            );
            self.window = Some(window.clone());

            self.ecs_world.states.wgpu_render.reinitialize_surface(window.clone())
                .expect("Failed to reinitialize window");

            /*
            // Drop any previous surfaces.
            self.state = None;
            let state = pollster::block_on(WgpuState::new(window.clone()));

            // Create a fallback normal map.
            let fallback_normals = state.fallback_normal_map();

            // Recreate the textures.
            let texture = state.texture_from_image("resources/textures/cover.webp", TextureType::Diffuse, Some("cover.jpg"))
                .expect("Failed to load cover texture");

            let bind_group = state.create_bind_group(&texture, &fallback_normals);

            // TODO: Refactor
            // The textures could be attributes on a `Renderable` component.
            let system = self.ecs_world.systems.as_render_background_mut();
            system.texture = Some(bind_group);

            // Set WGPU render state.
            self.state = Some(Arc::new(Mutex::new(state)));
            self.ecs_world.set_wgpu_state(self.state.clone());
            */
        }

        self.last_update = Instant::now();
        self.ecs_world.reset_frame_context();
    }

    fn handle_keypress_event(&mut self, active_event_loop: &ActiveEventLoop, event: KeyEvent) {
        if self.camera_controller.process_events(event.physical_key, event.state) {
            return;
        };

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
            PhysicalKey::Code(KeyCode::Space)
            if event.state == ElementState::Pressed =>
                {
                    let system = self.ecs_world.systems.as_render_background_mut();
                    system.selector = match system.selector {
                        RenderPipelineSelector::Boring => RenderPipelineSelector::Colored,
                        RenderPipelineSelector::Colored => RenderPipelineSelector::Boring
                    };
                }
            _ => {}
        }
    }

    fn handle_resize_event(&mut self, new_size: PhysicalSize<u32>) {
        debug!("Window resized: {:?}", new_size);
        let Some(state) = &mut self.state else {
            return;
        };

        let mut state = state.lock_arc();
        state.resize(new_size);
        state.projection.resize(new_size.width, new_size.height);
    }

    fn handle_redraw_event(&mut self) {
        trace!("Redraw requested");
        self.ecs_world.apply_system_phase_render();
    }

    fn handle_key_modifiers_event(&mut self, new_modifiers: Modifiers) {
        self.modifiers = new_modifiers.state();
    }

    fn handle_mouse_input_event(&mut self, state: ElementState, button: MouseButton) {
        debug!("Mouse button {:?} is {:?}", button, state);
        match button {
            MouseButton::Left => {
                self.mouse_pressed = state == ElementState::Pressed;
            }
            MouseButton::Right => {}
            MouseButton::Middle => {}
            MouseButton::Back => {}
            MouseButton::Forward => {}
            MouseButton::Other(_) => {}
        }
    }

    fn handle_mouse_wheel_event(&mut self, delta: MouseScrollDelta) {
        debug!("Mouse wheel {:?}", delta);
        self.camera_controller.process_scroll(&delta);
    }

    fn handle_cursor_moved_by_event(&mut self, position: (f64, f64)) {
        trace!("Cursor moved by: {:?}", position);
        if self.mouse_pressed {
            self.camera_controller.process_mouse(-position.0, -position.1);
        }
    }

    fn handle_cursor_moved_to_event(&mut self, position: PhysicalPosition<f64>) {
        debug!("Cursor moved to: {:?}", position);
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, active_event_loop: &ActiveEventLoop) {
        self.reinitialize_window(active_event_loop);
    }

    fn device_event(&mut self, event_loop: &ActiveEventLoop, device_id: DeviceId, event: DeviceEvent) {
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
            WindowEvent::Resized(new_size) => {
                self.handle_resize_event(new_size);
            }
            WindowEvent::RedrawRequested => {
                self.handle_redraw_event();
            }
            _ => (),
        }
    }

    fn about_to_wait(&mut self, _active_event_loop: &ActiveEventLoop) {
        let dt = self.last_update.elapsed();
        self.last_update = Instant::now();

        // Update the camera, light, etc.
        let state = &mut self.ecs_world.states.wgpu_render;
        if let Some(mut state) = state.state_mut() {
            state.update_camera_uniform(&mut self.camera_controller, dt);
            state.update_light();
        }

        // Update ECS.
        self.ecs_world.apply_system_phases();

        // Request a redraw each cycle.
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}

fn main() -> eyre::Result<()> {
    dotenvy::dotenv().ok();
    let _logging = setup_logging();

    let args = parse_arguments();
    let backend = args.get_one::<Backends>("backend").cloned().unwrap_or(Backends::GL);

    let wgpu_hardware = WgpuHardware::initialize(backend);
    let mut app = App::new(wgpu_hardware);

    // Set a continuous loop if needed (for games, often ControlFlow::Poll is used).
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut app)?;

    Ok(())
}
