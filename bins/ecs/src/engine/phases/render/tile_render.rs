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
        let Ok((device, frame_state)) = gpu.for_rendering() else {
            warn!("Skipping background rendering because hardware or frame is not ready.");
            return;
        };

        let mut instances = Vec::with_capacity(tiles.len());

        for (tile, pos) in tiles.iter().zip(positions) {
            let transform = compute_tile_transform(pos);

            instances.push(TileInstance {
                transform,
                tile_index: tile.floor_layer() as u32,
                decoration_index: 0,
                entity_id: 0,
            });

            spawn_wall_instances_from_tile(&tile, pos.x, pos.z, 0, &mut instances);
        }

        if instances.is_empty() {
            return;
        }

        let instance_buffer =
            device
                .device()
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Tile Instance Buffer"),
                    contents: bytemuck::cast_slice(&instances),
                    usage: wgpu::BufferUsages::VERTEX,
                });

        let mut pass = frame_state.render_pass_with(&device.pipelines().tile);
        pass.set_bind_group(0, &device.bind_groups().frame_global_uniform, &[]);
        pass.set_bind_group(1, &device.bind_groups().camera_uniform, &[]);
        pass.set_bind_group(2, &device.bind_groups().light_uniforms, &[]);
        // The texture atlas
        pass.set_bind_group(3, &device.bind_groups().bricks, &[]);

        pass.set_vertex_buffer(0, device.buffers().tile_vertex.slice(..));
        pass.set_vertex_buffer(1, instance_buffer.slice(..));

        pass.set_index_buffer(
            device.buffers().tile_index.slice(..),
            wgpu::IndexFormat::Uint16,
        );
        pass.draw_indexed(0..6, 0, 0..instances.len() as u32);
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
            tile_index: tile.wall_layer() as u32,
            decoration_index: 0,
            entity_id,
        });
    });
}
