use std::error::Error;
use std::fmt::Debug;
use std::marker::PhantomData;
use crate::{GetResourceError, GpuContext, RecreateReason, Resource, ResourceData, Resources, SurfaceInfo, BindGroupResourceId};
use crate::resources::macros::define_resource_boilerplate;

#[allow(dead_code)]
pub trait BindGroupResourceData: Debug {
    fn layout(&self) -> &wgpu::BindGroup;
}

define_resource_boilerplate!(
    bind_group,
    BindGroupResourceData,
    BoxedBindGroupResourceData,
    BindGroupResourceEntry,
    ResourceFactory,
    bind_groups,
    register_bind_group,
    get_bind_group_data,
    retain_bind_groups,
    recreate_bind_groups,
    BindGroupResourceId
);
