pub mod buffers;
pub mod depth_buffers;
pub mod models;
pub mod render_pipelines;
mod renderers;
pub mod shaders;
pub mod standard_render_layout;
pub mod texture_bind_groups;
pub mod texture_samplers;
pub mod textures;

use crate::wgpu::wgpu_hardware::resource::camera_uniform::CameraUniformResource;
use crate::wgpu::wgpu_hardware::resource::frame_global_uniform::FrameGlobalUniformResource;
use crate::wgpu::wgpu_hardware::resource::light_uniform::LightUniformResource;
use crate::wgpu::wgpu_hardware::resources::buffers::Buffers;
use crate::wgpu::wgpu_hardware::resources::depth_buffers::DepthBuffers;
use crate::wgpu::wgpu_hardware::resources::models::Models;
use crate::wgpu::wgpu_hardware::resources::render_pipelines::{
    RenderPipelineSelector, RenderPipelines,
};
use crate::wgpu::wgpu_hardware::resources::renderers::Renderers;
use crate::wgpu::wgpu_hardware::resources::shaders::Shaders;
use crate::wgpu::wgpu_hardware::resources::standard_render_layout::StandardRenderLayout;
use crate::wgpu::wgpu_hardware::resources::texture_bind_groups::TextureBindGroups;
use crate::wgpu::wgpu_hardware::resources::texture_samplers::TextureSamplers;
use crate::wgpu::wgpu_hardware::resources::textures::Textures;
use tracing::instrument;
use wgpu::{Device, Queue, SurfaceConfiguration};

#[derive(Debug)]
pub struct WgpuResources {
    samplers: TextureSamplers,
    buffers: Buffers,
    depths: DepthBuffers,
    _standard_render: StandardRenderLayout,
    pipelines: RenderPipelines,
    _shaders: Shaders,
    models: Models,
    textures: Textures,
    pub neo: NeoResources,
    renderers: Renderers,
    texture_bind_groups: TextureBindGroups,
    surface_config: SurfaceConfiguration,
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
}

impl WgpuResources {
    pub(super) fn new(device: &Device, queue: &Queue, config: SurfaceConfiguration) -> Self {
        let uniforms = NeoUniformResources {
            frame_globals: FrameGlobalUniformResource::new(device),
            camera: CameraUniformResource::new(device),
            light: LightUniformResource::new(device),
        };

        let neo = NeoResources { uniforms };

        let samplers = TextureSamplers::create_from(device);
        let textures = Textures::create_from(device, queue, &config);
        let buffers = Buffers::create_from(device);
        let depths = DepthBuffers::create_from(device, &config);
        let shaders = Shaders::create_from(device);
        let standard_render = StandardRenderLayout::create_from(
            device,
            &neo.uniforms.frame_globals,
            &neo.uniforms.camera,
            &neo.uniforms.light,
        );
        let texture_bind_groups =
            TextureBindGroups::create_from(device, &standard_render, &textures, &samplers);
        let pipelines = RenderPipelines::create_from(
            &device,
            &standard_render,
            config.format,
            &shaders,
            &neo.uniforms.frame_globals,
            &neo.uniforms.camera,
            &neo.uniforms.light,
        );
        let models = Models::create_from(&device, &queue, &standard_render, &samplers);
        let renderers = Renderers::new(
            device,
            &config,
            &standard_render,
            &neo.uniforms.frame_globals,
            &neo.uniforms.camera,
            &neo.uniforms.light,
        );
        Self {
            samplers,
            buffers,
            depths,
            texture_bind_groups,
            _standard_render: standard_render,
            pipelines,
            _shaders: shaders,
            models,
            textures,
            neo,
            renderers,
            surface_config: config,
        }
    }

    #[instrument(skip(self))]
    pub fn recreate_surface_dependent_buffers(&mut self, device: &Device) {
        self.depths.recreate(device, &self.surface_config);
        self.textures.recreate(device, &self.surface_config);
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

    pub const fn textures(&self) -> &Textures {
        &self.textures
    }

    pub const fn renderers(&self) -> &Renderers {
        &self.renderers
    }

    pub const fn texture_bind_groups(&self) -> &TextureBindGroups {
        &self.texture_bind_groups
    }

    pub const fn config(&self) -> &SurfaceConfiguration {
        &self.surface_config
    }

    #[instrument(skip(self, config))]
    pub fn set_config(&mut self, config: SurfaceConfiguration) {
        self.surface_config = config;
    }
}
