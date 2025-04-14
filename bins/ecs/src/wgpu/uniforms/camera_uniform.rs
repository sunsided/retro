//! This module defines the `CameraUniform` struct, which is used for managing
//! and passing camera-related data to GPU shaders in a graphics rendering pipeline.
//!
//! The `CameraUniform` struct contains two main components:
//! - `view_position`: The position of the camera in world space, which includes a padding
//!   component to ensure proper memory alignment for GPU usage.
//! - `view_proj`: A combined view and projection matrix, used to transform
//!   world space coordinates into screen space coordinates for rendering.
//!
//! The `CameraUniform` provides helper methods to initialize itself (`new`) and
//! update its values (`update_view_proj`) based on the current camera position
//! and projection parameters. The module relies on external crates like `cgmath` for matrix
//! operations and `bytemuck` for efficient memory representation.

use crate::camera::{Camera, Projection};
use crate::wgpu::uniforms::mat3x3_std140::Mat3x3Std140;
use cgmath::{Matrix3, SquareMatrix, Transform};

/// A uniform buffer structure for passing camera data to shaders.
/// This contains the camera's position in world space as well as
/// the combined view and projection matrix.
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    /// The position of the camera in world space, with an additional
    /// padding component to align the size to 16 bytes.
    view_position: [f32; 4],
    /// The combined view and projection matrix, used for transforming
    /// world space coordinates into screen space.
    view_proj: [[f32; 4]; 4],
    /// Inverse projection matrix.
    inverse_proj: [[f32; 4]; 4],
    /// The camera's pure rotation matrix (3x3, fit into a 4x3 for alignment)
    camera_rot: Mat3x3Std140,
}

impl CameraUniform {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_position: [0.0; 4],
            view_proj: cgmath::Matrix4::identity().into(),
            inverse_proj: cgmath::Matrix4::identity().into(),
            camera_rot: Matrix3::identity().into(),
        }
    }

    /// Updates the `CameraUniform`'s data with the current camera position and view-projection matrix.
    ///
    /// This method takes a `Camera` and a `Projection` as inputs to calculate and update:
    /// - `view_position`: The homogeneous coordinates of the camera's position in world space,
    ///   padded to align properly for GPU usage.
    /// - `view_proj`: The combined view and projection matrix, used to transform world space
    ///   coordinates into screen space.
    ///
    /// # Parameters
    /// - `camera`: A reference to the `Camera` object, which provides the position and view matrix.
    /// - `projection`: A reference to the `Projection` object, which provides the projection matrix.
    pub fn update_view_proj(&mut self, camera: &Camera, projection: &Projection) {
        self.view_position = camera.position.to_homogeneous().into();

        let proj = projection.calc_matrix();
        let view = camera.calc_rh_look_to();
        self.view_proj = (proj * view).into();
        self.inverse_proj = proj.invert().unwrap().into();

        // camera-to-world rotation matrix (upper 3x3 of view.inverse())
        let rot = view.inverse_transform().unwrap();
        let rot3x3 = Matrix3::from_cols(
            rot.x.truncate(), // right
            rot.y.truncate(), // up
            rot.z.truncate(), // forward
        );

        self.camera_rot = rot3x3.into();
    }
}

impl Default for CameraUniform {
    fn default() -> Self {
        Self::new()
    }
}
