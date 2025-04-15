mod configured_resource_manager;
mod device_and_queue;
mod instance_wrapper;
mod manager_id;
mod pixel_size;
mod resource_manager;
mod resource_manager_accessors;
mod resource_manager_traits;
mod surface_id;
mod surface_wrapper;

pub use crate::configured_resource_manager::*;
pub use crate::device_and_queue::DeviceAndQueue;
pub use crate::instance_wrapper::InstanceWrapper;
pub use crate::manager_id::WgpuResourceManagerId;
pub use crate::pixel_size::*;
pub use crate::resource_manager::WgpuResourceManager;
pub use crate::resource_manager_accessors::*;
pub use crate::surface_id::SurfaceId;
pub use crate::surface_wrapper::{ResizeSurfaceError, SurfaceWrapper};

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
fn init_test_tracing() {
    static INIT: std::sync::Once = std::sync::Once::new();

    INIT.call_once(|| {
        tracing_subscriber::fmt()
            .with_env_filter("wgpu_resource_manager=trace")
            .with_test_writer()
            .init();
    });
}
