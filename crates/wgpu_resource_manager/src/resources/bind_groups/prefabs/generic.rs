use std::convert::Infallible;
use std::fmt::Debug;
use wgpu::BindGroupDescriptor;
use crate::{GpuContext, RecreateReason, Resource, SurfaceInfo};
use crate::resources::bind_groups::bind_group::BindGroup;

#[derive(Debug)]
pub struct GenericBindGroup {
    desc: BindGroupDescriptor<'static>
}

impl GenericBindGroup {
    pub const fn new(desc: BindGroupDescriptor<'static>) -> Self {
        Self { desc }
    }
}

impl Resource for GenericBindGroup
{
    type Output = BindGroup;
    type Error = Infallible;

    fn recreate<'device>(&mut self, ctx: &GpuContext<'device>, _surface_info: &SurfaceInfo, _reason: RecreateReason) -> Result<Self::Output, Self::Error> {
        // TODO: Don't recreate if not necessary

        let group = ctx.device.create_bind_group(&self.desc);
        Ok(BindGroup::new(group))
    }
}
