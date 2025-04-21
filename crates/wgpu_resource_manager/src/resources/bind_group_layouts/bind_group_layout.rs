use crate::{ResourceData};
use crate::resources::bind_group_layouts::BindGroupLayoutResourceData;

#[derive(Debug)]
pub struct BindGroupLayout {
    pub layout: wgpu::BindGroupLayout,
}

impl ResourceData for BindGroupLayout {}

impl BindGroupLayoutResourceData for BindGroupLayout {
    fn layout(&self) -> &wgpu::BindGroupLayout {
        &self.layout
    }
}

impl BindGroupLayout {
    pub(crate) const fn new(layout: wgpu::BindGroupLayout) -> Self {
        Self {
            layout
        }
    }
}
