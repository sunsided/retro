use crate::map::tile::{FloorType, Tile};
use crate::map::tile_chunk::TileChunk;
use crate::map::tile_coord::{ChunkLocalTileCoord, RegionLocalChunkCoord, WorldGlobalRegionCoord};
use crate::map::tile_world::TileWorld;
use crate::map::world_region::WorldRegion;

pub struct WorldBuilder;

impl WorldBuilder {
    pub fn flat_floor() -> TileWorld {
        // Create 2x2 regions in the world.
        let mut world = TileWorld::with_capacity(4);
        for (wy, wx) in (-1..1).zip(-1..1) {
            // For each region, create a chunk.
            let mut region = WorldRegion::empty();
            for (cy, cx) in (0..WorldRegion::HEIGHT).zip(0..WorldRegion::HEIGHT) {
                // For each chunk, create flat floors.
                let mut chunk = TileChunk::new_empty();
                for (ty, tx) in (0..TileChunk::HEIGHT).zip(0..TileChunk::HEIGHT) {
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

        world
    }
}
