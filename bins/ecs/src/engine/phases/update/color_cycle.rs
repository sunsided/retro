//! This module defines the `BackgroundColorCycleSystem`, which is responsible for applying
//! a cyclic transformation to the background color over time. The color is adjusted by
//! gradually modifying the hue component while keeping the saturation and brightness intact.
//!
//! # Structures
//!
//! - `BackgroundColorCycleSystem`: A system that automates the cyclic modification of
//!   background colors.
//! - `BackgroundColorCycleSystemData`: Configuration data for the `BackgroundColorCycleSystem`,
//!   such as the cycle speed (degrees/second).
//!
//! # Implementations
//!
//! - `Default`: Provides default values for `BackgroundColorCycleSystemData` using a
//!   predefined cycle speed.
//! - `ApplyBackgroundColorCycleSystem`: Implements the core logic for how the color
//!   cycle is applied per frame.
//!
//! # Functions
//!
//! - `rgb_to_hsv(r, g, b)`: Converts RGB color values to HSV (Hue, Saturation, Value).
//! - `hsv_to_rgb(h, s, v)`: Converts HSV color values back to RGB.
//!
//! # Core Functionality
//!
//! The core of this system integrates the `FrameContext` to compute the elapsed time,
//! apply the hue modification based on the `cycle_speed`, and convert back and forth
//! between RGB and HSV color spaces. This allows background colors to transition
//! smoothly in a visually appealing way, useful for games, visual effects, or UI elements.

use crate::engine::{
    ApplyBackgroundColorCycleSystem, BackgroundColorComponent, BackgroundColorCycleSystem,
};
use sillyecs::FrameContext;
use std::convert::Infallible;

impl Default for BackgroundColorCycleSystem {
    fn default() -> Self {
        Self(BackgroundColorCycleSystemData::default())
    }
}

#[derive(Debug, Clone)]
pub struct BackgroundColorCycleSystemData {
    /// The speed of the color cycle in degrees per second.
    pub cycle_speed: f32,
}

impl Default for BackgroundColorCycleSystemData {
    fn default() -> Self {
        Self { cycle_speed: 100.0 }
    }
}

impl ApplyBackgroundColorCycleSystem for BackgroundColorCycleSystem {
    type Error = Infallible;

    fn apply_single(&mut self, context: &FrameContext, color: &mut BackgroundColorComponent) {
        let dt = context.delta_time_secs;

        // Convert current RGB to HSV.
        let (mut h, s, v) = rgb_to_hsv(color.r, color.g, color.b);

        // Increment hue by cycle speed * dt (cycle_speed in degrees/sec) and wrap around.
        h = (h + self.0.cycle_speed * dt) % 360.0;

        // Convert back to RGB.
        let (r, g, b) = hsv_to_rgb(h, s, v);
        color.r = r;
        color.g = g;
        color.b = b;
    }
}

/// Converts RGB (Red, Green, Blue) color values to HSV (Hue, Saturation, Value).
///
/// # Arguments
///
/// * `r` - Red component of the color as a floating-point value (0.0 to 1.0).
/// * `g` - Green component of the color as a floating-point value (0.0 to 1.0).
/// * `b` - Blue component of the color as a floating-point value (0.0 to 1.0).
///
/// # Returns
///
/// A tuple `(h, s, v)` where:
/// * `h` - Hue value in degrees (0.0 to 360.0).
/// * `s` - Saturation value as a fraction (0.0 to 1.0).
/// * `v` - Value (brightness) as a fraction (0.0 to 1.0).
fn rgb_to_hsv(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    let h = if delta < 1e-6 {
        0.0
    } else if (max - r).abs() < f32::EPSILON {
        60.0 * (((g - b) / delta) % 6.0)
    } else if (max - g).abs() < f32::EPSILON {
        60.0 * (((b - r) / delta) + 2.0)
    } else {
        60.0 * (((r - g) / delta) + 4.0)
    };

    let h = if h < 0.0 { h + 360.0 } else { h };
    let s = if max.abs() < f32::EPSILON {
        0.0
    } else {
        delta / max
    };
    let v = max;
    (h, s, v)
}

/// Converts HSV (Hue, Saturation, Value) color values to RGB (Red, Green, Blue).
///
/// # Arguments
///
/// * `h` - Hue value in degrees (0.0 to 360.0).
/// * `s` - Saturation value as a fraction (0.0 to 1.0).
/// * `v` - Value (brightness) as a fraction (0.0 to 1.0).
///
/// # Returns
///
/// A tuple `(r, g, b)` where:
/// * `r` - Red component of the color as a floating-point value (0.0 to 1.0).
/// * `g` - Green component of the color as a floating-point value (0.0 to 1.0).
/// * `b` - Blue component of the color as a floating-point value (0.0 to 1.0).
const fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    let c = v * s;
    let h_prime = h / 60.0;
    let x = c * (1.0 - ((h_prime % 2.0) - 1.0).abs());
    let (r1, g1, b1) = if h_prime < 1.0 {
        (c, x, 0.0)
    } else if h_prime < 2.0 {
        (x, c, 0.0)
    } else if h_prime < 3.0 {
        (0.0, c, x)
    } else if h_prime < 4.0 {
        (0.0, x, c)
    } else if h_prime < 5.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };
    let m = v - c;
    (r1 + m, g1 + m, b1 + m)
}
