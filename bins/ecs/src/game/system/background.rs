use std::convert::Infallible;
use crate::game::component::BackgroundColorComponent;
use crate::game::system::{ApplyRenderBackgroundSystem, RenderBackgroundSystem};
use crate::game::world::WgpuRenderState;

#[derive(Debug, Clone)]
pub struct BackgroundColorData;

#[derive(Debug)]
pub struct RenderBackgroundSystemData;

impl ApplyRenderBackgroundSystem for RenderBackgroundSystem {
    type Error = Infallible;

    fn apply_single(&mut self, wgpu_render: &mut WgpuRenderState, _background_color: &BackgroundColorComponent) {
        let Some(state) = wgpu_render.frame_state_mut() else {
            return;
        };
        let _render_pass = state
            .encoder
            .begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Clear Background Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &state.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
    }
}
