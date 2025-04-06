//! This module provides the [`WgpuState`] struct and its associated implementation for managing
//! WebGPU state within a Rust application. It abstracts over the low-level WebGPU API to
//! simplify tasks such as creating surfaces, configuring devices, handling window resizing,
//! and submitting command buffers to the GPU.
//!
//! # Overview
//!
//! The [`WgpuState`] struct is designed to handle core WebGPU initialization and management tasks,
//! including creating a suitable adapter, device, and queue, as well as configuring surfaces for
//! rendering. This abstraction makes it easier to manage the GPU rendering lifecycle in modern
//! applications that use the `wgpu` and `winit` crates.
//!
//! # Features
//!
//! - Creates and manages a WebGPU instance, adapter, device, queue, and surface.
//! - Configures the surface with optimal settings for rendering.
//! - Handles window resizing by reconfiguring the surface appropriately.
//! - Provides utility methods to poll the device and submit command buffers for rendering.
//!
//! # Structs
//!
//! - `WgpuState`: The main struct of this module. It wraps and manages WebGPU objects
//!   such as the `Instance`, `Surface`, `Adapter`, `Device`, and `Queue`.
//!
//! # References
//!
//! For more details about the underlying WebGPU API, see the [wgpu documentation](https://docs.rs/wgpu).

use std::sync::Arc;
use tracing::debug;
use wgpu::{Device, Instance, InstanceDescriptor, MaintainResult, PresentMode, Queue, Surface, TextureUsages};
use winit::dpi::PhysicalSize;
use winit::window::Window;
use crate::wgpu::TextureSampler;

pub struct WgpuState {
    _instance: Instance,
    // https://github.com/rust-windowing/winit/discussions/3667#discussioncomment-9329312
    surface: Surface<'static>,
    surface_config: wgpu::SurfaceConfiguration,
    device: Device,
    queue: Queue,
    linear_sampler: TextureSampler,
}

impl WgpuState {
    pub async fn new(window: Arc<Window>) -> WgpuState {
        // There's trouble with the Vulkan backend on NVIDIA + Wayland.
        // If the application hangs after a couple of seconds, use `WGPU_BACKEND=gl`
        let descriptor = InstanceDescriptor {
            backends: wgpu::Backends::GL,
            flags: Default::default(),
            backend_options: Default::default(),
        };
        let instance = wgpu::Instance::new(&descriptor);

        // TODO: See if we need to drop existing surfaces first?
        let surface = instance.create_surface(Arc::clone(&window)).unwrap();

        let adapter = pollster::block_on(instance
            .request_adapter(
                &wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::HighPerformance, // or LowPower (e.g., iGPU)
                    compatible_surface: Some(&surface),
                    force_fallback_adapter: false,
                },
            ))
            .unwrap();

        let (device, queue) = pollster::block_on(adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Device Descriptor"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    memory_hints: Default::default(),
                },
                None,
            ))
            .unwrap();

        // Shader code in this tutorial assumes an sRGB surface texture. Using a different
        // one will result in all the colors coming out darker. If you want to support non
        // sRGB surfaces, you'll need to account for that when drawing to the frame.
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);
        debug!("Using surface format {surface_format:?}");

        let size = window.inner_size();
        let surface_config = wgpu::SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: PresentMode::Fifo, // surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        let sampler = TextureSampler::with_clamp_to_edge_and_linear_filtering(&device);

        Self {
            _instance: instance,
            surface,
            surface_config,
            device,
            queue,
            linear_sampler: sampler
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            debug!("Resizing window to {:?}", new_size);
            self.surface_config.width = new_size.width;
            self.surface_config.height = new_size.height;
            self.surface.configure(&self.device, &self.surface_config);
        }
    }

    /// Returns a reference to the WGPU surface.
    #[inline]
    pub const fn surface(&self) -> &Surface<'static> {
        &self.surface
    }

    /// Returns a reference to the WGPU device.
    #[inline]
    pub const fn device(&self) -> &Device {
        &self.device
    }

    /// Returns a reference to the WGPU queue.
    #[inline]
    #[allow(dead_code)]
    pub const fn queue(&self) -> &Queue {
        &self.queue
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

    /// Polls the underlying device.
    #[inline]
    #[allow(dead_code)]
    pub fn poll_device(&mut self, maintain: wgpu::Maintain) -> MaintainResult {
        self.device.poll(maintain)
    }
}

impl AsRef<Surface<'static>> for WgpuState {
    #[inline]
    fn as_ref(&self) -> &Surface<'static> {
        &self.surface
    }
}

impl AsRef<Device> for WgpuState {
    #[inline]
    fn as_ref(&self) -> &Device {
        &self.device
    }
}

impl AsRef<Queue> for WgpuState {
    #[inline]
    fn as_ref(&self) -> &Queue {
        &self.queue
    }
}
