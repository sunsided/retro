use crate::map::tile::{FloorType, Tile};
use crate::map::tile_chunk::TileChunk;
use crate::map::tile_coord::{ChunkLocalTileCoord, RegionLocalChunkCoord, WorldGlobalRegionCoord};
use crate::map::tile_world::TileWorld;
use crate::map::world_region::WorldRegion;
use itertools::Itertools;

pub struct WorldBuilder;

impl WorldBuilder {
    pub fn flat_floor(with_walls: bool) -> TileWorld {
        // TODO: Add walls around the entire world if requested

        // Create 2x2 regions in the world.
        let mut world = TileWorld::with_capacity(4);
        for (wy, wx) in (-1..1).cartesian_product(-1..1) {
            // For each region, create a chunk.
            let mut region = WorldRegion::empty();
            for (cy, cx) in (0..WorldRegion::HEIGHT_IN_CHUNKS)
                .cartesian_product(0..WorldRegion::HEIGHT_IN_CHUNKS)
            {
                // For each chunk, create flat floors.
                let mut chunk = TileChunk::new_empty();
                for (ty, tx) in
                    (0..TileChunk::HEIGHT_IN_TILES).cartesian_product(0..TileChunk::HEIGHT_IN_TILES)
                {
                    let tile = Tile::new()
                        // TODO: Fix when tile atlas is ready
                        .with_floor(FloorType::Empty);

                    // TODO: Create a construct-from-iterator function.
                    chunk.set(ChunkLocalTileCoord::new(tx as u8, ty as u8), tile);
                }

                // TODO: Create a construct-from-iterator function.
                region.set(RegionLocalChunkCoord::new(cx as u8, cy as u8), chunk);
            }
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

                for (y, x) in
                    (0..TileChunk::HEIGHT_IN_TILES).cartesian_product(0..TileChunk::WIDTH_IN_TILES)
                {
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
