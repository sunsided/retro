use crate::{DeviceAndQueue, InstanceWrapper, PixelSize, ResizeSurfaceError, SurfaceId, SurfaceWrapper, WgpuResourceManagerId, wgpu_label, DeviceId};
use std::sync::Arc;
use wgpu::{
    Adapter, AdapterInfo, Backend, DeviceDescriptor, DeviceType, Instance, PowerPreference,
    SurfaceCapabilities, SurfaceConfiguration, SurfaceTarget,
};
use crate::resource_manager::accessor::WgpuResourceManagerAccessors;

/// A resource manager that is configured by a valid surface and non-zero surface size.
#[derive(Debug)]
#[allow(dead_code)]
pub struct ConfiguredWgpuResourceManager {
    manager_id: WgpuResourceManagerId,
    instance: InstanceWrapper,
    surface_wrapper: SurfaceWrapper,
    adapter: Adapter,
    dq: DeviceAndQueue,
}

impl ConfiguredWgpuResourceManager {
    #[cfg_attr(feature = "tracing_attributes", tracing::instrument(skip_all))]
    pub(crate) async fn initialize_and_configure_surface<I, W, S>(
        manager_id: WgpuResourceManagerId,
        instance: I,
        window: Arc<W>,
        power_preference: Option<PowerPreference>,
        device_descriptor: Option<wgpu::DeviceDescriptor<'_>>,
        surface_size: PixelSize,
        surface_config: Option<S>,
    ) -> Result<Self, WgpuResourceManagerInitializeSurfaceError>
    where
        I: Into<InstanceWrapper>,
        Arc<W>: Into<SurfaceTarget<'static>>,
        S: FnOnce(PixelSize, SurfaceCapabilities) -> Option<SurfaceConfiguration>,
    {
        let instance = instance.into();
        let surface = instance.create_surface(window).inspect_err(
            |error| tracing::error!(%error, %manager_id, "Failed to create surface"),
        )?;
        let surface_id = SurfaceId::next();
        let mut surface_wrapper = SurfaceWrapper::new(surface_id, surface);

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: power_preference
                    .or_else(PowerPreference::from_env)
                    .unwrap_or_default(),
                compatible_surface: Some(surface_wrapper.surface()),
                force_fallback_adapter: false,
            })
            .await
            .inspect_err(|error| tracing::error!(%error, %manager_id, %surface_id, "Failed to request adapter"))?;

        log_selected_adapter_info(&adapter.get_info());

        let descriptor = Self::get_descriptor(device_descriptor);
        let (device, queue) = adapter.request_device(&descriptor).await.inspect_err(
            |error| tracing::error!(%error, %manager_id, %surface_id, "Failed to request device"),
        )?;
        let device_id = DeviceId::next();

        let dq = DeviceAndQueue::new_from(device_id, device, queue);
        configure_surface(
            surface_size,
            surface_config,
            &mut surface_wrapper,
            &adapter,
            &dq,
        );

        Ok(Self {
            manager_id,
            instance,
            adapter,
            dq,
            surface_wrapper,
        })
    }

    #[cfg_attr(feature = "tracing_attributes", tracing::instrument(skip_all))]
    pub(crate) async fn reinitialize_surface<W, S>(
        &mut self,
        window: Arc<W>,
        power_preference: Option<PowerPreference>,
        device_descriptor: Option<DeviceDescriptor<'_>>,
        surface_size: PixelSize,
        surface_config: Option<S>,
    ) -> Result<(), WgpuResourceManagerInitializeSurfaceError>
    where
        Arc<W>: Into<SurfaceTarget<'static>>,
        S: FnOnce(PixelSize, SurfaceCapabilities) -> Option<SurfaceConfiguration>,
    {
        let surface = self.instance.create_surface(window).inspect_err(
            |error| tracing::error!(%error, manager_id = %self.manager_id(), "Failed to create surface"),
        )?;
        let surface_id = SurfaceId::next();
        let mut surface_wrapper = SurfaceWrapper::new(surface_id, surface);

        // Test whether we already have an adapter, and if we do, whether it is compatible
        // with the new surface.
        let surface_caps = surface_wrapper.surface().get_capabilities(self.adapter());
        let adapter_exists_and_is_compatible = !surface_caps.formats.is_empty();

        if adapter_exists_and_is_compatible {
            tracing::debug!(manager_id = %self.manager_id(), %surface_id, "Adapter already exists and is compatible with the new surface");
            configure_surface(
                surface_size,
                surface_config,
                &mut surface_wrapper,
                &self.adapter,
                &self.dq,
            );
            self.surface_wrapper = surface_wrapper;
            return Ok(());
        }

        let adapter = self
            .instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: power_preference
                    .or_else(PowerPreference::from_env)
                    .unwrap_or_default(),
                compatible_surface: Some(surface_wrapper.surface()),
                force_fallback_adapter: false,
            })
            .await
            .inspect_err(
                |error| tracing::error!(%error, manager_id = %self.manager_id(), %surface_id, "Failed to request adapter"),
            )?;

        log_selected_adapter_info(&adapter.get_info());

        let descriptor = Self::get_descriptor(device_descriptor);
        let (device, queue) = adapter.request_device(&descriptor).await
            .inspect_err(|error| tracing::error!(%error, manager_id = %self.manager_id(), %surface_id, "Failed to request device"))?;

        let device_id = DeviceId::next();
        let dq = DeviceAndQueue::new_from(device_id, device, queue);
        configure_surface(
            surface_size,
            surface_config,
            &mut surface_wrapper,
            &adapter,
            &dq,
        );

        self.dq = dq;
        self.surface_wrapper = surface_wrapper;
        Ok(())
    }

    fn get_descriptor(device_descriptor: Option<DeviceDescriptor>) -> DeviceDescriptor {
        device_descriptor.unwrap_or_else(|| DeviceDescriptor {
            label: Some(wgpu_label!("default_descriptor")),
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            memory_hints: Default::default(),
            trace: Default::default(),
        })
    }

    #[cfg_attr(feature = "tracing_attributes", tracing::instrument(skip_all))]
    pub fn resize_surface(
        &mut self,
        new_size: PixelSize,
    ) -> Result<(), WgpuResourceManagerResizeSurfaceError> {
        self.surface_wrapper
            .resize_surface(self.dq.device(), new_size)?;
        Ok(())
    }
}

