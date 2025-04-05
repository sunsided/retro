use crate::game::system::SystemFactory;
use crate::game::world::{
    MainWorld, RenderState, WorldCommand, WorldCommandReceiver, WorldCommandSender,
};
use crate::logging::setup_logging;
use crate::phase_events::PhaseEvents;
use crossbeam_channel::TryRecvError;
use parking_lot::Mutex;
use std::sync::Arc;
use std::time::Instant;
use tracing::{debug, trace};
use wgpu::{Adapter, Device, InstanceDescriptor, PresentMode, Queue};
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
    _instance: wgpu::Instance,
    surface: wgpu::Surface<'static>,
    _adapter: Adapter,
    device: Device,
    queue: Queue,
    surface_config: wgpu::SurfaceConfiguration,
}

impl WgpuState {
    pub async fn new(window: Arc<Window>) -> WgpuState {
        // https://github.com/rust-windowing/winit/discussions/3667#discussioncomment-9329312

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
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: PresentMode::Fifo, // surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        Self {
            _instance: instance,
            surface,
            surface_config,
            _adapter: adapter,
            device,
            queue
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            debug!("Resizing window to {:?}", new_size);
            self.surface_config.width = new_size.width;
            self.surface_config.height = new_size.height;
            self.surface.configure(&self.device, &self.surface_config);
        }
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

            // Drop any previous surfaces.
            self.state = None;

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
