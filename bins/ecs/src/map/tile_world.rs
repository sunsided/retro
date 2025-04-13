use crate::map::dense_tile_chunk::DenseTileChunk;
use crate::map::tile_coord::ChunkCoord;
use fxhash::FxHashMap;

pub struct TileWorld {
    pub chunks: FxHashMap<ChunkCoord, DenseTileChunk>,
}
