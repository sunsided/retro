use crate::wgpu::uniforms::{CameraUniform, FrameGlobalUniform, LightUniform};
use wgpu::{BindGroup, ShaderModule, ShaderModuleDescriptor, ShaderSource};

#[derive(Debug, Clone)]
pub enum UniformTypeData {
    FrameGlobal(FrameGlobalUniform),
    Camera(CameraUniform),
    Lighting(LightUniform),
}

#[derive(Debug, Clone)]
pub struct WgpuBindGroupLayoutData {
    pub layout: Option<wgpu::BindGroupLayout>,
}

#[derive(Debug, Clone)]
pub struct WgpuBindGroupData {
    pub group: Option<BindGroup>,
}

impl WgpuBindGroupLayoutData {
    pub const fn new() -> Self {
        Self { layout: None }
    }
}

impl Default for WgpuBindGroupLayoutData {
    fn default() -> Self {
        Self::new()
    }
}

impl WgpuBindGroupData {
    pub const fn new() -> Self {
        Self { group: None }
    }
}

impl Default for WgpuBindGroupData {
    fn default() -> Self {
        Self::new()
    }
}
