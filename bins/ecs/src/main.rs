use crate::cli::parse_arguments;
use crate::engine::components::ActiveCameraState;
use crate::engine::phases::render::WgpuRenderState;
use crate::engine::InitializeMainWorld;
use crate::engine::{CommandQueue, InputState, MainWorld, PhaseEvents, SystemFactory};
use crate::fps_counter::FpsCounter;
use crate::logging::setup_logging;
use crate::wgpu::wgpu_hardware::WgpuHardware;
use ::wgpu::Backends;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tracing::{debug, error, info, trace};
use winit::application::ApplicationHandler;
use winit::dpi::{LogicalSize, PhysicalPosition, PhysicalSize, Size};
use winit::event::{DeviceEvent, DeviceId, KeyEvent, Modifiers, MouseButton, MouseScrollDelta};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{KeyCode, ModifiersState, PhysicalKey};
use winit::monitor::MonitorHandle;
use winit::window::{Fullscreen, Window, WindowId};
use winit::{
    event::{ElementState, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

mod camera;
mod cli;
mod engine;
mod fps_counter;
mod logging;
mod resources;
mod seconds;
mod wgpu;

struct App {
    window: Option<Arc<Window>>,
    modifiers: ModifiersState,
    ecs_world: MainWorld<PhaseEvents, CommandQueue>,
    last_update: Instant,
    render_fps: FpsCounter,
    render_max_fps: FpsCounter,
    last_fps_output: Instant,
    frame_duration: Duration,
    next_frame_time: Instant,
    target_fps: f32,
}

impl App {
    pub fn new(wgpu_hardware: WgpuHardware) -> Self {
        let events = PhaseEvents::default();
        let render_state = WgpuRenderState::default();
        let input_state = InputState::default();
        let active_camera = ActiveCameraState::default();
        let mut world = MainWorld::new_with_events(
            &SystemFactory,
            input_state,
            render_state,
            active_camera,
            CommandQueue::new(),
            events,
        );
        world.states.wgpu_render.set_hardware(wgpu_hardware);
        world.create_initial_entities();

        let count = world
            .frontload_cubes_by_scan(|entity| entity.position.x > 7.0 && entity.position.z > 9.0);
        debug!("{count}");

        Self {
            window: None,
            modifiers: Default::default(),
            last_update: Instant::now(),
            ecs_world: world,
            render_fps: FpsCounter::new(0.1),
            render_max_fps: FpsCounter::new(0.1),
            last_fps_output: Instant::now(),
            target_fps: 60.0,
            frame_duration: Default::default(),
            next_frame_time: Instant::now(),
        }
    }

    fn update_frame_duration(&mut self, frame_duration: Duration) {
        self.target_fps = frame_duration.as_secs_f32().recip();
        self.frame_duration = frame_duration;
    }

    fn reinitialize_window(&mut self, active_event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            // Initialize if missing.
            self.update_frame_duration(Duration::from_secs_f32(self.target_fps.recip()));

            let attributes = Window::default_attributes()
                .with_title("Retro 2000")
                .with_inner_size(LogicalSize::new(1024.0, 640.0))
                .with_resizable(false);

            let window = Arc::new(
                active_event_loop
                    .create_window(attributes)
                    .expect("Failed to create window"),
            );

            if let Some(millihertz) = window
                .current_monitor()
                .and_then(|monitor: MonitorHandle| monitor.refresh_rate_millihertz())
            {
                let hz = (millihertz as f64) * 0.001;
                let refresh_duration = Duration::from_secs_f64(hz.recip());
                info!(
                    "Display refresh rate: {:.2} Hz, frame duration: {:?}",
                    hz, refresh_duration
                );
                self.update_frame_duration(refresh_duration);
            } else {
                error!("Failed to obtain display refresh rate, using default frame duration");
                self.update_frame_duration(Duration::from_secs_f32(60.0_f32.recip()));
            }

            self.window = Some(window.clone());
            self.ecs_world
                .states
                .wgpu_render
                .reinitialize_surface(window.clone())
                .expect("Failed to reinitialize window");
        }

        self.last_update = Instant::now();
        self.ecs_world.reset_frame_context();
    }

    fn handle_resize_event(&mut self, new_size: PhysicalSize<u32>) {
        debug!("Window resized: {:?}", new_size);
        self.ecs_world
            .states
            .wgpu_render
            .resize_surface(new_size)
            .expect("Failed to resize window");
    }

    fn handle_key_modifiers_event(&mut self, new_modifiers: Modifiers) {
        self.modifiers = new_modifiers.state();
    }

    fn handle_mouse_input_event(&mut self, state: ElementState, button: MouseButton) {
        match button {
            MouseButton::Left => {
                self.ecs_world
                    .states
                    .input
                    .next()
                    .set_lmb_pressed(state == ElementState::Pressed);
            }
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

    fn handle_mouse_wheel_event(&mut self, delta: MouseScrollDelta) {
        self.ecs_world
            .states
            .input
            .next()
            .accumulate_mouse_scroll_delta(delta);
    }

    fn handle_cursor_moved_by_event(&mut self, position: (f64, f64)) {
        let input = self.ecs_world.states.input.next();
        if input.lmb_pressed() {
            input.accumulate_mouse_move_delta(-position.0, -position.1);
        }
    }

    fn handle_keypress_event(&mut self, active_event_loop: &ActiveEventLoop, event: KeyEvent) {
        if self
            .ecs_world
            .states
            .input
            .next()
            .process_keys(event.physical_key, event.state)
        {
            return;
        }

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

            // Toggle shading mode
            PhysicalKey::Code(KeyCode::F1) if event.state == ElementState::Pressed => {
                self.ecs_world.states.wgpu_render.toggle_pipeline_selector();
            }

            // Toggle retro mode
            PhysicalKey::Code(KeyCode::F2) if event.state == ElementState::Pressed => {
                let retro_enabled = self.ecs_world.states.wgpu_render.retro_enabled();
                self.ecs_world
                    .states
                    .wgpu_render
                    .set_retro_enabled(!retro_enabled);
            }
            _ => {}
        }
    }

    /// Run a game loop iteration without calling the Render phase.
    /// See [`run_render_phase`](Self::run_render_phase).
    fn run_system_phases_without_render(&mut self) {
        // Update ECS.
        self.ecs_world.apply_system_phases();

        // Reset the input.
        self.ecs_world.states.input.reset_current_deltas();
        self.ecs_world.states.input.maybe_swap();

        // Render only when the scheduled time has been reached.
        let now = Instant::now();
        if now >= self.next_frame_time {
            if let Some(window) = &self.window {
                window.request_redraw();
            }

            // Update next_frame_time; catch up if we're behind.
            self.next_frame_time += self.frame_duration;
            while Instant::now() >= self.next_frame_time {
                self.next_frame_time += self.frame_duration;
            }
        }
    }

    /// Runs only the render phase.
    fn run_render_phase(&mut self) {
        let fps = self.render_fps.update_with(self.last_update);
        self.last_update = Instant::now();

        trace!(%fps, "Redraw requested");
        let render_start = Instant::now();
        self.ecs_world.apply_system_phase_render();
        let max_fps = self.render_max_fps.update_with(render_start);

        if self.last_fps_output.elapsed().as_secs() > 1 {
            self.last_fps_output = Instant::now();
            debug!(%fps, %max_fps, "Render FPS");
        }

        self.next_frame_time = Instant::now() + self.frame_duration;
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
                self.run_render_phase();
            }
            _ => (),
        }
    }

    fn about_to_wait(&mut self, _active_event_loop: &ActiveEventLoop) {
        self.run_system_phases_without_render();
    }
}

fn main() -> eyre::Result<()> {
    dotenvy::dotenv().ok();
    let _logging = setup_logging();

    let args = parse_arguments();
    let backend = args
        .get_one::<Backends>("backend")
        .cloned()
        .unwrap_or(Backends::GL);

    let wgpu_hardware = WgpuHardware::initialize(backend);
    let mut app = App::new(wgpu_hardware);

    // Set a continuous loop if needed (for games, often ControlFlow::Poll is used).
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut app)?;

    Ok(())
}
