use crate::map::dense_tile_chunk::DenseTileChunk;
use std::hash::{Hash, Hasher};
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
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
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

impl Hash for ChunkCoord {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Morton encoding (Z-order curve)
        let x = self.x as u32;
        let y = self.y as u32;
        fn part1by1(n: u32) -> u32 {
            let mut n = n & 0x0000ffff;
            n = (n | (n << 8)) & 0x00FF00FF;
            n = (n | (n << 4)) & 0x0F0F0F0F;
            n = (n | (n << 2)) & 0x33333333;
            n = (n | (n << 1)) & 0x55555555;
            n
        }
        let morton = (part1by1(y) << 1) | part1by1(x);
        morton.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map::identity_hasher::IdentityHasher;
    use std::hash::DefaultHasher;

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

    #[test]
    fn morton_hash_consistency() {
        let a = ChunkCoord { x: 5, y: 10 };
        let b = ChunkCoord { x: 5, y: 10 };
        let mut hasher_a = IdentityHasher::new();
        let mut hasher_b = IdentityHasher::new();
        a.hash(&mut hasher_a);
        b.hash(&mut hasher_b);
        assert_eq!(hasher_a.finish(), hasher_b.finish());
    }

    #[test]
    fn morton_spatial_locality() {
        let origin = ChunkCoord { x: 17, y: 23 };
        let neighbors = [
            ChunkCoord { x: 17, y: 24 },
            ChunkCoord { x: 18, y: 23 },
            ChunkCoord { x: 18, y: 24 },
            ChunkCoord { x: 16, y: 23 },
            ChunkCoord { x: 17, y: 22 },
            ChunkCoord { x: 10, y: 11 },
            ChunkCoord { x: 11, y: 10 },
            ChunkCoord { x: 11, y: 11 },
            ChunkCoord { x: 9, y: 10 },
            ChunkCoord { x: 10, y: 9 },
        ];

        let mut origin_hash = IdentityHasher::new();
        origin.hash(&mut origin_hash);
        let origin_value = origin_hash.finish();
        let origin_bucket = origin_value % 64; // Simulate hash map bucket size

        for neighbor in neighbors.iter() {
            let mut h = IdentityHasher::new();
            neighbor.hash(&mut h);
            let value = h.finish();
            let popdiff = origin_value.count_ones().abs_diff(value.count_ones());
            assert!(
                popdiff <= 2,
                "Hash popcount diff too high for neighbor {:?}: {}",
                neighbor,
                popdiff
            );

            let bucket = value % 64;
            let bucket_diff = origin_bucket.abs_diff(bucket);

            // Allow neighbor to land within ±1 bucket of origin
            assert!(
                bucket_diff <= 1,
                "Neighbor {:?} landed too far in bucket space: {} vs {} (diff {})",
                neighbor,
                origin_bucket,
                bucket,
                bucket_diff
            );
        }

        let far = ChunkCoord { x: 1000, y: 1000 };
        let mut far_hash = IdentityHasher::new();
        far.hash(&mut far_hash);
        let far_value = far_hash.finish();
        let far_bucket = far_value % 64;

        let popdiff = origin_value.count_ones().abs_diff(far_value.count_ones());
        assert!(
            popdiff >= 5,
            "Far chunk hash popcount unexpectedly close: {}",
            popdiff
        );
        assert!(
            origin_bucket.abs_diff(far_bucket) > 1,
            "Far chunk should land far from origin bucket but didn't: {} vs {}",
            origin_bucket,
            far_bucket
        );
    }
}
