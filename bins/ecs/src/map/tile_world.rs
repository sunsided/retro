use crate::map::tile_coord::{
    RegionLocalChunkCoord, WorldGlobalChunkCoord, WorldGlobalRegionCoord,
};
use crate::map::world_region::{ChunkWithCoordIter, ChunkWithCoordIterMut, WorldRegion};
use ahash::HashMapExt;
use fxhash::FxHashMap;

pub struct TileWorld {
    pub chunks: FxHashMap<WorldGlobalRegionCoord, WorldRegion>,
}

pub struct ChunkRect {
    pub min: WorldGlobalChunkCoord,
    pub max: WorldGlobalChunkCoord, // inclusive
}

impl ChunkRect {
    pub fn union(self, other: ChunkRect) -> ChunkRect {
        ChunkRect {
            min: WorldGlobalChunkCoord {
                x: self.min.x.min(other.min.x),
                y: self.min.y.min(other.min.y),
            },
            max: WorldGlobalChunkCoord {
                x: self.max.x.max(other.max.x),
                y: self.max.y.max(other.max.y),
            },
        }
    }
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
        let mut bounds: Option<ChunkRect> = None;

        for (region_coord, _region) in self.iter_regions_with_coords() {
            let base_chunk_x = region_coord.x as i32 * WorldRegion::WIDTH_IN_CHUNKS as i32;
            let base_chunk_y = region_coord.y as i32 * WorldRegion::HEIGHT_IN_CHUNKS as i32;

            let region_chunk_rect = ChunkRect {
                min: WorldGlobalChunkCoord {
                    x: base_chunk_x,
                    y: base_chunk_y,
                },
                max: WorldGlobalChunkCoord {
                    x: base_chunk_x + WorldRegion::WIDTH_IN_CHUNKS as i32 - 1,
                    y: base_chunk_y + WorldRegion::HEIGHT_IN_CHUNKS as i32 - 1,
                },
            };

            bounds = Some(if let Some(current) = bounds {
                current.union(region_chunk_rect)
            } else {
                region_chunk_rect
            });
        }

        bounds
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
