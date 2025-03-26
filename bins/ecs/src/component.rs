/// Marker trait for components.
pub trait Component: 'static + Send + Sync {}
impl<T: 'static + Send + Sync> Component for T {}

pub struct ComponentVec<T: Component> {
    data: Vec<T>,
}

pub struct Position(f32, f32);

pub struct Velocity(f32, f32);

pub struct Health(u32);
