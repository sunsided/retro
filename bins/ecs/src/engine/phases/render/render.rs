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

    fn apply_many(&mut self, wgpu_render: &mut WgpuRenderState, positions: &[PositionComponent], transforms: &[TransformComponent], models: &[ModelComponent]) {
        if positions.is_empty() {
            return;
        }

        let Ok((device, frame_state)) = wgpu_render.for_rendering() else {
            warn!("Skipping background rendering because hardware or frame is not ready.");
            return;
        };

        let mut render_pass = frame_state.passes.render_pass();

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
