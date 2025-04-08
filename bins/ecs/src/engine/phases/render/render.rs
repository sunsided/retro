use tracing::{debug, trace, warn};
use wgpu::{RenderPass, TextureView};
use crate::engine::{ApplyRenderSystem, BackgroundColorComponent, CreateSystem, ModelComponent, PositionComponent, RenderBackgroundSystem, RenderSystem, SystemFactory, TransformComponent};
use crate::engine::phases::render::{FrameState, WgpuRenderState};
use crate::wgpu::instance::{Instance, InstanceRaw};
use crate::wgpu::model::DrawModel;
use crate::wgpu::wgpu_hardware::{DeviceAndQueue, InstanceBuffer, RenderPipelineSelector};

#[derive(Debug)]
pub struct RenderSystemData {
    instance_buffer: Vec<InstanceRaw>,
}

impl Default for RenderSystemData {
    fn default() -> Self {
        Self {
            instance_buffer: Vec::with_capacity(InstanceBuffer::MAX_INSTANCES as usize),
        }
    }
}

impl CreateSystem<RenderSystem> for SystemFactory {
    fn create(&self) -> RenderSystem {
        RenderSystem::default()
    }
}

impl Default for RenderSystem {
    fn default() -> Self {
        Self(RenderSystemData::default())
    }
}

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

        let frame_entity_ids = frame_state.per_frame_entity_ids();
        let mut render_pass = frame_state.render_pass(&device);

        // Enable the instance buffer.
        render_pass.set_vertex_buffer(1, device.buffers().instances.slice(..));

        let max_instances = device.buffers().instances.len();

        // Fetch the (local) instance buffer and clear it in case something went wrong.
        let instance_buf = &mut self.instance_buffer;
        instance_buf.clear();

        for ((position, transforms), _models) in positions.iter().zip(transforms).zip(models) {
            let id = frame_entity_ids.next();
            let instance = Instance {
                position: position.as_vector3(),
                rotation: transforms.rotation,
                entity_id: id
            };
            instance_buf.push(instance.to_raw());

            // Render, then clear the local buffer.
            if instance_buf.len() >= max_instances {
                let num_instances = device.buffers().instances.send_to_device(&device.queue(), &instance_buf);
                render_pass.draw_model_instanced(&device.models().cube, 0..num_instances, &device.bind_groups().camera_uniform, &device.bind_groups().light_uniforms);
                instance_buf.clear();
            }
        }

        // Render the rest.
        if instance_buf.len() > 0 {
            let num_instances = device.buffers().instances.send_to_device(&device.queue(), &instance_buf);
            render_pass.draw_model_instanced(&device.models().cube, 0..num_instances, &device.bind_groups().camera_uniform, &device.bind_groups().light_uniforms);
            instance_buf.clear();
        }
    }
}
