pub mod render_targets;
pub mod textures;

use crate::wgpu::wgpu_hardware::resource::frame_global_uniform::FrameGlobalUniformResource;
use crate::wgpu::wgpu_hardware::resources::bind_group_layouts::BindGroupLayouts;
use crate::wgpu::wgpu_hardware::resources::bind_groups::render_targets::RenderTargetBindGroups;
use crate::wgpu::wgpu_hardware::resources::bind_groups::textures::TextureBindGroups;
use crate::wgpu::wgpu_hardware::resources::depth_buffers::DepthBuffers;
use crate::wgpu::wgpu_hardware::resources::texture_samplers::TextureSamplers;
use crate::wgpu::wgpu_hardware::resources::textures::Textures;
use tracing::instrument;

#[derive(Debug)]
pub struct BindGroups {
    pub textures: TextureBindGroups,
    pub render_targets: RenderTargetBindGroups,
}

impl BindGroups {
    #[instrument(skip_all)]
    pub fn create_from(
        device: &wgpu::Device,
        layouts: &BindGroupLayouts,
        textures: &Textures,
        samplers: &TextureSamplers,
        depths: &DepthBuffers,
        frame_global: &FrameGlobalUniformResource,
    ) -> Self {
        let textures_bgs = TextureBindGroups::create_from(device, layouts, textures, samplers);
        let render_targets = RenderTargetBindGroups::create_from(
            device,
            layouts,
            textures,
            samplers,
            depths,
            frame_global,
        );
        Self {
            textures: textures_bgs,
            render_targets,
        }
    }
}
