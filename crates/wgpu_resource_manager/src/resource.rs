use std::fmt::Debug;
use crate::{GpuContext, ResourceId};
use crate::RecreateReason;
use crate::surface_info::SurfaceInfo;

#[allow(dead_code)]
pub trait Resource: Debug {
    type Output: ResourceData;
    type Error: Send + Sync + 'static;

    fn recreate<'device>(
        &mut self,
        ctx: &GpuContext<'device>,
        surface_info: &SurfaceInfo,
        reason: RecreateReason,
    ) -> Result<Self::Output, Self::Error>;
}

#[allow(dead_code)]
pub trait ResourceData {}

#[allow(dead_code)]
#[deprecated(note = "Find a better way to handle this")]
pub trait SurfaceSensitive: Resource {
    /// Return `true` if this resource depends on surface config/size/etc.
    fn is_surface_dependent(&self) -> bool;

    /// Return `true` if this resource responds to a specific change in the surface configuration.
    fn must_recreate(&self, current: &SurfaceInfo, previous: &SurfaceInfo) -> bool;
}

#[allow(dead_code)]
#[deprecated(note = "Find a better way to handle this")]
pub trait DependentResource: Resource {
    fn get_dependencies(&self) -> &[ResourceId] {
        &[]
    }
}
