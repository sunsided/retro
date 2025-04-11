use crate::camera::{Camera, Projection};
use crate::wgpu::camera_uniform::CameraUniform;
use winit::window::Fullscreen;

#[derive(Debug, Clone)]
pub struct CameraData {
    pub active: bool,
    pub fixed_size: bool,
    pub camera: Camera,
    pub projection: Projection,
}

#[derive(Debug)]
pub struct ActiveCameraState {
    pub uniform: CameraUniform,
}

impl Default for ActiveCameraState {
    fn default() -> Self {
        Self {
            uniform: CameraUniform::new(),
        }
    }
}

impl Default for CameraData {
    fn default() -> Self {
        let camera = Camera::new((0.0, 5.0, 10.0), cgmath::Deg(-90.0), cgmath::Deg(-20.0));
        let projection = Projection::new(1024, 768, cgmath::Deg(45.0), 0.1, 100.0);
        Self {
            active: false,
            fixed_size: false,
            camera,
            projection,
        }
    }
}
