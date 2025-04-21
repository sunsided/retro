use crate::{ResourceData};
use crate::resources::bind_groups::BindGroupResourceData;

#[derive(Debug)]
pub struct BindGroup {
    pub layout: wgpu::BindGroup,
}

impl ResourceData for BindGroup {}

impl BindGroupResourceData for BindGroup {
    fn layout(&self) -> &wgpu::BindGroup {
        &self.layout
    }
}

impl BindGroup {
    pub(crate) const fn new(layout: wgpu::BindGroup) -> Self {
        Self {
            layout
        }
    }
}
