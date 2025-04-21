use std::convert::Infallible;
use std::fmt::Debug;
use std::mem::size_of;
use wgpu::{BufferAddress, COPY_BUFFER_ALIGNMENT};
use crate::{wgpu_label, GpuContext, RecreateReason, Resource, SurfaceInfo};
use crate::resources::buffers::buffer::Buffer;

#[derive(Debug)]
pub struct ZeroedOutBuffer
{
    label: &'static str,
    element_size: u64,
    element_count: u64,
    usage: wgpu::BufferUsages
}

impl ZeroedOutBuffer
{
    pub fn new<T>(label: &'static str, count: u64, usage: wgpu::BufferUsages) -> Self {
        let element_size = size_of::<T>() as u64;
        assert_eq!((count * element_size) % COPY_BUFFER_ALIGNMENT, 0, "Buffer size must be aligned to {COPY_BUFFER_ALIGNMENT}");
        Self { label, element_count: count, element_size, usage }
    }

    pub fn buffer_size(&self) -> BufferAddress {
        self.element_count * self.element_size
    }
}

impl Resource for ZeroedOutBuffer
{
    type Output = Buffer;
    type Error = Infallible;

    fn recreate<'device>(&mut self, ctx: &GpuContext<'device>, _surface_info: &SurfaceInfo, _reason: RecreateReason) -> Result<Self::Output, Self::Error> {
        // TODO: Don't recreate if not necessary

        let label = wgpu_label!(self.label);

        let size = self.buffer_size();
        let desc = wgpu::BufferDescriptor {
            label: Some(&label),
            size,
            usage: self.usage,
            mapped_at_creation: true,
        };
        let buffer = ctx.device.create_buffer(&desc);
        {
            let mut mapping = buffer.slice(..).get_mapped_range_mut();
            mapping.fill(0);
        }
        buffer.unmap();

        Ok(Buffer::new(buffer))
    }
}
