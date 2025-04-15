use crate::SurfaceWrapper;
use wgpu::{CommandBuffer, Device, Queue, SurfaceConfiguration};

#[derive(Debug)]
#[allow(dead_code)]
pub struct DeviceAndQueue {
    device: Device,
    queue: Queue,
}

impl DeviceAndQueue {
    #[inline]
    pub const fn new_from(device: Device, queue: Queue) -> Self {
        Self { device, queue }
    }

    /// Submit a command buffer to the WGPU queue for execution.
    ///
    /// This function is used to send GPU commands for processing.
    /// Make sure that the command buffer is properly recorded and finalized
    /// before submitting it.
    ///
    /// # Parameters
    ///
    /// - `command_buffer`: A finalized WGPU command buffer containing the commands to be executed.
    #[inline]
    pub fn submit_to_queue<B>(&self, command_buffer: B)
    where
        B: Into<Option<CommandBuffer>>,
    {
        let command_buffer = command_buffer.into();
        if command_buffer.is_none() {
            return;
        }
        self.submit_any_to_queue(command_buffer)
    }

    /// Submits command buffers to the WGPU queue for execution.
    ///
    /// This function is used to send GPU commands for processing.
    /// Make sure that the command buffer is properly recorded and finalized
    /// before submitting it.
    ///
    /// # Parameters
    ///
    /// - `command_buffer`: A finalized WGPU command buffer containing the commands to be executed.
    #[inline]
    pub fn submit_any_to_queue<I>(&self, command_buffers: I)
    where
        I: IntoIterator<Item = CommandBuffer>,
    {
        self.queue.submit(command_buffers);
    }

    pub const fn device(&self) -> &Device {
        &self.device
    }

    pub const fn queue(&self) -> &Queue {
        &self.queue
    }

    #[inline]
    pub fn configure_surface(&self, surface: &mut SurfaceWrapper, config: SurfaceConfiguration) {
        surface.configure_surface(self.device(), config)
    }
}
