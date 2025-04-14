//! This module contains various submodules responsible for handling
//! different aspects of the application lifecycle, including:
//!
//! - `fixed_update`: Manages fixed time-step updates.
//! - `update`: Handles variable time-step updates.
//! - `render`: Responsible for rendering-related functionality.
//! - `startup`: Performs actions needed during the application's startup.
//!
//! These submodules are designed to encapsulate specific functionality while
//! promoting modular and organized code structure for the application.

use cgmath::Vector3;

pub mod fixed_update;
pub mod render;
pub mod startup;
pub mod update;

#[inline(always)]
pub fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

#[inline(always)]
fn mixv(a: Vector3<f32>, b: Vector3<f32>, t: f32) -> Vector3<f32> {
    a * (1.0 - t) + b * t
}

#[inline(always)]
fn mix(a: f32, b: f32, t: f32) -> f32 {
    a * (1.0 - t) + b * t
}
