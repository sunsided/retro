use crate::map::tile::Tile;
use crate::map::tile_coord::ChunkLocalTileCoord;

/// A [`DenseTileChunk`] is a densely-packed are of [`Tile`] instances.
#[derive(Debug, Clone)]
pub struct DenseTileChunk {
    pub tiles: [Tile; DenseTileChunk::TILE_COUNT],
}

impl DenseTileChunk {
    pub const WIDTH: usize = 16;
    pub const HEIGHT: usize = 16;
    pub const TILE_COUNT: usize = Self::WIDTH * Self::HEIGHT;

    /// Creates a new chunk pre-filled with empty tiles.
    #[inline]
    pub const fn new_empty() -> Self {
        Self {
            tiles: [Tile::new(); DenseTileChunk::TILE_COUNT],
        }
    }

    #[inline]
    pub fn get(&self, coord: ChunkLocalTileCoord) -> &Tile {
        &self.tiles[coord.to_dense_tile_chunk_index()]
    }

    #[inline]
    pub fn set(&mut self, coord: ChunkLocalTileCoord, tile: Tile) {
        self.tiles[coord.to_dense_tile_chunk_index()] = tile;
    }
}
