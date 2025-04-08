use std::ops::{Deref, DerefMut};
use wgpu::{CommandEncoder, RenderPass, SurfaceTexture, TextureView};
use crate::engine::BackgroundColorComponent;
use crate::wgpu::wgpu_hardware::DeviceAndQueue;

pub struct RenderPasses {
    encoder: CommandEncoder,
    surface_tex: SurfaceTexture,
    surface_view: TextureView,
    vga_offscreen_view: TextureView,
    vga_depth_view: TextureView,
    retro_enabled: bool,
    surface_depth_view: TextureView,
    surface_frame_entity_id_view: TextureView,
    vga_frame_entity_id_view: TextureView,
}

impl Deref for RenderPasses {
    type Target = CommandEncoder;

    fn deref(&self) -> &Self::Target {
        &self.encoder
    }
}

impl DerefMut for RenderPasses {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.encoder
    }
}

impl RenderPasses {
    pub fn new(device: &DeviceAndQueue, retro_enabled: bool, encoder: CommandEncoder, surface_tex: SurfaceTexture, surface_view: TextureView) -> Self {
        let surface_depth_view = device.depths().screen.view.clone();
        let surface_frame_entity_id_view = device.textures().surface_frame_entity_id.view.clone();
        let vga_frame_entity_id_view = device.textures().vga_frame_entity_id.view.clone();
        let vga_offscreen_view = device.textures().vga_offscreen.view.clone();
        let vga_depth_view = device.depths().vga_offscreen.view.clone();

        Self {
            encoder,
            surface_tex,
            surface_view,
            surface_depth_view,
            surface_frame_entity_id_view,
            vga_frame_entity_id_view,
            vga_offscreen_view,
            vga_depth_view,
            retro_enabled,
        }
    }

    pub fn finish_and_present(self, device: &DeviceAndQueue) {
        let command_buffer = self.encoder.finish();
        device.submit_to_queue(command_buffer);
        self.surface_tex.present();
    }

    pub fn clear_background_pass(&mut self, background_color: &BackgroundColorComponent) -> RenderPass {
        if self.retro_enabled {
            self.create_retro_clear_pass(background_color)
        } else {
            self.create_quality_clear_pass(background_color)
        }
    }

    pub fn render_pass(&mut self) -> RenderPass {
        if self.retro_enabled {
            self.create_retro_render_pass()
        } else {
            self.create_quality_render_pass()
        }
    }

    pub fn final_pass(&mut self, background_color: &BackgroundColorComponent) -> Option<RenderPass> {
        if self.retro_enabled {
            Some(self.create_final_pass(background_color))
        } else {
            None
        }
    }

    fn create_quality_clear_pass(&mut self, background_color: &BackgroundColorComponent) -> RenderPass {
        Self::create_render_pass(
            &mut self.encoder,
            Some(background_color),
            &self.surface_view,
            Some(&self.surface_frame_entity_id_view),
            &self.surface_depth_view,
            "clear_background"
        )
    }

    fn create_retro_clear_pass(&mut self, background_color: &BackgroundColorComponent) -> RenderPass {
        Self::create_render_pass(
            &mut self.encoder,
            Some(background_color),
            &self.vga_offscreen_view,
            Some(&self.vga_frame_entity_id_view),
            &self.vga_depth_view,
            "vga_first_pass")
    }

    fn create_quality_render_pass(&mut self) -> RenderPass {
        Self::create_render_pass(
            &mut self.encoder,
            None,
            &self.surface_view,
            Some(&self.surface_frame_entity_id_view),
            &self.surface_depth_view,
            "quality_render"
        )
    }

    fn create_retro_render_pass(&mut self) -> RenderPass {
        Self::create_render_pass(
            &mut self.encoder,
            None,
            &self.vga_offscreen_view,
            Some(&self.vga_frame_entity_id_view),
            &self.vga_depth_view,
            "vga_render")
    }

    fn create_render_pass<'encoder>(encoder: &'encoder mut CommandEncoder, background_color: Option<&BackgroundColorComponent>, tex_view: &TextureView, entity_view: Option<&TextureView>, depth_view: &TextureView, label: &'static str) -> RenderPass<'encoder> {
        let tex_load = if let Some(background_color) = background_color {
            wgpu::LoadOp::Clear(background_color.as_wgpu_color())
        } else {
            wgpu::LoadOp::Load
        };

        let depth_load = if background_color.is_some() {
            wgpu::LoadOp::Clear(1.0)
        } else {
            wgpu::LoadOp::Load
        };

        let entity_id_attachment = entity_view.map(|view| {
            wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: tex_load,
                    store: wgpu::StoreOp::Store,
                },
            }
        });

        encoder
            .begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some(label),
                color_attachments: &[
                    Some(wgpu::RenderPassColorAttachment {
                        view: tex_view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: tex_load,
                            store: wgpu::StoreOp::Store,
                        },
                    }),
                    entity_id_attachment
                ],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: depth_load,
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            })
    }

    fn create_final_pass(&mut self, background_color: &BackgroundColorComponent) -> RenderPass {
        self.encoder
            .begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("vga_second_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.surface_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(background_color.as_wgpu_color()),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            })
    }
}
