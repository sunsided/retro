//! This module provides the implementation of the `CameraRenderSystemData` responsible for
//! handling the GPU buffer updates related to camera's view and projection matrices.
//!
//! It integrates with the rendering pipeline by ensuring the active camera's uniform data,
//! including view and projection matrices, is appropriately synchronized with the GPU.
//!
//! # Related Concepts
//!
//! - See [`CameraSystemData`](crate::engine::phases::update::camera::CameraSystemData) for
//!   updates to the active camera using the associated camera controller.
//!
//! # Main Concepts
//!
//! - **CameraRenderSystemData**: Implements `ApplyCameraRenderSystem` to handle active camera
//!   uniform updates during the rendering phase.
//! - **ActiveCameraState**: Stores the uniform data for the currently active camera.
//! - **CameraComponent**: Represents individual camera entities, containing the camera
//!   and projection data, as well as an `active` flag.
//! - **WgpuRenderState**: Provides access to necessary WGPU resources like the GPU device
//!   and queue for buffer updates.
//!
//! # Key Responsibilities
//!
//! - Identifying the active camera from the list of available camera components.
//! - Updating the view and projection matrix uniforms of the active camera.
//! - Writing updated camera uniform data to the GPU buffer for rendering.
//!
//! # Usage
//!
//! The functionality is encapsulated in the `apply_many` method of `CameraRenderSystemData`
//! which is invoked during the rendering phase. This ensures that the correct camera data
//! is written to the GPU, enabling accurate rendering.
//!
//! Additionally, the `SystemFactory` implementation provides an interface to create instances
//! of `CameraRenderSystem`, which internally uses `CameraRenderSystemData`.

use crate::engine::components::ActiveCameraState;
use crate::engine::input_state::InputState;
use crate::engine::phases::render::WgpuRenderState;
use crate::engine::{
    ApplyCameraRenderSystem, ApplyCameraSystem, CameraComponent, CameraFpsControllerComponent,
    CameraRenderSystem, CameraSystem, CreateSystem, FrameContext, SystemFactory,
};
use crate::seconds::Seconds;
use std::convert::Infallible;
use tracing::warn;

#[derive(Debug)]
pub struct CameraRenderSystemData;

impl ApplyCameraRenderSystem for CameraRenderSystem {
    type Error = Infallible;

    /// Updates the GPU buffer with the active camera's view and projection matrix data.
    ///
    /// # Parameters
    /// - `wgpu_render`: The WGPU render state, providing access to the GPU device and queue.
    /// - `active_camera`: The state of the currently active camera, storing the uniform data.
    /// - `cameras`: A list of camera components from which the active camera is determined.
    ///
    /// # Behavior
    /// - If the WGPU device is not available, the function returns early.
    /// - Identifies the active camera (marked with `active: true`) from the provided list of camera components.
    /// - Updates the active camera's uniform data with its view and projection matrices.
    /// - Writes the updated camera uniform data to the GPU buffer for rendering.
    fn apply_many(
        &mut self,
        wgpu_render: &WgpuRenderState,
        active_camera: &mut ActiveCameraState,
        cameras: &[CameraComponent],
    ) {
        let Some(device) = wgpu_render.hardware().device() else {
            warn!("Skipping camera rendering because no device is set.");
            return;
        };

        let Some(resources) = wgpu_render.hardware().resources() else {
            warn!("Skipping camera rendering because no resources are set.");
            return;
        };

        // Pick the active camera.
        let Some(camera) = cameras.iter().find(|c| c.active) else {
            warn!("Skipping camera rendering because no active camera was found.");
            return;
        };

        // This only needs to be done during the render phase unless we use the camera for clip testing in software.
        active_camera
            .uniform
            .update_view_proj(&camera.camera, &camera.projection);
        resources
            .buffers()
            .camera_uniform
            .send_to_device(&device.queue(), active_camera.uniform);
    }
}

impl CreateSystem<CameraRenderSystem> for SystemFactory {
    fn create(&self) -> CameraRenderSystem {
        CameraRenderSystem(CameraRenderSystemData)
    }
}
