use crate::engine::phases::render::background::RenderBackgroundSystemData;
use crate::engine::phases::render::{FrameState, WgpuRenderState};
use crate::engine::{
    ApplyRenderBackgroundSystem, ApplyVgaBlitSystem, BackgroundColorComponent, CreateSystem,
    RenderBackgroundSystem, SystemFactory, VgaBlitSystem,
};
use crate::wgpu::model::DrawModel;
use crate::wgpu::texture::Texture;
use std::borrow::Borrow;
use std::convert::Infallible;
use tracing::warn;
use wgpu::{BindGroup, IndexFormat, RenderPass, TextureView};

#[derive(Debug, Default)]
pub struct VgaBlitSystemData;

impl CreateSystem<VgaBlitSystem> for SystemFactory {
    fn create(&self) -> VgaBlitSystem {
        VgaBlitSystem(VgaBlitSystemData::default())
    }
}

impl ApplyVgaBlitSystem for VgaBlitSystem {
    type Error = Infallible;

    fn is_ready(&self, state: &mut WgpuRenderState) -> bool {
        state.retro_enabled()
    }

    fn apply_single(
        &mut self,
        wgpu_render: &mut WgpuRenderState,
        background_color: &BackgroundColorComponent,
    ) {
        let Ok((_device, resources, frame_state)) = wgpu_render.for_rendering() else {
            warn!("Skipping background rendering because hardware or frame is not ready.");
            return;
        };

        // Second pass: Presenting the VGA offscreen buffer to screen.
        let background_color = background_color.dim(0.5);
        if let Some(mut pass) = frame_state.passes.final_pass(&resources, &background_color) {
            pass.set_pipeline(resources.renderers().vga_fullscreen.pipeline());
            pass.set_bind_group(0, &resources.neo.uniforms.frame_globals, &[]);
            pass.set_bind_group(1, &resources.neo.uniforms.camera, &[]);
            pass.set_bind_group(2, &resources.neo.uniforms.light, &[]);
            pass.set_bind_group(3, resources.renderers().vga_fullscreen.bind_group(), &[]);

            // Draw exactly 3 vertices – no vertex buffer
            pass.draw(0..6, 0..1);
        }
    }
}
