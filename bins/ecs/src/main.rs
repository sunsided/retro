use std::time::Instant;
use tracing_appender::non_blocking;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{fmt, EnvFilter};
use tracing::trace;
use crate::system::{CreateSystems, PhysicsSystem, RenderSystem, SystemPhaseEvents, SystemPhase};
use crate::world::World;

mod archetype;
mod component;
mod system;
mod world;

fn main() {
    dotenvy::dotenv().ok();
    let _logging = setup_logging();
    let system_factory = SystemFactory;
    let events = PhaseEvents::default();
    let mut world = World::new(&system_factory, Some(events));
    world.apply_system_phases();
}

fn setup_logging() -> WorkerGuard {
    // Write logs to stdout with a non-blocking writer.
    let (non_blocking_writer, guard) = non_blocking(std::io::stdout());

    // Use an environment filter for dynamic log levels.
    // e.g., set RUST_LOG=debug before running.
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    fmt()
        .with_writer(non_blocking_writer)
        .with_env_filter(env_filter)
        .init();
    guard
}

struct SystemFactory;

#[derive(Debug)]
struct PhaseEvents(Instant);

impl Default for PhaseEvents {
    fn default() -> Self {
        Self(Instant::now())
    }
}

impl CreateSystems for SystemFactory {
    fn create_physics(&self) -> PhysicsSystem {
        PhysicsSystem::default()
    }

    fn create_render(&self) -> RenderSystem {
        RenderSystem::default()
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
