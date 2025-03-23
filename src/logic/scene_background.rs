#[derive(Debug, Clone)]
pub enum SceneBackground {
    None,
    Plasma(PlasmaBackground),
    Fire,
}

#[derive(Debug, Clone)]
pub struct PlasmaBackground {
    pub intensity: f32,
}

impl Default for PlasmaBackground {
    fn default() -> Self {
        Self { intensity: 0.2 }
    }
}
