use crate::wgpu::wgpu_hardware::resource::tile_renderer::TileRendererResource;
use crate::wgpu::wgpu_hardware::resource::WgpuResource;
use crate::wgpu::wgpu_hardware::resources::standard_render_layout::StandardRenderLayout;
use crate::wgpu::wgpu_hardware::resources::WgpuResources;
use crate::wgpu::wgpu_hardware::DeviceAndQueue;
use wgpu::{Device, SurfaceConfiguration};

#[derive(Debug)]
pub struct Renderers {
    pub tile: TileRendererResource,
}

impl WgpuResource for Renderers {
    fn recreate(&mut self, device: &DeviceAndQueue, resources: &WgpuResources) {
        *self = Self::new(
            device.device(),
            device.config(),
            resources.standard_render(),
        );
    }
}

impl Renderers {
    pub fn new(
        device: &Device,
        config: &SurfaceConfiguration,
        standard_render: &StandardRenderLayout,
    ) -> Self {
        Self {
            tile: TileRendererResource::new(device, config, standard_render),
        }
    }
}
