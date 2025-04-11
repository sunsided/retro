use crate::camera::CameraController;

#[derive(Debug, Clone)]
pub struct CameraFpsControllerData {
    pub controller: CameraController,
}

impl Default for CameraFpsControllerData {
    fn default() -> Self {
        Self {
            controller: CameraController::new(4.0, 0.4),
        }
    }
}
