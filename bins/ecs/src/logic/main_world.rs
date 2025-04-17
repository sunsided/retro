use crate::engine::components::{
    AmbientLightData, CameraData, ColorData, DynamicLightData, FogData, PositionData, SkydomeData,
    StarsData, SunData, TileData, TimeOfDayData,
};
use crate::engine::phases::render::background::BackgroundColorData;
use crate::engine::{
    CameraEntityData, DynamicLightEntityData, EntityAccess, EnvironmentEntityData,
    IterWgpuShaderEntities, IterWgpuTextureEntities, MainWorld, SceneBackgroundEntityData,
    TextureSourceData, TextureSourceType, WgpuShaderData, WgpuTextureData, WorldUserCommand,
    WorldUserCommandHandler,
};
use crate::engine::{TileEntityData, WgpuShaderEntityData, WgpuTextureEntityData};
use crate::map::tile::{CeilingType, FloorType, Tile};
use crate::map::world_builder::WorldBuilder;
use fxhash::FxHashSet;
use tracing::warn;
use wgpu::include_wgsl;

pub trait InitializeMainWorld {
    /// Initialize the main entities needed in this world, such as populating the
    /// [`SceneBackgroundArchetype`](crate::engine::SceneBackgroundArchetype), and more.
    fn create_initial_entities(&mut self);
}

impl<E, Q> InitializeMainWorld for MainWorld<E, Q> {
    fn create_initial_entities(&mut self) {
        let _fallback_normals = self.spawn_wgpu_texture(WgpuTextureEntityData {
            texture_source: TextureSourceData::new(TextureSourceType::FallbackNormals),
            wgpu_texture: WgpuTextureData::default(),
        });

        let _cover = self.spawn_wgpu_texture(WgpuTextureEntityData {
            texture_source: TextureSourceData::new(TextureSourceType::from_path(
                "textures/cover.webp",
            )),
            wgpu_texture: WgpuTextureData::default(),
        });

        let light_shader = self.spawn_wgpu_shader(WgpuShaderEntityData {
            wgpu_shader: WgpuShaderData::new(include_wgsl!("../shaders/light.wgsl")),
        });

        let _vga_shader = self.spawn_wgpu_shader(WgpuShaderEntityData {
            wgpu_shader: WgpuShaderData::new(include_wgsl!("../shaders/vga.wgsl")),
        });

        let _sky_shader = self.spawn_wgpu_shader(WgpuShaderEntityData {
            wgpu_shader: WgpuShaderData::new(include_wgsl!("../shaders/sky.wgsl")),
        });

        let _tile_shader = self.spawn_wgpu_shader(WgpuShaderEntityData {
            wgpu_shader: WgpuShaderData::new(include_wgsl!("../shaders/tile.wgsl")),
        });

        let _generic_shader = self.spawn_wgpu_shader(WgpuShaderEntityData {
            wgpu_shader: WgpuShaderData::new(include_wgsl!("../shaders/shader.wgsl")),
        });

        let _x = self.get_wgpu_shader_entity(light_shader);

        self.spawn_scene_background(SceneBackgroundEntityData {
            background_color: BackgroundColorData::default(),
        });

        self.spawn_environment(EnvironmentEntityData {
            time_of_day: TimeOfDayData::default(),
            ambient_light: AmbientLightData::default(),
            sun: SunData::default(),
            fog: FogData::default(),
            stars: StarsData::default(),
            skydome: SkydomeData::default(),
        });

        self.spawn_camera(CameraEntityData {
            camera: CameraData {
                active: true,
                ..Default::default()
            },
            camera_fps_controller: Default::default(),
        });

        self.spawn_dynamic_light(DynamicLightEntityData {
            dynamic_light: DynamicLightData::default(),
            color: ColorData {
                r: 1.0,
                g: 0.84,
                b: 0.7,
                a: 1.0,
            },
            position: PositionData {
                x: 2.0,
                y: 2.0,
                z: 2.0,
            },
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
                    position: PositionComponent(position),
                    velocity: VelocityComponent(VelocityData {
                        x: 0.0,
                        y: 0.0, // try another value
                        z: 0.0,
                    }),
                    transform: TransformComponent(rotation),
                    model: ModelComponent(Default::default()),
                });
            }
        }
         */

        let world = WorldBuilder::flat_floor(true);
        world
            .iter_regions_with_coords()
            .flat_map(|(wrc, region)| {
                region
                    .iter_with_coords()
                    .map(move |(rcc, chunk)| (*wrc + rcc, chunk))
            })
            .flat_map(|(wcc, chunk)| {
                chunk
                    .iter_with_coords()
                    .map(move |(ctc, chunk)| (wcc + ctc, chunk))
            })
            .for_each(|(wtc, tile)| {
                self.spawn_tile(TileEntityData {
                    tile: TileData::from(tile),
                    position: PositionData {
                        x: wtc.x as f32 - 2.0,
                        y: 0.0,
                        z: -(wtc.y as f32) + 2.0,
                    },
                });
            });

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

                self.spawn_tile(TileEntityData {
                    tile: TileData::from(tile),
                    position: PositionData {
                        x: x as f32 - 2.0,
                        y: 0.0,
                        z: -(y as f32) + 2.0,
                    },
                });
            }
        }

        // Add a tower in the center, stacked vertically
        /*
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
        */
    }
}

impl<E, Q> WorldUserCommandHandler for MainWorld<E, Q>
where
    Q: WorldUserCommand,
{
    fn handle_user_command(&mut self, command: Self::UserCommand) {
        warn!(?command, "Unhandled user command");
    }
}

pub trait CleanObsoleteResources {
    fn clean_obsolete_resources(&mut self);
}

impl<E, Q> CleanObsoleteResources for MainWorld<E, Q> {
    fn clean_obsolete_resources(&mut self) {
        // Textures
        let entites = FxHashSet::from_iter(self.iter_wgpu_texture_entities());
        self.states
            .resource_manager
            .textures
            .retain(|id, _| !entites.contains(id));

        // Shaders
        let entites = FxHashSet::from_iter(self.iter_wgpu_shader_entities());
        self.states
            .resource_manager
            .shaders
            .retain(|id, _| !entites.contains(id));
    }
}
