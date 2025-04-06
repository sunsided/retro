//! # Render Module
//!
//! This module provides components and systems for the rendering pipeline
//! of the application using the WGPU backend. It establishes the infrastructure
//! necessary for rendering operations, including management of GPU state,
//! per-frame states, and the integration of rendering phases with various systems.
//!
//! ## Overview
//!
//! Rendering involves multiple stages, and this module handles the setup,
//! execution, and teardown of these processes. It includes abstractions that
//! represent both the shared GPU state and the per-frame state, enabling smooth
//! operation of the rendering pipeline over multiple frames.
//!
//! ### Key Components
//!
//! - **`RenderSystem`**: Represents the main rendering system. It acts as an orchestrator
//!   for rendering operations by integrating with other systems in the game engine.
//!
//! - **`WgpuRenderState`**: Holds the GPU's shared state and its per-frame state. This allows
//!   for centralized management of the GPU resources and operations split between frames.
//!
//! - **`FrameState`**: Encompasses resources specific to a single frame, such as
//!   the surface texture, texture view, and GPU command encoder for rendering commands.
//!
//! ### Integration with Systems
//!
//! The module defines hooks and phases that integrate with other systems, for example:
//!
//! - **Render Phase Events**: These events, defined via `SystemRenderPhaseEvents`, enable
//!   initialization, execution, and cleanup of rendering phases.
//!
//! - **Background Rendering**: Systems like `ApplyRenderBackgroundSystem` are integrated,
//!   demonstrating how this module fits into a larger pipeline involving tasks, such as
//!   background rendering before primary graphics are presented.
//!
//! ### Logging and Debugging
//!
//! Detailed logging is used throughout to assist in debugging and ensuring smooth
//! rendering operations. This includes error logs for GPU texture acquisition issues,
//! and performance logging for tracking frame render durations.
//!
//! ## Example Flow
//!
//! 1. **Initialization**: The `WgpuRenderState` is set up with a shared `WgpuState`
//!    that contains the GPU resources and surface.
//! 2. **Frame Setup**: At the start of each frame, the `FrameState` is initialized,
//!    assigning the current frame texture and setting up a command encoder.
//! 3. **Drawing Commands**: Using the initialized `FrameState`, drawing commands are
//!    recorded and submitted to the GPU.
//! 4. **Finalization**: The frame is finalized and presented to the display.
//!
//! ## Modules and Key Types
//!
//! - `RenderSystem`, `RenderSystemData`: Implement the functionality to manage rendering processes.
//! - `WgpuRenderState`: Manages GPU and per-frame state.
//! - `FrameState`: Represents a single frame's rendering context.
//!
//! ## See Also
//!
//! - [`ApplyRenderBackgroundSystem`]: Example of a system that interacts with `RenderSystem`.
//! - [`SystemRenderPhaseEvents`]: Trait showing integration with phase-based systems.

pub mod background;

use crate::WgpuState;
use parking_lot::Mutex;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use tracing::{error, trace};
use wgpu::{CommandEncoder, SurfaceTexture, TextureView};
use crate::engine::{ApplyRenderSystem, CreateSystem, FrameContext, PhaseEvents, PhysicsSystem, PositionComponent, RenderBackgroundSystem, RenderSystem, SystemFactory, SystemRenderPhaseEvents};
use crate::engine::phases::render::background::RenderBackgroundSystemData;

impl CreateSystem<RenderBackgroundSystem> for SystemFactory {
    fn create(&self) -> RenderBackgroundSystem {
        RenderBackgroundSystem(RenderBackgroundSystemData)
    }
}

impl CreateSystem<RenderSystem> for SystemFactory {
    fn create(&self) -> RenderSystem {
        RenderSystem::default()
    }
}

impl Default for RenderSystem {
    fn default() -> Self {
        Self(RenderSystemData::default())
    }
}

#[derive(Default, Debug)]
pub struct RenderSystemData;

/// Represents the render state for the WGPU system.
///
/// This structure holds the current state of the WGPU system, including the
/// GPU state and per-frame data. It provides functionality for managing
/// the WGPU backend and accessing mutable frame state.
///
/// # Fields
/// - `state`: An optional shared GPU state wrapped in an `Arc<Mutex<WgpuState>>`.
/// - `frame_state`: An optional per-frame state that includes frame-specific
///   details such as the surface texture and command encoder.
///
/// # Uses
/// See e.g.
///
/// - [`ApplyRenderBackgroundSystem`]
#[derive(Default)]
pub struct WgpuRenderState {
    /// The shared GPU state. Uses `Arc<Mutex<WgpuState>>` to allow shared access.
    state: Option<Arc<Mutex<WgpuState>>>,
    /// The per-frame state, which contains details for the current frame.
    frame_state: Option<FrameState>,
}

impl WgpuRenderState {
    /// Sets the shared GPU state for the render system.
    ///
    /// # Parameters
    /// - `state`: An `Option` containing an `Arc<Mutex<WgpuState>>`. If `Some`,
    ///   it sets the GPU state; if `None`, it unsets the current state.
    pub fn set_state(&mut self, state: Option<Arc<Mutex<WgpuState>>>) {
        self.state = state;
    }

