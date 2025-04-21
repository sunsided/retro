use std::error::Error;
use std::fmt::Debug;
use std::marker::PhantomData;
use crate::{GetResourceError, GpuContext, RecreateReason, Resource, ResourceData, Resources, SurfaceInfo, UniformResourceId};

pub trait UniformBufferResourceData: Debug { }

type BoxedUniformBufferResourceData = Box<dyn UniformBufferResourceData>;
impl ResourceData for BoxedUniformBufferResourceData { }

pub struct UniformBufferResourceEntry<Lifecycle, GlobalError> {
    data: Option<BoxedUniformBufferResourceData>,
    factory: Box<dyn Resource<Output = BoxedUniformBufferResourceData, Error = GlobalError>>,
    lifecycle: Lifecycle
}

struct UniformResourceFactory<R, GlobalError> {
    inner: R,
    /// As annoying as it is, the only reason we carry this phantom around is
    /// so that we avoid an "unconstrained generic" error in the Resource trait
    /// implementation for this wrapper.
    _phantom: PhantomData<*const GlobalError>,
}

#[allow(dead_code)]
impl<R, GlobalError> UniformResourceFactory<R, GlobalError> {
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

impl<T, R, E, GlobalError> Resource for UniformResourceFactory<R, GlobalError>
where
    T: UniformBufferResourceData + 'static,
    R: Resource<Output = T, Error = E>,
    E: Error + Send + Sync + 'static,
    GlobalError: Error + Send + Sync + From<E> + 'static,
{
    type Output = BoxedUniformBufferResourceData;
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

#[allow(dead_code)]
impl<Lifecycle, GlobalError> Resources<Lifecycle, GlobalError>
where
    GlobalError: Error + Send + Sync + 'static,
    Lifecycle: Clone
{
    pub fn register_uniform<R, T, E>(&mut self, resource: R, lifecycle: Lifecycle) -> UniformResourceId
    where
        T: UniformBufferResourceData + 'static,
        R: Resource<Output = T, Error = E> + 'static,
        E: Error + Send + Sync + 'static,
        GlobalError: From<E>,
    {
        let id = UniformResourceId::next();

        let factory = UniformResourceFactory::boxed(resource);
        self.uniforms.insert(
            id,
            UniformBufferResourceEntry {
                data: None,
                factory,
                lifecycle: lifecycle.clone()
            },
        );

        id
    }

    pub fn get_uniform_data(
        &self,
        id: UniformResourceId,
    ) -> Result<&dyn UniformBufferResourceData, GetResourceError> {
        let entry = self.uniforms.get(&id).ok_or(GetResourceError::NotFound)?;
        entry.data.as_deref().ok_or(GetResourceError::Uninitialized)
    }

    pub fn retain_uniforms<F>(&mut self, mut lifecycle: F)
    where F:
    FnMut(UniformResourceId, &Lifecycle) -> bool {
        self.uniforms.retain(|&id, value| {
            lifecycle(id, &value.lifecycle)
        })
    }

    pub(crate) fn recreate_uniforms(
        &mut self,
        ctx: &GpuContext,
        surface_info: &SurfaceInfo,
        reason: RecreateReason
    ) -> Result<(), GlobalError> {
        for entry in self.uniforms.values_mut() {
            let result = entry.factory.recreate(ctx, surface_info, reason.clone())?;
            entry.data = Some(result);
        }
        Ok(())
    }
}
