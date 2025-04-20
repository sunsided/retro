use wgpu::{ShaderModule, ShaderModuleDescriptor, ShaderSource};

#[derive(Debug, Clone)]
pub struct WgpuBufferData {
    pub buffer: Option<wgpu::Buffer>,
}

impl WgpuBufferData {
    pub const fn new() -> Self {
        Self { buffer: None }
    }
}

impl Default for WgpuBufferData {
    fn default() -> Self {
        Self::new()
    }
}

impl From<wgpu::Buffer> for WgpuBufferData {
    fn from(buffer: wgpu::Buffer) -> Self {
        Self {
            buffer: Some(buffer),
        }
    }
}
