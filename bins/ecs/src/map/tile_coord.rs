use crate::map::div_floor;
use crate::map::tile_chunk::TileChunk;
use crate::map::world_region::WorldRegion;
use std::hash::Hash;
use std::ops::Add;

/// The local coordinates of a [`Tile`](super::tile::Tile) within a [`TileChunk`].
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct ChunkLocalTileCoord {
    pub x: u8,
    pub y: u8,
}

/// The local coordinate of a [`TileChunk`] in a [`WorldRegion`].
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct RegionLocalChunkCoord {
    pub x: u8,
    pub y: u8,
}

/// The translated local coordinate of a [`Tile`](super::tile::Tile) in a [`WorldRegion`].
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct RegionLocalTileCoord {
    pub x: u16,
    pub y: u16,
}

/// The local coordinate of a [`WorldRegion`] within the world.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct WorldGlobalRegionCoord {
    pub x: i8,
    pub y: i8,
}

/// The translated local coordinate of a [`TileChunk`] within the world.
/// Marks the global coordinate offset of the chunk's base coordinate in world space.
///
/// This is typically the outcome of adding region offset and chunk offset. For construction
/// of the map, you're likely more interested in [`WorldGlobalRegionCoord`].
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct WorldGlobalChunkCoord {
    pub x: i32,
    pub y: i32,
}

/// The translated local coordinate of a [`Tile`](super::tile::Tile) within the world.
/// Marks the global coordinate the tile coordinate in world space.
///
/// This is typically the outcome of adding region offset, chunk offset and tile offset.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct WorldGlobalTileCoord {
    pub x: i32,
    pub y: i32,
}

impl RegionLocalChunkCoord {
    pub const fn add_chunk_local_tile_coord(
        self,
        coord: ChunkLocalTileCoord,
    ) -> RegionLocalTileCoord {
        RegionLocalTileCoord {
            x: self.x as u16 * TileChunk::WIDTH_IN_TILES as u16 + coord.x as u16,
            y: self.y as u16 * TileChunk::HEIGHT_IN_TILES as u16 + coord.y as u16,
        }
    }

    pub const fn const_eq(&self, rhs: Self) -> bool {
        self.x == rhs.x && self.y == rhs.y
    }
}

impl Add<ChunkLocalTileCoord> for RegionLocalChunkCoord {
    type Output = RegionLocalTileCoord;

    fn add(self, rhs: ChunkLocalTileCoord) -> Self::Output {
        self.add_chunk_local_tile_coord(rhs)
    }
}

impl WorldGlobalRegionCoord {
    pub const fn add_region_local_tile_coord(
        self,
        coord: RegionLocalTileCoord,
    ) -> WorldGlobalTileCoord {
        WorldGlobalTileCoord {
            x: self.x as i32 * WorldRegion::WIDTH_IN_TILES as i32 + coord.x as i32,
            y: self.y as i32 * WorldRegion::HEIGHT_IN_TILES as i32 + coord.y as i32,
        }
    }

    pub const fn const_eq(&self, rhs: Self) -> bool {
        self.x == rhs.x && self.y == rhs.y
    }
}

impl Add<RegionLocalTileCoord> for WorldGlobalRegionCoord {
    type Output = WorldGlobalTileCoord;

    fn add(self, rhs: RegionLocalTileCoord) -> Self::Output {
        self.add_region_local_tile_coord(rhs)
    }
}

impl WorldGlobalRegionCoord {
    pub const fn add(
        self,
        rlc: RegionLocalChunkCoord,
        clc: ChunkLocalTileCoord,
    ) -> WorldGlobalTileCoord {
        self.add_region_local_tile_coord(rlc.add_chunk_local_tile_coord(clc))
    }

    pub const fn add_region_local_chunk_coord(
        self,
        coord: RegionLocalChunkCoord,
    ) -> WorldGlobalChunkCoord {
        WorldGlobalChunkCoord {
            x: self.x as i32 * WorldRegion::WIDTH_IN_TILES as i32
                + coord.x as i32 * TileChunk::WIDTH_IN_TILES as i32,
            y: self.y as i32 * WorldRegion::HEIGHT_IN_TILES as i32
                + coord.y as i32 * TileChunk::HEIGHT_IN_TILES as i32,
        }
    }
}

