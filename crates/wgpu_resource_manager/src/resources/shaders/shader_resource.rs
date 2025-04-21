use std::error::Error;
use std::fmt::Debug;
use std::marker::PhantomData;
use crate::{GetResourceError, GpuContext, RecreateReason, Resource, ResourceData, Resources, ShaderResourceId, SurfaceInfo, UniformResourceId};
use crate::resources::macros::define_resource_boilerplate;

#[allow(dead_code)]
pub trait ShaderResourceData: Debug {
    fn module(&self) -> &wgpu::ShaderModule;
}

define_resource_boilerplate!(
    shader,
    ShaderResourceData,
    BoxedShaderResourceData,
    ShaderResourceEntry,
    ShaderResourceFactory,
    shaders,
    register_shader,
    get_shader_data,
    retain_shaders,
    recreate_shaders,
    ShaderResourceId
);
