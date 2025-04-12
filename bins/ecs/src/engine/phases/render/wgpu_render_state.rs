use crate::engine::phases::render::frame_state::FrameState;
use crate::wgpu::uniforms::RetroUniform;
use crate::wgpu::wgpu_hardware::{
    DeviceAndQueue, RenderPipelineSelector, WgpuHardware, WgpuReinitializeWindowError,
    WgpuResizeSurfaceError,
};
use std::cell::Cell;
use std::fmt::{Debug, Formatter};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use tracing::{debug, instrument};
use winit::dpi::PhysicalSize;
use winit::window::Window;

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
    /// The hardware abstraction (devices, queues, ...)
    hardware: WgpuHardware,
    /// The per-frame state, which contains details for the current frame.
    frame_state: Option<FrameState>,
    /// Retro rendering state
    retro: Retro,
    /// Selects the render pipeline (wireframe, etc.)
    pipeline_selector: RenderPipelineSelector,
}

#[derive(Default)]
pub struct Retro {
    enabled: bool,
    uniform: RetroUniform,
    uniform_uploaded: AtomicBool,
}

impl WgpuRenderState {
    #[inline]
    pub const fn is_ready(&self) -> bool {
        self.hardware.is_ready()
    }

    #[inline]
    pub fn set_hardware(&mut self, hardware: WgpuHardware) {
        self.hardware = hardware;
    }

    #[inline]
    pub fn set_frame_state(&mut self, frame_state: Option<FrameState>) {
        self.frame_state = frame_state;
    }
    pub const fn pipeline_selector(&self) -> RenderPipelineSelector {
        self.pipeline_selector
    }

    /// Toggles the render pipeline selector.
    pub fn toggle_pipeline_selector(&mut self) {
        self.pipeline_selector = match self.pipeline_selector {
            RenderPipelineSelector::Textured => {
                debug!("Switching to boring pipeline");
                RenderPipelineSelector::Boring
            }
            RenderPipelineSelector::Boring => {
                // Skip wireframe if it's not supported by the GPU
                if self.wireframe_supported() {
                    debug!("Switching to wireframe pipeline");
                    RenderPipelineSelector::Wireframe
                } else {
                    debug!("Switching to entity pipeline (wireframe not supported)");
                    RenderPipelineSelector::Entity
                }
            }
            RenderPipelineSelector::Wireframe => {
                debug!("Switching to entity pipeline");
                RenderPipelineSelector::Entity
            }
            RenderPipelineSelector::Entity => {
                debug!("Switching to textured pipeline");
                RenderPipelineSelector::Textured
            }
        };
    }

    #[inline]
    pub const fn hardware(&self) -> &WgpuHardware {
        &self.hardware
    }

    #[instrument(skip(self))]
    pub fn for_rendering(
        &mut self,
    ) -> Result<(&DeviceAndQueue, &mut FrameState), RenderPhaseError> {
        let Some(hardware) = self.hardware.device() else {
            debug!("Device is not ready yet, cannot render");
            return Err(RenderPhaseError::DeviceNotReady);
        };

        let Some(frame_state) = &mut self.frame_state else {
            debug!("Frame state is not ready yet, cannot render");
            return Err(RenderPhaseError::FrameStateNotReady);
        };

        Ok((hardware, frame_state))
    }

    #[instrument(skip(self))]
    pub fn finish_frame(&mut self) -> Result<(&DeviceAndQueue, FrameState), RenderPhaseError> {
        let Some(hardware) = self.hardware.device() else {
            debug!("Device is not ready yet, cannot render");
            return Err(RenderPhaseError::DeviceNotReady);
        };

        let Some(frame_state) = self.frame_state.take() else {
            debug!("Frame state is not ready yet, cannot render");
            return Err(RenderPhaseError::FrameStateNotReady);
        };

        Ok((hardware, frame_state))
    }

    #[inline]
    pub fn reinitialize_surface(
        &mut self,
        window: Arc<Window>,
    ) -> Result<(), WgpuReinitializeWindowError> {
        self.retro.uniform.resize_surface(window.inner_size());
        self.retro
            .uniform_uploaded
            .store(false, std::sync::atomic::Ordering::Relaxed);

        self.hardware.reinitialize_surface(window)
    }

    #[inline]
    pub fn resize_surface(
        &mut self,
        new_size: PhysicalSize<u32>,
    ) -> Result<(), WgpuResizeSurfaceError> {
        self.retro.uniform.resize_surface(new_size);
        self.retro
            .uniform_uploaded
            .store(false, std::sync::atomic::Ordering::Relaxed);

        self.hardware.resize_surface(new_size)
    }

    /// Gets the size of the configured surface.
    pub fn surface_size(&self) -> Option<PhysicalSize<u32>> {
        self.hardware.device().map(|dev| {
            let width = dev.config().width;
            let height = dev.config().height;
            PhysicalSize::new(width, height)
        })
    }

    /// Indicates whether wireframe rendering is supported by the adapter.
    #[inline]
    pub fn wireframe_supported(&self) -> bool {
        self.hardware.wireframe_supported()
    }

    /// Updates the retro uniform on the GPU.
    // TODO: Refactor
    pub fn update_retro_uniform(&self) {
        // Set the flag to uploaded; if it was already uploaded, return.
        if self
            .retro
            .uniform_uploaded
            .swap(true, std::sync::atomic::Ordering::Relaxed)
        {
            return;
        }

        let Some(device) = self.hardware.device() else {
            return;
        };

        debug!("Updating retro uniform");
        device
            .buffers()
            .retro_uniform
            .send_to_device(device.queue(), self.retro.uniform);
    }

    /// Toggles retro rendering state.
    #[inline]
    pub fn set_retro_enabled(&mut self, enabled: bool) {
        self.retro.enabled = enabled;
    }

    /// Gets retro rendering state.
    pub const fn retro_enabled(&self) -> bool {
        self.retro.enabled
    }
}

impl Debug for WgpuRenderState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.hardware.device().is_some() {
            write!(f, "RenderSystemData")
        } else {
            write!(f, "RenderSystemData (uninitialized)")
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RenderPhaseError {
    #[error("The device is not ready yet")]
    DeviceNotReady,
    #[error("The frame state is not ready yet (e.g., the surface is not initialized yet)")]
    FrameStateNotReady,
}
