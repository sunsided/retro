use crate::engine::phases::smoothstep;
use crate::engine::{
    ApplyStarsSystem, CreateSystem, FrameContext, PhysicsSystem, StarsComponent, StarsSystem,
    SystemFactory, TimeOfDayComponent,
};
use std::convert::Infallible;
use std::f32::consts::PI;
use std::fmt::Debug;

#[derive(Debug, Default)]
pub struct StarsSystemData;

impl Default for StarsSystem {
    fn default() -> Self {
        Self(StarsSystemData)
    }
}

impl CreateSystem<StarsSystem> for SystemFactory {
    fn create(&self) -> StarsSystem {
        StarsSystem::default()
    }
}

impl ApplyStarsSystem for StarsSystem {
    type Error = Infallible;

    fn apply_single(&mut self, time: &TimeOfDayComponent, stars: &mut StarsComponent) {
        let cycle = 0.5 * (1.0 + ((time.time_of_day - 12.0) * (2.0 * PI / 24.0)).cos());
        let night_fade = smoothstep(0.5, 1.0, 1.0 - cycle);
        stars.intensity = night_fade;
    }
}
