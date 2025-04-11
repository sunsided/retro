use crate::wgpu::render_passes::RenderPasses;
use crate::wgpu::wgpu_hardware::{DeviceAndQueue, RenderPipelineSelector};
use std::sync::atomic::AtomicU32;
use std::sync::Arc;
use std::time::{Duration, Instant};
use wgpu::{CommandEncoder, RenderPass, SurfaceTexture, TextureView};

/// Represents the state for a single rendering frame.
///
/// This struct contains resources specific to the current frame,
/// such as the render passes, command encoder, and timing information.
/// It manages the GPU resources required for rendering operations
/// and tracks the start time of the frame rendering process for performance analysis.
///
/// # Fields
/// - `passes`: A `RenderPasses` struct, which encapsulates various render targets,
///   including different texture views and the command encoder used for rendering.
/// - `render_start`: The timestamp marking the beginning of the rendering process
///   for this frame, useful for debugging and measuring frame time performance.
///
/// # Uses
/// This struct is used to manage per-frame rendering resources and timing during
/// the rendering pipeline.
///
/// Example systems using `FrameState`:
/// - [`ApplyRenderSystem`]
/// - [`ApplyRenderBackgroundSystem`]
pub struct FrameState {
    pub passes: RenderPasses,
    pub render_start: Instant,
    pub render_pipeline_selector: RenderPipelineSelector,
    per_frame_entities: FrameEntityIds,
}

impl FrameState {
    pub fn new(
        passes: RenderPasses,
        render_start: Instant,
        render_pipeline_selector: RenderPipelineSelector,
    ) -> Self {
        Self {
            passes,
            render_start,
            render_pipeline_selector,
            per_frame_entities: FrameEntityIds::new(),
        }
    }

    pub fn render_pass(&mut self, device: &DeviceAndQueue) -> RenderPass {
        let mut pass = self.passes.render_pass();
        device.apply_render_pipeline_to_pass(&mut pass, self.render_pipeline_selector);
        pass
    }

    #[inline]
    pub fn per_frame_entity_ids(&self) -> FrameEntityIds {
        self.per_frame_entities.clone()
    }
}

#[derive(Debug, Clone)]
pub struct FrameEntityIds(Arc<AtomicU32>);

impl FrameEntityIds {
    #[inline]
    fn new() -> Self {
        Self(Arc::new(AtomicU32::new(0)))
    }

    /// Obtains the next entity ID for the frame.
    #[inline]
    pub fn next(&self) -> u32 {
        self.0.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
    }
}
