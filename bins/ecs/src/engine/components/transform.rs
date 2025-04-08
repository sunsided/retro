use cgmath::Quaternion;

#[derive(Debug, Clone)]
pub struct TransformData {
    pub rotation: Quaternion<f32>,
    pub scale: [f32; 3],
}

impl TransformData {
    #[inline]
    #[allow(dead_code)]
    pub fn scale_uniform(&mut self, value: f32) {
        self.scale = [value, value, value];
    }
}

impl Default for TransformData {
    fn default() -> Self {
        Self {
            rotation: Quaternion::new(1.0, 0.0, 0.0, 0.0),
            scale: [1.0, 1.0, 1.0],
        }
    }
}

impl From<Quaternion<f32>> for TransformData {
    fn from(value: Quaternion<f32>) -> Self {
        Self {
            rotation: value,
            scale: [1.0, 1.0, 1.0],
        }
    }
}
