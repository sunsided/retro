use std::convert::Infallible;
use std::time::Duration;
use tracing::debug;
use crate::engine::{ApplyCameraSystem, CameraComponent, CameraFpsControllerComponent, CameraSystem, CreateSystem, FrameContext, SystemFactory};
use crate::engine::components::ActiveCameraState;
use crate::engine::input_state::InputState;
use crate::engine::phases::render::WgpuRenderState;
use crate::seconds::Seconds;
use crate::wgpu::texture::Texture;

#[derive(Debug)]
pub struct CameraSystemData;

impl ApplyCameraSystem for CameraSystem {
    type Error = Infallible;

    fn apply_many(&mut self, input: &InputState, wgpu_render: &WgpuRenderState, cameras: &mut [CameraComponent], controllers: &mut [CameraFpsControllerComponent]) {
        // We only update the active camera. It could be interesting to use the cameras for presence
        // detection and even though they're not currently used for rendering, they might trigger
        // some events. Just thinking out loud!
        let Some((camera, controller)) = cameras.iter_mut().zip(controllers).find(|(c, _)| c.active) else {
            return;
        };

        controller.controller.process_input(input);
        controller.controller.update_camera(&mut camera.camera, input.delta());
        if camera.fixed_size {
            return;
        }

        // Update the aspect ratio given the render surface size.
        if let Some(size) = wgpu_render.surface_size() {
            if wgpu_render.retro_enabled() {
                // During VGA rendering, the aspect ratio must be clipped to the VGA ratio (1.6:1).
                camera.projection.resize(Texture::VGA_WIDTH, Texture::VGA_HEIGHT);
            }
            else {
                camera.projection.resize(size.width, size.height);
            }
        }
    }
}

impl CreateSystem<CameraSystem> for SystemFactory {
    fn create(&self) -> CameraSystem {
        CameraSystem(CameraSystemData)
    }
}
