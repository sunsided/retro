use std::convert::Infallible;
use std::fmt::Debug;
use wgpu::BindGroupDescriptor;
use crate::{DependentResource, GpuContext, RecreateReason, Resource, ResourceId, SurfaceInfo, SurfaceSensitive};
use crate::resources::bind_groups::bind_group::BindGroup;

#[derive(Debug)]
pub struct GenericBindGroup {
    desc: BindGroupDescriptor<'static>,
    dependencies: Box<[ResourceId]>
}

impl GenericBindGroup {
    pub fn new<D>(desc: BindGroupDescriptor<'static>, dependencies: D) -> Self
    where
        D: Into<Box<[ResourceId]>>
    {
        Self { desc, dependencies: dependencies.into() }
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

impl SurfaceSensitive for GenericBindGroup {
    fn is_surface_dependent(&self) -> bool {
        false
    }

    fn must_recreate(&self, current: &SurfaceInfo, previous: &SurfaceInfo) -> bool {
        false
    }
}

impl DependentResource for GenericBindGroup {
    fn get_dependencies(&self) -> &[ResourceId] {
        &self.dependencies
    }
}
