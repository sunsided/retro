include!(concat!(env!("OUT_DIR"), "/components.gen.rs"));

pub struct ComponentVec<T: Component> {
    data: Vec<T>,
}

#[derive(Debug, Clone)]
pub struct PositionData;

#[derive(Debug, Clone)]
pub struct VelocityData;

#[derive(Debug, Clone)]
pub struct HealthData;
