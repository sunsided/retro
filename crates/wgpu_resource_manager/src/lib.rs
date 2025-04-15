mod adapter_device_and_queue;
mod device_and_queue;
mod manager_id;
mod pixel_size;
mod resource_manager_traits;
mod surface_id;
mod surface_wrapper;

pub use self::adapter_device_and_queue::AdapterDeviceAndQueue;
pub use self::device_and_queue::DeviceAndQueue;
pub use self::manager_id::WgpuResourceManagerId;
pub use self::pixel_size::*;
pub use self::surface_id::SurfaceId;
pub use self::surface_wrapper::{ResizeSurfaceError, SurfaceWrapper};
use std::borrow::Borrow;
use std::sync::{Arc, LazyLock};
use wgpu::{
    Adapter, AdapterInfo, Backend, BackendOptions, Backends, DeviceType, Instance,
    InstanceDescriptor, InstanceFlags, PowerPreference, SurfaceCapabilities, SurfaceConfiguration,
    SurfaceTarget,
};

#[macro_export]
macro_rules! wgpu_label {
    () => {
        concat!(file!(), ":", line!())
    };
    ($name:literal) => {
        concat!($name, " @ ", file!(), ":", line!())
    };
    ($name:expr) => {
        format!(
            "{name} @ {file}:{line}",
            name = $name,
            file = file!(),
            line = line!()
        )
        .into_boxed_str()
    };
}

#[derive(Debug)]
pub struct WgpuResourceManager {
    id: WgpuResourceManagerId,
    instance: Instance,
    surface_wrapper: Option<SurfaceWrapper>,
    adq: Option<AdapterDeviceAndQueue>,
}

impl WgpuResourceManager {
    pub fn new() -> Self {
        Self::from_descriptor(InstanceDescriptor::default())
    }

    pub fn from_backends(backends: Backends) -> Self {
        let descriptor = InstanceDescriptor {
            backends,
            flags: InstanceFlags::from_env_or_default(),
            backend_options: BackendOptions::from_env_or_default(),
        };
        Self::from_descriptor(descriptor)
    }

    pub fn from_descriptor<D>(descriptor: D) -> Self
    where
        D: Borrow<InstanceDescriptor>,
    {
        let instance = Instance::new(descriptor.borrow());
        Self::from_instance(instance)
    }

    #[inline]
    #[cfg_attr(feature = "tracing_attributes", tracing::instrument(skip_all))]
    pub fn from_instance(instance: Instance) -> Self {
        let adapters = instance.enumerate_adapters(Backends::all());
        if adapters.is_empty() {
            tracing::warn!("No compatible adapters found!");
        } else {
            for (idx, adapter) in adapters.iter().enumerate() {
                log_available_adapter_info(idx, &adapter.get_info())
            }
        }

        Self {
            id: WgpuResourceManagerId::next(),
            instance,
            surface_wrapper: None,
            adq: None,
        }
    }

