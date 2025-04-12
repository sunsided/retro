#[derive(Debug, Clone)]
pub struct StarsData {
    pub intensity: f32, // 0.0 (day) to 1.0 (night)
    pub rotational_speed: f32,
}

impl Default for StarsData {
    fn default() -> Self {
        Self {
            intensity: 0.0,
            rotational_speed: 0.0127379,
        }
    }
}
