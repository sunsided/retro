use std::convert::Infallible;
use std::fmt::Debug;
use wgpu::util::DeviceExt;
use crate::{wgpu_label, GpuContext, RecreateReason, Resource, SurfaceInfo};
use crate::resources::buffers::buffer::Buffer;

#[derive(Debug)]
pub struct PrefilledBuffer<T>
where T: Debug
{
    label: &'static str,
    data: Box<[T]>,
    usage: wgpu::BufferUsages
}

impl<T> PrefilledBuffer<T>
where T: Debug
{
    pub fn new(label: &'static str, data: Box<[T]>, usage: wgpu::BufferUsages) -> Self {
        Self { label, data, usage }
    }
}

impl<T> Resource for PrefilledBuffer<T>
where T: Debug + Clone + bytemuck::Pod + bytemuck::Zeroable
{
    type Output = Buffer;
    type Error = Infallible;

    fn recreate<'device>(&mut self, ctx: &GpuContext<'device>, _surface_info: &SurfaceInfo, _reason: RecreateReason) -> Result<Self::Output, Self::Error> {
        // TODO: Don't recreate if not necessary

        let label = wgpu_label!(self.label);

        let desc = wgpu::util::BufferInitDescriptor {
            label: Some(&label),
            contents: bytemuck::cast_slice(&self.data),
            usage: self.usage,
        };
        let buffer = ctx.device.create_buffer_init(&desc);

        Ok(Buffer::new(buffer))
    }
}
