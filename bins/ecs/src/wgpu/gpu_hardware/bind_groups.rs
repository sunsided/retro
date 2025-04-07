use tracing::instrument;
use wgpu::BindGroup;
use crate::wgpu::gpu_hardware::bind_group_layouts::BindGroupLayouts;
use crate::wgpu::gpu_hardware::buffers::Buffers;

#[derive(Debug)]
pub struct BindGroups {
    pub camera: BindGroup,
    pub light_vb: BindGroup,
}

impl BindGroups {
    #[instrument(skip(device, layouts, buffers))]
    pub fn create_from(device: &wgpu::Device, layouts: &BindGroupLayouts, buffers: &Buffers) -> Self {
        let camera = Self::create_camera(device, layouts, buffers);
        let light_vb = Self::create_light(device, layouts, buffers);
        Self {
            camera,
            light_vb
        }
    }

    #[instrument(skip(device, layouts, buffers))]
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

    #[instrument(skip(device, layouts, buffers))]
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
}
