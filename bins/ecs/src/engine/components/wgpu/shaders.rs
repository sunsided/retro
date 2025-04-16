use wgpu::{ShaderModule, ShaderModuleDescriptor, ShaderSource};

#[derive(Debug, Clone)]
pub struct WgpuShaderData {
    pub descriptor: ShaderModuleDescriptor<'static>,
    pub module: Option<ShaderModule>
}

impl WgpuShaderData {
    pub const fn new(descriptor: ShaderModuleDescriptor<'static>) -> Self {
        Self { descriptor, module: None }
    }
}
