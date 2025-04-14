use tracing::instrument;
use wgpu::{Device, ShaderModule};

#[derive(Debug)]
pub struct Shaders {
    pub sky: SkyShaderModule,
    pub render: RenderShaderModule,
    pub light: LightShaderModule,
    pub vga: VgaShaderModule,
    pub tile: TileShaderModule,
}

impl Shaders {
    #[instrument(skip(device))]
    pub fn create_from(device: &Device) -> Self {
        let sky = SkyShaderModule::create(&device);
        let render = RenderShaderModule::create(&device);
        let light = LightShaderModule::create(&device);
        let vga = VgaShaderModule::create(&device);
        let tile = TileShaderModule::create(&device);
        Self {
            sky,
            render,
            light,
            vga,
            tile,
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
pub struct SkyShaderModule(ShaderModule);

impl SkyShaderModule {
    #[instrument(skip(device))]
    fn create(device: &Device) -> Self {
        let shader = wgpu::ShaderModuleDescriptor {
            label: Some("Sky"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../../shaders/sky.wgsl").into()),
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

#[derive(Debug)]
pub struct RenderShaderModule(ShaderModule);

impl RenderShaderModule {
    #[instrument(skip(device))]
    fn create(device: &Device) -> Self {
        let shader = wgpu::ShaderModuleDescriptor {
            label: Some("Normal Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../../shaders/shader.wgsl").into()),
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
            entry_point: "fs_flat_main",
            module: &self.0,
        }
    }

    pub fn instance_fragment_shader(&self) -> FragmentShaderRef {
        FragmentShaderRef {
            entry_point: "fs_instance_main",
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
    fn create(device: &Device) -> Self {
        let shader = wgpu::ShaderModuleDescriptor {
            label: Some("Light Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../../shaders/light.wgsl").into()),
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

#[derive(Debug)]
pub struct VgaShaderModule(ShaderModule);

impl VgaShaderModule {
    #[instrument(skip(device))]
    fn create(device: &Device) -> Self {
        let shader = wgpu::ShaderModuleDescriptor {
            label: Some("VGA Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../../shaders/vga.wgsl").into()),
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

#[derive(Debug)]
pub struct TileShaderModule(ShaderModule);

impl TileShaderModule {
    #[instrument(skip(device))]
    fn create(device: &Device) -> Self {
        let shader = wgpu::ShaderModuleDescriptor {
            label: Some("Tile Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../../shaders/tile.wgsl").into()),
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
