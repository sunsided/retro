pub mod rendering;
mod vertex;
pub mod physics;
pub mod color_cycle;

use crate::engine::{BackgroundColorCycleSystem, CreateSystems, PhysicsSystem, RenderBackgroundSystem, RenderSystem};
use crate::engine::system::rendering::{background::RenderBackgroundSystemData};
use crate::engine::system::color_cycle::BackgroundColorCycleSystemData;

pub struct SystemFactory;

impl CreateSystems for SystemFactory {
    fn create_physics(&self) -> PhysicsSystem {
        PhysicsSystem::default()
    }

    fn create_background_color_cycle(&self) -> BackgroundColorCycleSystem {
        BackgroundColorCycleSystem(BackgroundColorCycleSystemData::default())
    }

    fn create_render_background(&self) -> RenderBackgroundSystem {
        RenderBackgroundSystem(RenderBackgroundSystemData)
    }

    fn create_render(&self) -> RenderSystem {
        RenderSystem::default()
    }
}
