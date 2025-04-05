use crossbeam_channel::TryRecvError;
use crate::game::archetype::ParticleEntityData;
use crate::game::component::{PositionData, VelocityData};
use crate::logging::setup_logging;
use crate::phase_events::PhaseEvents;
use crate::game::system::SystemFactory;
use crate::game::world::{MainWorld, RenderState, WorldCommand, WorldCommandReceiver, WorldCommandSender};

mod logging;
mod phase_events;
mod game;

fn main() {
    dotenvy::dotenv().ok();
    let _logging = setup_logging();
    let events = PhaseEvents::default();
    let state = RenderState::default();
    let mut world = MainWorld::new(&SystemFactory, state, Some(events), CommandQueue::new());

    let entity = ParticleEntityData {
        position: PositionData::default().into(),
        velocity: VelocityData {
            x: 1.0,
            y: 0.1,
            z: -0.1,
        }.into(),
    };
    let id = entity.spawn_into(&mut world);
    debug_assert_eq!(world.len(), 1);

    world.apply_system_phases();
    std::thread::sleep(std::time::Duration::from_secs_f32(0.5));
    world.apply_system_phases();

    world.despawn_by_id(id).expect("Failed to despawn entity");
    debug_assert_eq!(world.len(), 0);
    world.apply_system_phases();
}

struct CommandQueue {
    sender: crossbeam_channel::Sender<WorldCommand>,
    receiver: crossbeam_channel::Receiver<WorldCommand>,
}

impl CommandQueue {
    pub fn new() -> Self {
        let (sender, receiver) = crossbeam_channel::unbounded();
        Self {
            sender,
            receiver,
        }
    }
}

impl WorldCommandReceiver for CommandQueue {
    type Error = TryRecvError;

    fn recv(&self) -> Result<Option<WorldCommand>, Self::Error> {
        match self.receiver.try_recv() {
            Ok(cmd) => Ok(Some(cmd)),
            Err(TryRecvError::Empty) => Ok(None),
            Err(err) => Err(err),
        }
    }
}

impl WorldCommandSender for CommandQueue {
    type Error = crossbeam_channel::SendError<WorldCommand>;

    fn send(&self, command: WorldCommand) -> Result<(), Self::Error> {
        self.sender.send(command)
    }
}
