use std::convert::Infallible;
use crate::game::system::{SystemFixedUpdatePhaseEvents, SystemStartupPhaseEvents, SystemUpdatePhaseEvents};
use crate::game::world::FrameContext;
use std::time::Instant;
use tracing::trace;

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

    fn on_begin_phase(&mut self, _context: &FrameContext) -> Result<Self::Result, Self::Error> {
        self.0 = Instant::now();
        Ok(())
    }

    fn on_end_phase(&mut self, _data: Option<Self::Result>, _context: &FrameContext) -> Result<(), Self::Error> {
        let elapsed = self.0.elapsed();
        trace!("{name} phase took {elapsed:?}", name = <PhaseEvents as SystemStartupPhaseEvents>::NAME);
        Ok(())
    }
}

impl SystemFixedUpdatePhaseEvents for PhaseEvents {
    type Result = ();
    type Error = Infallible;

    fn on_begin_phase(&mut self, _context: &FrameContext) -> Result<Self::Result, Self::Error> {
        self.0 = Instant::now();
        Ok(())
    }

    fn on_end_phase(&mut self, _data: Option<Self::Result>, _context: &FrameContext) -> Result<(), Self::Error> {
        let elapsed = self.0.elapsed();
        trace!("{name} phase took {elapsed:?}", name = <PhaseEvents as SystemFixedUpdatePhaseEvents>::NAME);
        Ok(())
    }
}

impl SystemUpdatePhaseEvents for PhaseEvents {
    type Result = ();
    type Error = Infallible;

    fn on_begin_phase(&mut self, _context: &FrameContext) -> Result<Self::Result, Self::Error> {
        self.0 = Instant::now();
        Ok(())
    }

    fn on_end_phase(&mut self, _data: Option<Self::Result>, _context: &FrameContext) -> Result<(), Self::Error> {
        let elapsed = self.0.elapsed();
        trace!("{name} phase took {elapsed:?}", name = <PhaseEvents as SystemUpdatePhaseEvents>::NAME);
        Ok(())
    }
}
