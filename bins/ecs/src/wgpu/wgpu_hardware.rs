//! This module provides an abstraction for interacting with WGPU, a WebGPU implementation.
//! It contains the definitions and implementations necessary for managing GPU hardware,
//! surfaces, devices, and configurations. The primary aim of this module is to facilitate
//! initialization, surface management, and device configuration for rendering tasks.
//!
//! # Main Structures
//!
//! - [`WgpuHardware`]: Represents the main entry point for managing GPU instances, surfaces,
//!   and devices. It abstracts the initialization and reinitialization process of a GPU context.
//! - [`SurfaceId`]: A unique identifier for distinguishing GPU surfaces.
//! - [`SurfaceWrapper`]: Wraps a WGPU surface to provide additional utility methods and
//!   ensures safe management of the underlying surface.
//! - [`DeviceAndQueue`]: Combines the GPU device, its command queue, and configuration for easy access.
//!
//! # Main Features
//!
//! - **Surface Management**: Easy creation and reinitialization of GPU surfaces.
//! - **Device and Adapter Management**: Requesting a suitable adapter and configuring a GPU device
//!   for rendering tasks based on the surface's capabilities.
//! - **Error Handling**: Provides error enumeration (`WgpuReinitializeWindowError`)
//!   for better handling of common errors during initialization or reinitialization.

pub mod resource;
pub mod resources;

use crate::wgpu::wgpu_hardware::resources::WgpuResources;
use std::sync::Arc;
use tracing::{debug, instrument, warn};
use wgpu::wgt::DeviceDescriptor;
use wgpu::{Backends, Features, PowerPreference, RequestAdapterError, RequestDeviceError};
use wgpu_resource_manager::{
    wgpu_label, DeviceAndQueue, PixelSize, PixelSizeError, SurfaceWrapper, WgpuResourceManager,
    WgpuResourceManagerInitializeSurfaceError, WgpuResourceManagerResizeSurfaceError,
    NO_SURFACE_CONFIG,
};
use winit::dpi::PhysicalSize;
use winit::window::Window;

#[derive(Debug, Default)]
pub struct WgpuHardware {
    manager: WgpuResourceManager,
    resource: Option<WgpuResources>,
}

impl WgpuHardware {
    /// Initializes a new `WgpuHardware` instance with the specified graphics backends.
    ///
    /// # Parameters
    /// - `backends`: Specifies which backends to use for rendering (e.g., Vulkan, OpenGL, DirectX).
    ///
    /// # Returns
    /// A `WgpuHardware` instance configured with the given backends.
    ///
    /// # Note
    /// - There's a known issue with the Vulkan backend on NVIDIA + Wayland, which might cause
    ///   the application to hang. If you encounter this, consider setting the environment
    ///   variable `WGPU_BACKEND=gl` to switch to the OpenGL backend.
    #[instrument(skip_all)]
    pub fn initialize(backends: Backends) -> WgpuHardware {
        let manager = WgpuResourceManager::from_backends(backends);

        let resources = Self::try_create_resources(&manager);
        WgpuHardware {
            manager,
            resource: resources,
        }
    }

    #[instrument(skip_all)]
    fn try_create_resources(manager: &WgpuResourceManager) -> Option<WgpuResources> {
        if let Some(dq) = manager.device_and_queue() {
            if let Some(surface) = manager.surface() {
                if let Some(config) = surface.config() {
                    return Some(WgpuResources::new(dq.device(), dq.queue(), config.clone()));
                }
            }
        }
        None
    }

    /// Get access to the device, if configured.
    #[inline]
    pub const fn device(&self) -> Option<&DeviceAndQueue> {
        self.manager.device_and_queue()
    }

    /// Get access to the resources, if configured.
    #[inline]
    pub const fn resources(&self) -> Option<&WgpuResources> {
        self.resource.as_ref()
    }

