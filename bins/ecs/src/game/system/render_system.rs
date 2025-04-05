use std::convert::Infallible;
use crate::WgpuState;
use parking_lot::Mutex;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use tracing::{error, trace};
use wgpu::{CommandEncoder, SurfaceTexture, TextureView};
use crate::game::component::PositionComponent;
use crate::game::system::{ApplyRenderSystem, RenderSystem, SystemRenderPhaseEvents};
use crate::game::world::FrameContext;
use crate::phase_events::PhaseEvents;

impl Default for RenderSystem {
    fn default() -> Self {
        Self(RenderSystemData::default())
    }
}

#[derive(Default, Debug)]
pub struct RenderSystemData;

#[derive(Default)]
pub struct WgpuRenderState {
    state: Option<Arc<Mutex<WgpuState>>>,
    frame_state: Option<FrameState>
}

impl WgpuRenderState {
    pub fn set_state(&mut self, state: Option<Arc<Mutex<WgpuState>>>) {
        self.state = state;
    }
}

impl Debug for WgpuRenderState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.state.is_some() {
            write!(f, "RenderSystemData")
        } else {
            write!(f, "RenderSystemData (uninitialized)")
        }
    }
}

struct FrameState {
    frame: SurfaceTexture,
    _view: TextureView,
    encoder: CommandEncoder,
    render_start: std::time::Instant,
}

impl SystemRenderPhaseEvents for PhaseEvents {
    type Result = ();
    type Error = wgpu::SurfaceError;

    fn on_begin_phase(&mut self, _context: &FrameContext, render: &mut WgpuRenderState) -> Result<Self::Result, Self::Error> {
        render.frame_state = {
            let render_start = std::time::Instant::now();
            let state = render.state.as_ref().expect("State exists as per readiness check").lock();
            let frame = state
                .surface
                .get_current_texture()
                .inspect_err(|err| error!(%err, "Failed to obtain texture from surface"))?;
            let view = frame
                .texture
                .create_view(&Default::default());
            let mut encoder = state
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

            // Clear background.
            {
                let _render_pass = encoder
                    .begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("Clear Background Render Pass"),
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
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

            Some(FrameState {
                frame,
                _view: view,
                encoder,
                render_start,
            })
        };
        Ok(())
    }

    fn on_end_phase(&mut self, data: Option<Self::Result>, context: &FrameContext, render: &mut WgpuRenderState) -> Result<(), Self::Error> {
        if let Some(frame_state) = render.frame_state.take() {
            let command_buffer = frame_state.encoder.finish();
            let state = render.state.as_ref().expect("State exists").lock();
            state.queue.submit(Some(command_buffer));
            // Optionally poll
            // state.device.poll(wgpu::Maintain::Poll);
            frame_state.frame.present();

            let render_duration = frame_state.render_start.elapsed();
            trace!(context.frame_number, "Rendered frame in {render_duration:?}");
        }
        Ok(())
    }
}


impl ApplyRenderSystem for RenderSystem {
    type Error = wgpu::SurfaceError;

    fn is_ready(&self, _context: &FrameContext, state: &WgpuRenderState) -> bool {
        state.state.is_some()
    }

    fn apply_single(
        &mut self,
        context: &FrameContext,
        _state: &WgpuRenderState,
        position: &PositionComponent,
    ) {
        trace!(
            "Rendering position: {:?} in frame {}",
            position,
            context.frame_number
        );

        todo!()
        // let frame = surface.get_current_texture().expect("Failed to acquire frame");
        // let view = frame.texture.create_view(&Default::default());
    }
}
