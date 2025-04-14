use crate::engine::phases::render::{FrameState, WgpuRenderState};
use crate::engine::{
    ApplyRenderBackgroundSystem, ApplyRenderSkySystem, BackgroundColorComponent, CreateSystem,
    RenderBackgroundSystem, RenderSkySystem, SkydomeComponent, SystemFactory, TimeOfDayComponent,
};
use crate::wgpu::model::DrawModel;
use crate::wgpu::texture::Texture;
use crate::wgpu::uniforms::FrameGlobalUniform;
use crate::wgpu::wgpu_hardware::DeviceAndQueue;
use std::convert::Infallible;
use tracing::warn;
use wgpu::{BindGroup, IndexFormat, RenderPass, TextureView};

#[derive(Debug, Clone)]
pub struct RenderSkySystemData;

impl Default for RenderSkySystemData {
    fn default() -> Self {
        Self
    }
}

impl CreateSystem<RenderSkySystem> for SystemFactory {
    fn create(&self) -> RenderSkySystem {
        RenderSkySystem(RenderSkySystemData::default())
    }
}

impl ApplyRenderSkySystem for RenderSkySystem {
    type Error = Infallible;

    fn apply_single(&mut self, gpu: &mut WgpuRenderState, _: &SkydomeComponent) {
        let Ok((_device, resources, frame_state)) = gpu.for_rendering() else {
            warn!("Skipping background rendering because hardware or frame is not ready.");
            return;
        };

        let mut render_pass = frame_state.render_pass_with(&resources.pipelines().sky);
        render_pass.set_bind_group(0, &resources.neo.frame_globals, &[]);
        render_pass.set_bind_group(1, &resources.neo.camera, &[]);

        // Draw a full-screen triangle.
        render_pass.draw(0..3, 0..1);
    }
}
