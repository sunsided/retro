use crate::engine::phases::render::{FrameState, WgpuRenderState};
use crate::engine::{
    AmbientLightComponent, ApplyFrameGlobalsSystem, ApplyRenderBackgroundSystem,
    ApplyRenderSkySystem, BackgroundColorComponent, CreateSystem, FogComponent, FrameGlobalsSystem,
    RenderBackgroundSystem, RenderSkySystem, SkydomeComponent, StarsComponent, SunComponent,
    SystemFactory, TimeOfDayComponent,
};
use crate::wgpu::model::DrawModel;
use crate::wgpu::texture::Texture;
use crate::wgpu::uniforms::FrameGlobalUniform;
use crate::wgpu::wgpu_hardware::DeviceAndQueue;
use std::convert::Infallible;
use tracing::warn;
use wgpu::{BindGroup, IndexFormat, RenderPass, TextureView};

#[derive(Debug, Clone)]
pub struct FrameGlobalsSystemData;

impl Default for FrameGlobalsSystemData {
    fn default() -> Self {
        Self
    }
}

impl CreateSystem<FrameGlobalsSystem> for SystemFactory {
    fn create(&self) -> FrameGlobalsSystem {
        FrameGlobalsSystem(FrameGlobalsSystemData::default())
    }
}

impl ApplyFrameGlobalsSystem for FrameGlobalsSystem {
    type Error = Infallible;

    fn apply_single(
        &mut self,
        gpu: &mut WgpuRenderState,
        time_of_day: &TimeOfDayComponent,
        sun: &SunComponent,
        fog: &FogComponent,
        ambient_light: &AmbientLightComponent,
        stars: &StarsComponent,
        skydome: &SkydomeComponent,
    ) {
        let Some(device) = gpu.hardware().device() else {
            warn!("Skipping frame globals setup because hardware is not ready.");
            return;
        };

        let Some(resources) = gpu.hardware().resources() else {
            warn!("Skipping frame globals setup because resources are not ready.");
            return;
        };

        let mut uniform = FrameGlobalUniform::new(time_of_day.time_of_day);
        uniform.star_visibility = stars.intensity;
        uniform.star_speed = stars.rotational_speed;
        uniform.sky_color = skydome.sky_color.into();
        uniform.ambient_light = ambient_light.ambient_light.into();
        uniform.sun_direction = sun.direction.into();
        uniform.sun_color = sun.color.into();
        uniform.sun_visibility = sun.intensity;
        uniform.window_aspect = gpu.screen_size().aspect_ratio;
        uniform.fog_distance = fog.fog_distance;
        uniform.fog_fade = fog.fog_fade;

        resources
            .neo
            .uniforms
            .frame_globals
            .send_to_device(device.queue(), uniform);
    }
}
