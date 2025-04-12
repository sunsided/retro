use cgmath::Vector3;

#[derive(Debug, Clone)]
pub struct SkydomeData {
    pub sky_color: Vector3<f32>, // RGB sky dome base color (interpolated day/night)
}

impl Default for SkydomeData {
    fn default() -> Self {
        Self {
            sky_color: Vector3::new(0.4, 0.75, 1.0), // bright daytime sky
        }
    }
}
