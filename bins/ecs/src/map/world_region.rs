use crate::map::tile_chunk::TileChunk;
use crate::map::tile_coord::RegionLocalChunkCoord;
use std::mem::MaybeUninit;

/// A [`WorldRegion`] is a densely-packed are of [`TileChunk`] instances.
#[derive(Debug, Clone)]
pub struct WorldRegion {
    pub chunks: [TileChunk; WorldRegion::CHUNK_COUNT],
}

impl WorldRegion {
    /// The width of a region in terms of chunks.
    pub const WIDTH_IN_CHUNKS: usize = 2;
    /// The height of a region in terms of chunks.
    pub const HEIGHT_IN_CHUNKS: usize = 2;
    /// The total number of chunks in a region.
    pub const CHUNK_COUNT: usize = Self::WIDTH_IN_CHUNKS * Self::HEIGHT_IN_CHUNKS;

    /// The width of a region in terms of tiles,
    /// based on [`WIDTH_IN_CHUNKS`](Self::WIDTH_IN_CHUNKS) and [`TileChunk::WIDTH_IN_TILES`](TileChunk::WIDTH_IN_TILES).
    pub const WIDTH_IN_TILES: usize = Self::WIDTH_IN_CHUNKS * TileChunk::WIDTH_IN_TILES;
    /// The height of a region in terms of tiles,
    /// based on [`HEIGHT_IN_CHUNKS`](Self::HEIGHT_IN_CHUNKS) and [`TileChunk::HEIGHT_IN_TILES`](TileChunk::HEIGHT_IN_TILES).
    pub const HEIGHT_IN_TILES: usize = Self::HEIGHT_IN_CHUNKS * TileChunk::HEIGHT_IN_TILES;
    /// The total number of tiles in a region.
    pub const TILE_COUNT: usize = Self::WIDTH_IN_TILES * Self::HEIGHT_IN_TILES;

    /// Creates a new chunk pre-filled with empty chunks.
    #[inline]
    pub const fn empty() -> Self {
        let mut chunks: [MaybeUninit<TileChunk>; Self::CHUNK_COUNT] =
            [const { MaybeUninit::uninit() }; Self::CHUNK_COUNT];
        let mut i = 0;
        while i < Self::CHUNK_COUNT {
            chunks[i] = MaybeUninit::new(TileChunk::new_empty());
            i += 1;
        }
        // SAFETY: All entries were just initialized above
        Self {
            chunks: unsafe { std::mem::transmute::<_, [TileChunk; Self::CHUNK_COUNT]>(chunks) },
        }
    }

    #[inline]
    pub fn get(&self, coord: RegionLocalChunkCoord) -> &TileChunk {
        &self.chunks[coord.to_dense_region_index()]
    }

    #[inline]
    pub fn get_mut(&mut self, coord: RegionLocalChunkCoord) -> &mut TileChunk {
        &mut self.chunks[coord.to_dense_region_index()]
    }

    #[inline]
    pub fn set(&mut self, coord: RegionLocalChunkCoord, chunk: TileChunk) {
        self.chunks[coord.to_dense_region_index()] = chunk;
    }

    #[inline]
    pub fn iter(&self) -> ChunkIter {
        self.chunks.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> ChunkIterMut {
        self.chunks.iter_mut()
    }

    #[inline]
    pub fn iter_with_coords(&self) -> ChunkWithCoordIter<'_> {
        self.chunks.iter().enumerate().map(|(i, chunk)| {
            let coord = RegionLocalChunkCoord::from_dense_region_index(i);
            (coord, chunk)
        })
    }

    #[inline]
    pub fn iter_mut_with_coords(&mut self) -> ChunkWithCoordIterMut<'_> {
        self.chunks.iter_mut().enumerate().map(|(i, chunk)| {
            let coord = RegionLocalChunkCoord::from_dense_region_index(i);
            (coord, chunk)
        })
    }
}

pub type ChunkIter<'a> = core::slice::Iter<'a, TileChunk>;

pub type ChunkIterMut<'a> = core::slice::IterMut<'a, TileChunk>;

pub type ChunkWithCoordIter<'a> = core::iter::Map<
    core::iter::Enumerate<core::slice::Iter<'a, TileChunk>>,
    fn((usize, &TileChunk)) -> (RegionLocalChunkCoord, &TileChunk),
>;

pub type ChunkWithCoordIterMut<'a> = core::iter::Map<
    core::iter::Enumerate<core::slice::IterMut<'a, TileChunk>>,
    fn((usize, &mut TileChunk)) -> (RegionLocalChunkCoord, &mut TileChunk),
>;
