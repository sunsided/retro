use crate::wgpu::wgpu_hardware::bind_group_layouts::BindGroupLayouts;
use crate::wgpu::wgpu_hardware::buffers::Buffers;
use crate::wgpu::wgpu_hardware::depth_buffers::DepthBuffers;
use crate::wgpu::wgpu_hardware::texture_samplers::TextureSamplers;
use crate::wgpu::wgpu_hardware::textures::Textures;
use tracing::instrument;
use wgpu::{BindGroup, BindGroupLayout, Sampler, TextureView};

#[derive(Debug)]
pub struct BindGroups {
    pub frame_global_uniform: BindGroup,
    pub camera_uniform: BindGroup,
    pub light_uniforms: BindGroup,
    #[allow(dead_code)]
    pub cover_tex: BindGroup,
    pub vga_fullscreen: BindGroup,
    pub bricks: BindGroup,
    pub stonepath: BindGroup,
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
        let frame_global_uniform = Self::create_frame_global_uniform(device, layouts, buffers);
        let camera_uniform = Self::create_camera_uniform(device, layouts, buffers);
        let light_uniforms = Self::create_light_uniform(device, layouts, buffers);
        let cover_tex = Self::create_cover(device, layouts, textures, samplers);
        let vga_fullscreen =
            Self::create_vga_fullscreen(device, layouts, textures, samplers, buffers, depths);
        let bricks = Self::create_bricks(device, layouts, textures, samplers);
        let stonepath = Self::create_stonepath(device, layouts, textures, samplers);
        Self {
            frame_global_uniform,
            camera_uniform,
            light_uniforms,
            cover_tex,
            vga_fullscreen,
            bricks,
            stonepath,
        }
    }

    #[instrument(skip_all)]
    fn create_frame_global_uniform(
        device: &wgpu::Device,
        layouts: &BindGroupLayouts,
        buffers: &Buffers,
    ) -> BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("frame_global"),
            layout: &layouts.frame_global_uniform,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffers.frame_global_uniform.as_entire_binding(),
            }],
        })
    }

    #[instrument(skip_all)]
    fn create_cover(
        device: &wgpu::Device,
        layouts: &BindGroupLayouts,
        textures: &Textures,
        samplers: &TextureSamplers,
    ) -> BindGroup {
        Self::create_diffuse_and_normal(
            device,
            &layouts.diffuse_and_normal,
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
        layouts: &BindGroupLayouts,
        textures: &Textures,
        samplers: &TextureSamplers,
    ) -> BindGroup {
        Self::create_diffuse_and_normal(
            device,
            &layouts.diffuse_and_normal,
            &textures.bricks_diffuse.view,
            &samplers.linear,
            &textures.bricks_normal.view,
            &samplers.linear,
            Some("cover_bind_group"),
        )
    }

    #[instrument(skip_all)]
    fn create_stonepath(
        device: &wgpu::Device,
        layouts: &BindGroupLayouts,
        textures: &Textures,
        samplers: &TextureSamplers,
    ) -> BindGroup {
        Self::create_diffuse_and_normal(
            device,
            &layouts.diffuse_and_normal,
            &textures.stonepath_diffuse.view,
            &samplers.linear,
            &textures.stonepath_normal.view,
            &samplers.linear,
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

    #[instrument(skip_all)]
    fn create_camera_uniform(
        device: &wgpu::Device,
        layouts: &BindGroupLayouts,
        buffers: &Buffers,
    ) -> BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &layouts.camera_uniforms,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffers.camera_uniform.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        })
    }

    #[instrument(skip_all)]
    fn create_light_uniform(
        device: &wgpu::Device,
        layouts: &BindGroupLayouts,
        buffers: &Buffers,
    ) -> BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &layouts.lighting_uniforms,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffers.light_uniforms.as_entire_binding(),
            }],
            label: Some("light_uniform_bind_group"),
        })
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
                    resource: wgpu::BindingResource::Sampler(&samplers.retro_sampler),
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
