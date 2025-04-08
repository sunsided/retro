use std::ops::RangeBounds;
use tracing::instrument;
use wgpu::{BindingResource, Buffer, BufferAddress, BufferSlice, Queue};
use wgpu::util::DeviceExt;
use crate::wgpu::camera_uniform::CameraUniform;
use crate::wgpu::instance::InstanceRaw;
use crate::wgpu::light::LightUniform;
use crate::wgpu::retro_uniform::RetroUniform;

#[derive(Debug)]
pub struct Buffers {
    pub camera: CameraBuffer,
    pub light_vb: LightBuffer,
    pub instances: InstanceBuffer,
    pub retro_uniform: RetroUniformBuffer,
}

impl Buffers {
    #[instrument(skip(device))]
    pub fn create_from(device: &wgpu::Device) -> Self {
        let camera = CameraBuffer::create(device);
        let light_vb = LightBuffer::create(device);
        let instances = InstanceBuffer::create(device);
        let retro_uniform = RetroUniformBuffer::create(device);
        Self {
            camera,
            light_vb,
            instances,
            retro_uniform
        }
    }
}

#[derive(Debug)]
pub struct CameraBuffer {
    buffer: Buffer,
}

impl CameraBuffer {
    #[instrument(skip(device))]
    fn create(device: &wgpu::Device) -> Self {
        let camera_uniform = CameraUniform::new();
        let buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("camera_buffer"),
                contents: bytemuck::cast_slice(&[camera_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );
        Self { buffer }
    }

    /// Update the buffer on device with a new [`CameraUniform`].
    #[inline]
    pub fn send_to_device(&self, queue: &Queue, uniform: CameraUniform) {
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[uniform]));
    }

    #[inline]
    pub fn as_entire_binding(&self) -> BindingResource<'_> {
        self.buffer.as_entire_binding()
    }

    #[inline]
    #[allow(dead_code)]
    pub fn slice<S: RangeBounds<BufferAddress>>(&self, bounds: S) -> BufferSlice<'_> {
        self.buffer.slice(bounds)
    }
}

impl AsRef<Buffer> for CameraBuffer {
    fn as_ref(&self) -> &Buffer {
        &self.buffer
    }
}

#[derive(Debug)]
pub struct LightBuffer {
    buffer: Buffer,
}

impl LightBuffer {
    #[instrument(skip(device))]
    fn create(device: &wgpu::Device) -> Self {
        let light_uniform = LightUniform::new(
            [2.0, 2.0, 2.0],
            [1.0, 1.0, 1.0]);
        let buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("light_vb"),
                contents: bytemuck::cast_slice(&[light_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );
        Self { buffer }
    }

    /// Update the buffer on device with a new [`LightUniform`].
    #[inline]
    #[allow(dead_code)]
    pub fn send_to_device(&mut self, queue: &Queue, uniform: LightUniform) {
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[uniform]));
    }

    #[inline]
    pub fn as_entire_binding(&self) -> BindingResource<'_> {
        self.buffer.as_entire_binding()
    }

    #[inline]
    #[allow(dead_code)]
    pub fn slice<S: RangeBounds<BufferAddress>>(&self, bounds: S) -> BufferSlice<'_> {
        self.buffer.slice(bounds)
    }
}

impl AsRef<Buffer> for LightBuffer {
    fn as_ref(&self) -> &Buffer {
        &self.buffer
    }
}

#[derive(Debug)]
pub struct InstanceBuffer {
    buffer: Buffer,
    num_instances: u32,
}

impl InstanceBuffer {
    // NOTE: Make sure that if you add new instances to the Vec, you recreate the
    //       instance_buffer as well as camera_bind_group. Otherwise, your new instances
    //       won't show up correctly.
    pub const NUM_INSTANCES_PER_ROW: u32 = 10;

    #[instrument(skip(device))]
    fn create(device: &wgpu::Device) -> Self {
        let num_instances = Self::NUM_INSTANCES_PER_ROW * Self::NUM_INSTANCES_PER_ROW;
        let buffer = device.create_buffer(
            &wgpu::BufferDescriptor {
                label: Some("Instance Buffer"),
                size: (num_instances as usize * size_of::<InstanceRaw>()) as BufferAddress,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: true
            }
        );

        {
            let mut mapping = buffer.slice(..).get_mapped_range_mut();
            mapping.fill(0);
        }
        buffer.unmap();

        Self { buffer, num_instances }
    }

    pub const fn len(&self) -> usize {
        self.num_instances as usize
    }

    /// Update the buffer on device with a new [`InstanceRaw`] slice.
    #[inline]
    #[allow(dead_code)]
    pub fn send_to_device(&self, queue: &Queue, instances: &[InstanceRaw]) -> u32 {
        let data_bytes = bytemuck::cast_slice(&instances);
        queue.write_buffer(&self.buffer, 0, data_bytes);
        instances.len() as _
    }

    #[inline]
    #[allow(dead_code)]
    pub fn as_entire_binding(&self) -> BindingResource<'_> {
        self.buffer.as_entire_binding()
    }

    #[inline]
    pub fn slice<S: RangeBounds<BufferAddress>>(&self, bounds: S) -> BufferSlice<'_> {
        self.buffer.slice(bounds)
    }
}

#[derive(Debug)]
pub struct RetroUniformBuffer {
    buffer: Buffer,
}

impl RetroUniformBuffer {
    #[instrument(skip(device))]
    fn create(device: &wgpu::Device) -> Self {
        let light_uniform = RetroUniform::default();
        let buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("retro_uniform"),
                contents: bytemuck::cast_slice(&[light_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );
        Self { buffer }
    }

    /// Update the buffer on device with a new [`RetroUniform`].
    #[inline]
    #[allow(dead_code)]
    pub fn send_to_device(&self, queue: &Queue, uniform: RetroUniform) {
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[uniform]));
    }

    #[inline]
    pub fn as_entire_binding(&self) -> BindingResource<'_> {
        self.buffer.as_entire_binding()
    }

    #[inline]
    #[allow(dead_code)]
    pub fn slice<S: RangeBounds<BufferAddress>>(&self, bounds: S) -> BufferSlice<'_> {
        self.buffer.slice(bounds)
    }
}

impl AsRef<Buffer> for RetroUniformBuffer {
    fn as_ref(&self) -> &Buffer {
        &self.buffer
    }
}
