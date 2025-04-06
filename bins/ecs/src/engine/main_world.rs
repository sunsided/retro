use std::sync::Arc;
use parking_lot::Mutex;
use crate::engine::{MainWorld, SceneBackgroundEntityData};
use crate::engine::phases::render::background::BackgroundColorData;
use crate::wgpu::WgpuState;

pub trait InitializeMainWorld {
    /// Initialize the main entities needed in this world, such as populating the
    /// [`SceneBackgroundArchetype`](crate::engine::SceneBackgroundArchetype), and more.
    fn initialize_entities(&mut self);

    /// Sets the [`WgpuState`].
    fn set_wgpu_state(&mut self, state: Option<Arc<Mutex<WgpuState>>>);
}

impl<E, Q> InitializeMainWorld for MainWorld<E, Q> {
    fn initialize_entities(&mut self) {
        self.spawn_scene_background(SceneBackgroundEntityData {
            background_color: BackgroundColorData::default().into()
        });
    }

    fn set_wgpu_state(&mut self, state: Option<Arc<Mutex<WgpuState>>>) {
        self.states.wgpu_render.set_state(state);
    }
}
