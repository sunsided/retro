use crate::wgpu::uniforms::RetroUniform;
use crate::wgpu::wgpu_hardware::resource::WgpuResource;
use wgpu::util::DeviceExt;
use wgpu::{BindGroup, BindGroupLayout, BindingResource, Buffer, Device, Queue};

#[derive(Debug)]
pub struct RetroUniformResource {
    pub layout: BindGroupLayout,
    pub buffer: Buffer,
    pub bind_group: BindGroup,
}

impl<'bg> From<&'bg RetroUniformResource> for Option<&'bg BindGroup> {
    fn from(value: &'bg RetroUniformResource) -> Self {
        Some(&value.bind_group)
    }
}

impl WgpuResource for RetroUniformResource {
    fn recreate(&mut self, device: &Device) {
        *self = Self::new(device);
    }
}

impl RetroUniformResource {
    pub fn new(device: &Device) -> Self {
        let frame_global_uniform = RetroUniform::default();
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(wgpu_label!("retro")),
            contents: bytemuck::cast_slice(&[frame_global_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some(wgpu_label!("retro")),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(size_of::<RetroUniform>() as _),
                },
                count: None,
            }],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(wgpu_label!("retro")),
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

    /// Update the buffer on device with a new [`RetroUniform`].
    #[inline]
    pub fn send_to_device(&self, queue: &Queue, uniform: RetroUniform) {
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[uniform]));
    }

    #[inline]
    #[allow(dead_code)]
    pub fn as_entire_binding(&self) -> BindingResource<'_> {
        self.buffer.as_entire_binding()
    }
}
