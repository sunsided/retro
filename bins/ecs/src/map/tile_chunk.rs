use crate::map::tile::Tile;
use crate::map::tile_coord::ChunkLocalTileCoord;

/// A [`TileChunk`] is a densely-packed are of [`Tile`] instances.
#[derive(Debug, Clone)]
pub struct TileChunk {
    pub tiles: [Tile; TileChunk::TILE_COUNT],
}

impl TileChunk {
    /// The number of tiles this chunk is wide.
    pub const WIDTH_IN_TILES: usize = 16;
    /// The number of tiles this chunk is high.
    pub const HEIGHT_IN_TILES: usize = 16;
    /// The total number of tiles in this chunk, i.e. its area.
    pub const TILE_COUNT: usize = Self::WIDTH_IN_TILES * Self::HEIGHT_IN_TILES;

    /// Creates a new chunk pre-filled with empty tiles.
    #[inline]
    pub const fn new_empty() -> Self {
        Self {
            tiles: [Tile::new(); TileChunk::TILE_COUNT],
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

    #[inline]
    pub fn iter(&self) -> TileIter {
        self.tiles.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> TileIterMut {
        self.tiles.iter_mut()
    }

    #[inline]
    pub fn iter_with_coords(&self) -> TileWithCoordIter<'_> {
        self.tiles.iter().enumerate().map(|(i, tile)| {
            let coord = ChunkLocalTileCoord::from_dense_tile_chunk_index(i);
            (coord, tile)
        })
    }

    #[inline]
    pub fn iter_mut_with_coords(&mut self) -> TileWithCoordIterMut<'_> {
        self.tiles.iter_mut().enumerate().map(|(i, tile)| {
            let coord = ChunkLocalTileCoord::from_dense_tile_chunk_index(i);
            (coord, tile)
        })
    }
}

pub type TileIter<'a> = core::slice::Iter<'a, Tile>;

pub type TileIterMut<'a> = core::slice::IterMut<'a, Tile>;

pub type TileWithCoordIter<'a> = core::iter::Map<
    core::iter::Enumerate<core::slice::Iter<'a, Tile>>,
    fn((usize, &Tile)) -> (ChunkLocalTileCoord, &Tile),
>;

pub type TileWithCoordIterMut<'a> = core::iter::Map<
    core::iter::Enumerate<core::slice::IterMut<'a, Tile>>,
    fn((usize, &mut Tile)) -> (ChunkLocalTileCoord, &mut Tile),
>;
