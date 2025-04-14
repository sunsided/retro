use crate::wgpu::wgpu_hardware::resource::camera_uniform::CameraUniformResource;
use crate::wgpu::wgpu_hardware::resource::frame_global_uniform::FrameGlobalUniformResource;
use crate::wgpu::wgpu_hardware::resource::sky_renderer::SkyRendererResource;
use crate::wgpu::wgpu_hardware::resource::tile_renderer::TileRendererResource;
use crate::wgpu::wgpu_hardware::resource::WgpuResource;
use crate::wgpu::wgpu_hardware::resources::standard_render_layout::StandardRenderLayout;
use crate::wgpu::wgpu_hardware::resources::WgpuResources;
use crate::wgpu::wgpu_hardware::DeviceAndQueue;
use wgpu::{Device, SurfaceConfiguration};

#[derive(Debug)]
pub struct Renderers {
    pub tile: TileRendererResource,
    pub sky: SkyRendererResource,
}

impl WgpuResource for Renderers {
    fn recreate(&mut self, device: &DeviceAndQueue, resources: &WgpuResources) {
        *self = Self::new(
            device.device(),
            device.config(),
            resources.standard_render(),
            &resources.neo.uniforms.frame_globals,
            &resources.neo.uniforms.camera,
        );
    }
}

impl Renderers {
    pub fn new(
        device: &Device,
        config: &SurfaceConfiguration,
        standard_render: &StandardRenderLayout,
        frame_global: &FrameGlobalUniformResource,
        camera: &CameraUniformResource,
    ) -> Self {
        Self {
            tile: TileRendererResource::new(device, config, standard_render),
            sky: SkyRendererResource::new(device, config, frame_global, camera),
        }
    }
}
