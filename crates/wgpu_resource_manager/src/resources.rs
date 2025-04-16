use crate::WgpuResourceManagerId;
use crate::device_id::DeviceId;
use crate::surface_info::SurfaceInfo;
use fxhash::FxHashMap;
use std::convert::Infallible;
use std::error::Error;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicU64, Ordering};
use wgpu::{Texture, TextureView};

type BoxedNewTextureResourceData = Box<dyn TextureResourceData>;
type BoxedTextureResource<GlobalError> = Box<dyn Resource<Output = BoxedNewTextureResourceData, Error = GlobalError>>;


pub trait Resource {
    type Output;
    type Error: Send + Sync + 'static;

    fn recreate<'device>(
        &mut self,
        ctx: &GpuContext<'device>,
        surface_info: &SurfaceInfo,
    ) -> Result<Self::Output, Self::Error>;
}

pub trait TextureResourceData: Debug {
    fn texture(&self) -> &Texture;
    fn view(&self) -> &TextureView;
}

pub struct TextureResourceEntry<GlobalError> {
    data: Option<BoxedNewTextureResourceData>,
    resource: BoxedTextureResource<GlobalError>,
}

pub struct Resources<GlobalError> {
    textures: FxHashMap<TextureResourceId, TextureResourceEntry<GlobalError>>,
    phantom_data: PhantomData<GlobalError>,
}

impl<GlobalError> Resources<GlobalError>
where
    GlobalError: Error + Send + Sync + 'static,
{
    pub fn register_texture<R, T, E>(&mut self, mut resource: R) -> TextureResourceId
    where
        R: Resource<Output = T, Error = E> + 'static,
        T: TextureResourceData + 'static,
        E: Into<GlobalError> + Error + Send + Sync + 'static,
        GlobalError: Error + Send + Sync + 'static,
    {
        let id = TextureResourceId::next();

        let wrapped_resource = Box::new(TextureResourceWrapper {
            inner: resource,
            _phantom: Default::default(),
        });

        self.textures.insert(
            id,
            TextureResourceEntry {
                data: None,
                resource: wrapped_resource,
            },
        );

        id
    }

    pub fn get_texture_data(
        &self,
        id: TextureResourceId,
    ) -> Result<&dyn TextureResourceData, GetResourceError> {
        let entry = self.textures.get(&id).ok_or(GetResourceError::NotFound)?;
        entry.data.as_deref().ok_or(GetResourceError::Uninitialized)
    }

    pub fn recreate(
        &mut self,
        ctx: &GpuContext,
        surface_info: &SurfaceInfo,
    ) -> Result<(), GlobalError> {
        for entry in self.textures.values_mut() {
            let result = entry.resource.recreate(ctx, surface_info)?;
            entry.data = Some(result);
        }
        Ok(())
    }
}

pub struct TextureResourceWrapper<R, GlobalError> {
    inner: R,
    _phantom: PhantomData<*const GlobalError>,
}

impl<R, T, E, GlobalError> Resource for TextureResourceWrapper<R, GlobalError>
where
    R: Resource<Output = T, Error = E>,
    T: TextureResourceData + 'static,
    E: Into<GlobalError>,
    GlobalError: Error + Send + Sync + 'static,
{
    type Output = BoxedNewTextureResourceData;
    type Error = GlobalError;

    fn recreate<'device>(
        &mut self,
        ctx: &GpuContext<'device>,
        surface_info: &SurfaceInfo,
    ) -> Result<Self::Output, Self::Error> {
        let raw = self.inner.recreate(ctx, surface_info).map_err(Into::into)?;
        Ok(Box::new(raw))
    }
}

#[derive(Debug, thiserror::Error)]
enum GetResourceError {
    #[error("The resource was not found")]
    NotFound,
    #[error("The resource is uninitialized")]
    Uninitialized,
}

#[cfg(test)]
mod tests {
    use wgpu::TextureFormat;
    use crate::examples::FallbackNormalTexture;
    use crate::wgpu_label;
    use super::*;

    async fn create_test_device() -> (wgpu::Device, wgpu::Queue) {
        let instance = wgpu::Instance::default();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::LowPower,
                force_fallback_adapter: true, // Use software renderer if possible
                compatible_surface: None,     // No window!
            })
            .await
            .expect("No suitable GPU adapters found on the system!");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some(wgpu_label!("test_device")),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::downlevel_webgl2_defaults(),
                    memory_hints: Default::default(),
                    trace: Default::default(),
                }
            )
            .await
            .expect("Failed to create device");

        (device, queue)
    }

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

        res.get_texture_data(id).expect_err("Should return an error");

        let (device, queue) = pollster::block_on(create_test_device());
        let ctx = GpuContext {
            device: &device, queue: &queue,
        };
        let info = SurfaceInfo {
            width: 0,
            height: 0,
            format: TextureFormat::R8Unorm,
            alpha_mode: Default::default(),
            sample_count: 0,
        };

        res.recreate(&ctx, &info).expect("Should recreate textures");
        let outcome = res.get_texture_data(id).expect("Should have created textures");
        let _ = outcome.texture();
    }
}



































pub trait SurfaceSensitive: Resource {
    /// Return `true` if this resource depends on surface config/size/etc.
    fn is_surface_dependent(&self) -> bool;

    /// Return `true` if this resource responds to a specific change in the surface configuration.
    fn must_recreate(&self, current: &SurfaceInfo, previous: &SurfaceInfo) -> bool;
}

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

    pub fn into_inner(self) -> ResourceId {
        self.0
    }
}
