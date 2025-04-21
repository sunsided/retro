use wgpu::{BindGroup, BindGroupLayout, Buffer};
use crate::{ResourceData};
use crate::resources::uniform_buffers::UniformBufferResourceData;

#[derive(Debug)]
pub struct UniformBuffer {
    pub buffer: Buffer,
    pub layout: BindGroupLayout,
    pub bind_group: BindGroup,
}

impl ResourceData for UniformBuffer {}

impl UniformBufferResourceData for UniformBuffer {
    fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    fn bind_group_layout(&self) -> &BindGroupLayout {
        &self.layout
    }

    fn bind_group(&self) -> &BindGroup {
        &self.bind_group
    }
}

impl UniformBuffer {
    pub(crate) const fn new(buffer: Buffer, layout: BindGroupLayout, bind_group: BindGroup) -> UniformBuffer {
        Self {
            buffer,
            layout,
            bind_group
        }
    }
}
