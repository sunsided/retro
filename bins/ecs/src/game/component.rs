include!("../../generated/components_gen.rs");

#[derive(Debug, Clone, Default)]
pub struct PositionData {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Default)]
pub struct VelocityData {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone)]
pub struct HealthData;

#[derive(Debug, Clone)]
pub struct ColliderData;
