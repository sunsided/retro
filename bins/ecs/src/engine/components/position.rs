use cgmath::Vector3;

#[derive(Debug, Clone, Default)]
pub struct PositionData {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl From<Vector3<f32>> for PositionData {
    #[inline]
    fn from(value: Vector3<f32>) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

impl From<PositionData> for Vector3<f32> {
    #[inline]
    fn from(value: PositionData) -> Self {
        Vector3::new(value.x, value.y, value.z)
    }
}
