use tracing::instrument;
use wgpu::{Device, Queue, Surface, SurfaceConfiguration};
use winit::dpi::PhysicalSize;

#[derive(Debug)]
#[allow(dead_code)]
pub struct DeviceAndQueue {
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
}

impl DeviceAndQueue {
    pub(super) fn new(
        device: Device,
        queue: Queue,
        config: SurfaceConfiguration,
    ) -> DeviceAndQueue {
        Self {
            device,
            queue,
            config,
        }
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
    }

    #[instrument(skip(self, config))]
    pub fn set_config(&mut self, config: SurfaceConfiguration) {
        self.config = config;
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
}