impl Add<RegionLocalChunkCoord> for WorldGlobalRegionCoord {
    type Output = WorldGlobalChunkCoord;

    fn add(self, rhs: RegionLocalChunkCoord) -> Self::Output {
        self.add_region_local_chunk_coord(rhs)
    }
}

impl WorldGlobalChunkCoord {
    pub const fn add_chunk_local_tile_coord(
        self,
        coord: ChunkLocalTileCoord,
    ) -> WorldGlobalTileCoord {
        WorldGlobalTileCoord {
            x: self.x + coord.x as i32,
            y: self.y + coord.y as i32,
        }
    }
}

impl Add<ChunkLocalTileCoord> for WorldGlobalChunkCoord {
    type Output = WorldGlobalTileCoord;

    fn add(self, rhs: ChunkLocalTileCoord) -> Self::Output {
        WorldGlobalTileCoord {
            x: self.x + rhs.x as i32,
            y: self.y + rhs.y as i32,
        }
    }
}

impl WorldGlobalTileCoord {
    #[inline(always)]
    pub const fn add(self, coord: ChunkLocalTileCoord) -> Self {
        WorldGlobalTileCoord {
            x: self.x + coord.x as i32,
            y: self.y + coord.y as i32,
        }
    }

    /// Alias for [`add`](Self::add).
    #[inline(always)]
    pub const fn add_chunk_local_tile_coord(self, coord: ChunkLocalTileCoord) -> Self {
        self.add(coord)
    }
}

impl Add<ChunkLocalTileCoord> for WorldGlobalTileCoord {
    type Output = WorldGlobalTileCoord;

    fn add(self, rhs: ChunkLocalTileCoord) -> Self::Output {
        self.add_chunk_local_tile_coord(rhs)
    }
}

impl ChunkLocalTileCoord {
    #[inline(always)]
    pub const fn new(x: u8, y: u8) -> Self {
        debug_assert!(x < TileChunk::WIDTH_IN_TILES as u8);
        debug_assert!(y < TileChunk::HEIGHT_IN_TILES as u8);
        Self { x, y }
    }

    #[inline(always)]
    pub const fn to_dense_tile_chunk_index(self) -> usize {
        (self.y as usize) * TileChunk::WIDTH_IN_TILES + (self.x as usize)
    }

    #[inline(always)]
    pub const fn from_dense_tile_chunk_index(index: usize) -> Self {
        debug_assert!(index < TileChunk::TILE_COUNT);
        let x = (index % TileChunk::WIDTH_IN_TILES) as u8;
        let y = (index / TileChunk::WIDTH_IN_TILES) as u8;
        Self { x, y }
    }
}

impl RegionLocalTileCoord {
    #[inline(always)]
    pub const fn new(x: u16, y: u16) -> Self {
        debug_assert!(x < WorldRegion::WIDTH_IN_TILES as u16);
        debug_assert!(y < WorldRegion::HEIGHT_IN_TILES as u16);
        Self { x, y }
    }

    /// Decomposes a [`RegionLocalTileCoord`] into its [`RegionLocalChunkCoord`]
    /// and [`ChunkLocalTileCoord`] within the region.
    #[inline(always)]
    pub const fn decompose(self) -> (RegionLocalChunkCoord, ChunkLocalTileCoord) {
        const CHUNK_WIDTH_IN_TILES: u16 = TileChunk::WIDTH_IN_TILES as u16;
        const CHUNK_HEIGHT_IN_TILES: u16 = TileChunk::HEIGHT_IN_TILES as u16;

        let chunk_x = self.x / CHUNK_WIDTH_IN_TILES;
        let chunk_y = self.y / CHUNK_HEIGHT_IN_TILES;
        let chunk = RegionLocalChunkCoord::new(chunk_x as u8, chunk_y as u8);

        let tile_x = self.x % CHUNK_WIDTH_IN_TILES;
        let tile_y = self.y % CHUNK_HEIGHT_IN_TILES;
        let tile = ChunkLocalTileCoord::new(tile_x as u8, tile_y as u8);

        (chunk, tile)
    }
}

