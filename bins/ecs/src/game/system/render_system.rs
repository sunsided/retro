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

    pub fn frame_state_mut(&mut self) -> Option<&mut FrameState> {
        self.frame_state.as_mut()
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

pub struct FrameState {
    frame: SurfaceTexture,
    pub view: TextureView,
    pub encoder: CommandEncoder,
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
            let encoder = state
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

            Some(FrameState {
                frame,
                view,
                encoder,
                render_start,
            })
        };
        Ok(())
    }

    fn on_end_phase(&mut self, _data: Option<Self::Result>, context: &FrameContext, render: &mut WgpuRenderState) -> Result<(), Self::Error> {
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

    fn is_ready(&self, state: &mut WgpuRenderState) -> bool {
        state.state.is_some()
    }

    fn apply_single(
        &mut self,
        _state: &mut WgpuRenderState,
        position: &PositionComponent,
    ) {
        trace!("Rendering position: {:?}", position);

        todo!()
        // let frame = surface.get_current_texture().expect("Failed to acquire frame");
        // let view = frame.texture.create_view(&Default::default());
    }
}
