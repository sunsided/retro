use std::error::Error;
use std::fmt::Debug;
use std::marker::PhantomData;
use crate::{GetResourceError, GpuContext, RecreateReason, Resource, ResourceData, Resources, SurfaceInfo, PipelineResourceId};
use crate::resources::macros::define_resource_boilerplate;

#[allow(dead_code)]
pub trait PipelineResourceData: Debug {
    fn pipeline(&self) -> &wgpu::RenderPipeline;
}

define_resource_boilerplate!(
    pipeline,
    PipelineResourceData,
    BoxedPipelineResourceData,
    PipelineResourceEntry,
    ResourceFactory,
    pipelines,
    register_pipeline,
    get_pipeline_data,
    retain_pipelines,
    recreate_pipelines,
    PipelineResourceId
);
