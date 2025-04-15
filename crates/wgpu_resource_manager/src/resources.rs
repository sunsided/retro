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

type BoxedError = Box<dyn Error + Send + Sync>;

type BoxedNewTextureResourceData = Box<dyn NewTextureResourceData>;
type BoxedTextureResource<GlobalError> = Box<dyn NewResource<Output = BoxedNewTextureResourceData, Error = GlobalError>>;

pub struct NewGpuContext<'device> {
    _phantom: PhantomData<&'device ()>,
}

pub trait NewResource {
    type Output;
    type Error: Send + Sync + 'static;

    fn recreate<'device>(
        &mut self,
        ctx: &NewGpuContext<'device>,
        surface_info: &SurfaceInfo,
    ) -> Result<Self::Output, Self::Error>;
}

pub trait NewTextureResourceData: Debug {
    fn texture(&self) -> &Texture;
    fn view(&self) -> &TextureView;
}

pub struct TextureResourceEntry<GlobalError> {
    data: Option<BoxedNewTextureResourceData>,
    resource: BoxedTextureResource<GlobalError>,
}

pub struct NewResources<GlobalError> {
    textures: FxHashMap<TextureResourceId, TextureResourceEntry<GlobalError>>,
    phantom_data: PhantomData<GlobalError>,
}

