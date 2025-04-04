use crate::archetype::ParticleEntityData;
use crate::component::VelocityData;
use crate::logging::setup_logging;
use crate::phase_events::PhaseEvents;
use crate::system::SystemFactory;
use crate::world::{MainWorld, UserState};

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
    let mut world = MainWorld::new(&SystemFactory, state, Some(events));

    let entity = ParticleEntityData {
        position: Default::default(),
        velocity: VelocityData {
            x: 1.0,
            y: 0.1,
            z: -0.1,
        },
    };
    let _id = entity.spawn_into(&mut world);

    world.apply_system_phases();
    std::thread::sleep(std::time::Duration::from_secs_f32(0.5));
    world.apply_system_phases();
}
