use crate::map::tile::{FloorType, Tile};
use crate::map::tile_chunk::TileChunk;
use crate::map::tile_coord::{ChunkLocalTileCoord, RegionLocalChunkCoord, WorldGlobalRegionCoord};
use crate::map::tile_world::TileWorld;
use crate::map::world_region::WorldRegion;
use range2d::Range2D;
use std::ops::Range;

pub struct WorldBuilder;

impl WorldBuilder {
    pub fn flat_floor(with_walls: bool) -> TileWorld {
        // Create 2x2 regions in the world.
        let mut world = TileWorld::with_capacity(4);
        for (wy, wx) in Range2D::new(-1..1, -1..1) {
            let region = WorldRegion::from_fn(|_coord| {
                TileChunk::from_fn(|_coord| {
                    Tile::new()
                        // TODO: Fix when tile atlas is ready
                        .with_floor(FloorType::Empty)
                })
            });

            world.insert(WorldGlobalRegionCoord::new(wx as i8, wy as i8), region);
        }

        if with_walls {
            Self::add_outer_walls(&mut world);
        }

        world
    }

    fn add_outer_walls(world: &mut TileWorld) {
        let Some(extents) = world.global_chunk_bounds() else {
            return;
        };

        for (region_coord, region) in world.iter_regions_with_coords_mut() {
            for (chunk_coord, chunk) in region.iter_with_coords_mut() {
                let chunk_pos_x = region_coord.x as i32 * WorldRegion::WIDTH_IN_CHUNKS as i32
                    + chunk_coord.x as i32;
                let chunk_pos_y = region_coord.y as i32 * WorldRegion::HEIGHT_IN_CHUNKS as i32
                    + chunk_coord.y as i32;

                let is_leftmost = chunk_pos_x == extents.min.x;
                let is_rightmost = chunk_pos_x == extents.max.x;
                let is_topmost = chunk_pos_y == extents.min.y;
                let is_bottommost = chunk_pos_y == extents.max.y;

                for (y, x) in Range2D::full(TileChunk::HEIGHT_IN_TILES, TileChunk::WIDTH_IN_TILES) {
                    let is_edge_x = (x == 0 && is_leftmost)
                        || (x == TileChunk::WIDTH_IN_TILES - 1 && is_rightmost);
                    let is_edge_y = (y == 0 && is_topmost)
                        || (y == TileChunk::HEIGHT_IN_TILES - 1 && is_bottommost);

                    if is_edge_x || is_edge_y {
                        chunk.apply(ChunkLocalTileCoord::new(x as u8, y as u8), |tile| {
                            if is_topmost && y == 0 {
                                *tile = tile.with_wall_south(true);
                            }
                            if is_bottommost && y == TileChunk::HEIGHT_IN_TILES - 1 {
                                *tile = tile.with_wall_north(true);
                            }
                            if is_leftmost && x == 0 {
                                *tile = tile.with_wall_west(true);
                            }
                            if is_rightmost && x == TileChunk::WIDTH_IN_TILES - 1 {
                                *tile = tile.with_wall_east(true);
                            }
                        });
                    }
                }
            }
        }
    }
}