impl<GlobalError> NewResources<GlobalError>
where
    GlobalError: Error + Send + Sync + 'static,
{
    pub fn register_texture<R, T, E>(&mut self, mut resource: R) -> TextureResourceId
    where
        R: NewResource<Output = T, Error = E> + 'static,
        T: NewTextureResourceData + 'static,
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
    ) -> Result<&dyn NewTextureResourceData, GetResourceError> {
        let entry = self.textures.get(&id).ok_or(GetResourceError::NotFound)?;
        entry.data.as_deref().ok_or(GetResourceError::Uninitialized)
    }

    pub fn recreate(
        &mut self,
        ctx: &NewGpuContext,
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

impl<R, T, E, GlobalError> NewResource for TextureResourceWrapper<R, GlobalError>
where
    R: NewResource<Output = T, Error = E>,
    T: NewTextureResourceData + 'static,
    E: Into<GlobalError>,
    GlobalError: Error + Send + Sync + 'static,
{
    type Output = BoxedNewTextureResourceData;
    type Error = GlobalError;

    fn recreate<'device>(
        &mut self,
        ctx: &NewGpuContext<'device>,
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
    use super::*;

    #[derive(Debug, thiserror::Error)]
    enum MyError {
        #[error(transparent)]
        SomeError(#[from] MyTestTextureError),
    }

    struct MyTestTexture;

    #[derive(Debug)]
    struct MyTestTextureData;

    #[derive(Debug, thiserror::Error)]
    enum MyTestTextureError {
        #[error("bar")]
        Foo,
    }

    impl NewResource for MyTestTexture {
        type Output = MyTestTextureData;
        type Error = MyTestTextureError;

        fn recreate<'device>(
            &mut self,
            _ctx: &NewGpuContext<'device>,
            _surface_info: &SurfaceInfo,
        ) -> Result<Self::Output, Self::Error> {
            Ok(MyTestTextureData)
        }
    }

    impl NewTextureResourceData for MyTestTextureData {
        fn texture(&self) -> &Texture {
            todo!()
        }

        fn view(&self) -> &TextureView {
            todo!()
        }
    }

    #[test]
    fn new_resources() {
        let mut res: NewResources<MyError> = NewResources {
            textures: Default::default(),
            phantom_data: Default::default(),
        };

        let tex = MyTestTexture;
        let id = res.register_texture(tex);

        res.get_texture_data(id).expect_err("Should return an error");

        let ctx = NewGpuContext {
            _phantom: Default::default(),
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



































pub struct Resources<State, ResourceError = Infallible>
where
    ResourceError: Error + 'static,
{
    manager_id: WgpuResourceManagerId,
    device_id: DeviceId,
    textures: FxHashMap<
        ResourceId,
        Box<dyn TextureResource<State = State, Error = Box<dyn Error + Send + Sync>>>,
    >,
    _phantom: std::marker::PhantomData<ResourceError>,
}

struct TextureResourceBox<R>
where
    R: TextureResource,
    R::Error: Error + Send + Sync + 'static,
{
    inner: R,
}

impl<R> Resource for TextureResourceBox<R>
where
    R: TextureResource,
    R::Error: Error + Send + Sync + 'static,
    R: 'static,
{
    type State = R::State;
    type Error = Box<dyn Error + Send + Sync + 'static>;

    fn recreate(
        &mut self,
        ctx: &GpuContext,
        surface: &SurfaceInfo,
        state: &Self::State,
    ) -> Result<(), Self::Error> {
        self.inner
            .recreate(ctx, surface, state)
            .map_err(|e| Box::new(e) as _)
    }
}

impl<R> SurfaceSensitive for TextureResourceBox<R>
where
    R: 'static + TextureResource,
    R::Error: 'static + Error + Send + Sync,
{
    fn is_surface_dependent(&self) -> bool {
        self.inner.is_surface_dependent()
    }

    fn must_recreate(&self, current: &SurfaceInfo, previous: &SurfaceInfo) -> bool {
        self.inner.must_recreate(current, previous)
    }
}

impl<R> DependentResource for TextureResourceBox<R>
where
    R: 'static + TextureResource,
    R::Error: 'static + Error + Send + Sync,
{
}

impl<R> TextureResource for TextureResourceBox<R>
where
    R: TextureResource,
    R::Error: Error + Send + Sync + 'static,
    R: 'static,
{
    fn texture(&self) -> &Texture {
        self.inner.texture()
    }

    fn view(&self) -> &TextureView {
        self.inner.view()
    }
}

impl<State, ResourceError> Resources<State, ResourceError>
where
    ResourceError: Error + From<Box<dyn Error + Send + Sync>> + 'static,
{
    pub fn register_texture<R>(&mut self, resource: R) -> TextureResourceId
    where
        R: TextureResource<State = State>,
        R::Error: Error + Send + Sync + 'static + Into<ResourceError>,
        R: 'static,
    {
        let texture_id = TextureResourceId::next();
        self.textures.insert(
            texture_id.into_inner(),
            Box::new(TextureResourceBox { inner: resource }),
        );
        texture_id
    }

    pub fn get_texture(
        &self,
        id: TextureResourceId,
    ) -> Option<&Box<dyn TextureResource<State = State, Error = Box<dyn Error + Send + Sync>>>>
    {
        self.textures.get(&id.into_inner())
    }

    pub fn delete_texture(
        &mut self,
        id: TextureResourceId,
    ) -> Option<Box<dyn TextureResource<State = State, Error = Box<dyn Error + Send + Sync>>>> {
        self.textures.remove(&id.into_inner())
    }

    pub fn recreate(
        &mut self,
        ctx: &GpuContext,
        surface_info: &SurfaceInfo,
        state: &State,
    ) -> Result<(), ResourceError> {
        for (_id, tex) in &mut self.textures {
            tex.recreate(ctx, surface_info, state)
                .map_err(|e| e.into())?;
        }

        Ok(())
    }
}

pub trait Resource {
    type State;
    type Error: Send + Sync + 'static;

    /// Recreate this resource based on the current surface info
    fn recreate<'device>(
        &mut self,
        ctx: &GpuContext<'device>,
        surface_info: &SurfaceInfo,
        state: &Self::State,
    ) -> Result<(), Self::Error>;
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

pub trait TextureResource : Resource + SurfaceSensitive + DependentResource {
    fn texture(&self) -> &wgpu::Texture;
    fn view(&self) -> &wgpu::TextureView;
}

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