    #[cfg_attr(feature = "tracing_attributes", tracing::instrument(skip_all))]
    pub async fn initialize_and_configure_surface<W, S>(
        &mut self,
        window: Arc<W>,
        power_preference: Option<PowerPreference>,
        device_descriptor: Option<wgpu::DeviceDescriptor<'_>>,
        surface_size: PixelSize,
        surface_config: Option<S>,
    ) -> Result<(), WgpuResourceManagerInitializeSurfaceError>
    where
        Arc<W>: Into<SurfaceTarget<'static>>,
        S: FnOnce(PixelSize, SurfaceCapabilities) -> Option<SurfaceConfiguration>,
    {
        let surface = self
            .instance
            .create_surface(window)
            .inspect_err(|error| tracing::error!(%error, "Failed to create surface"))?;
        let surface_id = SurfaceId::next();
        let mut surface_wrapper = SurfaceWrapper::new(surface_id, surface);

        // Test whether we already have an adapter, and if we do, whether it is compatible
        // with the new surface.
        let surface_caps = self
            .adq
            .as_ref()
            .map(|adq| adq.adapter())
            .map(|adapter| surface_wrapper.surface().get_capabilities(&adapter))
            .unwrap_or_default();
        let adapter_exists_and_is_compatible = !surface_caps.formats.is_empty();

        if adapter_exists_and_is_compatible {
            tracing::debug!("Adapter already exists and is compatible with the new surface");
            let adq = self
                .adq
                .as_ref()
                .expect("Adapter and Device are known to exist");
            configure_surface(surface_size, surface_config, &mut surface_wrapper, &adq);

            self.surface_wrapper = Some(surface_wrapper);
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
            .inspect_err(|error| tracing::error!(%error, "Failed to request adapter"))?;

        log_selected_adapter_info(&adapter.get_info());

        let label = LazyLock::new(|| {
            wgpu_label!(format!("initialize_surface (Manager {id})", id = self.id))
        });
        let descriptor = device_descriptor.unwrap_or_else(|| wgpu::DeviceDescriptor {
            label: Some(&label),
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            memory_hints: Default::default(),
            trace: Default::default(),
        });
        let (device, queue) = adapter.request_device(&descriptor).await
            .inspect_err(|error| tracing::error!(%error, manager_id = %self.id(), surface_id = %surface_id, "Failed to request device"))?;

        let adq = AdapterDeviceAndQueue::new_from(adapter, device, queue);
        configure_surface(surface_size, surface_config, &mut surface_wrapper, &adq);

        self.adq = Some(adq);
        self.surface_wrapper = Some(surface_wrapper);
        Ok(())
    }

    #[cfg_attr(feature = "tracing_attributes", tracing::instrument(skip_all))]
    pub fn resize_surface(
        &mut self,
        new_size: PixelSize,
    ) -> Result<(), WgpuResourceManagerResizeSurfaceError> {
        let Some(surface) = self.surface_wrapper.as_mut() else {
            return Err(WgpuResourceManagerResizeSurfaceError::NoSurface);
        };

        let Some(adq) = self.adq.as_mut() else {
            return Err(WgpuResourceManagerResizeSurfaceError::NoDevice);
        };

        surface.resize_surface(adq.device(), new_size)?;
        Ok(())
    }
}

impl WgpuResourceManager {
    #[inline]
    pub const fn id(&self) -> WgpuResourceManagerId {
        self.id
    }

    #[inline]
    pub const fn instance(&self) -> &Instance {
        &self.instance
    }

    #[inline]
    pub const fn instance_mut(&mut self) -> &mut Instance {
        &mut self.instance
    }

    #[inline]
    pub const fn adapter(&self) -> Option<&Adapter> {
        match self.adq.as_ref() {
            None => None,
            Some(adq) => Some(adq.adapter()),
        }
    }

    #[inline]
    pub const fn device_and_queue(&self) -> Option<&DeviceAndQueue> {
        match self.adq.as_ref() {
            None => None,
            Some(adq) => Some(adq.device_and_queue()),
        }
    }

    #[inline]
    pub const fn surface(&self) -> Option<&SurfaceWrapper> {
        self.surface_wrapper.as_ref()
    }

    #[inline]
    pub const fn is_ready(&self) -> bool {
        self.adq.is_some() && self.surface_wrapper.is_some()
    }
}

impl WgpuResourceManager {
    /// Indicates whether a specific feature is supported by the adapter.
    #[inline]
    pub fn feature_supported(&self, feature: wgpu::Features) -> bool {
        self.adapter()
            .as_ref()
            .map(|adapter| adapter.features().contains(feature))
            .unwrap_or_default()
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
    adq: &AdapterDeviceAndQueue,
) where
    S: FnOnce(PixelSize, SurfaceCapabilities) -> Option<SurfaceConfiguration>,
{
    // Configure the surface if possible.
    let surface_config = match surface_config {
        None => {
            SurfaceWrapper::default_surface_config(surface_size, surface.surface(), adq.adapter())
        }
        Some(factory) => {
            let surface_caps = surface.get_capabilities(adq.adapter());
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

    surface.configure_surface(adq.device(), surface_config);
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

impl Default for WgpuResourceManager {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

fn log_available_adapter_info(idx: usize, info: &AdapterInfo) {
    if !tracing::enabled!(tracing::Level::DEBUG) {
        return;
    }

    tracing::debug!(
        "Adapter {idx} ({device_type}): {name} ({vendor}:{device_id}); Driver: {driver_info}, Backend: {backend}",
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

#[cfg(test)]
fn init_test_tracing() {
    static INIT: std::sync::Once = std::sync::Once::new();

    INIT.call_once(|| {
        tracing_subscriber::fmt()
            .with_env_filter("wgpu_resource_manager=trace")
            .with_test_writer()
            .init();
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enumerate_adapters() {
        init_test_tracing();

        let man = WgpuResourceManager::new();
        let adapters = man.instance().enumerate_adapters(Backends::all());
        assert!(!adapters.is_empty(), "No adapters found!");
    }
}
