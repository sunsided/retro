use std::error::Error;
use crate::resources::{GetResourceError, GpuContext, Resource, Resources, TextureResourceId, TextureResourceWrapper};
use std::fmt::Debug;
use wgpu::{Texture, TextureView};
use crate::surface_info::SurfaceInfo;

#[allow(dead_code)]
pub trait TextureResourceData: Debug {
    fn texture(&self) -> &Texture;
    fn view(&self) -> &TextureView;
}

type BoxedNewTextureResourceData = Box<dyn TextureResourceData>;
type BoxedTextureResource<GlobalError> =
    Box<dyn Resource<Output = BoxedNewTextureResourceData, Error = GlobalError>>;

pub struct TextureResourceEntry<GlobalError> {
    data: Option<BoxedNewTextureResourceData>,
    resource: BoxedTextureResource<GlobalError>,
}

impl<GlobalError> Resources<GlobalError>
where
    GlobalError: Error + Send + Sync + 'static,
{
    pub fn register_texture<R, T, E>(&mut self, resource: R) -> TextureResourceId
    where
        T: TextureResourceData + 'static,
        R: Resource<Output = T, Error = E> + 'static,
        E: Error + Send + Sync + 'static,
        GlobalError: From<E>,
    {
        let id = TextureResourceId::next();

        let wrapped_resource = TextureResourceWrapper::boxed(resource);

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

impl<R, T, E, GlobalError> Resource for TextureResourceWrapper<R, GlobalError>
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
    ) -> Result<Self::Output, Self::Error> {
        let raw = self.inner.recreate(ctx, surface_info).map_err(Into::into)?;
        Ok(Box::new(raw))
    }
}
