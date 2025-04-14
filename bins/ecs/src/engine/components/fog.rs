use cgmath::Vector3;

#[derive(Debug, Clone)]
pub struct FogData {
    pub fog_distance: f32,
    pub fog_fade: f32,
}

impl Default for FogData {
    fn default() -> Self {
        Self {
            fog_distance: 10.0,
            fog_fade: 0.2,
        }
    }
}
