use std::convert::Infallible;
use std::fmt::Debug;
use wgpu::PipelineLayoutDescriptor;
use crate::{GpuContext, RecreateReason, Resource, SurfaceInfo};
use crate::resources::pipeline_layouts::pipeline_layout::PipelineLayout;

#[derive(Debug)]
pub struct GenericPipelineLayout {
    desc: PipelineLayoutDescriptor<'static>
}

impl GenericPipelineLayout {
    pub const fn new(desc: PipelineLayoutDescriptor<'static>) -> Self {
        Self { desc }
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
