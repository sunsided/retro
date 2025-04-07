use tracing::instrument;
use wgpu::Queue;
use crate::resources;
use crate::wgpu::wgpu_hardware::bind_group_layouts::BindGroupLayouts;
use crate::wgpu::wgpu_hardware::texture_samplers::TextureSamplers;
use crate::wgpu::model::Model;

#[derive(Debug)]
pub struct Models {
    pub cube: Model,
}

impl Models {
    #[instrument(skip(device, queue, layouts, samplers))]
    pub fn create_from(device: &wgpu::Device, queue: &Queue, layouts: &BindGroupLayouts, samplers: &TextureSamplers) -> Self {
        let cube = Self::create_cube(device, queue, layouts, samplers);
        Self {
            cube,
        }
    }

    #[instrument(skip(device, queue, layouts, samplers))]
    fn create_cube(device: &wgpu::Device, queue: &Queue, layouts: &BindGroupLayouts, samplers: &TextureSamplers) -> Model {
        pollster::block_on(
            resources::load_model("models/cube/cube.obj", device, queue, &layouts.diffuse_and_normal, &samplers.linear))
            .expect("Failed to load cube model")
    }
}
