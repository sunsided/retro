use std::error::Error;
use std::fmt::Debug;
use std::marker::PhantomData;
use crate::{GetResourceError, GpuContext, RecreateReason, Resource, ResourceData, Resources, SurfaceInfo, BufferResourceId};
use crate::resources::macros::define_resource_boilerplate;

#[allow(dead_code)]
pub trait BufferResourceData: Debug {
    fn buffer(&self) -> &wgpu::Buffer;
}

define_resource_boilerplate!(
    buffer,
    BufferResourceData,
    BoxedBufferResourceData,
    BufferResourceEntry,
    ResourceFactory,
    buffers,
    register_buffer,
    get_buffer_data,
    retain_buffers,
    recreate_buffers,
    BufferResourceId
);
