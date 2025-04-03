use crate::component::{PositionComponent, PositionData, VelocityComponent, VelocityData};
use crate::logging::setup_logging;
use crate::phase_events::PhaseEvents;
use crate::system::SystemFactory;
use crate::world::{UserState, World};

mod archetype;
mod component;
mod logging;
mod phase_events;
mod system;
mod world;

fn main() {
    dotenvy::dotenv().ok();
    let _logging = setup_logging();
    let events = PhaseEvents::default();
    let state = UserState::default();
    let mut world = World::new(&SystemFactory, state, Some(events));

    world.particle.spawn_with(
        PositionComponent::new(PositionData::default()),
        VelocityComponent::new(VelocityData {
            x: 1.0,
            y: 0.1,
            z: -0.1,
        }),
    );

    world.apply_system_phases();
    std::thread::sleep(std::time::Duration::from_secs_f32(0.5));
    world.apply_system_phases();
}
