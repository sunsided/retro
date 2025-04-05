use std::convert::Infallible;
use crate::game::system::{SystemFixedUpdatePhaseEvents, SystemRenderPhaseEvents, SystemStartupPhaseEvents, SystemUpdatePhaseEvents};
use crate::game::world::WgpuRenderState;
use crate::game::world::FrameContext;
use std::time::Instant;
use tracing::error;

#[derive(Debug)]
pub struct PhaseEvents(Instant);

impl Default for PhaseEvents {
    fn default() -> Self {
        Self(Instant::now())
    }
}

impl SystemStartupPhaseEvents for PhaseEvents {
    type Result = ();
    type Error = Infallible;

    fn on_begin_phase(&mut self, context: &FrameContext) -> Result<Self::Result, Self::Error> {
        Ok(())
    }

    fn on_end_phase(&mut self, data: Option<Self::Result>, context: &FrameContext) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl SystemFixedUpdatePhaseEvents for PhaseEvents {
    type Result = ();
    type Error = Infallible;

    fn on_begin_phase(&mut self, context: &FrameContext) -> Result<Self::Result, Self::Error> {
        Ok(())
    }

    fn on_end_phase(&mut self, data: Option<Self::Result>, context: &FrameContext) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl SystemUpdatePhaseEvents for PhaseEvents {
    type Result = ();
    type Error = Infallible;

    fn on_begin_phase(&mut self, context: &FrameContext) -> Result<Self::Result, Self::Error> {
        Ok(())
    }

    fn on_end_phase(&mut self, data: Option<Self::Result>, context: &FrameContext) -> Result<(), Self::Error> {
        Ok(())
    }
}
