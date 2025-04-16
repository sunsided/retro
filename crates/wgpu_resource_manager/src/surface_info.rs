use wgpu::{CompositeAlphaMode, SurfaceConfiguration};

#[derive(Clone, Eq, PartialEq)]
pub struct SurfaceInfo {
    pub width: u32,
    pub height: u32,
    pub format: wgpu::TextureFormat,
    pub alpha_mode: CompositeAlphaMode,
    pub sample_count: u32,
}

#[allow(dead_code)]
impl SurfaceInfo {
    pub fn new(config: &SurfaceConfiguration, sample_count: u32) -> Self {
        Self {
            width: config.width,
            height: config.height,
            format: config.format,
            alpha_mode: config.alpha_mode,
            sample_count,
        }
    }

    #[inline]
    pub const fn size_changed(&self, previous: &SurfaceInfo) -> bool {
        self.width != previous.width || self.height != previous.height
    }

    #[inline]
    pub fn format_changed(&self, previous: &SurfaceInfo) -> bool {
        self.format != previous.format
    }

    #[inline]
    pub const fn alpha_mode_changed(&self, previous: &SurfaceInfo) -> bool {
        (self.alpha_mode as u32) != (previous.alpha_mode as u32)
    }

    #[inline]
    pub const fn sample_count_changed(&self, previous: &SurfaceInfo) -> bool {
        self.sample_count != previous.sample_count
    }
}
