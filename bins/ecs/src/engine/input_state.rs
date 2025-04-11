use crate::seconds::Seconds;
use std::time::Instant;
use tracing::debug;
use winit::dpi::PhysicalPosition;
use winit::event::{ElementState, MouseScrollDelta};
use winit::keyboard::{KeyCode, PhysicalKey};

/// The sample rate of the input.
const SWAP_RATE_HZ: f32 = 120.0;
const SWAP_RATE_SECONDS: f32 = 1.0 / SWAP_RATE_HZ;

#[derive(Debug)]
pub struct InputState {
    last_swap: Instant,
    next: InputStateBuffer,
    current: InputStateBuffer,
}

impl Default for InputState {
    fn default() -> Self {
        Self {
            last_swap: Instant::now(),
            next: InputStateBuffer::default(),
            current: InputStateBuffer::default(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct InputStateBuffer {
    pub accumulation_delta: Seconds,
    pub lmb_pressed: KeyState,
    pub mouse_move_dx: f32,
    pub mouse_move_dy: f32,
    pub mouse_scroll_delta: f32,
    pub key_forward: KeyState,
    pub key_backward: KeyState,
    pub key_left: KeyState,
    pub key_right: KeyState,
    pub key_up: KeyState,
    pub key_down: KeyState,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum KeyState {
    #[default]
    Released,
    Pressed,
}

impl InputState {
    pub fn reset_current_deltas(&mut self) {
        self.current.reset();
    }

    pub fn maybe_swap(&mut self) {
        let elapsed = self.last_swap.elapsed().as_secs_f32();
        if elapsed < SWAP_RATE_SECONDS {
            return;
        }

        // Record the swapping time to restart tracking.
        self.last_swap = Instant::now();

        // Swap staging buffer with presented view.
        self.current = self.next.clone();

        // Ensure that the presented view sees the elapsed input time.
        self.current.accumulation_delta = Seconds::new(elapsed);

        // Reset the staging buffer.
        self.next.reset();
    }

    pub const fn delta(&self) -> Seconds {
        self.current.accumulation_delta
    }

    pub const fn current(&self) -> &InputStateBuffer {
        &self.current
    }

    pub const fn next(&mut self) -> &mut InputStateBuffer {
        &mut self.next
    }
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
        if value {
            KeyState::Pressed
        } else {
            KeyState::Released
        }
    }
}

impl From<ElementState> for KeyState {
    fn from(value: ElementState) -> Self {
        match value {
            ElementState::Pressed => KeyState::Pressed,
            ElementState::Released => KeyState::Released,
        }
    }
}

impl InputStateBuffer {
    fn reset(&mut self) {
        // Keep all persistent state like key presses but reset deltas.
        self.accumulation_delta = Seconds::new(0.0);
        self.mouse_move_dx = 0.0;
        self.mouse_move_dy = 0.0;
        self.mouse_scroll_delta = 0.0;
    }

    pub fn lmb_pressed(&mut self) -> bool {
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
            MouseScrollDelta::PixelDelta(PhysicalPosition { y: scroll, .. }) => scroll as f32,
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
