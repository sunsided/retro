use tracing::{debug, trace};
use crate::engine::{ApplyRenderSystem, ModelComponent, PositionComponent, RenderSystem, TransformComponent};
use crate::engine::phases::render::WgpuRenderState;

impl ApplyRenderSystem for RenderSystem {
    type Error = wgpu::SurfaceError;

    #[inline]
    fn is_ready(&self, state: &mut WgpuRenderState) -> bool {
        state.is_ready()
    }

    fn apply_single(
        &mut self,
        _state: &mut WgpuRenderState,
        position: &PositionComponent,
        transform: &TransformComponent,
        model: &ModelComponent,
    ) {
        debug!("Rendering a single entity at {position:?}, {transform:?}.");
    }
}
