use std::convert::Infallible;
use std::fmt::Debug;
use wgpu::BindGroupLayoutDescriptor;
use crate::{GpuContext, RecreateReason, Resource, SurfaceInfo};
use crate::resources::bind_group_layouts::bind_group_layout::BindGroupLayout;

#[derive(Debug)]
pub struct GenericBindGroupLayout {
    desc: BindGroupLayoutDescriptor<'static>
}

impl GenericBindGroupLayout {
    pub const fn new(desc: BindGroupLayoutDescriptor<'static>) -> Self {
        Self { desc }
    }
}

impl Resource for GenericBindGroupLayout
{
    type Output = BindGroupLayout;
    type Error = Infallible;

    fn recreate<'device>(&mut self, ctx: &GpuContext<'device>, _surface_info: &SurfaceInfo, _reason: RecreateReason) -> Result<Self::Output, Self::Error> {
        // TODO: Don't recreate if not necessary

        let layout = ctx.device.create_bind_group_layout(&self.desc);
        Ok(BindGroupLayout::new(layout))
    }
}
