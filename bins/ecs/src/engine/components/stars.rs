#[derive(Debug, Clone)]
pub struct StarsData {
    pub star_visibility: f32, // 0.0 (day) to 1.0 (night)
}

impl Default for StarsData {
    fn default() -> Self {
        Self {
            star_visibility: 0.0,
        }
    }
}
