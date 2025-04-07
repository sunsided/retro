use std::time::{Duration, Instant};
use wgpu::{CommandEncoder, SurfaceTexture, TextureView};

/// Represents the state for a single rendering frame.
///
/// This struct contains resources specific to the current frame,
/// such as the surface texture, the texture view for rendering,
/// and the command encoder for issuing GPU commands.
/// It also tracks the start time of the frame rendering process.
///
/// # Fields
/// - `frame`: The surface texture for the current frame, which represents
///   the frame buffer where rendering commands are drawn.
/// - `view`: A `TextureView` created from the `frame`, used as the target
///   for rendering operations.
/// - `encoder`: A `CommandEncoder` that records all rendering and compute
///   commands to be executed on the GPU for this frame.
/// - `render_start`: The timestamp marking the start of the rendering for
///   this frame, primarily used for performance measurement.
///
/// # Uses
/// See e.g.
///
/// - [`ApplyRenderBackgroundSystem`]
pub struct FrameState {
    pub frame: SurfaceTexture,
    pub view: TextureView,
    pub encoder: CommandEncoder,
    pub render_start: Instant,
}

impl FrameState {
    pub const fn new(frame: SurfaceTexture, view: TextureView, encoder: CommandEncoder, render_start: Instant) -> Self {
        Self {
            frame,
            view,
            encoder,
            render_start
        }
    }
}
