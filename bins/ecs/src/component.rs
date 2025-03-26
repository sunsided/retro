include!(concat!(env!("OUT_DIR"), "/components.gen.rs"));

pub struct ComponentVec<T: Component> {
    data: Vec<T>,
}

#[derive(Debug)]
pub struct PositionData;

#[derive(Debug)]
pub struct VelocityData;

#[derive(Debug)]
pub struct HealthData;
