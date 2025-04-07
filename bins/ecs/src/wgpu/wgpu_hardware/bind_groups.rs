use tracing::instrument;
use wgpu::{BindGroup, BindGroupLayout, Sampler, TextureView};
use crate::wgpu::wgpu_hardware::bind_group_layouts::BindGroupLayouts;
use crate::wgpu::wgpu_hardware::buffers::Buffers;
use crate::wgpu::wgpu_hardware::texture_samplers::TextureSamplers;
use crate::wgpu::wgpu_hardware::textures::Textures;

#[derive(Debug)]
pub struct BindGroups {
    pub camera: BindGroup,
    pub light_vb: BindGroup,
    #[allow(dead_code)]
    pub cover_tex: BindGroup,
    pub vga_fullscreen: BindGroup,
}

impl BindGroups {
    #[instrument(skip_all)]
    pub fn create_from(device: &wgpu::Device, layouts: &BindGroupLayouts, buffers: &Buffers, textures: &Textures, samplers: &TextureSamplers) -> Self {
        let camera = Self::create_camera(device, layouts, buffers);
        let light_vb = Self::create_light(device, layouts, buffers);
        let cover_tex = Self::create_cover(device, layouts, textures, samplers);
        let vga_fullscreen = Self::create_vga_fullscreen(device, layouts, textures, samplers, buffers);
        Self {
            camera,
            light_vb,
            cover_tex,
            vga_fullscreen
        }
    }

    #[instrument(skip_all)]
    fn create_cover(device: &wgpu::Device, layouts: &BindGroupLayouts, textures: &Textures, samplers: &TextureSamplers) -> BindGroup {
        Self::create_diffuse_and_normal(device, &layouts.diffuse_and_normal, &textures.cover.view, &samplers.linear, &textures.fallback_normals.view, &samplers.linear, Some("cover_bind_group"))
    }

    #[instrument(skip_all)]
    fn create_diffuse_and_normal(device: &wgpu::Device, layout: &BindGroupLayout, tex_view: &TextureView, tex_sampler: &Sampler, normal_view: &TextureView, normal_sampler: &Sampler, label: Option<&str>) -> BindGroup {
        device.create_bind_group(
            &wgpu::BindGroupDescriptor {
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
                    }
                ],
                label,
            })
    }

    #[instrument(skip_all)]
    fn create_camera(device: &wgpu::Device, layouts: &BindGroupLayouts, buffers: &Buffers) -> BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &layouts.camera_uniforms,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffers.camera.as_entire_binding(),
                }
            ],
            label: Some("camera_bind_group"),
        })
    }

    #[instrument(skip_all)]
    fn create_light(device: &wgpu::Device, layouts: &BindGroupLayouts, buffers: &Buffers) -> BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &layouts.lighting_uniforms,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffers.light_vb.as_entire_binding(),
                }
            ],
            label: Some("light_vb_bind_group"),
        })
    }

    #[instrument(skip_all)]
    fn create_vga_fullscreen(device: &wgpu::Device, layouts: &BindGroupLayouts, textures: &Textures, samplers: &TextureSamplers, buffers: &Buffers) -> BindGroup {
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
            ],
        })
    }
}
