use std::convert::Infallible;
use tracing::warn;
use crate::engine::{ApplyCameraRenderSystem, ApplyDynamicLightSystem, CameraComponent, CameraRenderSystem, ColorComponent, CreateSystem, DynamicLightComponent, DynamicLightSystem, FrameContext, SystemFactory};
use crate::engine::components::ActiveCameraState;
use crate::engine::phases::render::WgpuRenderState;

#[derive(Debug)]
pub struct DynamicLightSystemData;

impl ApplyDynamicLightSystem for DynamicLightSystem {
    type Error = Infallible;

    fn apply_many(&mut self, context: &FrameContext, gpu: &WgpuRenderState, colors: &[ColorComponent], lights: &mut [DynamicLightComponent]) {
        let Some(device) = gpu.hardware().device() else {
            warn!("Skipping light rendering because no device is set.");
            return;
        };

        for (light, color) in lights.iter_mut().zip(colors) {
            // TODO: Move this to the update phase
            light.update_position(context.delta_time_secs.into());
            light.update_color(color.r, color.g, color.b);

            // TODO: Keep this in the render phase
            light.write_to_buffer(&device);
        }
    }
}

impl CreateSystem<DynamicLightSystem> for SystemFactory {
    fn create(&self) -> DynamicLightSystem {
        DynamicLightSystem(DynamicLightSystemData)
    }
}
