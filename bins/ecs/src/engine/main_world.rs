use std::sync::Arc;
use parking_lot::Mutex;
use crate::camera::Projection;
use crate::engine::{CameraComponent, CameraEntityData, CameraFpsControllerComponent, ColorComponent, DynamicLightComponent, DynamicLightEntityData, MainWorld, PositionComponent, SceneBackgroundEntityData};
use crate::engine::components::{CameraData, ColorData, DynamicLightData, PositionData};
use crate::engine::phases::render::background::BackgroundColorData;

pub trait InitializeMainWorld {
    /// Initialize the main entities needed in this world, such as populating the
    /// [`SceneBackgroundArchetype`](crate::engine::SceneBackgroundArchetype), and more.
    fn create_initial_entities(&mut self);
}

impl<E, Q> InitializeMainWorld for MainWorld<E, Q> {
    fn create_initial_entities(&mut self) {
        self.spawn_scene_background(SceneBackgroundEntityData {
            background_color: BackgroundColorData::default().into()
        });

        self.spawn_camera(CameraEntityData {
            camera: CameraComponent(CameraData {
                active: true,
                ..Default::default()
            }),
            camera_fps_controller: CameraFpsControllerComponent(Default::default()),
        });

        self.spawn_dynamic_light(DynamicLightEntityData {
            dynamic_light: DynamicLightComponent(DynamicLightData::default()),
            color: ColorComponent(ColorData {
                r: 1.0,
                g: 0.84,
                b: 0.7,
                a: 1.0,
            }),
            position: PositionComponent(PositionData {
                x: 2.0,
                y: 2.0,
                z: 2.0,
            }),
        });
    }
}
