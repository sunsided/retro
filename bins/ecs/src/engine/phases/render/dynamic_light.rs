use std::convert::Infallible;
use tracing::warn;
use crate::engine::{ApplyCameraRenderSystem, ApplyDynamicLightSystem, CameraComponent, CameraRenderSystem, CreateSystem, DynamicLightComponent, DynamicLightSystem, FrameContext, SystemFactory};
use crate::engine::components::ActiveCameraState;
use crate::engine::phases::render::WgpuRenderState;

#[derive(Debug)]
pub struct DynamicLightSystemData;

impl ApplyDynamicLightSystem for DynamicLightSystem {
    type Error = Infallible;

    fn apply_many(&mut self, _context: &FrameContext, gpu: &WgpuRenderState, lights: &mut [DynamicLightComponent]) {
        let Some(device) = gpu.hardware().device() else {
            warn!("Skipping light rendering because no device is set.");
            return;
        };

        for light in lights {
            // TODO: Update position using relative time
            // TODO: Update Light using its specified color
            light.update_position();
            light.write_to_buffer(&device);
        }
    }
}

impl CreateSystem<DynamicLightSystem> for SystemFactory {
    fn create(&self) -> DynamicLightSystem {
        DynamicLightSystem(DynamicLightSystemData)
    }
}
