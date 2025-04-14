use crate::wgpu::wgpu_hardware::resources::bind_group_layouts::BindGroupLayouts;
use crate::wgpu::wgpu_hardware::resources::buffers::Buffers;
use tracing::instrument;
use wgpu::BindGroup;

#[derive(Debug)]
pub struct UniformBindGroups {
    pub camera_uniform: BindGroup,
    pub light_uniforms: BindGroup,
}

impl UniformBindGroups {
    #[instrument(skip_all)]
    pub fn create_from(
        device: &wgpu::Device,
        layouts: &BindGroupLayouts,
        buffers: &Buffers,
    ) -> Self {
        let camera_uniform = Self::create_camera_uniform(device, layouts, buffers);
        let light_uniforms = Self::create_light_uniform(device, layouts, buffers);
        Self {
            camera_uniform,
            light_uniforms,
        }
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
}
