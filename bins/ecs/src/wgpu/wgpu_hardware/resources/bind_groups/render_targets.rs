use crate::wgpu::wgpu_hardware::resources::bind_group_layouts::BindGroupLayouts;
use crate::wgpu::wgpu_hardware::resources::buffers::Buffers;
use crate::wgpu::wgpu_hardware::resources::depth_buffers::DepthBuffers;
use crate::wgpu::wgpu_hardware::resources::texture_samplers::TextureSamplers;
use crate::wgpu::wgpu_hardware::resources::textures::Textures;
use tracing::instrument;
use wgpu::BindGroup;

#[derive(Debug)]
pub struct RenderTargetBindGroups {
    pub vga_fullscreen: BindGroup,
}

impl RenderTargetBindGroups {
    #[instrument(skip_all)]
    pub fn create_from(
        device: &wgpu::Device,
        layouts: &BindGroupLayouts,
        buffers: &Buffers,
        textures: &Textures,
        samplers: &TextureSamplers,
        depths: &DepthBuffers,
    ) -> Self {
        let vga_fullscreen =
            Self::create_vga_fullscreen(device, layouts, textures, samplers, buffers, depths);
        Self { vga_fullscreen }
    }

    #[instrument(skip_all)]
    fn create_vga_fullscreen(
        device: &wgpu::Device,
        layouts: &BindGroupLayouts,
        textures: &Textures,
        samplers: &TextureSamplers,
        buffers: &Buffers,
        depths: &DepthBuffers,
    ) -> BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("vga_fullscreen"),
            layout: &layouts.vga_fullscreen,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffers.retro_uniform.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&textures.vga_offscreen.view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&samplers.retro),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::TextureView(
                        &textures.vga_frame_entity_id.view,
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: wgpu::BindingResource::TextureView(&depths.vga_offscreen.view),
                },
            ],
        })
    }
}
