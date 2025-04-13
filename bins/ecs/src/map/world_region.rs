use crate::map::tile_chunk::TileChunk;
use crate::map::tile_coord::{ChunkLocalTileCoord, RegionLocalChunkCoord};
use std::mem::MaybeUninit;

/// A [`WorldRegion`] is a densely-packed are of [`TileChunk`] instances.
#[derive(Debug, Clone)]
pub struct WorldRegion {
    pub chunks: [TileChunk; WorldRegion::CHUNK_COUNT],
}

impl WorldRegion {
    pub const WIDTH: usize = 2;
    pub const HEIGHT: usize = 2;
    pub const CHUNK_COUNT: usize = Self::WIDTH * Self::HEIGHT;

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
