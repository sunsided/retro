use crate::WgpuResourceManagerId;
use crate::device_id::DeviceId;
use crate::surface_info::SurfaceInfo;
use fxhash::FxHashMap;
use std::convert::Infallible;
use std::error::Error;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicU64, Ordering};
use wgpu::{Texture, TextureView};

type BoxedError = Box<dyn Error + Send + Sync>;

pub trait NewResource {
    type Error: Send + Sync + 'static;
    fn recreate<'device>(
        &mut self,
        ctx: &GpuContext<'device>,
        surface_info: &SurfaceInfo,
    ) -> Result<(), Self::Error>;
}


pub trait NewTextureResourceData {
    fn texture(&self) -> &Texture;
    fn view(&self) -> &TextureView;
}

pub trait NewTextureResource: NewResource {
    type Output: NewTextureResourceData;
    fn data(&self) -> Option<&Self::Output>;
}

// TODO: Make T: Resource, E: Error, GlobalError: Error
pub struct ResourceEntry<T, F, E, GlobalError> {
    pub data: Option<T>,
    pub factory: F,
    pub convert_error: fn(E) -> GlobalError,
}

pub struct TextureResourceAdapter<T, F, E, GlobalError> {
    pub entry: ResourceEntry<T, F, E, GlobalError>,
    _phantom: PhantomData<GlobalError>,
}

impl<T, F, E, GlobalError> NewResource for TextureResourceAdapter<T, F, E, GlobalError>
where
    F: Fn(&GpuContext, &SurfaceInfo) -> Result<T, E>,
    E: Error + Send + Sync + Into<GlobalError> + 'static,
    GlobalError: Error + Send + Sync + 'static + From<E>,
{
    type Error = GlobalError;

    fn recreate<'device>(
        &mut self,
        ctx: &GpuContext<'device>,
        surface_info: &SurfaceInfo,
    ) -> Result<(), Self::Error> {
        match (self.entry.factory)(ctx, surface_info) {
            Ok(resource) => {
                self.entry.data = Some(resource);
                Ok(())
            }
            Err(e) => Err((self.entry.convert_error)(e)),
        }
    }
}

impl<T, F, E, GlobalError> NewTextureResource for TextureResourceAdapter<T, F, E, GlobalError>
where
    T: NewTextureResourceData,
    F: Fn(&GpuContext, &SurfaceInfo) -> Result<T, E>,
    E: Error + Send + Sync + 'static,
    GlobalError: Error + Send + Sync + 'static + From<E>,
{
    type Output = T;

    fn data(&self) -> Option<&Self::Output> {
        self.entry.data.as_ref()
    }
}

pub struct NewResources<GlobalError> {
    textures: FxHashMap<TextureResourceId, Box<dyn NewTextureResource<Error = GlobalError, Output = dyn NewTextureResourceData>>>,
}

impl<GlobalError> NewResources<GlobalError>
where
    GlobalError: Error + Send + Sync + 'static,
{
    pub fn register_texture<T, F, E>(
        &mut self,
        id: TextureResourceId,
        factory: F,
        convert_error: fn(E) -> GlobalError,
    ) where
        T: NewTextureResourceData + 'static,
        F: Fn(&GpuContext, &SurfaceInfo) -> Result<T, E> + 'static,
        E: Error + Send + Sync + Into<GlobalError> + 'static,
        GlobalError: Error + Send + Sync + 'static + From<E>,
    {
        let adapter = TextureResourceAdapter {
            entry: ResourceEntry {
                data: None,
                factory,
                convert_error,
            },
            _phantom: Default::default(),
        };

        self.textures.insert(id, Box::new(adapter));
    }

    pub fn get_texture(
        &self,
        id: TextureResourceId,
    ) -> Option<&dyn NewTextureResource<Error = GlobalError, Output = dyn NewTextureResourceData>> {
        self.textures.get(&id).map(Box::as_ref)
    }

    pub fn recreate(
        &mut self,
        ctx: &GpuContext,
        surface_info: &SurfaceInfo,
    ) -> Result<(), GlobalError> {
        for (_id, resource) in self.textures.iter_mut() {
            resource.recreate(ctx, surface_info)?;
        }
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, thiserror::Error)]
    enum MyError {
        #[error(transparent)]
        SomeError(#[from] MyTestTextureError)
    }

    struct MyTestTexture;
    struct MyTestTextureData;

    #[derive(Debug, thiserror::Error)]
    enum MyTestTextureError {
        #[error("bar")]
        Foo
    }

    impl NewResource for MyTestTexture {
        type Error = MyTestTextureError;

        fn recreate<'device>(&mut self, ctx: &GpuContext<'device>, surface_info: &SurfaceInfo) -> Result<(), Self::Error> {
            todo!()
        }
    }

    impl NewTextureResource for MyTestTextureData {
        type Output = MyTestTextureData;
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
        };

        let tex = MyTestTexture;

        // TODO: I'd rather have `res.register_texture(tex)`
        res.register_texture(
            TextureResourceId::next(),
            |_ctx, _info| Ok(MyTestTextureData),
            |e| MyError::SomeError(e),
        );
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
