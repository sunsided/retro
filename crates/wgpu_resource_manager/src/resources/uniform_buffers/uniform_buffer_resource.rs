use std::error::Error;
use std::fmt::Debug;
use std::marker::PhantomData;
use crate::{GetResourceError, GpuContext, RecreateReason, Resource, ResourceData, Resources, SurfaceInfo, UniformResourceId};
use crate::resources::macros::define_resource_boilerplate;

#[allow(dead_code)]
pub trait UniformBufferResourceData: Debug {
    fn buffer(&self) -> &wgpu::Buffer;
    fn bind_group_layout(&self) -> &wgpu::BindGroupLayout;
    fn bind_group(&self) -> &wgpu::BindGroup;
}

define_resource_boilerplate!(
    uniform_buffer,
    UniformBufferResourceData,
    BoxedUniformBufferResourceData,
    UniformBufferResourceEntry,
    UniformResourceFactory,
    uniforms,
    register_uniform,
    get_uniform_data,
    retain_uniforms,
    recreate_uniforms,
    UniformResourceId
);
