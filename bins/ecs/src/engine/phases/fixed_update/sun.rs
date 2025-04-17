use crate::engine::phases::{mix, mixv};
use crate::engine::{
    AmbientLightComponent, ApplySunSystem, CreateSystem, PhysicsSystem, SkydomeComponent,
    SunComponent, SunSystem, SystemFactory, TimeOfDayComponent,
};
use cgmath::{InnerSpace, Vector3};
use std::convert::Infallible;
use std::f32::consts::PI;
use std::fmt::Debug;

#[derive(Debug, Default)]
pub struct SunSystemData;

impl Default for SunSystem {
    fn default() -> Self {
        Self(SunSystemData)
    }
}

impl CreateSystem<SunSystem> for SystemFactory {
    fn create(&self) -> SunSystem {
        SunSystem::default()
    }
}

impl ApplySunSystem for SunSystem {
    type Error = Infallible;

    fn apply_single(
        &mut self,
        time: &TimeOfDayComponent,
        sun: &mut SunComponent,
        skydome: &mut SkydomeComponent,
        ambient: &mut AmbientLightComponent,
    ) {
        let time = time.time_of_day;

        // Compute sun angle and direction
        let sun_angle = (time + 18.0) * (2.0 * PI / 24.0);
        let tilt = 145f32.to_radians();

        let sun_dir = Vector3::new(
            sun_angle.cos(),
            sun_angle.sin() * tilt.sin(),
            sun_angle.sin() * tilt.cos(),
        )
        .normalize();

        let sun_visibility = sun_dir.y.max(0.0);
        let sunset_factor = 1.0 - sun_dir.y.clamp(0.0, 1.0);

        let cycle = 0.5 * (1.0 + ((time - 12.0) * (2.0 * PI / 24.0)).cos());
        let sunset_strength = sunset_factor * cycle;

        let sun_color = sun_color_blend(sunset_strength);
        let night_color = Vector3::new(0.03, 0.03, 0.2);
        let day_color = Vector3::new(0.4, 0.75, 1.0);
        let sky_color = mixv(night_color, day_color, cycle);
        let ambient_color = mixv(
            sky_color,
            Vector3::new(1.0, 0.4, 0.2),
            sunset_strength * 0.25,
        );

        sun.direction = sun_dir;
        sun.color = sun_color;
        sun.intensity = sun_visibility;

        skydome.sky_color = sky_color;

        ambient.ambient_light = ambient_color;
    }
}

#[inline(always)]
fn sun_color_blend(sunset_strength: f32) -> Vector3<f32> {
    mixv(
        Vector3::new(1.2, 1.05, 0.85),
        Vector3::new(1.3, 0.5, 0.2),
        sunset_strength,
    )
}
