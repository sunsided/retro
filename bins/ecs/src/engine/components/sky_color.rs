#[derive(Debug, Clone)]
pub struct SkydomeData {
    pub sky_color: [f32; 3], // RGB sky dome base color (interpolated day/night)
}

impl Default for SkydomeData {
    fn default() -> Self {
        Self {
            sky_color: [0.4, 0.75, 1.0], // bright daytime sky
        }
    }
}
