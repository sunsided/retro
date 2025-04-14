use crate::engine::components::TileData;
use crate::engine::phases::render::WgpuRenderState;
use crate::engine::{
    ApplyTileRenderSystem, CreateSystem, PositionComponent, SystemFactory, TileComponent,
    TileRenderSystem,
};
use crate::map::tile_instance::TileInstance;
use cgmath::{Matrix4, Rad, Vector3};
use std::convert::Infallible;
use tracing::warn;
use wgpu::util::DeviceExt;

#[derive(Debug, Default)]
pub struct TileRenderSystemData;

impl CreateSystem<TileRenderSystem> for SystemFactory {
    fn create(&self) -> TileRenderSystem {
        TileRenderSystem(TileRenderSystemData)
    }
}

impl ApplyTileRenderSystem for TileRenderSystem {
    type Error = Infallible;

    fn apply_many(
        &mut self,
        gpu: &mut WgpuRenderState,
        tiles: &[TileComponent],
        positions: &[PositionComponent],
    ) {
        let Ok((device, resources, frame_state)) = gpu.for_rendering() else {
            warn!("Skipping background rendering because hardware or frame is not ready.");
            return;
        };

        let frame_entity_ids = frame_state.per_frame_entity_ids();
        let floor_id = frame_entity_ids.next();
        let wall_id = frame_entity_ids.next();
        let ceil_id = frame_entity_ids.next();

        let mut floors = Vec::with_capacity(tiles.len());
        let mut walls = Vec::with_capacity(tiles.len());
        let mut ceilings = Vec::with_capacity(tiles.len());

        for (tile, pos) in tiles.iter().zip(positions) {
            let transform = compute_tile_transform(pos);

            floors.push(TileInstance {
                transform,
                tile_index: tile.0.floor() as u32,
                decoration_index: 0,
                entity_id: floor_id,
            });

            if tile.has_ceiling() {
                let ceiling_transform =
                    Matrix4::from_translation(Vector3::new(pos.x, pos.y + 1.0, pos.z))
                        * Matrix4::from_angle_x(Rad(std::f32::consts::PI)); // flip upside down
                let _index = if tile.has_ceiling_variant() {
                    tile.ceiling() as u32
                } else {
                    tile.floor() as u32
                };

                // TODO: Implement proper texture atlas
                let index = 0;

                ceilings.push(TileInstance {
                    transform: ceiling_transform.into(),
                    tile_index: index,
                    decoration_index: 0,
                    entity_id: ceil_id,
                });
            }

            spawn_wall_instances_from_tile(&tile, pos.x, pos.z, wall_id, &mut walls);
        }

        if floors.is_empty() {
            return;
        }

        let floor_buffer = device
            .device()
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Tile Instance Buffer"),
                contents: bytemuck::cast_slice(&floors),
                usage: wgpu::BufferUsages::VERTEX,
            });

        let mut pass = frame_state.render_pass_with(&resources.pipelines().tile);
        pass.set_bind_group(0, &resources.neo.frame_globals.bind_group, &[]);
        pass.set_bind_group(1, &resources.bind_groups().uniforms.camera_uniform, &[]);
        pass.set_bind_group(2, &resources.bind_groups().uniforms.light_uniforms, &[]);
        // The texture atlas
        pass.set_bind_group(3, &resources.bind_groups().textures.stonepath, &[]);
        pass.set_index_buffer(
            resources.buffers().tile_index.slice(..),
            wgpu::IndexFormat::Uint16,
        );
        pass.set_vertex_buffer(0, resources.buffers().tile_vertex.slice(..));
        pass.set_vertex_buffer(1, floor_buffer.slice(..));
        pass.draw_indexed(0..6, 0, 0..floors.len() as u32);

        // Render walls
        if !walls.is_empty() {
            let wall_buffer =
                device
                    .device()
                    .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("Tile Instance Buffer"),
                        contents: bytemuck::cast_slice(&walls),
                        usage: wgpu::BufferUsages::VERTEX,
                    });

            pass.set_bind_group(3, &resources.bind_groups().textures.bricks, &[]);
            pass.set_vertex_buffer(0, resources.buffers().tile_vertex.slice(..));
            pass.set_vertex_buffer(1, wall_buffer.slice(..));
            pass.draw_indexed(0..6, 0, 0..walls.len() as u32);
        }

        // Ceilings
        if !ceilings.is_empty() {
            let ceiling_buffer =
                device
                    .device()
                    .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("Tile Ceiling Instance Buffer"),
                        contents: bytemuck::cast_slice(&ceilings),
                        usage: wgpu::BufferUsages::VERTEX,
                    });

            pass.set_bind_group(3, &resources.bind_groups().textures.bricks, &[]);
            pass.set_vertex_buffer(0, resources.buffers().tile_vertex.slice(..));
            pass.set_vertex_buffer(1, ceiling_buffer.slice(..));
            pass.draw_indexed(0..6, 0, 0..ceilings.len() as u32);
        }
    }
}

fn compute_tile_transform(pos: &PositionComponent) -> [[f32; 4]; 4] {
    let translation = Matrix4::from_translation(Vector3::new(pos.x, pos.y, pos.z));
    translation.into()
}

pub fn spawn_wall_instances_from_tile(
    tile: &TileData,
    tile_x: f32,
    tile_z: f32,
    entity_id: u32,
    out: &mut Vec<TileInstance>,
) {
    tile.each_wall(|dir| {
        let rot_y = dir.rotation_y_radians();
        let (dx, dy) = dir.offset();

        let base_pos = Vector3::new(tile_x + 0.5 * dx as f32, 0.5, tile_z + 0.5 * dy as f32);

        let upright = Matrix4::from_angle_x(Rad(-std::f32::consts::FRAC_PI_2)); // rotate X axis to stand up
        let rotate = Matrix4::from_angle_y(Rad(rot_y)); // rotate around Y based on wall direction
        let translate = Matrix4::from_translation(base_pos);

        let transform = translate * rotate * upright;

        out.push(TileInstance {
            transform: transform.into(),
            tile_index: tile.wall() as u32,
            decoration_index: 0,
            entity_id,
        });
    });
}
