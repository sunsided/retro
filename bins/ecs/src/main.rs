use crate::game::system::SystemFactory;
use crate::game::world::{
    MainWorld, RenderState, WorldCommand, WorldCommandReceiver, WorldCommandSender,
};
use crate::logging::setup_logging;
use crate::phase_events::PhaseEvents;
use crossbeam_channel::TryRecvError;
use parking_lot::Mutex;
use std::cell::{Ref, RefCell};
use std::sync::Arc;
use std::time::Instant;
use tracing::{debug, trace};
use wgpu::{Adapter, CompositeAlphaMode, Device, Queue, TextureFormat};
use winit::application::ApplicationHandler;
use winit::dpi::{LogicalSize, PhysicalSize, Size};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{KeyCode, ModifiersState, PhysicalKey};
use winit::window::{Fullscreen, Window, WindowId};
use winit::{
    event::{ElementState, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

mod game;
mod logging;
mod phase_events;

pub struct WgpuState {
    instance: wgpu::Instance,
    surface: wgpu::Surface<'static>,
    surface_format: TextureFormat,
    adapter: Adapter,
    device: Device,
    queue: Queue,
}

impl WgpuState {
    pub async fn new(window: Arc<Window>) -> WgpuState {
        // https://github.com/rust-windowing/winit/discussions/3667#discussioncomment-9329312
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(Arc::clone(&window)).unwrap();

        let adapter = pollster::block_on(instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ))
            .unwrap();

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: Default::default(),
            },
            None,
        ))
            .unwrap();

        let surface_format = surface
            .get_capabilities(&adapter)
            .formats
            .first()
            .cloned()
            .expect("No format found");

        let mut state = Self {
            instance,
            surface,
            surface_format,
            adapter,
            device,
            queue
        };
        state.resize(window.inner_size());

        state
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        debug!("Resizing window to {:?}", new_size);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: self.surface_format,
            width: new_size.width,
            height: new_size.height,
            present_mode: wgpu::PresentMode::Fifo,
            desired_maximum_frame_latency: 1, // Set to 1 for single frame in flight
            alpha_mode: CompositeAlphaMode::Opaque,
            view_formats: vec![self.surface_format], // Set to the surface's format
        };
        self.surface.configure(&self.device, &config);
    }

    pub fn draw(&mut self) {
        trace!("Requesting draw from surface");
    }
}

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
        let state = RenderState::default();
        let world = MainWorld::new(&SystemFactory, state, Some(events), CommandQueue::new());

        Self {
            window: None,
            state: None,
            modifiers: Default::default(),
            last_update: Instant::now(),
            ecs_world: world,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, active_event_loop: &ActiveEventLoop) {
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

            let state = pollster::block_on(WgpuState::new(window.clone()));
            self.state = Some(Arc::new(Mutex::new(state)));

            // Set WGPU render state.
            let render = self.ecs_world.systems.as_render_mut();
            render.set_state(self.state.clone());
        }

        self.last_update = Instant::now();
        self.ecs_world.reset_frame_context();
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
                // Update our tracked modifier state.
                self.modifiers = new_modifiers.state();
            }
            WindowEvent::KeyboardInput { event, .. } => {
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
            WindowEvent::MouseInput { state, button, .. } => {
                debug!("Mouse button {:?} is {:?}", button, state);
            }
            WindowEvent::CursorMoved { position, .. } => {
                debug!("Cursor moved to: {:?}", position);
            }
            WindowEvent::Resized(new_size) => {
                debug!("Window resized: {:?}", new_size);
                if let Some(surface) = &mut self.state {
                    surface.lock_arc().resize(new_size);
                }
            }

            // TODO: Allow external access to system data
            WindowEvent::RedrawRequested => {
                trace!("Redraw requested");
                if let Some(surface) = &mut self.state {
                    self.ecs_world.apply_system_phase_render();
                    surface.lock_arc().draw();
                }
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
        Self { sender, receiver }
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
