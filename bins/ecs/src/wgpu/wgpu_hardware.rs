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

mod device_and_queue;
pub mod resource;
pub mod resources;
mod surface_id;
mod surface_wrapper;

pub use self::device_and_queue::DeviceAndQueue;
pub use self::surface_id::SurfaceId;
use crate::wgpu::wgpu_hardware::resources::WgpuResources;
use crate::wgpu::wgpu_hardware::surface_wrapper::SurfaceWrapper;
use std::sync::Arc;
use tracing::{debug, info, instrument, warn};
use wgpu::{
    Adapter, Backends, Device, DeviceType, InstanceDescriptor, PresentMode, Queue,
    RequestAdapterError, RequestDeviceError, Surface, SurfaceConfiguration, TextureUsages,
};
use winit::dpi::PhysicalSize;
use winit::window::Window;

#[derive(Debug, Default)]
pub struct WgpuHardware {
    instance: wgpu::Instance,
    surface: SurfaceWrapper,
    adapter: Option<Adapter>,
    device: Option<DeviceAndQueue>,
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
    pub fn initialize(backends: Backends) -> WgpuHardware {
        debug!("Creating WGPU instance with backend: {backends:?}");
        let descriptor = InstanceDescriptor {
            backends,
            flags: Default::default(),
            backend_options: Default::default(),
        };
        let instance = wgpu::Instance::new(&descriptor);
        WgpuHardware {
            instance,
            ..Default::default()
        }
    }

