use crate::engine::components::ActiveCameraState;
use crate::engine::phases::render::WgpuRenderState;
use crate::engine::{
    ApplyCameraRenderSystem, ApplyDynamicLightSystem, CameraComponent, CameraRenderSystem,
    ColorComponent, CreateSystem, DynamicLightComponent, DynamicLightSystem, FrameContext,
    SystemFactory,
};
use crate::wgpu::uniforms::DrawLight;
use std::convert::Infallible;
use std::time::Duration;
use tracing::{debug, warn};

#[derive(Debug)]
pub struct DynamicLightSystemData;

impl ApplyDynamicLightSystem for DynamicLightSystem {
    type Error = Infallible;

    fn apply_many(
        &mut self,
        context: &FrameContext,
        gpu: &mut WgpuRenderState,
        colors: &[ColorComponent],
        lights: &mut [DynamicLightComponent],
    ) {
        let Ok((device, resources, frame_state)) = gpu.for_rendering() else {
            warn!("Skipping background rendering because hardware or frame is not ready.");
            return;
        };

        let mut render_pass = frame_state.render_pass(&resources);
        render_pass.set_pipeline(&resources.pipelines().light);

        for (light, color) in lights.iter_mut().zip(colors) {
            // TODO: Move this to the update phase
            light.update_position(context.delta_time_secs.into());
            light.update_color(color.r, color.g, color.b);

            // TODO: Keep this in the render phase
            light.write_to_buffer(&device, resources);

            // Render the light.
            render_pass.draw_light_model(
                &resources.models().cube,
                &resources.bind_groups().uniforms.camera_uniform,
                &resources.bind_groups().uniforms.light_uniforms,
            );
        }
    }
}

impl CreateSystem<DynamicLightSystem> for SystemFactory {
    fn create(&self) -> DynamicLightSystem {
        DynamicLightSystem(DynamicLightSystemData)
    }
}
