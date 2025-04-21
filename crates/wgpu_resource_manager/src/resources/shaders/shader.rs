use wgpu::ShaderModule;
use crate::{ResourceData};
use crate::resources::shaders::ShaderResourceData;

#[derive(Debug)]
pub struct Shader {
    pub module: ShaderModule,
}

impl ResourceData for Shader {}

impl ShaderResourceData for Shader {
    fn module(&self) -> &ShaderModule {
        &self.module
    }
}

impl Shader {
    pub(crate) const fn new(module: ShaderModule) -> Shader {
        Self {
            module
        }
    }
}
