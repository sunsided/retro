//! This module provides utilities for creating and managing perspective projection matrices
//! used in 3D rendering applications.
//!
//! The core functionality is provided by the [`Projection`] struct, which encapsulates parameters
//! for configuring a projection matrix such as the aspect ratio, field of view, and near/far
//! clipping planes. Additionally, it includes methods for dynamically updating the aspect
//! ratio and calculating the projection matrix in a format compatible with WGPU's coordinate
//! system.
//!
//! # Key Features
//! - Create a customizable perspective projection via the `Projection` struct.
//! - Resize the aspect ratio dynamically when the rendering dimensions change.
//! - Generate a perspective projection matrix that works with WGPU.

use crate::camera::OPENGL_TO_WGPU_MATRIX;
use cgmath::{perspective, Matrix4, Rad};

/// Represents a perspective projection matrix configuration for 3D rendering.
///
/// This struct defines the parameters required for creating a projection matrix,
/// such as the aspect ratio, field of view, and near/far clipping planes.
///
/// - `aspect`: The aspect ratio of the projection (width divided by height).
/// - `fovy`: The field of view in the y-direction, measured in radians.
/// - `znear`: The distance to the near clipping plane.
/// - `zfar`: The distance to the far clipping plane.
#[derive(Debug, Clone)]
pub struct Projection {
    /// The aspect ratio (width / height) of the projection.
    aspect: f32,
    /// The field of view in the y-direction, in radians.
    fovy: Rad<f32>,
    /// The near clipping plane distance.
    znear: f32,
    /// The far clipping plane distance.
    zfar: f32,
}

impl Projection {
    pub fn new<F: Into<Rad<f32>>>(width: u32, height: u32, fovy: F, znear: f32, zfar: f32) -> Self {
        Self {
            aspect: width as f32 / height as f32,
            fovy: fovy.into(),
            znear,
            zfar,
        }
    }

    /// Updates the aspect ratio of the projection based on the provided width and height.
    ///
    /// This function should be called whenever the dimensions of the rendering surface change,
    /// ensuring that the projection matrix remains accurate.
    ///
    /// # Parameters
    /// - `width`: The new width of the rendering surface.
    /// - `height`: The new height of the rendering surface.
    pub fn resize(&mut self, width: u32, height: u32) {
        if height != 0 {
            self.aspect = width as f32 / height as f32;
        } else {
            self.aspect = 1.0;
        }
    }

    /// Calculates the perspective projection matrix for the current configuration.
    ///
    /// This function generates a 4x4 projection matrix based on the configured
    /// field of view (`fovy`), aspect ratio (`aspect`), and near/far clipping planes.
    /// The resulting matrix is transformed to match WGPU's coordinate system.
    ///
    /// # Returns
    /// A `Matrix4<f32>` representing the transformed perspective projection matrix.
    pub fn calc_matrix(&self) -> Matrix4<f32> {
        OPENGL_TO_WGPU_MATRIX * perspective(self.fovy, self.aspect, self.znear, self.zfar)
    }
}
