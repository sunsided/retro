mod logging;

use crate::logging::setup_logging;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tracing::{debug, error, info};
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
}

impl App {
    pub fn new() -> Self {
        Self {
            window: None,
            modifiers: ModifiersState::default(),
        }
    }

    fn reinitialize_window(&mut self, active_event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
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
        }
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
    let mut app = App::new();
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut app)?;
    Ok(())
}
