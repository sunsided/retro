use std::convert::Infallible;
use tracing::{debug, trace};
use wgpu_resource_manager::{DeviceAndQueue, DeviceId};
use crate::engine::{ApplyWgpuInitShaderSystem, CreateSystem, EntityId, PhaseEvents, SystemFactory, SystemWgpuReinitPhaseEvents, WgpuInitShaderSystem, WgpuShaderComponent};
use crate::engine::phases::render::WgpuRenderState;

#[derive(Debug, Default)]
pub struct WgpuInitShaderSystemData {
    device_id: DeviceId
}

impl CreateSystem<WgpuInitShaderSystem> for SystemFactory {
    fn create(&self) -> WgpuInitShaderSystem {
        WgpuInitShaderSystem(WgpuInitShaderSystemData::default())
    }
}

impl ApplyWgpuInitShaderSystem for WgpuInitShaderSystem {
    type Error = Infallible;

    fn is_ready(&self, gpu: &mut WgpuRenderState) -> bool {
        gpu.is_ready()
    }

    fn apply_many(&mut self, gpu: &mut WgpuRenderState, entities: &[EntityId], shaders: &mut [WgpuShaderComponent]) {
        let Ok(device) = gpu.device() else {
            return;
        };

        let device_changed = device.id() != self.device_id;
        self.device_id = device.id();

        for (entity_id, shader) in entities.iter().zip(shaders) {
            // Skip over all already initialized shaders.
            if shader.module.is_some() && !device_changed {
                continue;
            }

            debug_log(device, entity_id, shader);
            let module = device.device().create_shader_module(shader.descriptor.clone());
            shader.module = Some(module);
        }
    }
}

fn debug_log(device: &DeviceAndQueue, entity_id: &EntityId, shader: &mut WgpuShaderComponent) {
    if tracing::enabled!(tracing::Level::TRACE) {
        debug!( // sic!
            %entity_id,
            shader_label = ?shader.descriptor.label,
            device_id = %device.id(),
            "Initializing shader"
        );
    } else {
        debug!(
            %entity_id,
            device_id = %device.id(),
            "Initializing shader"
        );
    }
}
