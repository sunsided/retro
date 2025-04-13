use crate::map::tile_coord::WorldGlobalRegionCoord;
use crate::map::world_region::WorldRegion;
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

    pub fn iter_regions(&self) -> RegionIter {
        self.chunks.iter()
    }

    pub fn iter_regions_mut(&mut self) -> RegionIterMut {
        self.chunks.iter_mut()
    }
}

pub type RegionIter<'a> = std::collections::hash_map::Iter<'a, WorldGlobalRegionCoord, WorldRegion>;
pub type RegionIterMut<'a> =
    std::collections::hash_map::IterMut<'a, WorldGlobalRegionCoord, WorldRegion>;
