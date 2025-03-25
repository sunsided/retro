mod archetypes;
mod component_type_set;
mod component_vec;
mod world;

use crossbeam_channel::{select, tick};
use multibuffer::TripleBuffer;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::{Duration, Instant};

pub type Entity = u32;

#[derive(Clone, Debug)]
pub struct Position(f32, f32);

#[derive(Clone, Debug)]
pub struct Velocity(f32, f32);

#[derive(Debug, Clone)]
pub struct ECSWorld {
    positions: Vec<Option<Position>>,
    velocities: Vec<Option<Velocity>>,
    entities: Vec<Entity>,
}

impl ECSWorld {
    fn new() -> Self {
        ECSWorld {
            positions: vec![],
            velocities: vec![],
            entities: vec![],
        }
    }

    fn spawn(&mut self, pos: Position, vel: Velocity) -> Entity {
        let id = self.entities.len() as u32;
        self.entities.push(id);
        self.positions.push(Some(pos));
        self.velocities.push(Some(vel));
        id
    }

    fn clear(&mut self) {
        self.positions.clear();
        self.velocities.clear();
        self.entities.clear();
    }
}

fn logic_system(world: &mut ECSWorld) {
    world.clear();
    for i in 0..100 {
        world.spawn(Position(i as f32, 0.0), Velocity(0.0, 10.0));
    }
}

fn physics_system(world: &mut ECSWorld, dt: f32) {
    for (pos_opt, vel_opt) in world.positions.iter_mut().zip(world.velocities.iter()) {
        if let (Some(pos), Some(vel)) = (pos_opt, vel_opt) {
            pos.0 += vel.0 * dt;
            pos.1 += vel.1 * dt;
        }
    }
}

fn render_system(world: &ECSWorld) {
    if let Some(pos) = world.positions.get(0).and_then(|p| p.as_ref()) {
        println!("Render Entity 0 at position: ({:.2}, {:.2})", pos.0, pos.1);
    }
}

fn logic_thread(buffer: Arc<TripleBuffer<ECSWorld>>, scene_id: Arc<AtomicU64>) {
    let mut frame = 0;
    let tick = tick(Duration::from_millis(33));
    loop {
        select! {
            recv(tick) -> _ => {
                let scene = scene_id.load(Ordering::Acquire);
                let world = buffer.get_mut(frame);
                logic_system(world);
                buffer.publish(frame, scene);
                frame = (frame + 1) % buffer.len();
            }
        }
    }
}

fn physics_thread(
    logic_buffer: Arc<TripleBuffer<ECSWorld>>,
    render_buffer: Arc<TripleBuffer<ECSWorld>>,
) {
    let tick = tick(Duration::from_millis(16));
    let mut frame = 0;
    let mut last_time = Instant::now();
    loop {
        select! {
            recv(tick) -> _ => {
                let dt = last_time.elapsed().as_secs_f32();
                last_time = Instant::now();

                let (logic_world, version) = logic_buffer.get_latest();
                let target_world = render_buffer.get_mut(frame);
                *target_world = logic_world.clone();
                physics_system(target_world, dt);
                render_buffer.publish(frame, version);
                frame = (frame + 1) % render_buffer.len();
            }
        }
    }
}

fn render_thread(
    logic_buffer: Arc<TripleBuffer<ECSWorld>>,
    render_buffer: Arc<TripleBuffer<ECSWorld>>,
) {
    let tick = tick(Duration::from_millis(16));
    loop {
        select! {
            recv(tick) -> _ => {
                let (_, lver) = logic_buffer.get_latest();
                let (world, rver) = render_buffer.get_latest();
                if lver == rver {
                    render_system(world);
                } else {
                    println!("Render skipped: version mismatch (logic={}, render={})", lver, rver);
                }
            }
        }
    }
}

fn main() {
    let logic_buffer = Arc::new(TripleBuffer::new(ECSWorld::new));
    let render_buffer = Arc::new(TripleBuffer::new(ECSWorld::new));
    let scene_id = Arc::new(AtomicU64::new(1));

    {
        let buf = Arc::clone(&logic_buffer);
        let sid = Arc::clone(&scene_id);
        thread::spawn(move || logic_thread(buf, sid));
    }

    {
        let logic_buf = Arc::clone(&logic_buffer);
        let render_buf = Arc::clone(&render_buffer);
        thread::spawn(move || physics_thread(logic_buf, render_buf));
    }

    {
        let logic_buf = Arc::clone(&logic_buffer);
        let render_buf = Arc::clone(&render_buffer);
        thread::spawn(move || render_thread(logic_buf, render_buf));
    }

    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
