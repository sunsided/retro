use crate::logging::setup_logging;
use crate::phase_events::PhaseEvents;
use crate::system::SystemFactory;
use crate::world::World;

mod archetype;
mod component;
mod system;
mod world;
mod logging;
mod phase_events;

fn main() {
    dotenvy::dotenv().ok();
    let _logging = setup_logging();
    let events = PhaseEvents::default();
    let mut world = World::new(&SystemFactory, Some(events));
    world.apply_system_phases();
}
