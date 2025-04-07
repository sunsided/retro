use std::ops::RangeBounds;
use cgmath::{InnerSpace, Rotation3, Zero};
use tracing::instrument;
use wgpu::{BindingResource, Buffer, BufferAddress, BufferSlice, Queue};
use wgpu::util::DeviceExt;
use crate::wgpu::camera_uniform::CameraUniform;
use crate::wgpu::instance::Instance;
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
    pub num_instances: u32,
}

impl InstanceBuffer {
    // NOTE: Make sure that if you add new instances to the Vec, you recreate the
    //       instance_buffer as well as camera_bind_group. Otherwise, your new instances
    //       won't show up correctly.
    const SPACE_BETWEEN: f32 = 3.0;
    const NUM_INSTANCES_PER_ROW: u32 = 10;

    #[instrument(skip(device))]
    fn create(device: &wgpu::Device) -> Self {
        let instances = (0..Self::NUM_INSTANCES_PER_ROW).flat_map(|z| {
            (0..Self::NUM_INSTANCES_PER_ROW).map(move |x| {
                let x = Self::SPACE_BETWEEN * (x as f32 - Self::NUM_INSTANCES_PER_ROW as f32 / 2.0);
                let z = Self::SPACE_BETWEEN * (z as f32 - Self::NUM_INSTANCES_PER_ROW as f32 / 2.0);

                let position = cgmath::Vector3 { x, y: 0.0, z };

                let rotation = if position.is_zero() {
                    cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.0))
                } else {
                    cgmath::Quaternion::from_axis_angle(position.normalize(), cgmath::Deg(45.0))
                };

                Instance {
                    position, rotation,
                }
            })
        }).collect::<Vec<_>>();
        let num_instances = instances.len() as u32;

        let instance_data = instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
        let buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&instance_data),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );
        Self { buffer, num_instances }
    }

    pub const fn len(&self) -> usize {
        self.num_instances as usize
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
