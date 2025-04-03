use std::time::Instant;
use tracing::trace;
use crate::system::{SystemPhase, SystemPhaseEvents};

#[derive(Debug)]
pub struct PhaseEvents(Instant);

impl Default for PhaseEvents {
    fn default() -> Self {
        Self(Instant::now())
    }
}

impl SystemPhaseEvents for PhaseEvents {
    fn on_begin_phase(&mut self, phase: SystemPhase)  {
        let duration = Instant::now() - self.0;
        self.0 = Instant::now();
        trace!(phase = ?phase, duration_since_last = ?duration, "Beginning phase");
    }

    fn on_end_phase(&mut self, phase: SystemPhase)  {
        let duration = Instant::now() - self.0;
        trace!(phase = ?phase, duration = ?duration, "End of phase");
    }
}