    /// Get access to the device, if configured.
    #[inline]
    pub const fn device(&self) -> Option<&DeviceAndQueue> {
        self.device.as_ref()
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
        let Some(hardware) = self.device.as_ref() else {
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
        self.device.is_some()
    }

    #[inline]
    pub const fn surface(&self) -> &SurfaceWrapper {
        &self.surface
    }

    /// Indicates whether wireframe rendering is supported by the adapter.
    #[inline]
    pub fn wireframe_supported(&self) -> bool {
        self.adapter
            .as_ref()
            .map(|adapter| {
                adapter
                    .features()
                    .contains(wgpu::Features::POLYGON_MODE_LINE)
            })
            .unwrap_or_default()
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
    #[error("No surface available to resize")]
    NoSurface,
    #[error("No device available to resize the surface with")]
    NoDevice,
    #[error("No resources available to resize the surface for")]
    NoResources,
}

#[derive(Debug, thiserror::Error)]
pub enum WgpuForRenderingError {
    #[error("No device available")]
    NoDevice,
    #[error("No resources available")]
    NoResources,
}

impl WgpuHardware {
    #[instrument(skip(self, window))]
    pub fn reinitialize_surface(
        &mut self,
        window: Arc<Window>,
    ) -> Result<(), WgpuReinitializeWindowError> {
        let size = window.inner_size();
        debug!(width = %size.width, height = %size.height, "Creating WGPU surface");
        self.surface.reset();
        let surface = self
            .instance
            .create_surface(window)
            .inspect_err(|error| tracing::error!(%error, "Failed to create surface"))?;
        self.surface = SurfaceWrapper::from(surface);

        let Some(surface) = self.surface.as_ref() else {
            unreachable!("A surface that was just created should always be available");
        };

        // If we already have an adapter, and it is compatible with the new surface, skip creation.
        let surface_caps = self
            .adapter
            .as_ref()
            .map(|adapter| surface.get_capabilities(&adapter))
            .unwrap_or_default();
        let recreate_device = surface_caps.formats.is_empty();
        if recreate_device {
            let adapter = self.request_adapter(surface)?;
            let (device, queue) = self.request_device(&adapter)?;
            let config = Self::get_surface_config(size, surface, &adapter);
            let resources = WgpuResources::new(&device, &queue, &config);

            self.adapter = Some(adapter);
            self.device = Some(DeviceAndQueue::new(device, queue, config));
            self.resource = Some(resources);
        } else {
            let Some(adapter) = self.adapter.as_mut() else {
                unreachable!("The adapter is known to exist");
            };

            let Some(device) = self.device.as_mut() else {
                unreachable!("Since the adapter existed, the surface also must exist");
            };

            device.set_config(Self::get_surface_config(size, surface, &adapter));
        }
        Ok(())
    }

    pub fn resize_surface(
        &mut self,
        new_size: PhysicalSize<u32>,
    ) -> Result<(), WgpuResizeSurfaceError> {
        if new_size.width <= 0 || new_size.height <= 0 {
            return Err(WgpuResizeSurfaceError::InvalidSize(new_size));
        }

        let Some(mut surface) = self.surface.as_mut() else {
            return Err(WgpuResizeSurfaceError::NoSurface);
        };

        let Some(device) = self.device.as_mut() else {
            return Err(WgpuResizeSurfaceError::NoDevice);
        };

        device.resize_surface(new_size, &mut surface);

        let Some(resources) = self.resource.as_mut() else {
            return Err(WgpuResizeSurfaceError::NoResources);
        };

        resources.recreate_surface_dependent_buffers(device.device(), device.config());

        Ok(())
    }

    #[instrument(skip(self, surface))]
    fn request_adapter(&self, surface: &Surface) -> Result<Adapter, WgpuReinitializeWindowError> {
        debug!(surface_id = %self.surface.id(), "Requesting GPU adapter due to surface change");
        let adapter = pollster::block_on(self.instance
            .request_adapter(
                &wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::HighPerformance, // or LowPower (e.g., iGPU)
                    compatible_surface: Some(&surface),
                    force_fallback_adapter: false,
                },
            ))
            .inspect_err(|error| tracing::error!(%error, surface_id = %self.surface.id(), "Failed to request adapter"))
            .map_err( WgpuReinitializeWindowError::NoAdapterFound)?;

        let info = adapter.get_info();
        match info.device_type {
            DeviceType::Other => warn!(
                "Adapter is of unsupported type: {name} ({vendor}, {driver})",
                name = info.name,
                vendor = info.vendor,
                driver = info.driver_info
            ),
            DeviceType::IntegratedGpu => info!(
                "Running on an integrated GPU: {name} ({vendor}, {driver})",
                name = info.name,
                vendor = info.vendor,
                driver = info.driver_info
            ),
            DeviceType::DiscreteGpu => info!(
                "Running on a discrete GPU: {name} ({vendor}, {driver})",
                name = info.name,
                vendor = info.vendor,
                driver = info.driver_info
            ),
            DeviceType::VirtualGpu => info!(
                "Running on a virtual GPU: {name} ({vendor}, {driver})",
                name = info.name,
                vendor = info.vendor,
                driver = info.driver_info
            ),
            DeviceType::Cpu => warn!(
                "Running using software renderer: {name} ({vendor}, {driver})",
                name = info.name,
                vendor = info.vendor,
                driver = info.driver_info
            ),
        }

        Ok(adapter)
    }

    #[instrument(skip(self, adapter))]
    fn request_device(
        &self,
        adapter: &Adapter,
    ) -> Result<(Device, Queue), WgpuReinitializeWindowError> {
        debug!(surface_id = %self.surface.id(), "Requesting GPU device and queue due to surface change");

        // Enable wireframe rendering support.
        let mut required_features = wgpu::Features::empty();
        if adapter
            .features()
            .contains(wgpu::Features::POLYGON_MODE_LINE)
        {
            debug!("Adapter supports wireframe rendering");
            required_features |= wgpu::Features::POLYGON_MODE_LINE;
        } else {
            debug!(
                "Adapter does not support wireframe rendering, falling back to empty feature set"
            );
        };

        let (device, queue) = pollster::block_on(adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Device Descriptor"),
                    required_features,
                    required_limits: wgpu::Limits::default(),
                    memory_hints: Default::default(),
                    trace: Default::default(),
                }
            ))
            .inspect_err(|error| tracing::error!(%error, surface_id = %self.surface.id(), "Failed to request device"))?;
        Ok((device, queue))
    }

    #[instrument(skip_all)]
    fn get_surface_config(
        size: PhysicalSize<u32>,
        surface: &Surface,
        adapter: &Adapter,
    ) -> SurfaceConfiguration {
        let surface_caps = surface.get_capabilities(&adapter);

        // Pick sRGB format (or the next-best one).
        let surface_format = surface_caps
            .formats
            .iter()
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
