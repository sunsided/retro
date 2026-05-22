use crate::map::tile::Tile;
use crate::map::tile_coord::ChunkLocalTileCoord;
use std::mem::MaybeUninit;
use std::ptr;

/// A [`TileChunk`] is a densely-packed are of [`Tile`] instances.
#[derive(Debug, Clone)]
pub struct TileChunk {
    pub tiles: [Tile; TileChunk::TILE_COUNT],
}

#[allow(dead_code)]
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
    pub fn from_fn<F>(mut f: F) -> Self
    where
        F: FnMut(ChunkLocalTileCoord) -> Tile,
    {
        // SAFETY: We're creating uninitialized memory for CHUNK_COUNT TileChunks
        let mut tiles: [MaybeUninit<Tile>; Self::TILE_COUNT] =
            unsafe { MaybeUninit::uninit().assume_init() };

        struct Guard {
            ptr: *mut [MaybeUninit<Tile>],
            initialized: usize,
        }

        impl Drop for Guard {
            fn drop(&mut self) {
                // SAFETY: only the first `initialized` elements are initialized
                for i in 0..self.initialized {
                    unsafe {
                        ptr::drop_in_place((*self.ptr)[i].as_mut_ptr());
                    }
                }
            }
        }

        let mut guard = Guard {
            ptr: &mut tiles,
            initialized: 0,
        };

        for (i, slot) in tiles.iter_mut().enumerate() {
            let coord = ChunkLocalTileCoord::from_dense_tile_chunk_index(i);
            *slot = MaybeUninit::new(f(coord));
            guard.initialized += 1;
        }

        // All elements initialized — no need to run Drop
        std::mem::forget(guard);

        // SAFETY: All chunks have been initialized
        Self {
            tiles: unsafe { std::mem::transmute_copy::<_, [Tile; Self::TILE_COUNT]>(&tiles) },
        }
    }

    #[inline]
    pub const fn get(&self, coord: ChunkLocalTileCoord) -> &Tile {
        &self.tiles[coord.to_dense_tile_chunk_index()]
    }

    #[inline]
    pub const fn set(&mut self, coord: ChunkLocalTileCoord, tile: Tile) {
        self.tiles[coord.to_dense_tile_chunk_index()] = tile;
    }

    #[inline]
    pub fn apply<F>(&mut self, coord: ChunkLocalTileCoord, mut tile: F)
    where
        F: FnMut(&mut Tile),
    {
        tile(&mut self.tiles[coord.to_dense_tile_chunk_index()])
    }

    #[inline]
    pub fn visit_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(ChunkLocalTileCoord, &mut Tile),
    {
        for (coord, tile) in self.iter_with_coords_mut() {
            f(coord, tile)
        }
    }

    #[inline]
    pub fn iter(&self) -> TileIter<'_> {
        self.tiles.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> TileIterMut<'_> {
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
    pub fn iter_with_coords_mut(&mut self) -> TileWithCoordIterMut<'_> {
        self.tiles.iter_mut().enumerate().map(|(i, tile)| {
            let coord = ChunkLocalTileCoord::from_dense_tile_chunk_index(i);
            (coord, tile)
        })
    }
}

#[allow(dead_code)]
pub type TileIter<'a> = core::slice::Iter<'a, Tile>;

#[allow(dead_code)]
pub type TileIterMut<'a> = core::slice::IterMut<'a, Tile>;

#[allow(dead_code)]
pub type TileWithCoordIter<'a> = core::iter::Map<
    core::iter::Enumerate<core::slice::Iter<'a, Tile>>,
    fn((usize, &Tile)) -> (ChunkLocalTileCoord, &Tile),
>;

#[allow(dead_code)]
pub type TileWithCoordIterMut<'a> = core::iter::Map<
    core::iter::Enumerate<core::slice::IterMut<'a, Tile>>,
    fn((usize, &mut Tile)) -> (ChunkLocalTileCoord, &mut Tile),
>;
