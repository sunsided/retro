use crate::engine::phases::render::WgpuRenderState;
use crate::engine::{
    ApplyWgpuInitShaderSystem, CreateSystem, PhaseEvents, ResourceManagerState, SystemFactory,
    SystemWgpuReinitPhaseEvents, WgpuInitShaderSystem, WgpuShaderComponent, WgpuShaderData,
};
use ahash::HashSet;
use fxhash::{FxHashMap, FxHashSet};
use sillyecs::EntityId;
use std::convert::Infallible;
use tracing::{debug, trace};
use wgpu::ShaderModule;
use wgpu_resource_manager::{DeviceAndQueue, DeviceId};

#[derive(Debug, Default)]
pub struct WgpuInitShaderSystemData {
    device_id: DeviceId,
    updated: FxHashMap<EntityId, ShaderModule>,
}

impl CreateSystem<WgpuInitShaderSystem> for SystemFactory {
    fn create(&self) -> WgpuInitShaderSystem {
        WgpuInitShaderSystem(WgpuInitShaderSystemData::default())
    }
}

impl ApplyWgpuInitShaderSystem for WgpuInitShaderSystem {
    type Error = Infallible;

    fn is_ready(&self, gpu: &WgpuRenderState) -> bool {
        gpu.is_ready()
    }

    fn on_begin_phase(&mut self) -> Result<(), Self::Error> {
        self.updated.clear();
        Ok(())
    }

    fn apply_many(
        &mut self,
        gpu: &WgpuRenderState,
        entities: &[EntityId],
        shaders: &mut [WgpuShaderComponent],
    ) {
        let Ok(device) = gpu.device() else {
            return;
        };

        let current_device_id = device.id();
        let device_changed = current_device_id != self.device_id;
        self.device_id = current_device_id;

        for (&entity_id, shader) in entities.iter().zip(shaders) {
            // Skip over all already initialized shaders.
            if shader.module.is_some() && !device_changed {
                continue;
            }

            debug_log(&entity_id, device, shader);
            let module = device
                .device()
                .create_shader_module(shader.descriptor.clone());

            // Remember the shader for updates.
            self.updated.insert(entity_id, module.clone());

            shader.module = Some(module);
        }
    }

    fn postflight(
        &mut self,
        resource_manager: &mut ResourceManagerState,
        entities: &[EntityId],
        _wgpu_shaders: &[WgpuShaderComponent],
    ) {
        // Remove all resources that do not exist anymore.
        let to_retain = FxHashSet::from_iter(entities.iter().copied());
        resource_manager
            .shaders
            .retain(|k, _| to_retain.contains(k));
    }

    fn on_end_phase(
        &mut self,
        resource_manager: &mut ResourceManagerState,
    ) -> Result<(), Self::Error> {
        for (entity_id, shader) in self.updated.drain() {
            resource_manager.shaders.insert(entity_id, shader);
        }
        Ok(())
    }
}

fn debug_log(entity_id: &EntityId, device: &DeviceAndQueue, shader: &mut WgpuShaderComponent) {
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
