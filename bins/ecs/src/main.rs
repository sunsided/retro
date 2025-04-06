use crate::logging::setup_logging;
use parking_lot::Mutex;
use std::sync::Arc;
use std::time::Instant;
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
use winit::event::{KeyEvent, Modifiers, MouseButton};
use crate::engine::{CommandQueue, MainWorld, PhaseEvents, SystemFactory};
use crate::engine::phases::render::WgpuRenderState;
use crate::engine::InitializeMainWorld;
use crate::wgpu::{RenderPipelineSelector, WgpuState};
use crate::wgpu::texture::CreateTexture;

mod engine;
mod logging;
mod wgpu;

struct App {
    window: Option<Arc<Window>>,
    state: Option<Arc<Mutex<WgpuState>>>,
    modifiers: ModifiersState,
    ecs_world: MainWorld<PhaseEvents, CommandQueue>,
    last_update: Instant,
}

impl App {
    pub fn new() -> Self {
        let events = PhaseEvents::default();
        let state = WgpuRenderState::default();
        let mut world = MainWorld::new_with_events(&SystemFactory, state, CommandQueue::new(), events);

        world.initialize_entities();

        Self {
            window: None,
            state: None,
            modifiers: Default::default(),
            last_update: Instant::now(),
            ecs_world: world,
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

            // Drop any previous surfaces.
            self.state = None;
            let state = pollster::block_on(WgpuState::new(window.clone()));

            // Recreate the textures.
            let texture = state.texture_from_image("resources/textures/rain-clouds.png", Some("cover.jpg"))
                .expect("Failed to load cover texture");

            // TODO: Refactor
            // The textures could be attributes on a `Renderable` component.
            let system = self.ecs_world.systems.as_render_background_mut();
            system.texture = Some(texture);

            // Set WGPU render state.
            self.state = Some(Arc::new(Mutex::new(state)));
            self.ecs_world.set_wgpu_state(self.state.clone());
        }

        self.last_update = Instant::now();
        self.ecs_world.reset_frame_context();
    }

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
        if let Some(surface) = &mut self.state {
            surface.lock_arc().resize(new_size);
        }
    }

    fn handle_redraw_event(&mut self) {
        trace!("Redraw requested");
        self.ecs_world.apply_system_phase_render();
    }

    fn handle_key_modifiers_event(&mut self, new_modifiers: Modifiers) {
        self.modifiers = new_modifiers.state();
    }

    fn handle_mouse_input_event(&self, state: ElementState, button: MouseButton) {
        debug!("Mouse button {:?} is {:?}", button, state);
    }

    fn handle_cursor_moved_event(&self, position: PhysicalPosition<f64>) {
        debug!("Cursor moved to: {:?}", position);
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, active_event_loop: &ActiveEventLoop) {
        self.reinitialize_window(active_event_loop);
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
                self.handle_cursor_moved_event(position);
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

    let event_loop = EventLoop::new()?;
    let mut app = App::new();

    // Set a continuous loop if needed (for games, often ControlFlow::Poll is used).
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut app)?;

    /*
    let events = PhaseEvents::default();
    let state = RenderState::default();
    let mut world = MainWorld::new(&SystemFactory, state, Some(events), CommandQueue::new());

    let entity = ParticleEntityData {
        position: PositionData::default().into(),
        velocity: VelocityData {
            x: 1.0,
            y: 0.1,
            z: -0.1,
        }.into(),
    };
    let id = entity.spawn_into(&mut world);
    debug_assert_eq!(world.len(), 1);

    // Request a render.
    world.request_render_phase();
    world.apply_system_phases();

    // Do not request a render.
    std::thread::sleep(std::time::Duration::from_secs_f32(0.5));
    world.apply_system_phases();

    // Request another render.
    world.request_render_phase();
    world.apply_system_phases();

    world.despawn_by_id(id).expect("Failed to despawn entity");
    debug_assert_eq!(world.len(), 0);
    world.apply_system_phases();

    */

    Ok(())
}
