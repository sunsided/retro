use crate::map::tile_instance::TileVertex;
use crate::wgpu::instance::InstanceRaw;
use crate::wgpu::uniforms::{CameraUniform, FrameGlobalUniform, LightUniform, RetroUniform};
use std::ops::RangeBounds;
use tracing::instrument;
use wgpu::util::DeviceExt;
use wgpu::{BindingResource, Buffer, BufferAddress, BufferSlice, Queue};

#[derive(Debug)]
pub struct Buffers {
    pub frame_global_uniform: FrameGlobalUniformBuffer,
    pub camera_uniform: CameraUniformBuffer,
    pub light_uniforms: LightUniformBuffer,
    pub instances: InstanceBuffer,
    pub retro_uniform: RetroUniformBuffer,
    pub tile_vertex: TileVertexBuffer,
    pub tile_index: TileIndexBuffer,
}

impl Buffers {
    #[instrument(skip(device))]
    pub fn create_from(device: &wgpu::Device) -> Self {
        let frame_global_uniform = FrameGlobalUniformBuffer::create(device);
        let camera_uniform = CameraUniformBuffer::create(device);
        let light_uniforms = LightUniformBuffer::create(device);
        let instances = InstanceBuffer::create(device);
        let retro_uniform = RetroUniformBuffer::create(device);
        let tile_vertex = TileVertexBuffer::create(device);
        let tile_index = TileIndexBuffer::create(device);
        Self {
            frame_global_uniform,
            camera_uniform,
            light_uniforms,
            instances,
            retro_uniform,
            tile_vertex,
            tile_index,
        }
    }
}

/// The frame-global uniforms are updated once per frame.
#[derive(Debug)]
pub struct FrameGlobalUniformBuffer {
    buffer: Buffer,
}

impl FrameGlobalUniformBuffer {
    #[instrument(skip(device))]
    fn create(device: &wgpu::Device) -> Self {
        let frame_global_uniform = FrameGlobalUniform::new(12.0);
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("frame_global_uniform"),
            contents: bytemuck::cast_slice(&[frame_global_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        Self { buffer }
    }

    /// Update the buffer on device with a new [`FrameGlobalUniform`].
    #[inline]
    pub fn send_to_device(&self, queue: &Queue, uniform: FrameGlobalUniform) {
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

impl AsRef<Buffer> for FrameGlobalUniformBuffer {
    fn as_ref(&self) -> &Buffer {
        &self.buffer
    }
}

/// The camera uniforms are updated once or twice per frame, depending on the viewport
/// count. OpenXR rendering may require us to render the same scene from multiple locations;
/// the [`FrameGlobalUniformBuffer`] wouldn't change in this case, wheras the camera would.
#[derive(Debug)]
pub struct CameraUniformBuffer {
    buffer: Buffer,
}

impl CameraUniformBuffer {
    #[instrument(skip(device))]
    fn create(device: &wgpu::Device) -> Self {
        let camera_uniform = CameraUniform::new();
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("camera_uniform"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
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

impl AsRef<Buffer> for CameraUniformBuffer {
    fn as_ref(&self) -> &Buffer {
        &self.buffer
    }
}

#[derive(Debug)]
pub struct LightUniformBuffer {
    buffer: Buffer,
}

impl LightUniformBuffer {
    #[instrument(skip(device))]
    fn create(device: &wgpu::Device) -> Self {
        let light_uniform = LightUniform::new([2.0, 2.0, 2.0], [1.0, 1.0, 1.0]);
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("light_uniform"),
            contents: bytemuck::cast_slice(&[light_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
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

impl AsRef<Buffer> for LightUniformBuffer {
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
    const NUM_INSTANCES_PER_ROW: u32 = 10;

    pub const MAX_INSTANCES: u32 = Self::NUM_INSTANCES_PER_ROW * Self::NUM_INSTANCES_PER_ROW;

    #[instrument(skip(device))]
    fn create(device: &wgpu::Device) -> Self {
        let num_instances = Self::MAX_INSTANCES;
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Instance Buffer"),
            size: (num_instances as usize * size_of::<InstanceRaw>()) as BufferAddress,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: true,
        });

        {
            let mut mapping = buffer.slice(..).get_mapped_range_mut();
            mapping.fill(0);
        }
        buffer.unmap();

        Self {
            buffer,
            num_instances,
        }
    }

    pub const fn len(&self) -> usize {
        self.num_instances as usize
    }

    /// Update the buffer on device with a new [`InstanceRaw`] slice.
    #[inline]
    #[allow(dead_code)]
    pub fn send_to_device(&self, queue: &Queue, instances: &[InstanceRaw]) -> u32 {
        let data_bytes = bytemuck::cast_slice(instances);
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
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("retro_uniform"),
            contents: bytemuck::cast_slice(&[light_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
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

/// The vertex buffer for map tiles.
#[derive(Debug)]
pub struct TileVertexBuffer {
    buffer: Buffer,
}

impl TileVertexBuffer {
    #[instrument(skip(device))]
    fn create(device: &wgpu::Device) -> Self {
        const TILE_VERTICES: &[TileVertex] = &[
            TileVertex {
                position: [-0.5, 0.0, -0.5],
                uv: [0.0, 1.0],
            },
            TileVertex {
                position: [0.5, 0.0, -0.5],
                uv: [1.0, 1.0],
            },
            TileVertex {
                position: [-0.5, 0.0, 0.5],
                uv: [0.0, 0.0],
            },
            TileVertex {
                position: [0.5, 0.0, 0.5],
                uv: [1.0, 0.0],
            },
        ];

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Tile Vertex Buffer"),
            contents: bytemuck::cast_slice(TILE_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });
        Self { buffer }
    }

    #[inline]
    #[allow(dead_code)]
    pub fn as_entire_binding(&self) -> BindingResource<'_> {
        self.buffer.as_entire_binding()
    }

    #[inline]
    #[allow(dead_code)]
    pub fn slice<S: RangeBounds<BufferAddress>>(&self, bounds: S) -> BufferSlice<'_> {
        self.buffer.slice(bounds)
    }
}

/// The index buffer for map tile vertices.
#[derive(Debug)]
pub struct TileIndexBuffer {
    buffer: Buffer,
}

impl TileIndexBuffer {
    #[instrument(skip(device))]
    fn create(device: &wgpu::Device) -> Self {
        const TILE_INDICES: &[u16] = &[0, 2, 1, 2, 3, 1];

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Tile Index Buffer"),
            contents: bytemuck::cast_slice(TILE_INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });
        Self { buffer }
    }

    #[inline]
    #[allow(dead_code)]
    pub fn as_entire_binding(&self) -> BindingResource<'_> {
        self.buffer.as_entire_binding()
    }

    #[inline]
    #[allow(dead_code)]
    pub fn slice<S: RangeBounds<BufferAddress>>(&self, bounds: S) -> BufferSlice<'_> {
        self.buffer.slice(bounds)
    }
}
