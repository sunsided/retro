use std::convert::Infallible;
use std::fmt::Debug;
use wgpu::RenderPipelineDescriptor;
use crate::{DependentResource, GpuContext, RecreateReason, Resource, ResourceId, SurfaceInfo, SurfaceSensitive};
use crate::resources::pipelines::pipeline::Pipeline;

#[derive(Debug)]
pub struct GenericPipeline {
    desc: RenderPipelineDescriptor<'static>,
    dependencies: Box<[ResourceId]>
}

impl GenericPipeline {
    pub fn new<D>(desc: RenderPipelineDescriptor<'static>, dependencies: D) -> Self
    where
        D: Into<Box<[ResourceId]>>,
    {
        Self { desc, dependencies: dependencies.into() }
    }
}

impl Resource for GenericPipeline
{
    type Output = Pipeline;
    type Error = Infallible;

    fn recreate<'device>(&mut self, ctx: &GpuContext<'device>, _surface_info: &SurfaceInfo, _reason: RecreateReason) -> Result<Self::Output, Self::Error> {
        // TODO: Don't recreate if not necessary

        let pipeline = ctx.device.create_render_pipeline(&self.desc);
        Ok(Pipeline::new(pipeline))
    }
}

impl SurfaceSensitive for GenericPipeline {
    fn is_surface_dependent(&self) -> bool {
        false
    }

    fn must_recreate(&self, current: &SurfaceInfo, previous: &SurfaceInfo) -> bool {
        false
    }
}

impl DependentResource for GenericPipeline {
    fn get_dependencies(&self) -> &[ResourceId] {
        &self.dependencies
    }
}
