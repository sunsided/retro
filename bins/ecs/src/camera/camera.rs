//! This module defines a 3D camera struct and its associated functionality.
//!
//! The primary purpose of the `Camera` struct is to calculate a view matrix
//! used in rendering 3D scenes. This view matrix is derived from the camera's
//! position and orientation in 3D space. The camera orientation is represented
//! by yaw (horizontal rotation) and pitch (vertical rotation) angles in radians.
//!
//! ## Overview
//! - `Camera`: Represents a camera in 3D space.
//! - `Camera::new`: Constructs a new `Camera` instance.
//! - `Camera::calc_matrix`: Calculates the 4x4 view matrix based on the
//!   camera's position and orientation.

use cgmath::{InnerSpace, Matrix4, Point3, Rad, Vector3};

/// Represents a 3D camera with a position, yaw, and pitch angles.
///
/// The `Camera` struct is used to calculate the view matrix for rendering
/// based on its position and orientation in 3D space.
///
/// - `position`: The position of the camera in world space.
/// - `yaw`: The horizontal rotation of the camera in radians.
/// - `pitch`: The vertical rotation of the camera in radians.
#[derive(Debug, Clone)]
pub struct Camera {
    /// The position of the camera in 3D space.
    pub position: Point3<f32>,
    /// The yaw (horizontal rotation) of the camera in radians.
    pub(crate) yaw: Rad<f32>,
    /// The pitch (vertical rotation) of the camera in radians.
    pub(crate) pitch: Rad<f32>,
}

impl Camera {
    pub fn new<
        V: Into<Point3<f32>>,
        Y: Into<Rad<f32>>,
        P: Into<Rad<f32>>,
    >(
        position: V,
        yaw: Y,
        pitch: P,
    ) -> Self {
        Self {
            position: position.into(),
            yaw: yaw.into(),
            pitch: pitch.into(),
        }
    }

    /// Calculates the view matrix for the camera based on its position and orientation.
    ///
    /// This function computes a right-handed look-to matrix using the camera's position,
    /// direction vector (calculated from its yaw and pitch angles), and an up vector.
    ///
    /// # Returns
    /// A `Matrix4<f32>` representing the camera's view transformation.
    pub fn calc_rh_look_to(&self) -> Matrix4<f32> {
        let (sin_pitch, cos_pitch) = self.pitch.0.sin_cos();
        let (sin_yaw, cos_yaw) = self.yaw.0.sin_cos();

        Matrix4::look_to_rh(
            self.position,
            Vector3::new(
                cos_pitch * cos_yaw,
                sin_pitch,
                cos_pitch * sin_yaw,
            ).normalize(),
            Vector3::unit_y(),
        )
    }
}
