use crate::engine::{PhaseEvents, SystemStartupPhaseEvents};
use sillyecs::FrameContext;
use std::convert::Infallible;
use tracing::trace;

impl SystemStartupPhaseEvents for PhaseEvents {
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
            name = <PhaseEvents as SystemStartupPhaseEvents>::NAME
        );
        Ok(())
    }
}
