use std::sync::Arc;
use cgmath::{InnerSpace, Rotation3, Zero};
use parking_lot::Mutex;
use crate::camera::Projection;
use crate::engine::{CameraComponent, CameraEntityData, CameraFpsControllerComponent, ColorComponent, CubeEntityData, DynamicLightComponent, DynamicLightEntityData, MainWorld, ModelComponent, PositionComponent, SceneBackgroundEntityData, TransformComponent, VelocityComponent};
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

        const NUM_INSTANCES_PER_ROW: u32 = 10;
        const SPACE_BETWEEN: f32 = 3.0;
        // Spawn some cubes.
        for z in 0..NUM_INSTANCES_PER_ROW {
            for x in 0..NUM_INSTANCES_PER_ROW {
                let x = SPACE_BETWEEN * (x as f32 - NUM_INSTANCES_PER_ROW as f32 / 2.0);
                let z = SPACE_BETWEEN * (z as f32 - NUM_INSTANCES_PER_ROW as f32 / 2.0);

                let position = cgmath::Vector3 { x, y: 0.0, z };

                let rotation = if position.is_zero() {
                    cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.0))
                } else {
                    cgmath::Quaternion::from_axis_angle(position.normalize(), cgmath::Deg(45.0))
                };

                self.spawn_cube(CubeEntityData {
                    position: PositionComponent(position.into()),
                    velocity: VelocityComponent(Default::default()),
                    transform: TransformComponent(rotation.into()),
                    model: ModelComponent(Default::default()),
                });
            }
        }
    }
}
