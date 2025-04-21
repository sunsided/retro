use crate::{ResourceData};
use crate::resources::buffers::BufferResourceData;

#[derive(Debug)]
pub struct Buffer {
    pub buffer: wgpu::Buffer,
}

impl ResourceData for Buffer {}

impl BufferResourceData for Buffer {
    fn buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }
}

impl Buffer {
    pub(crate) const fn new(buffer: wgpu::Buffer) -> Self {
        Self {
            buffer
        }
    }
}