impl WgpuResourceManagerAccessors for ConfiguredWgpuResourceManager {
    fn id(&self) -> WgpuResourceManagerId {
        Self::manager_id(self)
    }

    fn instance(&self) -> &Instance {
        Self::instance(self)
    }

    fn instance_mut(&mut self) -> &mut Instance {
        Self::instance_mut(self)
    }

    fn adapter(&self) -> &Adapter {
        Self::adapter(self)
    }

    fn device_and_queue(&self) -> &DeviceAndQueue {
        Self::device_and_queue(self)
    }

    fn surface(&self) -> &SurfaceWrapper {
        Self::surface(self)
    }

    fn is_ready(&self) -> bool {
        Self::is_ready(self)
    }
}

impl ConfiguredWgpuResourceManager {
    #[inline]
    pub fn manager_id(&self) -> WgpuResourceManagerId {
        self.manager_id
    }

    #[inline]
    pub const fn instance(&self) -> &Instance {
        self.instance.as_ref()
    }

    #[inline]
    pub const fn instance_mut(&mut self) -> &mut Instance {
        self.instance.as_mut()
    }

    #[inline]
    pub const fn adapter(&self) -> &Adapter {
        &self.adapter
    }

    #[inline]
    pub const fn device_and_queue(&self) -> &DeviceAndQueue {
        &self.dq
    }

