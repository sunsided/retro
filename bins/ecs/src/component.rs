include!("../generated/components.gen.rs");

#[derive(Debug, Clone)]
pub struct PositionData {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone)]
pub struct VelocityData {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone)]
pub struct HealthData;

#[derive(Debug, Clone)]
pub struct ColliderData;
