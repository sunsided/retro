use crate::wgpu::uniforms::CameraUniform;
use crate::wgpu::wgpu_hardware::resource::WgpuResource;
use crate::wgpu::wgpu_hardware::resources::WgpuResources;
use crate::wgpu::wgpu_hardware::DeviceAndQueue;
use wgpu::util::DeviceExt;
use wgpu::{BindGroup, BindGroupLayout, Buffer, Device, Queue};

/// A bind group for the camera uniforms.
///
/// ## Bindings
/// - `@binding(0)`: The camera uniforms
#[derive(Debug)]
pub struct CameraUniformResource {
    pub layout: BindGroupLayout,
    pub buffer: Buffer,
    pub bind_group: BindGroup,
}

impl<'bg> From<&'bg CameraUniformResource> for Option<&'bg BindGroup> {
    fn from(value: &'bg CameraUniformResource) -> Self {
        Some(&value.bind_group)
    }
}

impl WgpuResource for CameraUniformResource {
    fn recreate(&mut self, device: &DeviceAndQueue, _resources: &WgpuResources) {
        *self = Self::new(device.device());
    }
}

impl CameraUniformResource {
    pub fn new(device: &Device) -> Self {
        let frame_global_uniform = CameraUniform::default();
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(wgpu_label!("camera")),
            contents: bytemuck::cast_slice(&[frame_global_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some(wgpu_label!("camera")),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(size_of::<CameraUniform>() as _),
                },
                count: None,
            }],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(wgpu_label!("camera")),
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });

        Self {
            layout,
            buffer,
            bind_group,
        }
    }

    /// Update the buffer on device with a new [`CameraUniform`].
    #[inline]
    pub fn send_to_device(&self, queue: &Queue, uniform: CameraUniform) {
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[uniform]));
    }
}
