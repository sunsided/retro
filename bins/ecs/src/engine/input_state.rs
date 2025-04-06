use winit::dpi::PhysicalPosition;
use winit::event::{ElementState, MouseScrollDelta};
use winit::keyboard::{KeyCode, PhysicalKey};

#[derive(Debug, Default)]
pub struct InputState {
    lmb_pressed: KeyState,
    mouse_move_dx: f32,
    mouse_move_dy: f32,
    mouse_scroll_delta: f32,
    key_forward: KeyState,
    key_backward: KeyState,
    key_left: KeyState,
    key_right: KeyState,
    key_up: KeyState,
    key_down: KeyState
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum KeyState {
    #[default]
    Released,
    Pressed,
}

impl KeyState {
    pub fn is_pressed(&self) -> bool {
        match self {
            KeyState::Pressed => true,
            KeyState::Released => false,
        }
    }
}

impl From<bool> for KeyState {
    fn from(value: bool) -> Self {
        if value { KeyState::Pressed } else { KeyState::Released }
    }
}

impl From<ElementState> for KeyState {
    fn from(value: ElementState) -> Self {
        match value {
            ElementState::Pressed => KeyState::Pressed,
            ElementState::Released => KeyState::Released,
        }
    }}

impl InputState {
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub const fn lmb_pressed(&mut self) -> bool {
        self.lmb_pressed.is_pressed()
    }

    pub fn set_lmb_pressed(&mut self, pressed: bool) {
        self.lmb_pressed = pressed.into();
    }

    pub fn accumulate_mouse_move_delta(&mut self, dx: f64, dy: f64) {
        self.mouse_move_dx += dx as f32;
        self.mouse_move_dy += dy as f32;
    }

    pub fn accumulate_mouse_scroll_delta(&mut self, delta: MouseScrollDelta) {
        let delta = -match delta {
            // I'm assuming a line is about 1 pixels
            MouseScrollDelta::LineDelta(_, scroll) => scroll * 1.0,
            MouseScrollDelta::PixelDelta(PhysicalPosition {
                                             y: scroll,
                                             ..
                                         }) => scroll as f32,
        };
        self.mouse_scroll_delta += delta;
    }

    pub fn process_keys(&mut self, key: PhysicalKey, state: ElementState) -> bool {
        let PhysicalKey::Code(keycode) = key else {
            return false;
        };

        match keycode {
            KeyCode::KeyW | KeyCode::ArrowUp => {
                self.key_forward = state.into();
                true
            }
            KeyCode::KeyS | KeyCode::ArrowDown => {
                self.key_backward = state.into();
                true
            }
            KeyCode::KeyA | KeyCode::ArrowLeft => {
                self.key_left = state.into();
                true
            }
            KeyCode::KeyD | KeyCode::ArrowRight => {
                self.key_right = state.into();
                true
            }
            KeyCode::Space => {
                self.key_up = state.into();
                true
            }
            KeyCode::ShiftLeft => {
                self.key_down = state.into();
                true
            }
            _ => false,
        }
    }
}
