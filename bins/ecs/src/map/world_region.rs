use crate::map::dense_tile_chunk::DenseTileChunk;
use crate::map::tile_coord::{ChunkLocalTileCoord, RegionLocalChunkCoord};
use std::mem::MaybeUninit;

/// A [`WorldRegion`] is a densely-packed are of [`DenseTileChunk`] instances.
#[derive(Debug, Clone)]
pub struct WorldRegion {
    pub chunks: [DenseTileChunk; WorldRegion::CHUNK_COUNT],
}

impl WorldRegion {
    pub const WIDTH: usize = 2;
    pub const HEIGHT: usize = 2;
    pub const CHUNK_COUNT: usize = Self::WIDTH * Self::HEIGHT;

    /// Creates a new chunk pre-filled with empty chunks.
    #[inline]
    pub const fn empty() -> Self {
        let mut chunks: [MaybeUninit<DenseTileChunk>; Self::CHUNK_COUNT] =
            [const { MaybeUninit::uninit() }; Self::CHUNK_COUNT];
        let mut i = 0;
        while i < Self::CHUNK_COUNT {
            chunks[i] = MaybeUninit::new(DenseTileChunk::new_empty());
            i += 1;
        }
        // SAFETY: All entries were just initialized above
        Self {
            chunks: unsafe {
                std::mem::transmute::<_, [DenseTileChunk; Self::CHUNK_COUNT]>(chunks)
            },
        }
    }

    #[inline]
    pub fn get(&self, coord: RegionLocalChunkCoord) -> &DenseTileChunk {
        &self.chunks[coord.to_dense_region_index()]
    }

    #[inline]
    pub fn get_mut(&mut self, coord: RegionLocalChunkCoord) -> &mut DenseTileChunk {
        &mut self.chunks[coord.to_dense_region_index()]
    }

    #[inline]
    pub fn set(&mut self, coord: RegionLocalChunkCoord, chunk: DenseTileChunk) {
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

pub type ChunkIter<'a> = core::slice::Iter<'a, DenseTileChunk>;

pub type ChunkIterMut<'a> = core::slice::IterMut<'a, DenseTileChunk>;

pub type ChunkWithCoordIter<'a> = core::iter::Map<
    core::iter::Enumerate<core::slice::Iter<'a, DenseTileChunk>>,
    fn((usize, &DenseTileChunk)) -> (RegionLocalChunkCoord, &DenseTileChunk),
>;

pub type ChunkWithCoordIterMut<'a> = core::iter::Map<
    core::iter::Enumerate<core::slice::IterMut<'a, DenseTileChunk>>,
    fn((usize, &mut DenseTileChunk)) -> (RegionLocalChunkCoord, &mut DenseTileChunk),
>;
