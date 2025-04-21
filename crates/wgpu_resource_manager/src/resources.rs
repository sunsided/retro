mod textures;

pub use self::textures::TextureResourceData;
use crate::resources::textures::TextureResourceEntry;
use crate::surface_info::SurfaceInfo;
use crate::{DeviceId, SurfaceId};
use fxhash::FxHashMap;
use std::error::Error;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicU64, Ordering};

#[allow(dead_code)]
pub trait Resource {
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
pub struct Resources<Lifecycle, GlobalError> {
    textures: FxHashMap<TextureResourceId, TextureResourceEntry<Lifecycle, GlobalError>>,
    phantom_data: PhantomData<GlobalError>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecreateReason {
    DeviceRecreated(DeviceId),
    SurfaceChanged(SurfaceChangedReason),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SurfaceChangedReason {
    SizeChanged,
    SurfaceRecreated(SurfaceId),
}

impl RecreateReason {
    pub fn is_surface_changed(&self) -> bool {
        matches!(self, RecreateReason::SurfaceChanged(_))
    }
    
    pub fn is_device_recreated(&self) -> bool {
        matches!(self, RecreateReason::DeviceRecreated(_))
    }
}

#[allow(dead_code)]
impl<Lifecycle, GlobalError> Resources<Lifecycle, GlobalError>
where
    GlobalError: Error + Send + Sync + 'static,
    Lifecycle: Clone,
{
    pub fn recreate(
        &mut self,
        ctx: &GpuContext,
        surface_info: &SurfaceInfo,
        reason: RecreateReason,
    ) -> Result<(), GlobalError> {
        self.recreate_textures(ctx, surface_info, reason)?;
        Ok(())
    }

    pub fn retain<F>(&mut self, mut lifecycle: F)
    where
        F: FnMut(&Lifecycle) -> bool,
    {
        self.retain_textures(|_, l| lifecycle(l))
    }
}

#[allow(dead_code)]
#[derive(Debug, thiserror::Error)]
pub enum GetResourceError {
    #[error("The resource was not found")]
    NotFound,
    #[error("The resource is uninitialized")]
    Uninitialized,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::create_test_device;
    use crate::examples::FallbackNormalTexture;
    use std::convert::Infallible;
    use wgpu::TextureFormat;

    #[derive(Debug, thiserror::Error)]
    enum MyError {
        // #[error(transparent)]
        // SomeError(#[from] FileTextureError),
    }

    impl From<Infallible> for MyError {
        fn from(_: Infallible) -> Self {
            unreachable!("Infallible cannot be constructed")
        }
    }

    #[test]
    fn new_resources() {
        let mut res: Resources<(), MyError> = Resources {
            textures: Default::default(),
            phantom_data: Default::default(),
        };

        let tex = FallbackNormalTexture;
        let id = res.register_texture(tex, ());

        res.get_texture_data(id)
            .expect_err("Should return an error");

        let (device, queue) = create_test_device();
        let ctx = GpuContext {
            device_id: DeviceId::next(),
            device: &device,
            queue: &queue,
        };
        let info = SurfaceInfo {
            width: 0,
            height: 0,
            format: TextureFormat::R8Unorm,
            alpha_mode: Default::default(),
            sample_count: 0,
        };

        res.recreate(&ctx, &info).expect("Should recreate textures");
        let outcome = res
            .get_texture_data(id)
            .expect("Should have created textures");
        let _ = outcome.texture();
    }
}

#[allow(dead_code)]
pub trait SurfaceSensitive: Resource {
    /// Return `true` if this resource depends on surface config/size/etc.
    fn is_surface_dependent(&self) -> bool;

    /// Return `true` if this resource responds to a specific change in the surface configuration.
    fn must_recreate(&self, current: &SurfaceInfo, previous: &SurfaceInfo) -> bool;
}

#[allow(dead_code)]
pub trait DependentResource: Resource {
    fn get_dependencies(&self) -> &[ResourceId] {
        &[]
    }
}

pub struct GpuContext<'device> {
    pub device_id: DeviceId,
    pub device: &'device wgpu::Device,
    pub queue: &'device wgpu::Queue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct ResourceId(u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextureResourceId(ResourceId);

#[allow(dead_code)]
impl ResourceId {
    pub fn next() -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(1);
        Self(NEXT_ID.fetch_add(1, Ordering::SeqCst))
    }
}

#[allow(dead_code)]
impl TextureResourceId {
    pub fn next() -> Self {
        Self(ResourceId::next())
    }

    #[allow(dead_code)]
    pub fn into_inner(self) -> ResourceId {
        self.0
    }
}
