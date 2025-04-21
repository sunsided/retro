use std::convert::Infallible;
use std::fmt::Debug;
use wgpu::util::DeviceExt;
use crate::{wgpu_label, GpuContext, RecreateReason, Resource, SurfaceInfo, UniformResourceId};
use crate::resources::uniform_buffers::uniform_buffer::UniformBuffer;

#[derive(Debug)]
pub struct SingleUniformBuffer<T>
where T: Debug
{
    pub label: &'static str,
    pub data: T,
}

impl<T> SingleUniformBuffer<T>
where T: Debug
{
    const BUFFER_SIZE: Option<wgpu::BufferSize> = wgpu::BufferSize::new(size_of::<T>() as _);

    pub fn new(label: &'static str, data: T) -> Self {
        Self { label, data }
    }
}

impl<T> Resource for SingleUniformBuffer<T>
where
    T: Debug + Clone + bytemuck::Pod + bytemuck::Zeroable
{
    type Output = UniformBuffer;
    type Error = Infallible;

    fn recreate<'device>(&mut self, ctx: &GpuContext<'device>, _surface_info: &SurfaceInfo, _reason: RecreateReason) -> Result<Self::Output, Self::Error> {
        // TODO: Don't recreate if not necessary

        let label = wgpu_label!(self.label);

        let data = [self.data.clone()];
        let contents = bytemuck::cast_slice(&data);

        let desc = wgpu::util::BufferInitDescriptor {
            label: Some(&label),
            contents,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        };

        let buffer = ctx.device.create_buffer_init(&desc);

        let layout =
            ctx.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some(&label),
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: Self::BUFFER_SIZE,
                        },
                        count: None,
                    }],
                });

        // TODO: Rework this; have a single bind group for all uniforms, but keep the buffers separate
        let bind_group = ctx
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some(&label),
                layout: &layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                }],
            });

        Ok(UniformBuffer::new(buffer, layout, bind_group))
    }
}
