use std::borrow::Borrow;
use std::ops::{Deref, DerefMut};
use wgpu::{
    AdapterInfo, Backend, BackendOptions, Backends, DeviceType, Instance, InstanceDescriptor,
    InstanceFlags,
};

#[derive(Debug, Clone)]
pub struct InstanceWrapper {
    instance: Instance,
}

impl InstanceWrapper {
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

        Self { instance }
    }

    #[inline]
    pub const fn as_ref(&self) -> &Instance {
        &self.instance
    }

    #[inline]
    pub const fn as_mut(&mut self) -> &mut Instance {
        &mut self.instance
    }
}

impl Default for InstanceWrapper {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl AsRef<Instance> for InstanceWrapper {
    #[inline]
    fn as_ref(&self) -> &Instance {
        &self.instance
    }
}

impl AsMut<Instance> for InstanceWrapper {
    #[inline]
    fn as_mut(&mut self) -> &mut Instance {
        &mut self.instance
    }
}

impl Deref for InstanceWrapper {
    type Target = Instance;

    fn deref(&self) -> &Self::Target {
        &self.instance
    }
}

impl DerefMut for InstanceWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.instance
    }
}

impl From<Instance> for InstanceWrapper {
    #[inline]
    fn from(instance: Instance) -> Self {
        Self::from_instance(instance)
    }
}

impl From<Backends> for InstanceWrapper {
    #[inline]
    fn from(backends: Backends) -> Self {
        Self::from_backends(backends)
    }
}

impl From<InstanceDescriptor> for InstanceWrapper {
    #[inline]
    fn from(descriptor: InstanceDescriptor) -> Self {
        Self::from_descriptor(&descriptor)
    }
}

impl From<&InstanceDescriptor> for InstanceWrapper {
    #[inline]
    fn from(descriptor: &InstanceDescriptor) -> Self {
        Self::from_descriptor(descriptor)
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
