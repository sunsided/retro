use crate::engine::phases::render::background::RenderBackgroundSystemData;
use crate::engine::phases::update::color_cycle::BackgroundColorCycleSystemData;
use crate::engine::{
    BackgroundColorCycleSystem, CreateSystem, CreateSystems, PhaseEvents, PhysicsSystem,
    RenderBackgroundSystem, RenderSystem, SystemFactory, SystemFixedUpdatePhaseEvents,
};
use sillyecs::FrameContext;
use std::convert::Infallible;
use tracing::trace;

pub mod fog;
pub mod physics;
pub mod stars;
pub mod sun;
pub mod time_of_day;

impl CreateSystem<PhysicsSystem> for SystemFactory {
    fn create(&self) -> PhysicsSystem {
        PhysicsSystem::default()
    }
}

impl SystemFixedUpdatePhaseEvents for PhaseEvents {
    type Result = ();
    type Error = Infallible;

    fn on_begin_phase(&mut self, _context: &FrameContext) -> Result<Self::Result, Self::Error> {
        self.start_timing();
        Ok(())
    }

    fn on_end_phase(
        &mut self,
        _data: Option<Self::Result>,
        _context: &FrameContext,
    ) -> Result<(), Self::Error> {
        let elapsed = self.elapsed();
        trace!(
            "{name} phase took {elapsed:?}",
            name = <PhaseEvents as SystemFixedUpdatePhaseEvents>::NAME
        );
        Ok(())
    }
}