impl RegionLocalChunkCoord {
    #[inline(always)]
    pub const fn new(x: u8, y: u8) -> Self {
        debug_assert!(x < WorldRegion::WIDTH_IN_CHUNKS as u8);
        debug_assert!(y < WorldRegion::HEIGHT_IN_CHUNKS as u8);
        Self { x, y }
    }

    #[inline(always)]
    pub const fn to_dense_region_index(self) -> usize {
        (self.y as usize) * WorldRegion::WIDTH_IN_CHUNKS + (self.x as usize)
    }

    #[inline(always)]
    pub const fn from_dense_region_index(index: usize) -> Self {
        debug_assert!(index < WorldRegion::CHUNK_COUNT);
        let x = (index % WorldRegion::WIDTH_IN_CHUNKS) as u8;
        let y = (index / WorldRegion::WIDTH_IN_CHUNKS) as u8;
        Self { x, y }
    }
}

impl WorldGlobalRegionCoord {
    #[inline(always)]
    pub const fn new(x: i8, y: i8) -> Self {
        Self { x, y }
    }
}

impl WorldGlobalTileCoord {
    #[inline(always)]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Decomposes a [`WorldGlobalTileCoord`] into its fundamental [`WorldGlobalRegionCoord`],
    /// [`RegionLocalChunkCoord`] and [`ChunkLocalTileCoord`].
    ///
    /// Note that this function does not verify if such a coordinate actually exists on the map.
    pub const fn decompose(
        self,
    ) -> (
        WorldGlobalRegionCoord,
        RegionLocalChunkCoord,
        ChunkLocalTileCoord,
    ) {
        // Tile sizes
        const CHUNK_WIDTH_IN_TILES: i32 = TileChunk::WIDTH_IN_TILES as i32;
        const CHUNK_HEIGHT_IN_TILES: i32 = TileChunk::HEIGHT_IN_TILES as i32;
        const REGION_WIDTH_IN_TILES: i32 =
            WorldRegion::WIDTH_IN_CHUNKS as i32 * CHUNK_WIDTH_IN_TILES;
        const REGION_HEIGHT_IN_TILES: i32 =
            WorldRegion::HEIGHT_IN_CHUNKS as i32 * CHUNK_HEIGHT_IN_TILES;

        // Compute the region this tile belongs to
        let region_x = div_floor(self.x, REGION_WIDTH_IN_TILES);
        let region_y = div_floor(self.y, REGION_HEIGHT_IN_TILES);
        let region = WorldGlobalRegionCoord::new(region_x as i8, region_y as i8);

        // Tile offset within region
        let tile_x_in_region = self.x - region_x * REGION_WIDTH_IN_TILES;
        let tile_y_in_region = self.y - region_y * REGION_HEIGHT_IN_TILES;

        // Compute chunk and tile within chunk
        let chunk_x = tile_x_in_region / CHUNK_WIDTH_IN_TILES;
        let chunk_y = tile_y_in_region / CHUNK_HEIGHT_IN_TILES;
        let chunk = RegionLocalChunkCoord::new(chunk_x as u8, chunk_y as u8);

        let tile_x = tile_x_in_region % CHUNK_WIDTH_IN_TILES;
        let tile_y = tile_y_in_region % CHUNK_HEIGHT_IN_TILES;
        let tile = ChunkLocalTileCoord::new(tile_x as u8, tile_y as u8);

        (region, chunk, tile)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_tile_indexing_is_correct() {
        let coord = ChunkLocalTileCoord::new(3, 2);
        let index = coord.to_dense_tile_chunk_index();
        assert_eq!(index, 2 * TileChunk::WIDTH_IN_TILES + 3);
    }

    #[test]
    fn local_chunk_indexing_is_correct() {
        let coord = RegionLocalChunkCoord::new(1, 1);
        let index = coord.to_dense_region_index();
        assert_eq!(index, 1 * WorldRegion::WIDTH_IN_CHUNKS + 1);
    }

    #[test]
    fn tile_index_roundtrip() {
        for y in 0..TileChunk::HEIGHT_IN_TILES {
            for x in 0..TileChunk::WIDTH_IN_TILES {
                let original = ChunkLocalTileCoord::new(x as u8, y as u8);
                let index = original.to_dense_tile_chunk_index();
                let recovered = ChunkLocalTileCoord::from_dense_tile_chunk_index(index);
                assert_eq!(original, recovered);
            }
        }
    }

    #[test]
    fn coordinate_transforms_zero_zero_zero() {
        let world_coord = WorldGlobalRegionCoord::new(0, 0); // A region is e.g. 2x2 chunks
        let chunk_coord = RegionLocalChunkCoord::new(0, 0); // A chunk is e.g. 16x16 tiles
        let tile_coord = ChunkLocalTileCoord::new(0, 0);

        let coord = world_coord + (chunk_coord + tile_coord);
        let coord2 = (world_coord + chunk_coord) + tile_coord;
        assert_eq!(coord, coord2);
        assert_eq!(coord.x, 0);
        assert_eq!(coord.y, 0);
    }

    #[test]
    fn coordinate_transforms_zero_zero_max() {
        let world_coord = WorldGlobalRegionCoord::new(0, 0); // A region is e.g. 2x2 chunks
        let chunk_coord = RegionLocalChunkCoord::new(0, 0); // A chunk is e.g. 16x16 tiles
        let tile_coord = ChunkLocalTileCoord::new(15, 15);

        let coord = world_coord + (chunk_coord + tile_coord);
        let coord2 = (world_coord + chunk_coord) + tile_coord;
        assert_eq!(coord.x, 15); // 0 * WorldRegion::WIDTH_IN_TILES + 0 * TileChunk::WIDTH_IN_TILES + 15
        assert_eq!(coord.y, 15); // 0 * WorldRegion::HEIGHT_IN_TILES + 0 * TileChunk::HEIGHT_IN_TILES + 15
        assert_eq!(coord, coord2);
    }

    #[test]
    fn coordinate_transforms_zero_max_zero() {
        let world_coord = WorldGlobalRegionCoord::new(0, 0); // A region is e.g. 2x2 chunks
        let chunk_coord = RegionLocalChunkCoord::new(1, 1); // A chunk is e.g. 16x16 tiles
        let tile_coord = ChunkLocalTileCoord::new(0, 0);

        let coord = world_coord + (chunk_coord + tile_coord);
        let coord2 = (world_coord + chunk_coord) + tile_coord;
        assert_eq!(coord.x, 16); // 0 * WorldRegion::WIDTH_IN_TILES + 1 * TileChunk::WIDTH_IN_TILES + 0
        assert_eq!(coord.y, 16); // 0 * WorldRegion::HEIGHT_IN_TILES + 1 * TileChunk::HEIGHT_IN_TILES + 0
        assert_eq!(coord, coord2);
    }

    #[test]
    fn coordinate_transforms_max_zero_zero() {
        let world_coord = WorldGlobalRegionCoord::new(1, 1); // A region is e.g. 2x2 chunks
        let chunk_coord = RegionLocalChunkCoord::new(0, 0); // A chunk is e.g. 16x16 tiles
        let tile_coord = ChunkLocalTileCoord::new(0, 0);

        let coord = world_coord + (chunk_coord + tile_coord);
        let coord2 = (world_coord + chunk_coord) + tile_coord;
        assert_eq!(coord.x, 32); // 1 * WorldRegion::WIDTH_IN_TILES + 0 * TileChunk::WIDTH_IN_TILES + 0
        assert_eq!(coord.y, 32); // 1 * WorldRegion::HEIGHT_IN_TILES + 0 * TileChunk::HEIGHT_IN_TILES + 0
        assert_eq!(coord, coord2);
    }

    #[test]
    fn coordinate_transforms_work() {
        // A region is e.g. 2x2 chunks
        let world_coord = WorldGlobalRegionCoord::new(-4, 7);

        // A chunk is e.g. 16x16 tiles
        let chunk_coord = RegionLocalChunkCoord::new(0, 1);

        // Some random tile.
        let tile_coord = ChunkLocalTileCoord::new(3, 2);

        let coord = world_coord + (chunk_coord + tile_coord);
        let coord2 = (world_coord + chunk_coord) + tile_coord;
        assert_eq!(coord, coord2);
        assert_eq!(
            coord.x,
            -4 * WorldRegion::WIDTH_IN_TILES as i32 + 0 * TileChunk::WIDTH_IN_TILES as i32 + 3
        );
        assert_eq!(
            coord.y,
            7 * WorldRegion::HEIGHT_IN_TILES as i32 + 1 * TileChunk::HEIGHT_IN_TILES as i32 + 2
        );
    }

    #[test]
    fn test_decompose_origin_tile() {
        let coord = WorldGlobalTileCoord { x: 0, y: 0 };
        let (region, chunk, tile) = coord.decompose();
        assert_eq!(region, WorldGlobalRegionCoord::new(0, 0));
        assert_eq!(chunk, RegionLocalChunkCoord::new(0, 0));
        assert_eq!(tile, ChunkLocalTileCoord::new(0, 0));
    }

    #[test]
    fn test_decompose_max_tile_in_first_chunk() {
        let coord = WorldGlobalTileCoord {
            x: TileChunk::WIDTH_IN_TILES as i32 - 1,
            y: TileChunk::HEIGHT_IN_TILES as i32 - 1,
        };
        let (region, chunk, tile) = coord.decompose();
        assert_eq!(region, WorldGlobalRegionCoord::new(0, 0));
        assert_eq!(chunk, RegionLocalChunkCoord::new(0, 0));
        assert_eq!(
            tile,
            ChunkLocalTileCoord::new(
                (TileChunk::WIDTH_IN_TILES - 1) as u8,
                (TileChunk::HEIGHT_IN_TILES - 1) as u8
            )
        );
    }

    #[test]
    fn test_decompose_tile_in_second_chunk_x() {
        let coord = WorldGlobalTileCoord {
            x: TileChunk::WIDTH_IN_TILES as i32,
            y: 0,
        };
        let (region, chunk, tile) = coord.decompose();
        assert_eq!(region, WorldGlobalRegionCoord::new(0, 0));
        assert_eq!(chunk, RegionLocalChunkCoord::new(1, 0));
        assert_eq!(tile, ChunkLocalTileCoord::new(0, 0));
    }

    #[test]
    fn test_decompose_tile_at_region_boundary() {
        let coord = WorldGlobalTileCoord {
            x: WorldRegion::WIDTH_IN_CHUNKS as i32 * TileChunk::WIDTH_IN_TILES as i32,
            y: 0,
        };
        let (region, chunk, tile) = coord.decompose();
        assert_eq!(region, WorldGlobalRegionCoord::new(1, 0));
        assert_eq!(chunk, RegionLocalChunkCoord::new(0, 0));
        assert_eq!(tile, ChunkLocalTileCoord::new(0, 0));
    }

    #[test]
    fn test_decompose_negative_coordinates() {
        let coord = WorldGlobalTileCoord { x: -1, y: -1 };
        let (region, chunk, tile) = coord.decompose();

        let region_width = WorldRegion::WIDTH_IN_CHUNKS as i32 * TileChunk::WIDTH_IN_TILES as i32;
        let region_height =
            WorldRegion::HEIGHT_IN_CHUNKS as i32 * TileChunk::HEIGHT_IN_TILES as i32;

        let expected_region_x = div_floor(-1, region_width);
        let expected_region_y = div_floor(-1, region_height);
        assert_eq!(
            region,
            WorldGlobalRegionCoord::new(expected_region_x as i8, expected_region_y as i8)
        );

        // Sanity check
        assert_eq!(expected_region_x, -1);
        assert_eq!(expected_region_y, -1);
    }
}
