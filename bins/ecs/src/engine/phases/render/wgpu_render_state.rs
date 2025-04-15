use crate::engine::phases::render::frame_state::FrameState;
use crate::wgpu::wgpu_hardware::resources::render_pipelines::RenderPipelineSelector;
use crate::wgpu::wgpu_hardware::resources::WgpuResources;
use crate::wgpu::wgpu_hardware::{
    WgpuForRenderingError, WgpuHardware, WgpuHardwareInitializeSurfaceError,
    WgpuReinitializeWindowError, WgpuResizeSurfaceError,
};
use std::cell::Cell;
use std::fmt::{Debug, Formatter};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use tracing::{debug, instrument};
use wgpu_resource_manager::{
    ConfiguredWgpuResourceManager, DeviceAndQueue, WgpuResourceManagerInitializeSurfaceError,
};
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
    hardware: Option<WgpuHardware>,
    /// The per-frame state, which contains details for the current frame.
    frame_state: Option<FrameState>,
    /// Retro rendering state
    retro: Retro,
    /// The current size of the screen.
    screen_size: ScreenSize,
    /// Selects the render pipeline (wireframe, etc.)
    pipeline_selector: RenderPipelineSelector,
}

#[derive(Debug, Copy, Clone)]
#[allow(unused)]
pub struct ScreenSize {
    pub width: f32,
    pub height: f32,
    pub aspect_ratio: f32,
}

#[derive(Default)]
pub struct Retro {
    enabled: bool,
}

impl Default for ScreenSize {
    fn default() -> Self {
        Self {
            width: 300.0,
            height: 200.0,
            aspect_ratio: 300.0 / 200.0,
        }
    }
}

impl WgpuRenderState {
    #[inline]
    pub const fn is_ready(&self) -> bool {
        match self.hardware.as_ref() {
            None => false,
            Some(hw) => hw.is_ready(),
        }
    }

    #[inline]
    pub fn set_hardware(&mut self, hardware: WgpuHardware) {
        self.hardware = Some(hardware);
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
    pub const fn hardware(&self) -> Option<&WgpuHardware> {
        self.hardware.as_ref()
    }

    #[instrument(skip(self))]
    pub fn for_rendering(
        &mut self,
    ) -> Result<(&DeviceAndQueue, &mut WgpuResources, &mut FrameState), RenderPhaseError> {
        let hardware = self
            .hardware
            .as_mut()
            .ok_or(RenderPhaseError::DeviceNotReady)?;
        let (dq, resources) = hardware.for_rendering()?;

        let Some(frame_state) = &mut self.frame_state else {
            debug!("Frame state is not ready yet, cannot render");
            return Err(RenderPhaseError::FrameStateNotReady);
        };

        Ok((dq, resources, frame_state))
    }

    #[instrument(skip(self))]
    pub fn finish_frame(&mut self) -> Result<(&DeviceAndQueue, FrameState), RenderPhaseError> {
        let Some(hardware) = self.hardware.as_ref() else {
            debug!("Device is not ready yet, cannot render");
            return Err(RenderPhaseError::DeviceNotReady);
        };

        let Some(hardware) = hardware.device() else {
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
    ) -> Result<(), WgpuHardwareInitializeSurfaceError> {
        let size = window.inner_size();
        self.screen_size = ScreenSize {
            width: size.width as _,
            height: size.height as _,
            aspect_ratio: size.width as f32 / size.height as f32,
        };

        if let Some(hardware) = self.hardware.as_mut() {
            hardware.reinitialize_surface(window)?;
            Ok(())
        } else {
            Err(WgpuHardwareInitializeSurfaceError::NoHardware)
        }
    }

    #[inline]
    pub fn resize_surface(
        &mut self,
        new_size: PhysicalSize<u32>,
    ) -> Result<(), WgpuResizeSurfaceError> {
        self.screen_size = ScreenSize {
            width: new_size.width as _,
            height: new_size.height as _,
            aspect_ratio: new_size.width as f32 / new_size.height as f32,
        };

        if let Some(hardware) = self.hardware.as_mut() {
            hardware.resize_surface(new_size)
        } else {
            Err(WgpuResizeSurfaceError::NoHardware)
        }
    }

    /// Gets the size of the configured surface.
    pub fn surface_size(&self) -> Option<PhysicalSize<u32>> {
        self.hardware
            .as_ref()
            .map(|hardware| {
                hardware.resources().and_then(|res| {
                    let width = res.config().width;
                    let height = res.config().height;
                    Some(PhysicalSize::new(width, height))
                })
            })
            .flatten()
    }

    /// Indicates whether wireframe rendering is supported by the adapter.
    #[inline]
    pub fn wireframe_supported(&self) -> bool {
        self.hardware
            .as_ref()
            .map(|hw| hw.wireframe_supported())
            .unwrap_or_default()
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

    #[inline]
    pub const fn screen_size(&self) -> ScreenSize {
        self.screen_size
    }
}

impl Debug for WgpuRenderState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let device = self.hardware.as_ref().map(|hw| hw.device()).flatten();
        if device.is_some() {
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
    #[error("The device or its resources are not ready yet")]
    DeviceOrResourcesNotReady(#[from] WgpuForRenderingError),
    #[error("The frame state is not ready yet (e.g., the surface is not initialized yet)")]
    FrameStateNotReady,
}
