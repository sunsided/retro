use crate::engine::components::{
    AmbientLightData, CameraData, ColorData, DynamicLightData, PositionData, SkydomeData,
    StarsData, SunData, TileData, TimeOfDayData,
};
use crate::engine::phases::render::background::BackgroundColorData;
use crate::engine::TileEntityData;
use crate::engine::{
    CameraComponent, CameraEntityData, CameraFpsControllerComponent, ColorComponent,
    DynamicLightComponent, DynamicLightEntityData, EnvironmentEntityData, MainWorld,
    PositionComponent, SceneBackgroundEntityData,
};
use crate::map::tile::{CeilingType, FloorType, Tile, WallType};
use crate::map::tile_map::SparseTileMap;
use crate::map::world_builder::WorldBuilder;

pub trait InitializeMainWorld {
    /// Initialize the main entities needed in this world, such as populating the
    /// [`SceneBackgroundArchetype`](crate::engine::SceneBackgroundArchetype), and more.
    fn create_initial_entities(&mut self);
}

impl<E, Q> InitializeMainWorld for MainWorld<E, Q> {
    fn create_initial_entities(&mut self) {
        self.spawn_scene_background(SceneBackgroundEntityData {
            background_color: BackgroundColorData::default().into(),
        });

        self.spawn_environment(EnvironmentEntityData {
            time_of_day: TimeOfDayData::default().into(),
            ambient_light: AmbientLightData::default().into(),
            sun: SunData::default().into(),
            stars: StarsData::default().into(),
            skydome: SkydomeData::default().into(),
        });

        self.spawn_camera(CameraEntityData {
            camera: CameraComponent::new(CameraData {
                active: true,
                ..Default::default()
            }),
            camera_fps_controller: CameraFpsControllerComponent::new(Default::default()),
        });

        self.spawn_dynamic_light(DynamicLightEntityData {
            dynamic_light: DynamicLightComponent::new(DynamicLightData::default()),
            color: ColorComponent::new(ColorData {
                r: 1.0,
                g: 0.84,
                b: 0.7,
                a: 1.0,
            }),
            position: PositionComponent::new(PositionData {
                x: 2.0,
                y: 2.0,
                z: 2.0,
            }),
        });

        /*
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
                    velocity: VelocityComponent(VelocityData {
                        x: 0.0,
                        y: 0.0, // try another value
                        z: 0.0,
                    }),
                    transform: TransformComponent(rotation.into()),
                    model: ModelComponent(Default::default()),
                });
            }
        }
         */

        let world = WorldBuilder::flat_floor();

        let mut tilemap = SparseTileMap::new();

        // Create a floor with walls around the edge
        for y in 0..5 {
            for x in 0..5 {
                let is_edge = x == 0 || y == 0 || x == 4 || y == 4;
                let mut tile = Tile::default().with_floor(FloorType::Empty);

                let tile = if is_edge {
                    tile.set_wall_south(y == 0);
                    tile.set_wall_north(y == 4);
                    tile.set_wall_west(x == 0);
                    tile.set_wall_east(x == 4);
                    tile.set_ceiling(CeilingType::CopyWall);
                    tile
                } else {
                    if x < 2 || x > 2 || y < 2 || y > 2 {
                        tile.set_ceiling(CeilingType::CopyWall);
                        tile
                    } else {
                        tile
                    }
                };
                tilemap.set(x, y, 0, tile);

                self.spawn_tile(TileEntityData {
                    tile: TileData::from(tile).into(),
                    position: PositionData {
                        x: x as f32 - 2.0,
                        y: 0.0,
                        z: -(y as f32) + 2.0,
                    }
                    .into(),
                });
            }
        }

        // Add a tower in the center, stacked vertically
        tilemap.set(
            2,
            2,
            1,
            Tile::default()
                .with_floor(FloorType::Stone)
                .with_wall(WallType::Brick),
        );
        tilemap.set(
            2,
            2,
            2,
            Tile::default()
                .with_floor(FloorType::Stone)
                .with_wall(WallType::Brick),
        );
    }
}
