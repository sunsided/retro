#[derive(Debug, Clone)]
pub struct AmbientLightData {
    pub ambient_light: [f32; 3], // RGB ambient lighting (for scene)
}

impl Default for AmbientLightData {
    fn default() -> Self {
        Self {
            ambient_light: [0.2, 0.25, 0.3], // soft daylight ambient
        }
    }
}
