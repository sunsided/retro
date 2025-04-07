use tracing::{debug, trace, warn};
use wgpu::{RenderPass, TextureView};
use crate::engine::{ApplyRenderSystem, BackgroundColorComponent, ModelComponent, PositionComponent, RenderBackgroundSystem, RenderSystem, TransformComponent};
use crate::engine::phases::render::{FrameState, WgpuRenderState};
use crate::wgpu::instance::Instance;
use crate::wgpu::model::DrawModel;
use crate::wgpu::wgpu_hardware::{DeviceAndQueue, RenderPipelineSelector};

impl ApplyRenderSystem for RenderSystem {
    type Error = wgpu::SurfaceError;

    #[inline]
    fn is_ready(&self, state: &mut WgpuRenderState) -> bool {
        state.is_ready()
    }

    fn apply_single(
        &mut self,
        _state: &mut WgpuRenderState,
        position: &PositionComponent,
        transform: &TransformComponent,
        model: &ModelComponent,
    ) {
        debug!("Rendering a single entity at {position:?}, {transform:?}.");
    }

    fn apply_many(&mut self, wgpu_render: &mut WgpuRenderState, positions: &[PositionComponent], transforms: &[TransformComponent], models: &[ModelComponent]) {
        if positions.is_empty() {
            return;
        }

        let retro_enabled = wgpu_render.retro_enabled();
        let Ok((device, frame_state)) = wgpu_render.for_rendering() else {
            warn!("Skipping background rendering because hardware or frame is not ready.");
            return;
        };

        let vga_offscreen = &device.textures().vga_offscreen.view;
        let vga_depth = &device.depths().vga_offscreen.view;
        let mut render_pass = if retro_enabled {
            Self::retro_first_pass(frame_state, &vga_offscreen, &vga_depth)
        } else {
            Self::quality_single_pass(frame_state, &device)
        };

        // Enable the instance buffer.
        render_pass.set_vertex_buffer(1, device.buffers().instances.slice(..));

        let max_instances = device.buffers().instances.len();
        let mut instance_buf = Vec::with_capacity(max_instances);
        for ((position, transforms), _models) in positions.iter().zip(transforms).zip(models) {
            let instance = Instance {
                position: position.as_vector3(),
                rotation: transforms.rotation,
            };
            instance_buf.push(instance.to_raw());

            // Render, then clear the local buffer.
            if instance_buf.len() >= max_instances {
                let num_instances = device.buffers().instances.send_to_device(&device.queue(), &instance_buf);
                render_pass.set_pipeline(device.pipelines().render_pipeline(RenderPipelineSelector::Textured));
                render_pass.draw_model_instanced(&device.models().cube, 0..num_instances, &device.bind_groups().camera, &device.bind_groups().light_vb);
                instance_buf.clear();
            }
        }

        // Render the rest.
        if instance_buf.len() > 0 {
            let num_instances = device.buffers().instances.send_to_device(&device.queue(), &instance_buf);
            render_pass.set_pipeline(device.pipelines().render_pipeline(RenderPipelineSelector::Textured));
            render_pass.draw_model_instanced(&device.models().cube, 0..num_instances, &device.bind_groups().camera, &device.bind_groups().light_vb);
        }
    }
}

impl RenderSystem {
    fn quality_single_pass<'pass>(frame_state: &'pass mut FrameState, device: &'pass DeviceAndQueue) -> RenderPass<'pass> {
        frame_state
            .encoder
            .begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Cube Render Pass"),
                color_attachments: &[
                    Some(wgpu::RenderPassColorAttachment {
                        view: &frame_state.view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: wgpu::StoreOp::Store,
                        },
                    })
                ],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &device.depths().screen.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            })
    }

    fn retro_first_pass<'pass>(frame_state: &'pass mut FrameState, vga_offscreen: &'pass TextureView, vga_depth: &'pass TextureView) -> RenderPass<'pass> {
        frame_state
            .encoder
            .begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("VGA Render Pass"),
                color_attachments: &[
                    Some(wgpu::RenderPassColorAttachment {
                        view: &vga_offscreen,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: wgpu::StoreOp::Store,
                        },
                    })
                ],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &vga_depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            })
    }
}
