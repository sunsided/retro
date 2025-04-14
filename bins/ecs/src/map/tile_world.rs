use crate::map::tile_coord::{
    RegionLocalChunkCoord, WorldGlobalChunkCoord, WorldGlobalRegionCoord,
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

    pub fn global_chunk_bounds(&self) -> Option<(WorldGlobalChunkCoord, WorldGlobalChunkCoord)> {
        let mut min_chunk_x = i32::MAX;
        let mut max_chunk_x = i32::MIN;
        let mut min_chunk_y = i32::MAX;
        let mut max_chunk_y = i32::MIN;

        for (region_coord, region) in self.iter_regions_with_coords() {
            for (chunk_coord, _) in region.iter_with_coords() {
                let chunk_pos_x = region_coord.x as i32 * WorldRegion::WIDTH_IN_CHUNKS as i32
                    + chunk_coord.x as i32;
                let chunk_pos_y = region_coord.y as i32 * WorldRegion::HEIGHT_IN_CHUNKS as i32
                    + chunk_coord.y as i32;

                min_chunk_x = min_chunk_x.min(chunk_pos_x);
                max_chunk_x = max_chunk_x.max(chunk_pos_x);
                min_chunk_y = min_chunk_y.min(chunk_pos_y);
                max_chunk_y = max_chunk_y.max(chunk_pos_y);
            }
        }

        if min_chunk_x <= max_chunk_x && min_chunk_y <= max_chunk_y {
            Some((
                WorldGlobalChunkCoord {
                    x: min_chunk_x,
                    y: min_chunk_y,
                },
                WorldGlobalChunkCoord {
                    x: max_chunk_x,
                    y: max_chunk_y,
                },
            ))
        } else {
            None
        }
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
