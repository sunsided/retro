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


use std::fmt::Display;
use std::sync::Arc;
use derive_more::AsRef;
use tracing::{debug, instrument, warn};
use wgpu::{Adapter, Backends, Device, InstanceDescriptor, PresentMode, Queue, RequestDeviceError, Surface, SurfaceCapabilities, SurfaceConfiguration, TextureUsages};
use wgpu::hal::DynSurface;
use winit::dpi::PhysicalSize;
use winit::window::Window;

#[derive(Debug, Default)]
pub struct WgpuHardware {
    instance: wgpu::Instance,
    surface: SurfaceWrapper,
    adapter: Option<Adapter>,
    device: Option<DeviceAndQueue>,
}

/// The ID of a WGPU surface.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SurfaceId(u64);

impl SurfaceId {
    /// Generates the next unique `SurfaceId`.
    ///
    /// This function uses an atomic counter to ensure that each generated
    /// `SurfaceId` is unique across threads.
    ///
    /// # Returns
    /// A new `SurfaceId` with a unique, incremented value.
    pub fn next() -> Self {
        static NEXT_ID: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
        SurfaceId(NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst))
    }

    /// Checks if the `SurfaceId` is uninitialized or has a default value.
    ///
    /// # Returns
    /// - `true` if the `SurfaceId` is 0 (uninitialized), otherwise `false`.
    pub fn is_uninitialized(&self) -> bool {
        self.0 == 0
    }
}

impl Display for SurfaceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

/// A wrapper for a WGPU surface.
#[derive(Debug, Default)]
pub struct SurfaceWrapper {
    id: SurfaceId,
    surface: Option<Surface<'static>>,
}

impl SurfaceWrapper {
    pub fn reset(&mut self) {
        *self = Default::default()
    }

    pub const fn id(&self) -> SurfaceId {
        self.id
    }

    pub const fn as_ref(&self) -> Option<&Surface<'static>> {
        self.surface.as_ref()
    }

    pub const fn as_mut(&mut self) -> Option<&mut Surface<'static>> {
        self.surface.as_mut()
    }
}

impl From<Surface<'static>> for SurfaceWrapper {
    fn from(value: Surface<'static>) -> Self {
        Self {
            id: SurfaceId::next(),
            surface: Some(value),
            ..Default::default()
        }
    }
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
    pub fn initialize(backends: Backends) -> WgpuHardware {
        debug!("Creating WGPU instance with backend: {backends:?}");
        let descriptor = InstanceDescriptor {
            backends,
            flags: Default::default(),
            backend_options: Default::default(),
        };
        let instance = wgpu::Instance::new(&descriptor);
        WgpuHardware { instance, ..Default::default() }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum WgpuReinitializeWindowError {
    #[error("Failed to create GPU surface: {0}")]
    CreateSurface(#[from] wgpu::CreateSurfaceError),
    #[error("No fitting GPU adapter found")]
    NoAdapterFound,
    #[error("Failed to request device: {0}")]
    NoDevice(#[from] RequestDeviceError)
}

#[derive(Debug, AsRef)]
pub struct DeviceAndQueue {
    pub device: Device,
    pub queue: Queue,
    pub config: SurfaceConfiguration,
}

impl WgpuHardware {
    #[instrument(skip(self, window))]
    pub fn reinitialize_surface(&mut self, window: Arc<Window>) -> Result<(), WgpuReinitializeWindowError> {
        let size = window.inner_size();
        debug!(width = %size.width, height = %size.height, "Creating WGPU surface");
        self.surface.reset();
        let surface = self.instance.create_surface(window)
            .inspect_err(|error| tracing::error!(%error, "Failed to create surface"))?;
        self.surface = SurfaceWrapper::from(surface);

        let Some(surface) = self.surface.as_ref() else {
            unreachable!("A surface that was just created should always be available");
        };

        // If we already have an adapter, and it is compatible with the new surface, skip creation.
        let mut surface_caps = self.adapter.as_ref()
            .map(|adapter| surface.get_capabilities(&adapter))
            .unwrap_or_default();
        let recreate_device = surface_caps.formats.is_empty();
        if recreate_device {
            let adapter = self.request_adapter(surface)?;
            let (device, queue) = self.request_device(&adapter)?;
            let config = Self::get_surface_config(size, surface, &adapter);

            self.adapter = Some(adapter);
            self.device = Some(DeviceAndQueue {
                device,
                queue,
                config
            });
        } else {
            let Some(adapter) = self.adapter.as_mut() else {
                unreachable!("The adapter is known to exist");
            };

            let Some(device) = self.device.as_mut() else {
                unreachable!("Since the adapter existed, the surface also must exist");
            };

            device.config = Self::get_surface_config(size, surface, &adapter);
        }
        Ok(())
    }

    #[instrument(skip(self, surface))]
    fn request_adapter(&self, surface: &Surface) -> Result<Adapter, WgpuReinitializeWindowError> {
        debug!(surface_id = %self.surface.id, "Requesting GPU adapter due to surface change");
        let adapter = pollster::block_on(self.instance
            .request_adapter(
                &wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::HighPerformance, // or LowPower (e.g., iGPU)
                    compatible_surface: Some(&surface),
                    force_fallback_adapter: false,
                },
            ))
            .ok_or(WgpuReinitializeWindowError::NoAdapterFound)
            .inspect_err(|error| tracing::error!(%error, surface_id = %self.surface.id, "Failed to request adapter"))?;

        Ok(adapter)
    }

    #[instrument(skip(self, adapter))]
    fn request_device(&self, adapter: &Adapter) -> Result<(Device, Queue), WgpuReinitializeWindowError> {
        debug!(surface_id = %self.surface.id, "Requesting GPU device and queue due to surface change");
        let (device, queue) = pollster::block_on(adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Device Descriptor"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    memory_hints: Default::default(),
                },
                None,
            ))
            .inspect_err(|error| tracing::error!(%error, surface_id = %self.surface.id, "Failed to request device"))?;
        Ok((device, queue))
    }

    #[instrument(skip_all)]
    fn get_surface_config(size: PhysicalSize<u32>, surface: &Surface, adapter: &Adapter) -> SurfaceConfiguration {
        let surface_caps = surface.get_capabilities(&adapter);

        // Pick sRGB format (or the next-best one).
        let surface_format = surface_caps.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);
        if surface_format.is_srgb() {
            debug!("Using surface format {surface_format:?}");
        } else {
            warn!("Surface format {surface_format:?} is not sRGB, which might lead to color artifacts");
        }

        SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: PresentMode::Fifo, // surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        }
    }
}
