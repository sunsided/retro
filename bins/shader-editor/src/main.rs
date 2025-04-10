mod logging;

use crate::logging::setup_logging;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tracing::{debug, error, info};
use wgpu::{
    Adapter, Backends, Device, Instance, InstanceDescriptor, PresentMode, Surface,
    SurfaceConfiguration, TextureUsages,
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

struct App {
    window: Option<Arc<Window>>,
    modifiers: ModifiersState,
    instance: Instance,
    surface: Option<Surface<'static>>,
    device: Option<Device>,
    adapter: Option<Adapter>,
}

impl App {
    pub fn new(instance: Instance) -> Self {
        Self {
            window: None,
            modifiers: ModifiersState::default(),
            instance,
            device: None,
            surface: None,
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
        let size = window.inner_size();
        debug!(width = %size.width, height = %size.height, "Creating WGPU surface");
        let surface = self
            .instance
            .create_surface(window)
            .inspect_err(|error| tracing::error!(%error, "Failed to create surface"))?;
        self.surface = Some(surface);

        let Some(surface) = self.surface.as_ref() else {
            unreachable!("A surface that was just created should always be available");
        };

        // If we already have an adapter, and it is compatible with the new surface, skip creation.
        let surface_caps = &self
            .adapter
            .map(|adapter| surface.get_capabilities(&adapter))
            .unwrap_or_default();
        let recreate_device = surface_caps.formats.is_empty();
        if recreate_device {
            let adapter = self.request_adapter(surface)?;
            let (device, queue) = self.request_device(&adapter)?;

            let surface_caps = surface.get_capabilities(&adapter);

            let config = SurfaceConfiguration {
                usage: TextureUsages::RENDER_ATTACHMENT,
                format: surface_caps.formats[0],
                width: size.width,
                height: size.height,
                present_mode: PresentMode::Fifo, // surface_caps.present_modes[0],
                alpha_mode: surface_caps.alpha_modes[0],
                view_formats: vec![],
                desired_maximum_frame_latency: 2,
            };

            self.adapter = Some(adapter);
            self.device = Some(device);
        } else {
            let Some(adapter) = self.adapter.as_mut() else {
                unreachable!("The adapter is known to exist");
            };

            let Some(device) = self.device.as_mut() else {
                unreachable!("Since the adapter existed, the surface also must exist");
            };

            device.set_config(Self::get_surface_config(size, surface, &adapter));
        }
        Ok(())
    }

    fn wgpu_resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width <= 0 || new_size.height <= 0 {
            return;
        }

        let Some(mut surface) = &mut self.surface else {
            return;
        };

        let Some(device) = &mut self.device else {
            return;
        };

        device.resize_surface(new_size, &mut surface);
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
            WindowEvent::RedrawRequested => {
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            _ => (),
        }
    }

    fn about_to_wait(&mut self, _active_event_loop: &ActiveEventLoop) {
        // request another r
    }
}

fn main() -> eyre::Result<()> {
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
