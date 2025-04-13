use crate::map::dense_tile_chunk::DenseTileChunk;
use std::hash::Hash;
use std::ops::Add;

/// The local coordinates of a tile within a [`DenseTileChunk`](DenseTileChunk).
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct LocalTileCoord {
    pub x: u8,
    pub y: u8,
}

/// The world coordinates of a tile after applying transformations from both
/// [`ChunkCoord`] and [`LocalTileCoord`].
///
/// Note that these coordinates still do not reflect actual 3D world coordinates
/// as these are governed by transformations performed by the ECS.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct WorldTileCoord {
    pub x: i32,
    pub y: i32,
}

/// The coordinates of a [`DenseTileChunk`](DenseTileChunk)
/// in chunk-world space.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct ChunkCoord {
    pub x: i32,
    pub y: i32,
}

impl LocalTileCoord {
    #[inline(always)]
    pub const fn new(x: u8, y: u8) -> Self {
        debug_assert!(x < DenseTileChunk::WIDTH as u8);
        debug_assert!(y < DenseTileChunk::HEIGHT as u8);
        Self { x, y }
    }

    #[inline(always)]
    pub const fn to_dense_tile_chunk_index(self) -> usize {
        (self.y as usize) * DenseTileChunk::WIDTH + (self.x as usize)
    }

    #[inline(always)]
    pub const fn from_dense_tile_chunk_index(index: usize) -> Self {
        debug_assert!(index < DenseTileChunk::COUNT);
        let x = (index % DenseTileChunk::WIDTH) as u8;
        let y = (index / DenseTileChunk::WIDTH) as u8;
        Self { x, y }
    }
}

impl ChunkCoord {
    #[inline(always)]
    pub const fn add(self, rhs: LocalTileCoord) -> WorldTileCoord {
        let x = self.x + rhs.x as i32;
        let y = self.y + rhs.y as i32;
        WorldTileCoord { x, y }
    }
}

impl Add<LocalTileCoord> for ChunkCoord {
    type Output = WorldTileCoord;

    #[inline(always)]
    fn add(self, rhs: LocalTileCoord) -> Self::Output {
        ChunkCoord::add(self, rhs)
    }
}

impl Add<ChunkCoord> for LocalTileCoord {
    type Output = WorldTileCoord;

    #[inline(always)]
    fn add(self, rhs: ChunkCoord) -> Self::Output {
        ChunkCoord::add(rhs, self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_tile_indexing_is_correct() {
        let coord = LocalTileCoord::new(3, 2);
        let index = coord.to_dense_tile_chunk_index();
        assert_eq!(index, 2 * DenseTileChunk::WIDTH + 3);
    }

    #[test]
    fn world_tile_coord_from_chunk_and_local_addition() {
        let chunk = ChunkCoord { x: 16, y: 32 };
        let local = LocalTileCoord::new(5, 7);
        let world = chunk + local;

        assert_eq!(world, WorldTileCoord { x: 21, y: 39 });

        let reverse = local + chunk;
        assert_eq!(reverse, world);
    }

    #[test]
    fn chunk_and_local_add_symmetry() {
        let chunk = ChunkCoord { x: -16, y: -8 };
        let local = LocalTileCoord::new(15, 15);

        let a = chunk + local;
        let b = local + chunk;

        assert_eq!(a, b);
        assert_eq!(a.x, -1);
        assert_eq!(a.y, 7);
    }

    #[test]
    fn index_roundtrip() {
        for y in 0..DenseTileChunk::HEIGHT {
            for x in 0..DenseTileChunk::WIDTH {
                let original = LocalTileCoord::new(x as u8, y as u8);
                let index = original.to_dense_tile_chunk_index();
                let recovered = LocalTileCoord::from_dense_tile_chunk_index(index);
                assert_eq!(original, recovered);
            }
        }
    }
}
