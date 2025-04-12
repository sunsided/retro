use cgmath::Vector3;

#[derive(Debug, Clone)]
pub struct SunData {
    pub direction: Vector3<f32>, // normalized world-space direction to the sun
    pub intensity: f32,          // 0.0 (sun below horizon) to 1.0 (fully visible)
    pub color: Vector3<f32>,     // RGB tint of sun light (reddish at horizon)
}

impl Default for SunData {
    fn default() -> Self {
        Self {
            direction: Vector3::new(0.0, 1.0, 0.0), // overhead sun
            intensity: 1.0,
            color: Vector3::new(1.2, 1.05, 0.85), // warm midday sun
        }
    }
}
