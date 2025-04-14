use crate::wgpu::wgpu_hardware::resource::camera_uniform::CameraUniformResource;
use crate::wgpu::wgpu_hardware::resource::frame_global_uniform::FrameGlobalUniformResource;
use crate::wgpu::wgpu_hardware::resource::light_uniform::LightUniformResource;
use crate::wgpu::wgpu_hardware::resource::sky_renderer::SkyRendererResource;
use crate::wgpu::wgpu_hardware::resource::tile_renderer::TileRendererResource;
use crate::wgpu::wgpu_hardware::resource::vga_fullscreen_renderer::VgaFullscreenRendererResource;
use crate::wgpu::wgpu_hardware::resources::standard_render_layout::StandardRenderLayout;
use wgpu::{Device, SurfaceConfiguration};

#[derive(Debug)]
pub struct Renderers {
    pub tile: TileRendererResource,
    pub sky: SkyRendererResource,
    pub vga_fullscreen: VgaFullscreenRendererResource,
}

impl Renderers {
    pub fn new(
        device: &Device,
        config: &SurfaceConfiguration,
        standard_render: &StandardRenderLayout,
        frame_global: &FrameGlobalUniformResource,
        camera: &CameraUniformResource,
        light: &LightUniformResource,
    ) -> Self {
        Self {
            tile: TileRendererResource::new(device, config, standard_render),
            sky: SkyRendererResource::new(device, config, frame_global, camera),
            vga_fullscreen: VgaFullscreenRendererResource::new(
                device,
                config,
                frame_global,
                camera,
                light,
            ),
        }
    }
}
