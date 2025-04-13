use crate::map::tile::Tile;
use crate::map::tile_coord::LocalTileCoord;

/// A [`DenseTileChunk`] is a densely-packed are of [`Tile`] instances.
#[derive(Debug, Clone)]
pub struct DenseTileChunk {
    pub tiles: [Tile; DenseTileChunk::COUNT],
}

impl DenseTileChunk {
    pub const WIDTH: usize = 16;
    pub const HEIGHT: usize = 16;
    pub const COUNT: usize = Self::WIDTH * Self::HEIGHT;

    /// Creates a new chunk pre-filled with empty tiles.
    #[inline]
    pub const fn empty() -> Self {
        Self {
            tiles: [Tile::new(); DenseTileChunk::COUNT],
        }
    }

    #[inline]
    pub fn get(&self, coord: LocalTileCoord) -> &Tile {
        &self.tiles[coord.to_dense_tile_chunk_index()]
    }

    #[inline]
    pub fn set(&mut self, coord: LocalTileCoord, tile: Tile) {
        self.tiles[coord.to_dense_tile_chunk_index()] = tile;
    }
}
