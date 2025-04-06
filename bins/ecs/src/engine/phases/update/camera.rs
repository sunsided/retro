use crate::engine::{ApplyCameraSystem, CameraComponent, CameraFpsControllerComponent, CameraSystem, CreateSystem, FrameContext, SystemFactory};
use crate::engine::components::ActiveCameraState;
use crate::engine::input_state::InputState;

#[derive(Debug)]
pub struct CameraSystemData;

impl ApplyCameraSystem for CameraSystem {
    type Error = ();

    fn apply_single(&mut self, _input: &InputState, _active_camera: &mut ActiveCameraState, _camera: &mut CameraComponent, _camera_fps_controller: &mut CameraFpsControllerComponent) {
        unimplemented!(
            "CameraSystem::apply_single is not implemented. \
             Please use CameraSystem::apply_many instead.")
    }

    fn apply_many(&mut self, _input: &InputState, active_camera: &mut ActiveCameraState, cameras: &mut [CameraComponent], _camera_fps_controllers: &mut [CameraFpsControllerComponent]) {
        for camera in cameras.iter().filter(|c| c.active) {
            active_camera.uniform.update_view_proj(&camera.camera, &camera.projection);
        }
    }
}

impl CreateSystem<CameraSystem> for SystemFactory {
    fn create(&self) -> CameraSystem {
        CameraSystem(CameraSystemData)
    }
}
