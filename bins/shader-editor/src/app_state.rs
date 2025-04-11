use egui_demo_lib::DemoWindows;
use egui_wgpu_backend::RenderPass;
use egui_winit_platform::Platform;
use std::sync::Arc;
use std::time::Instant;
use wgpu::{
    Adapter, Buffer, Device, Instance, Queue, RenderPipeline, Surface, SurfaceConfiguration,
};
use winit::keyboard::ModifiersState;
use winit::window::Window;

pub enum AppState {
    Uninitialized(Transient),
    Bootstrapped(UninitializedAppState),
    Initialized(InitializedAppState),
}

/// Internal struct to prevent creation of the [`AppState::Uninitialized`] variant
/// outside of this module.
struct Transient;

impl Default for AppState {
    fn default() -> Self {
        Self::Uninitialized(Transient)
    }
}

pub struct UninitializedAppState {
    pub instance: Instance,
    pub start_time: Instant,
}

pub struct InitializedAppState {
    pub window: Arc<Window>,
    pub modifiers: ModifiersState,
    pub instance: Instance,
    pub surface: Surface<'static>,
    pub device: Device,
    pub adapter: Adapter,
    pub config: SurfaceConfiguration,
    pub queue: Queue,
    pub pipeline: RenderPipeline,
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
    pub egui_platform: Platform,
    pub egui_egui_rpass: RenderPass,
    pub egui_demo_app: DemoWindows,
    pub uniform_buffer: Buffer,
    pub uniform_bind_group: wgpu::BindGroup,
    pub start_time: Instant,
}
