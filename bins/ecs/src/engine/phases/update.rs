use crate::engine::phases::update::color_cycle::BackgroundColorCycleSystemData;
use crate::engine::{
    BackgroundColorCycleSystem, CreateSystem, PhaseEvents, RenderSystem, SystemFactory,
    SystemFixedUpdatePhaseEvents, SystemUpdatePhaseEvents,
};
use sillyecs::FrameContext;
use std::convert::Infallible;
use std::time::Instant;
use tracing::trace;

pub mod camera;
pub mod color_cycle;

impl CreateSystem<BackgroundColorCycleSystem> for SystemFactory {
    fn create(&self) -> BackgroundColorCycleSystem {
        BackgroundColorCycleSystem(BackgroundColorCycleSystemData::default())
    }
}

impl SystemUpdatePhaseEvents for PhaseEvents {
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
            name = <PhaseEvents as SystemUpdatePhaseEvents>::NAME
        );
        Ok(())
    }
}
