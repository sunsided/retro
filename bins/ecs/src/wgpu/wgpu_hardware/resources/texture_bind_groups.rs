use crate::wgpu::wgpu_hardware::resources::standard_render_layout::StandardRenderLayout;
use crate::wgpu::wgpu_hardware::resources::texture_samplers::TextureSamplers;
use crate::wgpu::wgpu_hardware::resources::textures::Textures;
use tracing::instrument;
use wgpu::{BindGroup, BindGroupLayout, Sampler, TextureView};

#[derive(Debug)]
pub struct TextureBindGroups {
    #[allow(dead_code)]
    pub cover_tex: BindGroup,
    pub bricks: BindGroup,
    pub stonepath: BindGroup,
}

impl TextureBindGroups {
    #[instrument(skip_all)]
    pub fn create_from(
        device: &wgpu::Device,
        layout: &StandardRenderLayout,
        textures: &Textures,
        samplers: &TextureSamplers,
    ) -> Self {
        let cover_tex = Self::create_cover(device, layout, textures, samplers);
        let bricks = Self::create_bricks(device, layout, textures, samplers);
        let stonepath = Self::create_stonepath(device, layout, textures, samplers);
        Self {
            cover_tex,
            bricks,
            stonepath,
        }
    }

    #[instrument(skip_all)]
    fn create_cover(
        device: &wgpu::Device,
        layout: &StandardRenderLayout,
        textures: &Textures,
        samplers: &TextureSamplers,
    ) -> BindGroup {
        Self::create_diffuse_and_normal(
            device,
            &layout.texture_bind_group,
            &textures.cover.view,
            &samplers.linear,
            &textures.fallback_normals.view,
            &samplers.linear,
            Some("cover_bind_group"),
        )
    }

    #[instrument(skip_all)]
    fn create_bricks(
        device: &wgpu::Device,
        layout: &StandardRenderLayout,
        textures: &Textures,
        samplers: &TextureSamplers,
    ) -> BindGroup {
        Self::create_diffuse_and_normal(
            device,
            &layout.texture_bind_group,
            &textures.bricks_diffuse.view,
            &samplers.tile,
            &textures.bricks_normal.view,
            &samplers.normals,
            Some("cover_bind_group"),
        )
    }

    #[instrument(skip_all)]
    fn create_stonepath(
        device: &wgpu::Device,
        layout: &StandardRenderLayout,
        textures: &Textures,
        samplers: &TextureSamplers,
    ) -> BindGroup {
        Self::create_diffuse_and_normal(
            device,
            &layout.texture_bind_group,
            &textures.stonepath_diffuse.view,
            &samplers.tile,
            &textures.stonepath_normal.view,
            &samplers.normals,
            Some("stonepath"),
        )
    }

    #[instrument(skip_all)]
    fn create_diffuse_and_normal(
        device: &wgpu::Device,
        layout: &BindGroupLayout,
        tex_view: &TextureView,
        tex_sampler: &Sampler,
        normal_view: &TextureView,
        normal_sampler: &Sampler,
        label: Option<&str>,
    ) -> BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout,
            entries: &[
                // Diffuse map
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(tex_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(tex_sampler),
                },
                // Normal map
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(normal_view),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::Sampler(normal_sampler),
                },
            ],
            label,
        })
    }
}
