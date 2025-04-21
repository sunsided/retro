use crate::configured_resource_manager::WgpuResourceManagerInitializeSurfaceError;
use crate::{ConfiguredWgpuResourceManager, DeviceAndQueue, DeviceId, InstanceWrapper, PixelSize, SurfaceId, SurfaceWrapper, WgpuResourceManagerId, WgpuResourceManagerRef, WgpuResourceManagerResizeSurfaceError};
use std::sync::Arc;
use wgpu::{
    Adapter, Instance, PowerPreference, SurfaceCapabilities, SurfaceConfiguration, SurfaceTarget,
};

/// A wrapper around a potentially configured [`ConfiguredWgpuResourceManager`].
#[derive(Debug)]
pub struct WgpuResourceManager {
    id: WgpuResourceManagerId,
    instance: InstanceWrapper,
    manager: Option<ConfiguredWgpuResourceManager>,
}

impl WgpuResourceManager {
    #[inline]
    pub fn unconfigured<I>(instance: I) -> Self
    where
        I: Into<InstanceWrapper>,
    {
        Self {
            id: WgpuResourceManagerId::next(),
            instance: instance.into(),
            manager: None,
        }
    }

    #[cfg_attr(feature = "tracing_attributes", tracing::instrument(skip_all))]
    pub async fn initialize_and_configure<W, S>(
        &mut self,
        window: Arc<W>,
        power_preference: Option<PowerPreference>,
        device_descriptor: Option<wgpu::DeviceDescriptor<'_>>,
        surface_size: PixelSize,
        surface_config: Option<S>,
    ) -> Result<&Self, WgpuResourceManagerInitializeSurfaceError>
    where
        Arc<W>: Into<SurfaceTarget<'static>>,
        S: FnOnce(PixelSize, SurfaceCapabilities) -> Option<SurfaceConfiguration>,
    {
        // Actively free the previous resources first.
        self.manager.take();

        if let Some(manager) = &mut self.manager {
            manager
                .reinitialize_surface(
                    window,
                    power_preference,
                    device_descriptor,
                    surface_size,
                    surface_config,
                )
                .await?;
            return Ok(self);
        }

        let manager = ConfiguredWgpuResourceManager::initialize_and_configure_surface(
            self.id,
            self.instance.clone(),
            window,
            power_preference,
            device_descriptor,
            surface_size,
            surface_config,
        )
        .await?;

        self.manager = Some(manager);
        Ok(self)
    }

    #[cfg_attr(feature = "tracing_attributes", tracing::instrument(skip_all))]
    pub fn resize_surface(
        &mut self,
        new_size: PixelSize,
    ) -> Result<(), WgpuResourceManagerResizeSurfaceError> {
        if let Some(manager) = &mut self.manager {
            return manager.resize_surface(new_size);
        }

        // TODO: Use the correct error type.
        Err(WgpuResourceManagerResizeSurfaceError::NoDevice)
    }

    #[inline]
    pub const fn as_ref(&self) -> Option<&ConfiguredWgpuResourceManager> {
        self.manager.as_ref()
    }

    #[inline]
    pub const fn as_mut(&mut self) -> Option<&mut ConfiguredWgpuResourceManager> {
        self.manager.as_mut()
    }

    /// Indicates whether the adapter supports a specific feature.
    #[inline]
    pub fn feature_supported(&self, feature: wgpu::Features) -> bool {
        match self.manager.as_ref() {
            Some(manager) => manager.feature_supported(feature),
            None => false,
        }
    }

    /// Gets the currently configured device ID, or [`DeviceId::default`](DeviceId::default) if none exists.
    pub const fn device_id(&self) -> DeviceId {
        match &self.manager {
            None => DeviceId::default(),
            Some(manager) => manager.device_id(),
        }
    }

    /// Gets the currently configured surface ID, or [`SurfaceId::default`](SurfaceId::default) if none exists.
    pub const fn surface_id(&self) -> SurfaceId {
        match &self.manager {
            None => SurfaceId::default(),
            Some(manager) => manager.surface_id(),
        }
    }
}

impl WgpuResourceManager {
    #[inline]
    pub fn id(&self) -> WgpuResourceManagerId {
        self.id
    }

    #[inline]
    pub const fn instance(&self) -> Option<&Instance> {
        match self.manager.as_ref() {
            Some(manager) => Some(manager.instance()),
            None => None,
        }
    }

    #[inline]
    pub const fn instance_mut(&mut self) -> Option<&mut Instance> {
        match self.manager.as_mut() {
            Some(manager) => Some(manager.instance_mut()),
            None => None,
        }
    }

    #[inline]
    pub const fn adapter(&self) -> Option<&Adapter> {
        match self.manager.as_ref() {
            Some(manager) => Some(manager.adapter()),
            None => None,
        }
    }

    #[inline]
    pub const fn device_and_queue(&self) -> Option<&DeviceAndQueue> {
        match self.manager.as_ref() {
            Some(manager) => Some(manager.device_and_queue()),
            None => None,
        }
    }

    #[inline]
    pub const fn surface(&self) -> Option<&SurfaceWrapper> {
        match self.manager.as_ref() {
            Some(manager) => Some(manager.surface()),
            None => None,
        }
    }

    #[inline]
    pub const fn is_ready(&self) -> bool {
        match self.manager.as_ref() {
            Some(manager) => manager.is_ready(),
            None => false,
        }
    }
}

impl WgpuResourceManagerRef for WgpuResourceManager {
    fn id(&self) -> WgpuResourceManagerId {
        Self::id(self)
    }

    fn instance(&self) -> Option<&Instance> {
        Self::instance(self)
    }

    fn instance_mut(&mut self) -> Option<&mut Instance> {
        Self::instance_mut(self)
    }

    fn adapter(&self) -> Option<&Adapter> {
        Self::adapter(self)
    }

    fn device_and_queue(&self) -> Option<&DeviceAndQueue> {
        Self::device_and_queue(self)
    }

    fn surface(&self) -> Option<&SurfaceWrapper> {
        Self::surface(self)
    }

    fn is_ready(&self) -> bool {
        Self::is_ready(self)
    }
}
