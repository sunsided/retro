use crossbeam_channel::TryRecvError;
use crate::game::archetype::ParticleEntityData;
use crate::game::component::{PositionData, VelocityData};
use crate::logging::setup_logging;
use crate::phase_events::PhaseEvents;
use crate::game::system::SystemFactory;
use crate::game::world::{MainWorld, RenderState, WorldCommand, WorldCommandReceiver, WorldCommandSender};
use winit::{
    event::{Event, WindowEvent, ElementState},
    event_loop::{ControlFlow, EventLoop},
};
use winit::application::ApplicationHandler;
use winit::dpi::{LogicalSize, Size};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{Key, KeyCode, ModifiersState, NamedKey, PhysicalKey};
use winit::window::{Fullscreen, Window, WindowAttributes, WindowId};

mod logging;
mod phase_events;
mod game;

#[derive(Default)]
struct App {
    window: Option<Window>,
    modifiers: ModifiersState,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, active_event_loop: &ActiveEventLoop) {
        let mut attributes = Window::default_attributes();
        attributes.title = "Retro 2000".to_string();
        attributes.inner_size = Some(Size::Logical(LogicalSize::new(1024.0, 640.0)));
        attributes.resizable = false;
        let window = active_event_loop.create_window(attributes).unwrap();
        self.window = Some(window);
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
                println!("Mouse button {:?} is {:?}", button, state);
            }
            WindowEvent::CursorMoved { position, .. } => {
                println!("Cursor moved to: {:?}", position);
            }
            WindowEvent::Resized(new_size) => {
                println!("Window resized: {:?}", new_size);
            }
            _ => (),
        }
    }

    fn about_to_wait(&mut self, _active_event_loop: &ActiveEventLoop) {
        // Request a redraw each cycle.
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}

fn main() -> eyre::Result<()> {
    dotenvy::dotenv().ok();
    let _logging = setup_logging();

    let event_loop = EventLoop::new().unwrap();
    let mut app = App::default();

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
