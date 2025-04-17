use std::path::Path;
use wgpu::{ShaderModule, ShaderModuleDescriptor, ShaderSource};
use wgpu_resource_manager::TextureWithView;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TextureSourceType {
    FallbackNormals,
    Resource(TextureResourcePath),
}

impl TextureSourceType {
    pub fn from_path<P>(path: P) -> Self
    where
        P: Into<String>,
    {
        Self::Resource(TextureResourcePath::from(path))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TextureResourcePath(String);

impl<S> From<S> for TextureResourcePath
where
    S: Into<String>,
{
    fn from(value: S) -> Self {
        Self(value.into())
    }
}

impl AsRef<Path> for TextureResourcePath {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
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