    /// Gets the currently configured device ID, or [`DeviceId::default`](DeviceId::default) if none exists.
    #[inline]
    pub const fn device_id(&self) -> DeviceId {
        self.dq.id()
    }

    /// Gets the currently configured surface ID, or [`SurfaceId::default`](SurfaceId::default) if none exists.
    #[inline]
    pub const fn surface_id(&self) -> SurfaceId {
        self.surface_wrapper.id()
    }

    #[inline]
    pub const fn surface(&self) -> &SurfaceWrapper {
        &self.surface_wrapper
    }

    #[inline]
    pub const fn is_ready(&self) -> bool {
        self.surface_wrapper.configured()
    }

    /// Indicates whether a specific feature is supported by the adapter.
    #[inline]
    pub fn feature_supported(&self, feature: wgpu::Features) -> bool {
        self.adapter().features().contains(feature)
    }
}

pub const NO_SURFACE_CONFIG: Option<
    fn(PixelSize, SurfaceCapabilities) -> Option<SurfaceConfiguration>,
> = None;

#[cfg_attr(feature = "tracing_attributes", tracing::instrument(skip_all))]
fn configure_surface<S>(
    surface_size: PixelSize,
    surface_config: Option<S>,
    surface: &mut SurfaceWrapper,
    adapter: &Adapter,
    dq: &DeviceAndQueue,
) where
    S: FnOnce(PixelSize, SurfaceCapabilities) -> Option<SurfaceConfiguration>,
{
    // Configure the surface if possible.
    let surface_config = match surface_config {
        None => SurfaceWrapper::default_surface_config(surface_size, surface.surface(), &adapter),
        Some(factory) => {
            let surface_caps = surface.get_capabilities(&adapter);
            match factory(surface_size, surface_caps) {
                None => {
                    tracing::warn!(
                        "Surface configuration factory returned nothing; not configuring surface"
                    );
                    return;
                }
                Some(config) => config,
            }
        }
    };

    surface.configure_surface(dq.device(), surface_config);
}

#[derive(Debug, thiserror::Error)]
pub enum WgpuResourceManagerInitializeSurfaceError {
    #[error("Failed to create surface: {0}")]
    CreateSurface(#[from] wgpu::CreateSurfaceError),
    #[error("Failed to request adapter: {0}")]
    RequestAdapter(#[from] wgpu::RequestAdapterError),
    #[error("Failed to request device: {0}")]
    RequestDevice(#[from] wgpu::RequestDeviceError),
    #[error("Hardware is not configured")]
    NoHardware,
}

#[derive(Debug, thiserror::Error)]
pub enum WgpuResourceManagerResizeSurfaceError {
    #[error("No surface is available")]
    NoSurface,
    #[error("No device is available")]
    NoDevice,
    #[error("The surface is unconfigured")]
    Unconfigured(#[from] ResizeSurfaceError),
}

fn log_selected_adapter_info(info: &AdapterInfo) {
    if !tracing::enabled!(tracing::Level::INFO) {
        return;
    }

    tracing::info!(
        "Selected {device_type} adapter: {name} ({vendor}:{device_id}); Driver: {driver_info}, Backend: {backend}",
        name = info.name,
        vendor = info.vendor,
        device_id = info.device,
        driver_info = info.driver_info,
        device_type = match info.device_type {
            DeviceType::Other => "other",
            DeviceType::IntegratedGpu => "iGPU",
            DeviceType::DiscreteGpu => "dGPU",
            DeviceType::VirtualGpu => "vGPU",
            DeviceType::Cpu => "CPU",
        },
        backend = match info.backend {
            Backend::Noop => "no-op",
            Backend::Vulkan => "Vulkan",
            Backend::Metal => "Metal",
            Backend::Dx12 => "DX12",
            Backend::Gl => "GL",
            Backend::BrowserWebGpu => "WebGPU",
        },
    );
}
