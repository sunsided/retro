use crate::{ResourceData};
use crate::resources::pipelines::PipelineResourceData;

#[derive(Debug)]
pub struct Pipeline {
    pub pipeline: wgpu::RenderPipeline,
}

impl ResourceData for Pipeline {}

impl PipelineResourceData for Pipeline {
    fn pipeline(&self) -> &wgpu::RenderPipeline {
        &self.pipeline
    }
}

impl Pipeline {
    pub(crate) const fn new(pipeline: wgpu::RenderPipeline) -> Self {
        Self {
            pipeline
        }
    }
}
