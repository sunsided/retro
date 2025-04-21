use std::error::Error;
use crate::resources::{GetResourceError, RecreateReason, Resources};
use std::fmt::Debug;
use std::marker::PhantomData;
use wgpu::{Texture, TextureView};
use crate::{GpuContext, Resource, TextureResourceId};
use crate::resource::ResourceData;
use crate::surface_info::SurfaceInfo;

#[allow(dead_code)]
pub trait TextureResourceData: Debug {
    fn texture(&self) -> &Texture;
    fn view(&self) -> &TextureView;
}

impl<T> ResourceData for T where T: TextureResourceData { }
impl ResourceData for Box<dyn TextureResourceData> { }

type BoxedNewTextureResourceData = Box<dyn TextureResourceData>;

/// The [`TextureResourceEntry`] is used in the [`Resources`] manager to type-erase
/// texture resources.
pub struct TextureResourceEntry<Lifecycle, GlobalError> {
    data: Option<BoxedNewTextureResourceData>,
    factory: Box<dyn Resource<Output = BoxedNewTextureResourceData, Error = GlobalError>>,
    lifecycle: Lifecycle
}

/// The [`TextureResourceFactory`] is another layer of type-erasure stored within
/// the [`BoxedTextureResource`], which in turn is used in the [`TextureResourceEntry`].
#[allow(dead_code)]
struct TextureResourceFactory<R, GlobalError> {
    inner: R,
    /// As annoying as it is, the only reason we carry this phantom around is
    /// so that we avoid an "unconstrained generic" error in the Resource trait
    /// implementation for this wrapper.
    _phantom: PhantomData<*const GlobalError>,
}

#[allow(dead_code)]
impl<R, GlobalError> TextureResourceFactory<R, GlobalError> {
    #[inline]
    pub const fn new(inner: R) -> Self {
        Self {
            inner,
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub fn boxed(inner: R) -> Box<Self> {
        Box::new(Self::new(inner))
    }
}

#[allow(dead_code)]
impl<Lifecycle, GlobalError> Resources<Lifecycle, GlobalError>
where
    GlobalError: Error + Send + Sync + 'static,
    Lifecycle: Clone
{
    pub fn register_texture<R, T, E>(&mut self, resource: R, lifecycle: Lifecycle) -> TextureResourceId
    where
        T: TextureResourceData + 'static,
        R: Resource<Output = T, Error = E> + 'static,
        E: Error + Send + Sync + 'static,
        GlobalError: From<E>,
    {
        let id = TextureResourceId::next();

        let factory = TextureResourceFactory::boxed(resource);
        self.textures.insert(
            id,
            TextureResourceEntry {
                data: None,
                factory,
                lifecycle: lifecycle.clone()
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

    pub fn retain_textures<F>(&mut self, mut lifecycle: F)
    where F:
    FnMut(TextureResourceId, &Lifecycle) -> bool {
        self.textures.retain(|&id, value| {
            lifecycle(id, &value.lifecycle)
        })
    }

    pub(crate) fn recreate_textures(
        &mut self,
        ctx: &GpuContext,
        surface_info: &SurfaceInfo,
        reason: RecreateReason
    ) -> Result<(), GlobalError> {
        for entry in self.textures.values_mut() {
            let result = entry.factory.recreate(ctx, surface_info, reason.clone())?;
            entry.data = Some(result);
        }
        Ok(())
    }
}

impl<T, R, E, GlobalError> Resource for TextureResourceFactory<R, GlobalError>
where
    T: TextureResourceData + 'static,
    R: Resource<Output = T, Error = E>,
    E: Error + Send + Sync + 'static,
    GlobalError: Error + Send + Sync + From<E> + 'static,
{
    type Output = BoxedNewTextureResourceData;
    type Error = GlobalError;

    fn recreate<'device>(
        &mut self,
        ctx: &GpuContext<'device>,
        surface_info: &SurfaceInfo,
        reason: RecreateReason
    ) -> Result<Self::Output, Self::Error> {
        let raw = self.inner.recreate(ctx, surface_info, reason).map_err(Into::into)?;
        Ok(Box::new(raw))
    }
}
