use tracing::instrument;
use wgpu::{Device, MaintainResult, Queue, Surface, SurfaceConfiguration};
use winit::dpi::PhysicalSize;
use crate::wgpu::wgpu_hardware::depth_buffers::DepthBuffers;
use crate::wgpu::wgpu_hardware::bind_group_layouts::BindGroupLayouts;
use crate::wgpu::wgpu_hardware::bind_groups::BindGroups;
use crate::wgpu::wgpu_hardware::buffers::Buffers;
use crate::wgpu::wgpu_hardware::models::Models;
use crate::wgpu::wgpu_hardware::render_pipelines::RenderPipelines;
use crate::wgpu::wgpu_hardware::RenderPipelineSelector;
use crate::wgpu::wgpu_hardware::shaders::Shaders;
use crate::wgpu::wgpu_hardware::texture_samplers::TextureSamplers;
use crate::wgpu::wgpu_hardware::textures::Textures;

#[derive(Debug)]
#[allow(dead_code)]
pub struct DeviceAndQueue {
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    samplers: TextureSamplers,
    buffers: Buffers,
    depths: DepthBuffers,
    bind_group_layouts: BindGroupLayouts,
    bind_groups: BindGroups,
    pipelines: RenderPipelines,
    shaders: Shaders,
    models: Models,
    textures: Textures,
}

impl DeviceAndQueue {
    pub(super) fn new(device: Device, queue: Queue, config: SurfaceConfiguration) -> DeviceAndQueue {
        let samplers = TextureSamplers::create_from(&device);
        let textures = Textures::create_from(&device, &queue, config.format);
        let buffers = Buffers::create_from(&device);
        let depths = DepthBuffers::create_from(&device, &config);
        let shaders = Shaders::create_from(&device);
        let bind_group_layouts = BindGroupLayouts::create_from(&device);
        let bind_groups = BindGroups::create_from(&device, &bind_group_layouts, &buffers, &textures, &samplers);
        let pipelines = RenderPipelines::create_from(&device, &bind_group_layouts, config.format, &shaders);
        let models = Models::create_from(&device, &queue, &bind_group_layouts, &samplers);
        Self {
            device,
            queue,
            config,
            samplers,
            buffers,
            depths,
            bind_group_layouts,
            bind_groups,
            pipelines,
            shaders,
            models,
            textures
        }
    }

    /// Polls the underlying device.
    #[inline]
    #[allow(dead_code)]
    pub fn poll_device(&mut self, maintain: wgpu::Maintain) -> MaintainResult {
        self.device.poll(maintain)
    }

    /// Submits a command buffer to the WGPU queue for execution.
    ///
    /// This function is used to send GPU commands for processing.
    /// Make sure that the command buffer is properly recorded and finalized
    /// before submitting it.
    ///
    /// # Parameters
    ///
    /// - `command_buffer`: A finalized WGPU command buffer containing the commands to be executed.
    #[inline]
    pub fn submit_to_queue(&self, command_buffer: wgpu::CommandBuffer) {
        self.queue.submit(Some(command_buffer));
    }

    #[instrument(skip(self, surface))]
    pub fn resize_surface(&mut self, new_size: PhysicalSize<u32>, surface: &mut Surface) {
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        surface.configure(&self.device, &self.config);
        self.recreate_surface_dependent_buffers();
    }

    #[instrument(skip(self, config))]
    pub fn set_config(&mut self, config: SurfaceConfiguration) {
        self.config = config;
        self.recreate_surface_dependent_buffers();
    }

    #[instrument(skip(self))]
    pub fn recreate_surface_dependent_buffers(&mut self) {
        self.depths.recreate(&self.device, &self.config);
    }

    pub fn apply_render_pipeline_to_pass(&self, render_pass: &mut wgpu::RenderPass, pipeline_selector: RenderPipelineSelector) {
        let pipeline = self.pipelines.render_pipeline(pipeline_selector);
        render_pass.set_pipeline(&pipeline);
    }

    pub const fn device(&self) -> &Device {
        &self.device
    }

    pub const fn queue(&self) -> &Queue {
        &self.queue
    }

    pub const fn config(&self) -> &SurfaceConfiguration {
        &self.config
    }

    #[allow(dead_code)]
    pub const fn samplers(&self) -> &TextureSamplers {
        &self.samplers
    }

    pub const fn depths(&self) -> &DepthBuffers {
        &self.depths
    }

    pub const fn buffers(&self) -> &Buffers {
        &self.buffers
    }

    pub const fn pipelines(&self) -> &RenderPipelines {
        &self.pipelines
    }

    pub const fn models(&self) -> &Models {
        &self.models
    }

    pub const fn bind_groups(&self) -> &BindGroups {
        &self.bind_groups
    }

    pub const fn textures(&self) -> &Textures {
        &self.textures
    }
}
