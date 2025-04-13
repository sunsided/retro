use crate::map::dense_tile_chunk::DenseTileChunk;
use crate::map::identity_hasher::BuildIdentityHasher;
use crate::map::tile_coord::ChunkCoord;
use std::collections::HashMap;

type MortonEncodedHashMap = HashMap<ChunkCoord, DenseTileChunk, BuildIdentityHasher>;

pub struct TileWorld {
    pub chunks: MortonEncodedHashMap,
}
