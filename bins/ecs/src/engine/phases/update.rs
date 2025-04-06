use crate::engine::{BackgroundColorCycleSystem, CreateSystem, RenderSystem, SystemFactory};
use crate::engine::phases::update::color_cycle::BackgroundColorCycleSystemData;

pub mod color_cycle;

impl CreateSystem<BackgroundColorCycleSystem> for SystemFactory {
    fn create(&self) -> BackgroundColorCycleSystem {
        BackgroundColorCycleSystem(BackgroundColorCycleSystemData::default())
    }
}
