use crate::wgpu::uniforms::LightUniform;
use crate::wgpu::wgpu_hardware::resource::WgpuResource;
use wgpu::util::DeviceExt;
use wgpu::{BindGroup, BindGroupLayout, Buffer, Device, Queue};

/// A bind group for the lighting uniforms.
///
/// ## Bindings
/// - `@binding(0)`: The lighting uniforms
#[derive(Debug)]
pub struct LightUniformResource {
    pub layout: BindGroupLayout,
    pub buffer: Buffer,
    pub bind_group: BindGroup,
}

impl<'bg> From<&'bg LightUniformResource> for Option<&'bg BindGroup> {
    fn from(value: &'bg LightUniformResource) -> Self {
        Some(&value.bind_group)
    }
}

impl WgpuResource for LightUniformResource {
    fn recreate(&mut self, device: &Device) {
        *self = Self::new(device);
    }
}

impl LightUniformResource {
    pub fn new(device: &Device) -> Self {
        let light_uniform = LightUniform::default();
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(wgpu_label!("light")),
            contents: bytemuck::cast_slice(&[light_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some(wgpu_label!("light")),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(size_of::<LightUniform>() as _),
                },
                count: None,
            }],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(wgpu_label!("light")),
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

    /// Update the buffer on device with a new [`LightUniform`].
    #[inline]
    pub fn send_to_device(&self, queue: &Queue, uniform: LightUniform) {
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[uniform]));
    }
}

impl AsRef<BindGroup> for LightUniformResource {
    fn as_ref(&self) -> &BindGroup {
        &self.bind_group
    }
}
