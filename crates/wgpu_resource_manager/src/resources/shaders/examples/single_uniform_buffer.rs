use std::convert::Infallible;
use std::fmt::Debug;
use wgpu::ShaderModuleDescriptor;
use wgpu::util::DeviceExt;
use crate::{wgpu_label, GpuContext, RecreateReason, Resource, SurfaceInfo, UniformResourceId};
use crate::resources::shaders::shader::Shader;

#[derive(Debug)]
pub struct ShaderFromSource
{
    pub label: &'static str,
    pub source: wgpu::ShaderSource<'static>,
}

impl ShaderFromSource
{
    pub const fn new(label: &'static str, data: wgpu::ShaderSource<'static>) -> Self {
        Self { label, source: data }
    }

    pub fn from_descriptor(desc: ShaderModuleDescriptor<'static>) -> Self {
        Self::new(desc.label.unwrap_or("shader"), desc.source)
    }
}

impl Resource for ShaderFromSource
{
    type Output = Shader;
    type Error = Infallible;

    fn recreate<'device>(&mut self, ctx: &GpuContext<'device>, _surface_info: &SurfaceInfo, _reason: RecreateReason) -> Result<Self::Output, Self::Error> {
        // TODO: Don't recreate if not necessary

        let label = wgpu_label!(self.label);

        let shader = ShaderModuleDescriptor {
            label: Some(&label),
            source: self.source.clone(),
        };

        let module = ctx.device.create_shader_module(shader);

        Ok(Shader::new(module))
    }
}
