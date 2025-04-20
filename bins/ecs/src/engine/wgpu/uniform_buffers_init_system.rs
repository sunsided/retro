use crate::engine::phases::render::WgpuRenderState;
use crate::engine::{
    ApplyWgpuInitShaderSystem, ApplyWgpuInitUniformBindGroupSystem, CreateSystem, PhaseEvents,
    ResourceManagerState, SystemFactory, SystemWgpuReinitPhaseEvents, UniformTypeComponent,
    UniformTypeData, WgpuBindGroupComponent, WgpuBindGroupLayoutComponent, WgpuBufferComponent,
    WgpuInitShaderSystem, WgpuInitUniformBindGroupSystem, WgpuShaderComponent, WgpuShaderData,
};
use crate::wgpu::uniforms::{CameraUniform, FrameGlobalUniform, LightUniform};
use ahash::HashSet;
use fxhash::{FxHashMap, FxHashSet};
use sillyecs::EntityId;
use std::convert::Infallible;
use tracing::{debug, trace};
use wgpu::util::DeviceExt;
use wgpu::{BindGroup, Buffer, ShaderModule};
use wgpu_resource_manager::{wgpu_label, DeviceAndQueue, DeviceId};

#[derive(Debug, Default)]
pub struct WgpuInitUniformBindGroupSystemData {
    device_id: DeviceId,
    updated: FxHashMap<EntityId, BindGroup>,
}

impl CreateSystem<WgpuInitUniformBindGroupSystem> for SystemFactory {
    fn create(&self) -> WgpuInitUniformBindGroupSystem {
        WgpuInitUniformBindGroupSystem(WgpuInitUniformBindGroupSystemData::default())
    }
}

impl ApplyWgpuInitUniformBindGroupSystem for WgpuInitUniformBindGroupSystem {
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
        uniform_types: &[UniformTypeComponent],
        wgpu_bind_group_layouts: &mut [WgpuBindGroupLayoutComponent],
        wgpu_bind_groups: &mut [WgpuBindGroupComponent],
        wgpu_buffers: &mut [WgpuBufferComponent],
    ) {
        let Ok(device) = gpu.device() else {
            return;
        };

        let current_device_id = device.id();
        let device_changed = current_device_id != self.device_id;
        self.device_id = current_device_id;

        for ((((entity_id, type_comp), layout_comp), group_comp), buffer_comp) in entities
            .iter()
            .zip(uniform_types)
            .zip(wgpu_bind_group_layouts)
            .zip(wgpu_bind_groups)
            .zip(wgpu_buffers)
        {
            // Skip over all already initialized shaders.
            if buffer_comp.buffer.is_some() && !device_changed {
                continue;
            }

            let (slice, binding_size, label) = match type_comp.0 {
                UniformTypeData::FrameGlobal(data) => (
                    bytemuck::cast_slice(&[data]),
                    wgpu::BufferSize::new(size_of::<FrameGlobalUniform>() as _),
                    wgpu_label!("frame_global"),
                ),
                UniformTypeData::Camera(data) => (
                    bytemuck::cast_slice(&[data]),
                    wgpu::BufferSize::new(size_of::<CameraUniform>() as _),
                    wgpu_label!("camera"),
                ),
                UniformTypeData::Lighting(data) => (
                    bytemuck::cast_slice(&[data]),
                    wgpu::BufferSize::new(size_of::<LightUniform>() as _),
                    wgpu_label!("lighting"),
                ),
            };

            debug_log(entity_id, device);
            let desc = wgpu::util::BufferInitDescriptor {
                label: Some(label),
                contents: slice,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            };

            let buffer = device.device().create_buffer_init(&desc);

            let layout =
                device
                    .device()
                    .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                        label: Some(label),
                        entries: &[wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                has_dynamic_offset: false,
                                min_binding_size: binding_size,
                            },
                            count: None,
                        }],
                    });

            // TODO: Rework this; have a single bind group for all uniforms, but keep the buffers separate
            let bind_group = device
                .device()
                .create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some(label),
                    layout: &layout,
                    entries: &[wgpu::BindGroupEntry {
                        binding: 0,
                        resource: buffer.as_entire_binding(),
                    }],
                });

            // Remember the shader for updates.
            self.updated.insert(*entity_id, bind_group.clone());

            group_comp.group = Some(bind_group);
            layout_comp.layout = Some(layout);
            buffer_comp.buffer = Some(buffer);
        }
    }

    fn postflight(
        &mut self,
        resource_manager: &mut ResourceManagerState,
        entities: &[EntityId],
        _uniform_types: &[UniformTypeComponent],
        _wgpu_bind_group_layouts: &[WgpuBindGroupLayoutComponent],
        _wgpu_bind_groups: &[WgpuBindGroupComponent],
        _wgpu_buffers: &[WgpuBufferComponent],
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
        for (entity_id, bind_group) in self.updated.drain() {
            resource_manager.bind_groups.insert(entity_id, bind_group);
        }
        Ok(())
    }
}

fn debug_log(entity_id: &EntityId, device: &DeviceAndQueue) {
    debug!(
        %entity_id,
        device_id = %device.id(),
        "Initializing uniform buffer"
    );
}
