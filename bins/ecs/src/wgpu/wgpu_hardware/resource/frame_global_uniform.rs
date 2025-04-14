use crate::wgpu::uniforms::CameraUniform;
use crate::wgpu::wgpu_hardware::resource::WgpuResource;
use wgpu::util::DeviceExt;
use wgpu::{BindGroup, BindGroupLayout, Buffer, Device, Queue};

/// Frame-global uniforms.
///
/// ## Bindings
/// - `@binding(0)`: The frame-global uniforms
#[derive(Debug)]
pub struct FrameGlobalUniform {
    pub layout: BindGroupLayout,
    pub buffer: Buffer,
    pub bind_group: BindGroup,
}

impl WgpuResource for FrameGlobalUniform {
    fn recreate(&mut self, device: &Device) {
        *self = Self::new(device);
    }
}

impl FrameGlobalUniform {
    pub fn new(device: &Device) -> Self {
        let frame_global_uniform = crate::wgpu::uniforms::FrameGlobalUniform::new(12.0);
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(wgpu_label!("frame_global")),
            contents: bytemuck::cast_slice(&[frame_global_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some(wgpu_label!("frame_global")),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(size_of::<
                        crate::wgpu::uniforms::FrameGlobalUniform,
                    >() as _),
                },
                count: None,
            }],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(wgpu_label!("frame_global")),
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

    /// Update the buffer on device with a new [`FrameGlobalUniform`](crate::wgpu::uniforms::FrameGlobalUniform).
    #[inline]
    pub fn send_to_device(
        &self,
        queue: &Queue,
        uniform: crate::wgpu::uniforms::FrameGlobalUniform,
    ) {
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[uniform]));
    }
}
