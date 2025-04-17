use crate::engine::phases::{mix, mixv};
use crate::engine::{
    AmbientLightComponent, ApplyFogSystem, ApplySunSystem, CreateSystem, FogComponent, FogSystem,
    PhysicsSystem, SkydomeComponent, SunComponent, SunSystem, SystemFactory, TimeOfDayComponent,
};
use cgmath::{InnerSpace, Vector3};
use std::convert::Infallible;
use std::f32::consts::PI;
use std::fmt::Debug;

#[derive(Debug, Default)]
pub struct FogSystemData;

impl Default for FogSystem {
    fn default() -> Self {
        Self(FogSystemData)
    }
}

impl CreateSystem<FogSystem> for SystemFactory {
    fn create(&self) -> FogSystem {
        FogSystem::default()
    }
}

impl ApplyFogSystem for FogSystem {
    type Error = Infallible;

    fn apply_single(&mut self, time: &TimeOfDayComponent, fog: &mut FogComponent) {
        let time = time.time_of_day;

        // Modulate fog
        let fog_peak = 5.5; // at 5:30 it's the foggiest
        let fog_span = 1.5;
        let fog_factor = 1.0 - ((time - fog_peak).abs() / fog_span).clamp(0.0, 1.0);
        fog.fog_fade = mix(0.1, 0.3, fog_factor);
        fog.fog_distance = mix(10.0, 8.0, fog_factor);
    }
}
