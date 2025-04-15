use crate::SurfaceId;
use crate::pixel_size::PixelSize;
use std::borrow::{Borrow, BorrowMut};
use wgpu::{
    Adapter, Device, PresentMode, Surface, SurfaceCapabilities, SurfaceConfiguration, TextureUsages,
};

/// A wrapper for a WGPU surface.
#[derive(Debug)]
pub struct SurfaceWrapper {
    id: SurfaceId,
    surface: Surface<'static>,
    config: Option<SurfaceConfiguration>,
}

impl SurfaceWrapper {
    pub(super) fn new(id: SurfaceId, surface: Surface<'static>) -> Self {
        Self {
            id,
            surface,
            config: None,
        }
    }

    #[inline]
    pub const fn id(&self) -> SurfaceId {
        self.id
    }

    #[inline]
    pub const fn surface(&self) -> &Surface<'static> {
        &self.surface
    }

    #[inline]
    pub const fn surface_mut(&mut self) -> &mut Surface<'static> {
        &mut self.surface
    }

    #[inline]
    pub const fn config(&self) -> Option<&SurfaceConfiguration> {
        self.config.as_ref()
    }

    #[inline]
    pub const fn configured(&self) -> bool {
        self.config.is_some()
    }

    #[inline]
    pub fn configure_surface<C>(&mut self, device: &Device, config: C)
    where
        C: Borrow<SurfaceConfiguration> + Clone,
    {
        let config = config.borrow();
        self.surface.configure(device, &config);
        self.config = Some(config.clone());
    }

    #[inline]
    pub fn resize_surface(
        &mut self,
        device: &Device,
        size: PixelSize,
    ) -> Result<(), ResizeSurfaceError> {
        if let Some(config) = self.config.as_mut() {
            config.width = size.width();
            config.height = size.height();
            self.surface.configure(device, &config);
            Ok(())
        } else {
            Err(ResizeSurfaceError::SurfaceNotConfigured)
        }
    }

    #[inline]
    pub fn get_capabilities(&self, adapter: &Adapter) -> SurfaceCapabilities {
        self.surface.get_capabilities(adapter)
    }

    #[cfg_attr(feature = "tracing_attributes", tracing::instrument(skip_all))]
    pub fn default_surface_config(
        size: PixelSize,
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
            tracing::debug!("Using surface format {surface_format:?}");
        } else {
            tracing::warn!(
                "Surface format {surface_format:?} is not sRGB, which might lead to color artifacts"
            );
        }

        SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width.get(),
            height: size.height.get(),
            present_mode: PresentMode::Fifo,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ResizeSurfaceError {
    #[error("Surface is not configured")]
    SurfaceNotConfigured,
}

impl From<Surface<'static>> for SurfaceWrapper {
    fn from(value: Surface<'static>) -> Self {
        Self {
            id: SurfaceId::next(),
            surface: value,
            config: None,
        }
    }
}

impl Borrow<Surface<'static>> for SurfaceWrapper {
    fn borrow(&self) -> &Surface<'static> {
        &self.surface
    }
}

impl BorrowMut<Surface<'static>> for SurfaceWrapper {
    fn borrow_mut(&mut self) -> &mut Surface<'static> {
        &mut self.surface
    }
}

impl AsRef<Surface<'static>> for SurfaceWrapper {
    fn as_ref(&self) -> &Surface<'static> {
        &self.surface
    }
}

impl AsMut<Surface<'static>> for SurfaceWrapper {
    fn as_mut(&mut self) -> &mut Surface<'static> {
        &mut self.surface
    }
}
