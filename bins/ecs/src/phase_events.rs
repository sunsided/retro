use crate::game::system::{SystemPhase, SystemPhaseEvents};
use crate::game::world::{FrameContext, MainWorldStates};
use std::time::Instant;
use tracing::trace;

#[derive(Debug)]
pub struct PhaseEvents(Instant);

impl Default for PhaseEvents {
    fn default() -> Self {
        Self(Instant::now())
    }
}

impl SystemPhaseEvents<MainWorldStates> for PhaseEvents {
    fn on_begin_phase(
        &mut self,
        phase: SystemPhase,
        context: &FrameContext,
        _state: &mut MainWorldStates,
    ) {
        let duration = Instant::now() - self.0;
        self.0 = Instant::now();
        trace!(phase = ?phase, duration_since_last = ?duration, frame = context.frame_number, "Beginning phase");
    }

    fn on_end_phase(
        &mut self,
        phase: SystemPhase,
        context: &FrameContext,
        _state: &mut MainWorldStates,
    ) {
        let duration = Instant::now() - self.0;
        trace!(phase = ?phase, duration = ?duration, frame = context.frame_number, "End of phase");
    }
}
