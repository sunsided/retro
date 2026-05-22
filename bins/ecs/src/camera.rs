#[allow(clippy::module_inception)]
mod camera;
mod camera_controller;
mod projection;

pub use crate::camera::camera::Camera;
pub use crate::camera::camera_controller::CameraController;
pub use crate::camera::projection::Projection;

/// The coordinate system in Wgpu is based on DirectX and Metal's coordinate systems.
/// That means that in normalized device coordinates, the x-axis and y-axis are in the
/// range of -1.0 to +1.0, and the z-axis is 0.0 to +1.0.
/// The cgmath crate (as well as most game math crates) is built for OpenGL's
/// coordinate system. This matrix will scale and translate our scene from OpenGL's
/// coordinate system to WGPU's.
#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);
