pub mod bind_group_layouts;
pub mod bind_groups;
pub mod buffers;
pub mod depth_buffers;
pub mod models;
pub mod render_pipelines;
pub mod shaders;
pub mod texture_samplers;
pub mod textures;

use crate::wgpu::wgpu_hardware::resource::camera_uniform::CameraUniformResource;
use crate::wgpu::wgpu_hardware::resource::frame_global_uniform::FrameGlobalUniformResource;
use crate::wgpu::wgpu_hardware::resource::light_uniform::LightUniformResource;
use crate::wgpu::wgpu_hardware::resource::retro_uniform::RetroUniformResource;
use crate::wgpu::wgpu_hardware::resources::bind_group_layouts::BindGroupLayouts;
use crate::wgpu::wgpu_hardware::resources::bind_groups::BindGroups;
use crate::wgpu::wgpu_hardware::resources::buffers::Buffers;
use crate::wgpu::wgpu_hardware::resources::depth_buffers::DepthBuffers;
use crate::wgpu::wgpu_hardware::resources::models::Models;
use crate::wgpu::wgpu_hardware::resources::render_pipelines::{
    RenderPipelineSelector, RenderPipelines,
};
use crate::wgpu::wgpu_hardware::resources::shaders::Shaders;
use crate::wgpu::wgpu_hardware::resources::texture_samplers::TextureSamplers;
use crate::wgpu::wgpu_hardware::resources::textures::Textures;
use tracing::instrument;
use wgpu::{Device, Queue, SurfaceConfiguration};

#[derive(Debug)]
pub struct WgpuResources {
    samplers: TextureSamplers,
    buffers: Buffers,
    depths: DepthBuffers,
    _bind_group_layouts: BindGroupLayouts,
    bind_groups: BindGroups,
    pipelines: RenderPipelines,
    _shaders: Shaders,
    models: Models,
    textures: Textures,
    pub neo: NeoResources,
}

#[derive(Debug)]
pub struct NeoResources {
    pub uniforms: NeoUniformResources,
}

#[derive(Debug)]
pub struct NeoUniformResources {
    pub frame_globals: FrameGlobalUniformResource,
    pub camera: CameraUniformResource,
    pub light: LightUniformResource,
    pub retro: RetroUniformResource,
}

impl WgpuResources {
    pub(super) fn new(device: &Device, queue: &Queue, config: &SurfaceConfiguration) -> Self {
        let uniforms = NeoUniformResources {
            frame_globals: FrameGlobalUniformResource::new(device),
            camera: CameraUniformResource::new(device),
            light: LightUniformResource::new(device),
            retro: RetroUniformResource::new(device),
        };

        let neo = NeoResources { uniforms };

        let samplers = TextureSamplers::create_from(device);
        let textures = Textures::create_from(device, queue, config);
        let buffers = Buffers::create_from(device);
        let depths = DepthBuffers::create_from(device, config);
        let shaders = Shaders::create_from(device);
        let bind_group_layouts = BindGroupLayouts::create_from(device);
        let bind_groups = BindGroups::create_from(
            device,
            &bind_group_layouts,
            &textures,
            &samplers,
            &depths,
            &neo.uniforms.retro,
        );
        let pipelines = RenderPipelines::create_from(
            &device,
            &bind_group_layouts,
            config.format,
            &shaders,
            &neo.uniforms.frame_globals,
            &neo.uniforms.camera,
            &neo.uniforms.light,
        );
        let models = Models::create_from(&device, &queue, &bind_group_layouts, &samplers);
        Self {
            samplers,
            buffers,
            depths,
            _bind_group_layouts: bind_group_layouts,
            bind_groups,
            pipelines,
            _shaders: shaders,
            models,
            textures,
            neo,
        }
    }

    #[instrument(skip(self))]
    pub fn recreate_surface_dependent_buffers(
        &mut self,
        device: &Device,
        config: &SurfaceConfiguration,
    ) {
        self.depths.recreate(device, config);
        self.textures.recreate(device, config);
    }

    pub fn apply_render_pipeline_to_pass(
        &self,
        render_pass: &mut wgpu::RenderPass,
        pipeline_selector: RenderPipelineSelector,
    ) {
        let pipeline = self.pipelines.render_pipeline(pipeline_selector);
        render_pass.set_pipeline(&pipeline);
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
