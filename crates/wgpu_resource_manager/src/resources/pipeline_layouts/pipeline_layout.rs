use crate::{ResourceData};
use crate::resources::pipeline_layouts::PipelineLayoutResourceData;

#[derive(Debug)]
pub struct PipelineLayout {
    pub layout: wgpu::PipelineLayout,
}

impl ResourceData for PipelineLayout {}

impl PipelineLayoutResourceData for PipelineLayout {
    fn layout(&self) -> &wgpu::PipelineLayout {
        &self.layout
    }
}

impl PipelineLayout {
    pub(crate) const fn new(layout: wgpu::PipelineLayout) -> Self {
        Self {
            layout
        }
    }
}
