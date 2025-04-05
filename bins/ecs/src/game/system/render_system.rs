use crate::WgpuState;
use parking_lot::Mutex;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use tracing::{error, trace, warn};
use wgpu::{CommandEncoder, SurfaceTexture, TextureView};
use crate::game::component::PositionComponent;
use crate::game::system::{ApplyRenderSystem, RenderSystem};
use crate::game::world::{FrameContext, RenderState};

impl Default for RenderSystem {
    fn default() -> Self {
        Self(RenderSystemData::default())
    }
}

#[derive(Default)]
pub struct RenderSystemData {
    state: Option<Arc<Mutex<WgpuState>>>,
    frame_state: Option<FrameState>
}

impl RenderSystemData {
    pub fn set_state(&mut self, state: Option<Arc<Mutex<WgpuState>>>) {
        self.state = state;
    }
}

impl Debug for RenderSystemData {
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
    view: TextureView,
    encoder: CommandEncoder,
}

impl ApplyRenderSystem for RenderSystem {
    type Error = wgpu::SurfaceError;

    fn is_ready(&self, _context: &FrameContext, _state: &RenderState) -> bool {
        self.state.is_some()
    }

    fn on_begin_phase(&mut self, _context: &FrameContext, _state: &RenderState) -> Result<(), Self::Error> {
        self.frame_state = {
            let state = self.state.as_ref().expect("State exists as per readiness check").lock();
            let frame = state.surface.get_current_texture().inspect_err(|err| error!(%err, "Failed to obtain texture from surface"))?;
            let view = frame.texture.create_view(&Default::default());

            let mut encoder = state.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
            Some(FrameState {
                frame,
                view,
                encoder
            })
        };
        Ok(())
    }

    fn on_end_phase(&mut self, _context: &FrameContext, _state: &RenderState)  -> Result<(), Self::Error> {
        self.frame_state = None;
        Ok(())
    }

    fn apply_single(
        &mut self,
        context: &FrameContext,
        _state: &RenderState,
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
