mod grid_2d;

#[allow(unused_imports)]
pub use self::grid_2d::GridNavigator2D;
use crate::map::tile_coord::{
    ChunkLocalTileCoord, RegionLocalChunkCoord, RegionLocalTileCoord, WorldGlobalRegionCoord,
    WorldGlobalTileCoord,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavigatorStep {
    /// Tile is in the same chunk.
    Local { tile: ChunkLocalTileCoord },
    /// Tile is in a different chunk of the same region.
    NeighborChunk {
        chunk: RegionLocalChunkCoord,
        tile: ChunkLocalTileCoord,
    },
    /// Tile is in a different region of the world.
    NeighborRegion {
        region: WorldGlobalRegionCoord,
        chunk: RegionLocalChunkCoord,
        tile: ChunkLocalTileCoord,
    },
}

impl NavigatorStep {
    /// Rebuilds a step from a chunk coordinate.
    pub const fn from_chunk(coord: ChunkLocalTileCoord) -> Self {
        NavigatorStep::Local { tile: coord }
    }

    /// Rebuilds a step from a region coordinate.
    pub const fn from_region(coord: RegionLocalTileCoord) -> Self {
        let (chunk, tile) = coord.decompose();
        NavigatorStep::NeighborChunk { chunk, tile }
    }

    /// Rebuilds a step from a world coordinate.
    pub const fn from_world(coord: WorldGlobalTileCoord) -> Self {
        let (region, chunk, tile) = coord.decompose();
        NavigatorStep::NeighborRegion {
            region,
            chunk,
            tile,
        }
    }

    /// Resets the step into a chunk-local step.
    #[allow(dead_code)]
    pub fn to_local(self) -> Self {
        NavigatorStep::Local {
            tile: match self {
                NavigatorStep::Local { tile } => tile,
                NavigatorStep::NeighborChunk { tile, .. } => tile,
                NavigatorStep::NeighborRegion { tile, .. } => tile,
            },
        }
    }
}

impl From<ChunkLocalTileCoord> for NavigatorStep {
    #[inline]
    fn from(tile: ChunkLocalTileCoord) -> Self {
        Self::from_chunk(tile)
    }
}

impl From<RegionLocalTileCoord> for NavigatorStep {
    #[inline]
    fn from(tile: RegionLocalTileCoord) -> Self {
        Self::from_region(tile)
    }
}

impl From<WorldGlobalTileCoord> for NavigatorStep {
    #[inline]
    fn from(tile: WorldGlobalTileCoord) -> Self {
        Self::from_world(tile)
    }
}