    /// Provides a mutable reference to the per-frame state, if available.
    ///
    /// # Returns
    /// - `Option<&mut FrameState>`: A mutable reference to the current frame state
    ///   if it exists, otherwise `None`.
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
    frame: SurfaceTexture,
    pub view: TextureView,
    pub encoder: CommandEncoder,
    render_start: std::time::Instant,
}

impl SystemRenderPhaseEvents for PhaseEvents {
    type Result = ();
    type Error = wgpu::SurfaceError;

    /// Initializes the frame state at the beginning of the render phase.
    ///
    /// This function sets up the `FrameState` for the current rendering frame, which includes:
    /// - Obtaining the current surface texture from the WGPU surface.
    /// - Creating a texture view for the frame buffer.
    /// - Creating a command encoder for recording rendering commands.
    /// - Marking the start time of the rendering phase for performance measurement.
    ///
    /// # Parameters
    /// - `context`: A reference to the [`FrameContext`] containing information about the current frame.
    /// - `render`: A mutable reference to the [`WgpuRenderState`], which holds the GPU state and per-frame data.
    ///
    /// # Side Effects
    /// - Locks the WGPU shared state (`Arc<Mutex<WgpuState>`) to access GPU resources.
    /// - Attempts to obtain the current texture from the WGPU `surface`.
    /// - Logs an error if the surface texture cannot be retrieved.
    /// - Updates the `frame_state` field of the `render` parameter with the newly created [`FrameState`].
    ///
    /// # Returns
    /// - `Ok(())` if the frame state was successfully initialized.
    /// - `Err(wgpu::SurfaceError)` if there was an issue obtaining the surface texture.
    fn on_begin_phase(&mut self, _context: &FrameContext, render: &mut WgpuRenderState) -> Result<Self::Result, Self::Error> {
        // TODO: Have the application (through the ECS) react to wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated by recreating the window.
        // TODO: Have the application react to wgpu::SurfaceError::OutOfMemory | wgpu::SurfaceError::Other by exiting
        // TODO: Have the application react to wgpu::SurfaceError::Timeout by hoping for thee best :)

        self.start_timing();
        render.frame_state = {
            let render_start = std::time::Instant::now();
            let state = render.state.as_ref().expect("State exists as per readiness check").lock();

            // The `get_current_texture` function will wait for the surface to provide a new
            // `SurfaceTexture` that we will render to. This can time out, particularly
            // on Vulkan/Wayland/NVIDIA setups. (In that case, use the GL backend, e.g. via `WGPU_BACKEND=gl`).
            let frame = state
                .surface()
                .get_current_texture()
                .inspect_err(|err| error!(%err, "Failed to obtain texture from surface"))?;

            // Create a `TextureView` to control how we interact with the render texture.
            let view = frame
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());

            // We also create a CommandEncoder to create the actual commands to send to the GPU.
            // Most modern graphics frameworks expect commands to be stored in a command buffer
            // before being sent to the GPU.
            let encoder = state
                .device()
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

    /// Finalizes the rendering phase, submits commands, and presents the frame.
    ///
    /// This function is called at the end of the render phase to handle cleanup
    /// and submission of rendering commands. It performs the following steps:
    /// - Retrieves and takes ownership of the `FrameState` from the `WgpuRenderState`.
    /// - Finishes encoding GPU commands into a command buffer.
    /// - Submits the command buffer to the GPU queue.
    /// - Presents the frame to the display.
    /// - Logs the duration of the rendering process for debugging purposes.
    ///
    /// # Parameters
    /// - `_data`: An optional phase result that is not used here.
    /// - `context`: A reference to the [`FrameContext`] that provides frame-specific metadata.
    /// - `render`: A mutable reference to the [`WgpuRenderState`], which holds GPU and per-frame state.
    ///
    /// # Side Effects
    /// - Locks the WGPU shared state (`Arc<Mutex<WgpuState>`) to access GPU resources.
    /// - Submits the GPU command buffer to the queue via `wgpu::Queue::submit`.
    /// - Presents the frame to the GPU surface using `SurfaceTexture::present`.
    /// - Logs the rendering duration using the `tracing` crate for debugging.
    ///
    /// # Returns
    /// - `Ok(())` on success.
    /// - `Err(wgpu::SurfaceError)` if any errors occur during the rendering pipeline.
    fn on_end_phase(&mut self, _data: Option<Self::Result>, context: &FrameContext, render: &mut WgpuRenderState) -> Result<(), Self::Error> {
        if let Some(frame_state) = render.frame_state.take() {
            let command_buffer = frame_state.encoder.finish();
            let state = render.state.as_ref().expect("State exists").lock();
            state.submit_to_queue(command_buffer);
            // Optionally poll
            // state.poll_device(wgpu::Maintain::Wait);
            frame_state.frame.present();

            let render_duration = frame_state.render_start.elapsed();
            trace!(context.frame_number, "Rendered frame in {render_duration:?}");
        }
        let elapsed = self.elapsed();
        trace!("{name} phase took {elapsed:?}", name = <PhaseEvents as SystemRenderPhaseEvents>::NAME);
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
