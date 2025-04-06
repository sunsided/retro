use crate::engine::{BackgroundColorCycleSystem, CreateSystem, CreateSystems, PhysicsSystem, RenderBackgroundSystem, RenderSystem, SystemFactory};
use crate::engine::phases::render::background::RenderBackgroundSystemData;
use crate::engine::phases::update::color_cycle::BackgroundColorCycleSystemData;

pub mod physics;

impl CreateSystem<PhysicsSystem> for SystemFactory {
    fn create(&self) -> PhysicsSystem {
        PhysicsSystem::default()
    }
}