    #[instrument(skip(self))]
    pub fn for_rendering(
        &mut self,
    ) -> Result<(&DeviceAndQueue, &mut WgpuResources), WgpuForRenderingError> {
        let Some(hardware) = self.manager.device_and_queue() else {
            debug!("Device is not ready yet, cannot render");
            return Err(WgpuForRenderingError::NoDevice);
        };

        let Some(resources) = self.resource.as_mut() else {
            debug!("Resources are not ready yet, cannot render");
            return Err(WgpuForRenderingError::NoResources);
        };

        Ok((hardware, resources))
    }

    #[inline]
    #[allow(dead_code)]
    pub const fn is_ready(&self) -> bool {
        self.manager.is_ready() && self.resource.is_some()
    }

    #[inline]
    pub const fn surface(&self) -> Option<&SurfaceWrapper> {
        self.manager.surface()
    }

    /// Indicates whether wireframe rendering is supported by the adapter.
    #[inline]
    pub fn wireframe_supported(&self) -> bool {
        self.manager
            .feature_supported(wgpu::Features::POLYGON_MODE_LINE)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum WgpuReinitializeWindowError {
    #[error("Failed to create GPU surface: {0}")]
    CreateSurface(#[from] wgpu::CreateSurfaceError),
    #[error("No fitting GPU adapter found")]
    NoAdapterFound(#[from] RequestAdapterError),
    #[error("Failed to request device: {0}")]
    NoDevice(#[from] RequestDeviceError),
}

#[derive(Debug, thiserror::Error)]
pub enum WgpuResizeSurfaceError {
    #[error("Invalid surface size")]
    InvalidSize(PhysicalSize<u32>),
    #[error(transparent)]
    Inner(#[from] WgpuResourceManagerResizeSurfaceError),
    #[error("Hardware is not configured")]
    NoHardware,
}

#[derive(Debug, thiserror::Error)]
pub enum WgpuForRenderingError {
    #[error("No device available")]
    NoDevice,
    #[error("No resources available")]
    NoResources,
}

#[derive(Debug, thiserror::Error)]
pub enum WgpuHardwareInitializeSurfaceError {
    #[error("Invalid surface size: {0:?}")]
    InvalidSize(#[from] PixelSizeError),
    #[error(transparent)]
    ResourceManager(#[from] WgpuResourceManagerInitializeSurfaceError),
    #[error("Hardware is not configured")]
    NoHardware,
}

impl WgpuHardware {
    #[instrument(skip_all)]
    pub fn reinitialize_surface(
        &mut self,
        window: Arc<Window>,
    ) -> Result<(), WgpuHardwareInitializeSurfaceError> {
        let size = window.inner_size();
        debug!(width = %size.width, height = %size.height, "Creating WGPU surface");
        let size = PixelSize::try_from(size)?;

        pollster::block_on(self.manager.initialize_and_configure_surface(
            window,
            Some(PowerPreference::HighPerformance),
            Some(DeviceDescriptor {
                label: Some(wgpu_label!("device_descriptor")),
                required_features: Features::POLYGON_MODE_LINE,
                required_limits: Default::default(),
                memory_hints: Default::default(),
                trace: Default::default(),
            }),
            size,
            NO_SURFACE_CONFIG,
        ))?;

        let Some(device) = self.manager.device_and_queue() else {
            return Err(WgpuHardwareInitializeSurfaceError::NoHardware);
        };

        if let Some(resources) = self.resource.as_mut() {
            resources.recreate_surface_dependent_buffers(device.device());
        } else {
            self.resource = Self::try_create_resources(&self.manager);
        }

        Ok(())
    }

    #[instrument(skip_all)]
    pub fn resize_surface(
        &mut self,
        new_size: PhysicalSize<u32>,
    ) -> Result<(), WgpuResizeSurfaceError> {
        let size = PixelSize::try_from(new_size)
            .map_err(|_| WgpuResizeSurfaceError::InvalidSize(new_size))?;
        self.manager.resize_surface(size)?;
        Ok(())
    }
}
