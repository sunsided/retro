use crate::wgpu::wgpu_hardware::SurfaceId;
use wgpu::Surface;

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
