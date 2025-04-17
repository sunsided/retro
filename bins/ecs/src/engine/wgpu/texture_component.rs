use wgpu::{ShaderModule, ShaderModuleDescriptor, ShaderSource};
use wgpu_resource_manager::TextureWithView;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TextureSourceType {
    FallbackNormals,
}

#[derive(Debug, Clone)]
pub struct TextureSourceData {
    pub source: TextureSourceType,
}

#[derive(Default, Debug, Clone)]
pub struct WgpuTextureData {
    pub twv: Option<TextureWithView>,
}

impl TextureSourceData {
    pub const fn new(source: TextureSourceType) -> Self {
        Self { source }
    }
}
