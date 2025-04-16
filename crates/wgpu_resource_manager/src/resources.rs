mod textures;

use crate::surface_info::SurfaceInfo;
use fxhash::FxHashMap;
use std::convert::Infallible;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicU64, Ordering};
use crate::resources::textures::TextureResourceEntry;
pub use self::textures::TextureResourceData;

pub trait Resource {
    type Output;
    type Error: Send + Sync + 'static;

    fn recreate<'device>(
        &mut self,
        ctx: &GpuContext<'device>,
        surface_info: &SurfaceInfo,
    ) -> Result<Self::Output, Self::Error>;
}

pub struct Resources<GlobalError> {
    textures: FxHashMap<TextureResourceId, TextureResourceEntry<GlobalError>>,
    phantom_data: PhantomData<GlobalError>,
}

pub struct TextureResourceWrapper<R, GlobalError> {
    inner: R,
    _phantom: PhantomData<*const GlobalError>,
}

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
        let mut res: Resources<MyError> = Resources {
            textures: Default::default(),
            phantom_data: Default::default(),
        };

        let tex = FallbackNormalTexture;
        let id = res.register_texture(tex);

        res.get_texture_data(id)
            .expect_err("Should return an error");

        let (device, queue) = create_test_device();
        let ctx = GpuContext {
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
    pub device: &'device wgpu::Device,
    pub queue: &'device wgpu::Queue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct ResourceId(u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextureResourceId(ResourceId);

impl ResourceId {
    pub fn next() -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(1);
        Self(NEXT_ID.fetch_add(1, Ordering::SeqCst))
    }
}

impl TextureResourceId {
    pub fn next() -> Self {
        Self(ResourceId::next())
    }

    #[allow(dead_code)]
    pub fn into_inner(self) -> ResourceId {
        self.0
    }
}
