use std::error::Error;
use std::fmt::Debug;
use std::marker::PhantomData;
use crate::{GetResourceError, GpuContext, RecreateReason, Resource, ResourceData, Resources, SurfaceInfo, BindGroupLayoutResourceId};
use crate::resources::macros::define_resource_boilerplate;

#[allow(dead_code)]
pub trait BindGroupLayoutResourceData: Debug {
    fn layout(&self) -> &wgpu::BindGroupLayout;
}

define_resource_boilerplate!(
    bind_group_layout,
    BindGroupLayoutResourceData,
    BoxedBindGroupLayoutResourceData,
    BindGroupLayoutResourceEntry,
    ResourceFactory,
    bind_group_layouts,
    register_bind_group_layout,
    get_bind_group_layout_data,
    retain_bind_group_layouts,
    recreate_bind_group_layouts,
    BindGroupLayoutResourceId
);
