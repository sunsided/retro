use std::convert::Infallible;
use std::fmt::Debug;
use wgpu::PipelineLayoutDescriptor;
use crate::{DependentResource, GpuContext, RecreateReason, Resource, ResourceId, SurfaceInfo, SurfaceSensitive};
use crate::resources::pipeline_layouts::pipeline_layout::PipelineLayout;

#[derive(Debug)]
pub struct GenericPipelineLayout {
    desc: PipelineLayoutDescriptor<'static>,
    dependencies: Box<[ResourceId]>
}

impl GenericPipelineLayout {
    pub fn new<D>(desc: PipelineLayoutDescriptor<'static>, dependencies: D) -> Self
    where
        D: Into<Box<[ResourceId]>>,
    {
        Self { desc, dependencies: dependencies.into() }
    }
}

impl Resource for GenericPipelineLayout
{
    type Output = PipelineLayout;
    type Error = Infallible;

    fn recreate<'device>(&mut self, ctx: &GpuContext<'device>, _surface_info: &SurfaceInfo, _reason: RecreateReason) -> Result<Self::Output, Self::Error> {
        // TODO: Don't recreate if not necessary

        let layout = ctx.device.create_pipeline_layout(&self.desc);
        Ok(PipelineLayout::new(layout))
    }
}

impl SurfaceSensitive for GenericPipelineLayout {
    fn is_surface_dependent(&self) -> bool {
        false
    }

    fn must_recreate(&self, _current: &SurfaceInfo, _previous: &SurfaceInfo) -> bool {
        false
    }
}

impl DependentResource for GenericPipelineLayout {
    fn get_dependencies(&self) -> &[ResourceId] {
        &self.dependencies
    }
}
