use crate::map::tile_coord::{
    ChunkRect, RegionLocalChunkCoord, WorldGlobalChunkCoord, WorldGlobalRegionCoord,
};
use crate::map::world_region::{ChunkWithCoordIter, ChunkWithCoordIterMut, WorldRegion};
use ahash::HashMapExt;
use fxhash::FxHashMap;

pub struct TileWorld {
    pub chunks: FxHashMap<WorldGlobalRegionCoord, WorldRegion>,
}

impl TileWorld {
    pub fn new() -> Self {
        Self {
            chunks: FxHashMap::default(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            chunks: FxHashMap::with_capacity(capacity),
        }
    }

    pub fn insert(
        &mut self,
        coord: WorldGlobalRegionCoord,
        region: WorldRegion,
    ) -> Option<WorldRegion> {
        self.chunks.insert(coord, region)
    }

    pub fn global_chunk_bounds(&self) -> Option<ChunkRect> {
        self.iter_regions_with_coords()
            .map(|(coord, _)| coord.chunk_extent())
            .reduce(|a, b| a.union(b))
    }

    pub fn iter_regions(&self) -> RegionIter {
        self.chunks.values()
    }

    pub fn iter_regions_mut(&mut self) -> RegionIterMut {
        self.chunks.values_mut()
    }

    #[inline]
    pub fn iter_regions_with_coords(&self) -> RegionWithCoordIter<'_> {
        self.chunks.iter()
    }

    #[inline]
    pub fn iter_regions_with_coords_mut(&mut self) -> RegionWithCoordIterMut<'_> {
        self.chunks.iter_mut()
    }
}

pub type RegionIter<'a> =
    std::collections::hash_map::Values<'a, WorldGlobalRegionCoord, WorldRegion>;
pub type RegionIterMut<'a> =
    std::collections::hash_map::ValuesMut<'a, WorldGlobalRegionCoord, WorldRegion>;

pub type RegionWithCoordIter<'a> =
    std::collections::hash_map::Iter<'a, WorldGlobalRegionCoord, WorldRegion>;
pub type RegionWithCoordIterMut<'a> =
    std::collections::hash_map::IterMut<'a, WorldGlobalRegionCoord, WorldRegion>;
