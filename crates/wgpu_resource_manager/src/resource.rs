use crate::{GpuContext, ResourceId};
use crate::RecreateReason;
use crate::surface_info::SurfaceInfo;

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

macro_rules! define_resource_boilerplate {
    (
        // base: “texture” or “uniform_buffer”
        $base:ident,
        // Trait name
        $Trait:ident,
        // Boxed alias
        $Boxed:ident,
        // Entry struct
        $Entry:ident,
        // Factory struct
        $Factory:ident,
        // field in Resources
        $field:ident,
        // registration fn
        $register_fn:ident,
        // getter fn
        $get_fn:ident,
        // retain fn
        $retain_fn:ident,
        // recreate fn
        $recreate_fn:ident,
        // ID type
        $Id:ident
    ) => {
        pub trait $Trait: Debug { }
        type $Boxed = Box<dyn $Trait>;
        impl ResourceData for $Boxed { }

        pub struct $Entry<Lifecycle, GlobalError> {
            data: Option<$Boxed>,
            factory: Box<dyn Resource<Output = $Boxed, Error = GlobalError>>,
            lifecycle: Lifecycle,
        }

        struct $Factory<R, GlobalError> {
            inner: R,
            _phantom: PhantomData<*const GlobalError>,
        }

        impl<R, GlobalError> $Factory<R, GlobalError> {
            #[inline]
            pub const fn new(inner: R) -> Self {
                Self { inner, _phantom: PhantomData }
            }
            #[inline]
            pub fn boxed(inner: R) -> Box<Self> {
                Box::new(Self::new(inner))
            }
        }

        impl<T, R, E, GlobalError> Resource for $Factory<R, GlobalError>
        where
            T: $Trait + 'static,
            R: Resource<Output = T, Error = E>,
            E: Error + Send + Sync + 'static,
            GlobalError: Error + Send + Sync + From<E> + 'static,
        {
            type Output = $Boxed;
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

        impl<Lifecycle, GlobalError> Resources<Lifecycle, GlobalError>
        where
            GlobalError: Error + Send + Sync + 'static,
            Lifecycle: Clone,
        {
            pub fn $register_fn<R, T, E>(
                &mut self,
                resource: R,
                lifecycle: Lifecycle
            ) -> $Id
            where
                T: $Trait + 'static,
                R: Resource<Output = T, Error = E> + 'static,
                E: Error + Send + Sync + 'static,
                GlobalError: From<E>,
            {
                let id = $Id::next();
                let factory = $Factory::boxed(resource);
                self.$field.insert(
                    id,
                    $Entry {
                        data: None,
                        factory,
                        lifecycle: lifecycle.clone(),
                    },
                );
                id
            }

            pub fn $get_fn(
                &self,
                id: $Id,
            ) -> Result<&dyn $Trait, GetResourceError> {
                let entry = self.$field.get(&id).ok_or(GetResourceError::NotFound)?;
                entry.data.as_deref().ok_or(GetResourceError::Uninitialized)
            }

            pub fn $retain_fn<F>(&mut self, mut lifecycle: F)
            where
                F: FnMut($Id, &Lifecycle) -> bool,
            {
                self.$field.retain(|&id, value| lifecycle(id, &value.lifecycle));
            }

            pub(crate) fn $recreate_fn(
                &mut self,
                ctx: &GpuContext,
                surface_info: &SurfaceInfo,
                reason: RecreateReason
            ) -> Result<(), GlobalError> {
                for entry in self.$field.values_mut() {
                    let result = entry.factory.recreate(ctx, surface_info, reason.clone())?;
                    entry.data = Some(result);
                }
                Ok(())
            }
        }
    };
}
