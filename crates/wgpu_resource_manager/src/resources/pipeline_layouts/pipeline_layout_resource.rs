use std::error::Error;
use std::fmt::Debug;
use std::marker::PhantomData;
use crate::{GetResourceError, GpuContext, RecreateReason, Resource, ResourceData, Resources, SurfaceInfo, PipelineLayoutResourceId};
use crate::resources::macros::define_resource_boilerplate;

#[allow(dead_code)]
pub trait PipelineLayoutResourceData: Debug {
    fn layout(&self) -> &wgpu::PipelineLayout;
}

define_resource_boilerplate!(
    pipeline_layout,
    PipelineLayoutResourceData,
    BoxedPipelineLayoutResourceData,
    PipelineLayoutResourceEntry,
    ResourceFactory,
    pipeline_layouts,
    register_pipeline_layout,
    get_pipeline_layout_data,
    retain_pipeline_layouts,
    recreate_pipeline_layouts,
    PipelineLayoutResourceId
);
