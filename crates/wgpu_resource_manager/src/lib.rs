mod device_and_queue;
mod instance_wrapper;
mod pixel_size;
mod resource_manager;
mod resources;
mod surface_info;
mod surface_wrapper;
mod resource_valid;
mod resource;
mod gpu_context;
mod ids;

pub mod examples {
    pub use crate::resources::textures::examples as textures;
    pub use crate::resources::uniform_buffers::examples as uniform_buffers;
    pub use crate::resources::shaders::examples as shaders;
}

pub use crate::device_and_queue::DeviceAndQueue;
pub use crate::instance_wrapper::InstanceWrapper;
pub use crate::pixel_size::*;
pub use crate::resource_manager::{WgpuResourceManager, ConfiguredWgpuResourceManager, WgpuResourceManagerResizeSurfaceError, WgpuResourceManagerInitializeSurfaceError, NO_SURFACE_CONFIG};
pub use crate::surface_wrapper::{ResizeSurfaceError, SurfaceWrapper};
pub use crate::gpu_context::GpuContext;
pub use crate::resource::{Resource, ResourceData, DependentResource, SurfaceSensitive};
pub use crate::resources::{RecreateReason, Resources, SurfaceChangedReason, GetResourceError, textures::TextureWithView};
pub use crate::surface_info::SurfaceInfo;
pub use ids::*;

#[macro_export]
macro_rules! wgpu_label {
    () => {
        concat!(file!(), ":", line!())
    };
    ($name:literal) => {
        concat!($name, " @ ", file!(), ":", line!())
    };
    ($name:expr) => {
        format!(
            "{name} @ {file}:{line}",
            name = $name,
            file = file!(),
            line = line!()
        )
        .into_boxed_str()
    };
}

#[cfg(test)]
#[allow(dead_code)]
fn init_test_tracing() {
    static INIT: std::sync::Once = std::sync::Once::new();

    INIT.call_once(|| {
        tracing_subscriber::fmt()
            .with_env_filter("wgpu_resource_manager=trace")
            .with_test_writer()
            .init();
    });
}

#[cfg(test)]
fn create_test_device() -> (wgpu::Device, wgpu::Queue) {
    pollster::block_on(create_test_device_async())
}

#[cfg(test)]
async fn create_test_device_async() -> (wgpu::Device, wgpu::Queue) {
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
        .request_device(&wgpu::DeviceDescriptor {
            label: Some(wgpu_label!("test_device")),
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::downlevel_webgl2_defaults(),
            memory_hints: Default::default(),
            trace: Default::default(),
        })
        .await
        .expect("Failed to create device");

    (device, queue)
}
