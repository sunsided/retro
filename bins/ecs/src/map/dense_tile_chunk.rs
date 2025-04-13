use crate::engine::components::TileData;

/// A [`DenseTileChunk`] is a densely-packed are of [`TileData`]
#[derive(Debug, Clone)]
pub struct DenseTileChunk {
    pub tiles: [TileData; DenseTileChunk::COUNT],
}

impl DenseTileChunk {
    const WIDTH: usize = 16;
    const HEIGHT: usize = 16;
    const COUNT: usize = Self::WIDTH * Self::HEIGHT;

    #[inline]
    pub fn get(&self, x: usize, y: usize) -> &TileData {
        &self.tiles[y * Self::WIDTH + x]
    }

    #[inline]
    pub fn set(&mut self, x: usize, y: usize, tile: TileData) {
        self.tiles[y * Self::WIDTH + x] = tile;
    }
}
