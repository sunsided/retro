use cgmath::Vector3;

#[derive(Debug, Clone)]
pub struct AmbientLightData {
    pub ambient_light: Vector3<f32>, // RGB ambient lighting (for scene)
}

impl Default for AmbientLightData {
    fn default() -> Self {
        Self {
            ambient_light: Vector3::new(0.2, 0.25, 0.3), // soft daylight ambient
        }
    }
}
