use crate::logic::scene_background::{PlasmaBackground, SceneBackground};

#[derive(Debug, Clone)]
pub struct GameState {
    pub tick: u64,
    pub x: usize,
    pub y: usize,
    pub background: SceneBackground,
    pub crt: bool,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            tick: 0,
            x: 0,
            y: 0,
            background: SceneBackground::Plasma(PlasmaBackground::default()),
            crt: true,
        }
    }
}
