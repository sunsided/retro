mod macros;
pub mod textures;
pub mod uniform_buffers;
pub mod shaders;
pub mod buffers;
pub mod texture_samplers;

use crate::surface_info::SurfaceInfo;
use crate::{BufferResourceId, DeviceId, ShaderResourceId, SurfaceId, TextureResourceId, TextureSamplerResourceId, UniformResourceId};
use fxhash::FxHashMap;
use std::error::Error;
use std::fmt::Debug;
use crate::GpuContext;
use crate::resources::shaders::ShaderResourceEntry;
use crate::resources::buffers::BufferResourceEntry;
use crate::resources::texture_samplers::TextureSamplerResourceEntry;
use crate::resources::textures::TextureResourceEntry;
use crate::resources::uniform_buffers::UniformBufferResourceEntry;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Resources<Lifecycle, GlobalError> {
    pub(crate) textures: FxHashMap<TextureResourceId, TextureResourceEntry<Lifecycle, GlobalError>>,
    pub(crate) uniforms: FxHashMap<UniformResourceId, UniformBufferResourceEntry<Lifecycle, GlobalError>>,
    pub(crate) shaders: FxHashMap<ShaderResourceId, ShaderResourceEntry<Lifecycle, GlobalError>>,
    pub(crate) buffers: FxHashMap<BufferResourceId, BufferResourceEntry<Lifecycle, GlobalError>>,
    pub(crate) texture_samplers: FxHashMap<TextureSamplerResourceId, TextureSamplerResourceEntry<Lifecycle, GlobalError>>,
}

impl<Lifecycle, GlobalError> Default for Resources<Lifecycle, GlobalError> {
    fn default() -> Self {
        Self {
            textures: Default::default(),
            uniforms: Default::default(),
            shaders: Default::default(),
            buffers: Default::default(),
            texture_samplers: Default::default(),
        }
    }
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
        self.recreate_uniforms(ctx, surface_info, reason)?;
        self.recreate_shaders(ctx, surface_info, reason)?;
        self.recreate_buffers(ctx, surface_info, reason)?;
        self.recreate_texture_samplers(ctx, surface_info, reason)?;
        Ok(())
    }

    pub fn retain<F>(&mut self, mut lifecycle: F)
    where
        F: FnMut(&Lifecycle) -> bool,
    {
        self.retain_textures(|_, l| lifecycle(l));
        self.retain_uniforms(|_, l| lifecycle(l));
        self.retain_shaders(|_, l| lifecycle(l));
        self.retain_buffers(|_, l| lifecycle(l));
        self.retain_texture_samplers(|_, l| lifecycle(l));
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
    use std::convert::Infallible;
    use wgpu::TextureFormat;
    use crate::gpu_context::GpuContext;
    use crate::resources::textures::examples::FallbackNormalTexture;

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
            ..Default::default()
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

        res.recreate(&ctx, &info, RecreateReason::DeviceRecreated(ctx.device_id)).expect("Should recreate textures");
        let outcome = res
            .get_texture_data(id)
            .expect("Should have created textures");
        let _ = outcome.texture();
    }
}
