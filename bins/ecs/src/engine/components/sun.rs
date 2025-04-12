#[derive(Debug, Clone)]
pub struct SunData {
    pub sun_direction: [f32; 3], // normalized world-space direction to the sun
    pub sun_visibility: f32,     // 0.0 (sun below horizon) to 1.0 (fully visible)
    pub sun_color: [f32; 3],     // RGB tint of sun light (reddish at horizon)
}

impl Default for SunData {
    fn default() -> Self {
        Self {
            sun_direction: [0.0, 1.0, 0.0], // overhead sun
            sun_visibility: 1.0,
            sun_color: [1.2, 1.05, 0.85],    // warm midday sun
        }
    }
}
