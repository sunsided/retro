pub mod render_targets;
pub mod textures;
pub mod uniforms;

use crate::wgpu::wgpu_hardware::resources::bind_group_layouts::BindGroupLayouts;
use crate::wgpu::wgpu_hardware::resources::bind_groups::render_targets::RenderTargetBindGroups;
use crate::wgpu::wgpu_hardware::resources::bind_groups::textures::TextureBindGroups;
use crate::wgpu::wgpu_hardware::resources::bind_groups::uniforms::UniformBindGroups;
use crate::wgpu::wgpu_hardware::resources::buffers::Buffers;
use crate::wgpu::wgpu_hardware::resources::depth_buffers::DepthBuffers;
use crate::wgpu::wgpu_hardware::resources::texture_samplers::TextureSamplers;
use crate::wgpu::wgpu_hardware::resources::textures::Textures;
use tracing::instrument;

#[derive(Debug)]
pub struct BindGroups {
    pub textures: TextureBindGroups,
    pub uniforms: UniformBindGroups,
    pub render_targets: RenderTargetBindGroups,
}

impl BindGroups {
    #[instrument(skip_all)]
    pub fn create_from(
        device: &wgpu::Device,
        layouts: &BindGroupLayouts,
        buffers: &Buffers,
        textures: &Textures,
        samplers: &TextureSamplers,
        depths: &DepthBuffers,
    ) -> Self {
        let textures_bgs = TextureBindGroups::create_from(device, layouts, textures, samplers);
        let uniforms = UniformBindGroups::create_from(device, layouts, buffers);
        let render_targets = RenderTargetBindGroups::create_from(
            device, layouts, buffers, textures, samplers, depths,
        );
        Self {
            textures: textures_bgs,
            uniforms,
            render_targets,
        }
    }
}
