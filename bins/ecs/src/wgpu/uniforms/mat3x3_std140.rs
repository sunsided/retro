#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Mat3x3Std140 {
    pub x: [f32; 4],
    pub y: [f32; 4],
    pub z: [f32; 4],
}

impl From<cgmath::Matrix3<f32>> for Mat3x3Std140 {
    fn from(m: cgmath::Matrix3<f32>) -> Self {
        Self {
            x: [m.x[0], m.x[1], m.x[2], 0.0],
            y: [m.y[0], m.y[1], m.y[2], 0.0],
            z: [m.z[0], m.z[1], m.z[2], 0.0],
        }
    }
}
