use std::time::Instant;
use crossbeam_channel::TryRecvError;
use tracing::{debug, trace};
use crate::logging::setup_logging;
use crate::phase_events::PhaseEvents;
use crate::game::system::SystemFactory;
use crate::game::world::{MainWorld, RenderState, WorldCommand, WorldCommandReceiver, WorldCommandSender};
use winit::{
    event::{WindowEvent, ElementState},
    event_loop::{ControlFlow, EventLoop},
};
use winit::application::ApplicationHandler;
use winit::dpi::{LogicalSize, Size};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{KeyCode, ModifiersState, PhysicalKey};
use winit::window::{Fullscreen, Window,  WindowId};

mod logging;
mod phase_events;
mod game;

struct App {
    window: Option<Window>,
    modifiers: ModifiersState,
    ecs_world: MainWorld<PhaseEvents, CommandQueue>,
    last_update: Instant,
}

impl App {
    pub fn new() -> Self {
        let events = PhaseEvents::default();
        let state = RenderState::default();
        let world = MainWorld::new(&SystemFactory, state, Some(events), CommandQueue::new());

        Self {
            window: Default::default(),
            modifiers: Default::default(),
            last_update: Instant::now(),
            ecs_world: world
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, active_event_loop: &ActiveEventLoop) {
        let attributes = Window::default_attributes()
            .with_title("Retro 2000")
            .with_inner_size(LogicalSize::new(1024.0, 640.0))
            .with_resizable(false);

        let window = active_event_loop.create_window(attributes).unwrap();
        self.window = Some(window);
        self.last_update = Instant::now();
        self.ecs_world.reset_frame_context();
    }

    fn window_event(&mut self, active_event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                active_event_loop.exit();
            }
            WindowEvent::ModifiersChanged(new_modifiers) => {
                // Update our tracked modifier state.
                self.modifiers = new_modifiers.state();
            }
            WindowEvent::KeyboardInput { event, .. } => {
                match event.physical_key {
                    PhysicalKey::Code(KeyCode::Escape) if event.state == ElementState::Pressed => {
                        active_event_loop.exit();
                    }
                    PhysicalKey::Code(KeyCode::Enter) if event.state == ElementState::Pressed && self.modifiers.alt_key() => {
                        // Toggle fullscreen mode.
                        if let Some(window) = &self.window {
                            if window.fullscreen().is_some() {
                                window.set_max_inner_size(Some(Size::Logical(LogicalSize::new(1024.0, 640.0))));
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
            WindowEvent::MouseInput { state, button, .. } => {
                debug!("Mouse button {:?} is {:?}", button, state);
            }
            WindowEvent::CursorMoved { position, .. } => {
                debug!("Cursor moved to: {:?}", position);
            }
            WindowEvent::Resized(new_size) => {
                debug!("Window resized: {:?}", new_size);
            }
            WindowEvent::RedrawRequested => {
                trace!("Redraw requested");
                self.ecs_world.apply_system_phase_render();

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

struct CommandQueue {
    sender: crossbeam_channel::Sender<WorldCommand>,
    receiver: crossbeam_channel::Receiver<WorldCommand>,
}

impl CommandQueue {
    pub fn new() -> Self {
        let (sender, receiver) = crossbeam_channel::unbounded();
        Self {
            sender,
            receiver,
        }
    }
}

impl WorldCommandReceiver for CommandQueue {
    type Error = TryRecvError;

    fn recv(&self) -> Result<Option<WorldCommand>, Self::Error> {
        match self.receiver.try_recv() {
            Ok(cmd) => Ok(Some(cmd)),
            Err(TryRecvError::Empty) => Ok(None),
            Err(err) => Err(err),
        }
    }
}

impl WorldCommandSender for CommandQueue {
    type Error = crossbeam_channel::SendError<WorldCommand>;

    fn send(&self, command: WorldCommand) -> Result<(), Self::Error> {
        self.sender.send(command)
    }
}
