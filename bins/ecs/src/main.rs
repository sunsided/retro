use std::time::Instant;
use tracing::trace;
use crate::logging::setup_logging;
use crate::system::{SystemPhaseEvents, SystemPhase, SystemFactory};
use crate::world::World;

mod archetype;
mod component;
mod system;
mod world;
mod logging;

fn main() {
    dotenvy::dotenv().ok();
    let _logging = setup_logging();
    let events = PhaseEvents::default();
    let mut world = World::new(&SystemFactory, Some(events));
    world.apply_system_phases();
}


#[derive(Debug)]
struct PhaseEvents(Instant);

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
