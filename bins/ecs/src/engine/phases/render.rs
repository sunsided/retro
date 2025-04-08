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
pub mod camera_render;
pub mod dynamic_light;
mod wgpu_render_state;
mod frame_state;
mod render;
pub mod vga_blit;

use crate::engine::{BackgroundColorComponent, ModelComponent};
use parking_lot::{ArcMutexGuard, Mutex, RawMutex};
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use parking_lot::lock_api::MutexGuard;
use tracing::{error, trace, warn};
use wgpu::{CommandEncoder, RenderPass, SurfaceTexture, TextureView};
use wgpu::hal::empty::Encoder;
use winit::dpi::PhysicalSize;
use winit::window::Window;
use crate::engine::{ApplyRenderSystem, CreateSystem, FrameContext, PhaseEvents, PhysicsSystem, PositionComponent, RenderBackgroundSystem, RenderSystem, SystemFactory, SystemRenderPhaseEvents, TransformComponent};
use crate::engine::phases::render::background::RenderBackgroundSystemData;
use crate::wgpu::render_passes::RenderPasses;
use crate::wgpu::wgpu_hardware::{DeviceAndQueue, WgpuHardware, WgpuReinitializeWindowError, WgpuResizeSurfaceError};
pub use self::wgpu_render_state::WgpuRenderState;
pub use self::frame_state::FrameState;

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

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RenderReady {
    #[default]
    NotReady,
    Ready,
}

impl RenderReady {
    pub fn is_ready(&self) -> bool {
        *self == RenderReady::Ready
    }
}

impl SystemRenderPhaseEvents for PhaseEvents {
    type Result = RenderReady;
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

        let Some(device) = render.hardware().device() else {
            return Ok(RenderReady::NotReady);
        };

        // TODO: Refactor
        render.update_retro_uniform();

        let Some(surface) = render.hardware().surface().as_ref() else {
            return Ok(RenderReady::NotReady);
        };

        self.start_timing();
        render.set_frame_state({
            let render_start = std::time::Instant::now();

            // The `get_current_texture` function will wait for the surface to provide a new
            // `SurfaceTexture` that we will render to. This can time out, particularly
            // on Vulkan/Wayland/NVIDIA setups. (In that case, use the GL backend, e.g. via `WGPU_BACKEND=gl`).
            let frame = surface
                .get_current_texture()
                .inspect_err(|err| error!(%err, "Failed to obtain texture from surface"))?;

            // Create a `TextureView` to control how we interact with the render texture.
            let screen_texture = frame
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());

            // We also create a CommandEncoder to create the actual commands to send to the GPU.
            // Most modern graphics frameworks expect commands to be stored in a command buffer
            // before being sent to the GPU.
            let encoder = device
                .device()
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

            let passes = RenderPasses::new(&device, render.retro_enabled(), encoder, frame, screen_texture);
            Some(FrameState::new(passes, render_start, render.pipeline_selector()))
        });
        Ok(RenderReady::Ready)
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
    fn on_end_phase(&mut self, data: Option<Self::Result>, context: &FrameContext, render: &mut WgpuRenderState) -> Result<(), Self::Error> {
        if !data.unwrap_or_default().is_ready() {
            return Ok(());
        }

        if let Ok((device, frame_state)) = render.finish_frame() {
            frame_state.passes.finish_and_present(device);
            let render_duration = frame_state.render_start.elapsed();
            trace!(context.frame_number, "Rendered frame in {render_duration:?}");
        }
        else {
            warn!("Skipping background rendering because hardware or frame is not ready.");
        };

        // Trace the time of the phase itself.
        let elapsed = self.elapsed();
        trace!("{name} phase took {elapsed:?}", name = <PhaseEvents as SystemRenderPhaseEvents>::NAME);
        Ok(())
    }
}

