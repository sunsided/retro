//! This module provides the [`WgpuState`] struct and its associated implementation for managing
//! WebGPU state within a Rust application. It abstracts over the low-level WebGPU API to
//! simplify tasks such as creating surfaces, configuring devices, handling window resizing,
//! and submitting command buffers to the GPU.
//!
//! # Overview
//!
//! The [`WgpuState`] struct is designed to handle core WebGPU initialization and management tasks,
//! including creating a suitable adapter, device, and queue, as well as configuring surfaces for
//! rendering. This abstraction makes it easier to manage the GPU rendering lifecycle in modern
//! applications that use the `wgpu` and `winit` crates.
//!
//! # Features
//!
//! - Creates and manages a WebGPU instance, adapter, device, queue, and surface.
//! - Configures the surface with optimal settings for rendering.
//! - Handles window resizing by reconfiguring the surface appropriately.
//! - Provides utility methods to poll the device and submit command buffers for rendering.
//!
//! # Structs
//!
//! - `WgpuState`: The main struct of this module. It wraps and manages WebGPU objects
//!   such as the `Instance`, `Surface`, `Adapter`, `Device`, and `Queue`.
//!
//! # References
//!
//! For more details about the underlying WebGPU API, see the [wgpu documentation](https://docs.rs/wgpu).

use cgmath::prelude::*;
use std::sync::Arc;
use std::time::Duration;
use winit::window::Window;
use crate::camera::CameraController;
use crate::wgpu::light::LightUniform;

#[allow(dead_code)]
pub struct WgpuState {
}

impl WgpuState {
    pub async fn new(window: Arc<Window>) -> WgpuState {
        // TODO: Brutally refactor this
        let light_uniform = LightUniform::new(
            [2.0, 2.0, 2.0],
            [1.0, 1.0, 1.0]);

        Self {
        }
    }

    /*
    /// Update the camera uniform buffer.
    // TODO: Refactor
    pub fn update_camera_uniform(&mut self, controller: &mut CameraController, dt: Duration) {
        controller.update_camera(&mut self.camera, dt);
        self.camera_uniform.update_view_proj(&self.camera, &self.projection);
        self.queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[self.camera_uniform]));
    }

    // TODO: Refactor
    pub fn update_light(&mut self) {
        let old_position: cgmath::Vector3<_> = self.light_uniform.position.into();
        self.light_uniform.position =
            (cgmath::Quaternion::from_axis_angle((0.0, 1.0, 0.0).into(), cgmath::Deg(1.0))
                * old_position)
                .into();
        self.queue.write_buffer(&self.light_buffer, 0, bytemuck::cast_slice(&[self.light_uniform]));
    }
     */
}
