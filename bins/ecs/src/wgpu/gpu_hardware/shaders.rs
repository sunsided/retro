use tracing::instrument;
use wgpu::{Device, ShaderModule};

#[derive(Debug)]
pub struct Shaders {
    pub render: RenderShaderModule,
    pub light: LightShaderModule,
}

impl Shaders {
    #[instrument(skip(device))]
    pub fn create_from(device: &Device) -> Self {
        let render = RenderShaderModule::create_render(&device);
        let light = LightShaderModule::create_render(&device);
        Self {
            render,
            light
        }
    }
}

#[derive(Debug, Clone)]
pub struct VertexShaderRef<'module> {
    entry_point: &'module str,
    module: &'module ShaderModule,
}

#[derive(Debug, Clone)]
pub struct FragmentShaderRef<'module> {
    entry_point: &'module str,
    module: &'module ShaderModule,
}

impl VertexShaderRef<'_> {
    pub const fn entry_point(&self) -> Option<&str> {
        Some(self.entry_point)
    }

    pub const fn module(&self) -> &ShaderModule {
        self.module
    }
}

impl FragmentShaderRef<'_> {
    pub const fn entry_point(&self) -> Option<&str> {
        Some(self.entry_point)
    }

    pub const fn module(&self) -> &ShaderModule {
        self.module
    }
}

#[derive(Debug)]
pub struct RenderShaderModule(ShaderModule);

impl RenderShaderModule {
    #[instrument(skip(device))]
    fn create_render(device: &Device) -> Self {
        let shader = wgpu::ShaderModuleDescriptor {
            label: Some("Normal Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/shader.wgsl").into()),
        };

        Self(device.create_shader_module(shader))
    }

    pub fn vertex_shader(&self) -> VertexShaderRef {
        VertexShaderRef {
            entry_point: "vs_main",
            module: &self.0,
        }
    }

    pub fn flat_fragment_shader(&self) -> FragmentShaderRef {
        FragmentShaderRef {
            entry_point: "fs_main",
            module: &self.0,
        }
    }

    pub fn textured_fragment_shader(&self) -> FragmentShaderRef {
        FragmentShaderRef {
            entry_point: "fs_textured_main",
            module: &self.0,
        }
    }
}

#[derive(Debug)]
pub struct LightShaderModule(ShaderModule);

impl LightShaderModule {
    #[instrument(skip(device))]
    fn create_render(device: &Device) -> Self {
        let shader = wgpu::ShaderModuleDescriptor {
            label: Some("Light Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/light.wgsl").into()),
        };

        Self(device.create_shader_module(shader))
    }

    pub fn vertex_shader(&self) -> VertexShaderRef {
        VertexShaderRef {
            entry_point: "vs_main",
            module: &self.0,
        }
    }

    pub fn fragment_shader(&self) -> FragmentShaderRef {
        FragmentShaderRef {
            entry_point: "fs_main",
            module: &self.0,
        }
    }
}
