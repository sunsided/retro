use crate::map::navigator::NavigatorStep;
use crate::map::tile_coord::WorldGlobalTileCoord;

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct GridNavigator2D {
    origin: WorldGlobalTileCoord,
    step: NavigatorStep,
}

impl GridNavigator2D {
    /// Creates a new navigator centered on a world tile coordinate.
    /// The internal step starts at `Local`.
    pub const fn new(origin: WorldGlobalTileCoord) -> Self {
        let (_, _, tile) = origin.decompose();
        Self {
            origin,
            step: NavigatorStep::Local { tile },
        }
    }

    /// Returns the world-global origin of the navigator.
    #[inline]
    pub const fn origin(&self) -> WorldGlobalTileCoord {
        self.origin
    }

    /// Returns the current navigator step (relative to the origin).
    #[inline]
    pub const fn step(&self) -> NavigatorStep {
        self.step
    }

    /// Re-centers the navigator to the current tile.
    /// This sets `origin += current step` and resets step to `Local`.
    pub const fn recenter(&mut self) {
        let world = self.current_coord();
        let (_, _, tile) = world.decompose();
        self.origin = world;
        self.step = NavigatorStep::Local { tile };
    }

    /// Re-centers the navigator to the current tile.
    /// This sets `origin += current step` and resets step to `Local`.
    pub const fn recentered(mut self) -> Self {
        self.recenter();
        self
    }

    /// Computes the current absolute world-global tile coordinate,
    /// combining the origin with the current relative step.
    pub const fn current_coord(&self) -> WorldGlobalTileCoord {
        match self.step {
            NavigatorStep::Local { tile } => {
                let (region, chunk, _) = self.origin.decompose();
                region.add(chunk, tile)
            }
            NavigatorStep::NeighborChunk { chunk, tile } => {
                let (region, _, _) = self.origin.decompose();
                region.add(chunk, tile)
            }
            NavigatorStep::NeighborRegion {
                region,
                chunk,
                tile,
            } => region.add(chunk, tile),
        }
    }

    /// Moves from the origin by a delta in world tile space.
    /// Only recomputes region/chunk decomposition if necessary.
    pub const fn move_by(&mut self, dx: i32, dy: i32) -> NavigatorStep {
        let base = self.current_coord(); // ← FIXED: use current position
        let next = WorldGlobalTileCoord::new(base.x + dx, base.y + dy);

        let (base_region, base_chunk, _) = base.decompose();
        let (next_region, next_chunk, next_tile) = next.decompose();

        self.step = if base_region.const_eq(next_region) {
            if base_chunk.const_eq(next_chunk) {
                NavigatorStep::Local { tile: next_tile }
            } else {
                NavigatorStep::NeighborChunk {
                    chunk: next_chunk,
                    tile: next_tile,
                }
            }
        } else {
            NavigatorStep::NeighborRegion {
                region: next_region,
                chunk: next_chunk,
                tile: next_tile,
            }
        };

        self.step
    }

    #[inline(always)]
    pub const fn step_north(&mut self) -> NavigatorStep {
        self.move_by(0, -1)
    }

    #[inline(always)]
    pub const fn step_south(&mut self) -> NavigatorStep {
        self.move_by(0, 1)
    }

    #[inline(always)]
    pub const fn step_east(&mut self) -> NavigatorStep {
        self.move_by(1, 0)
    }

    #[inline(always)]
    pub const fn step_west(&mut self) -> NavigatorStep {
        self.move_by(-1, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map::tile_chunk::TileChunk;
    use crate::map::world_region::WorldRegion;

    #[test]
    fn step_within_same_chunk_stays_local() {
        let start = WorldGlobalTileCoord::new(10, 10);
        let mut nav = GridNavigator2D::new(start);

        // Perform step
        let step_west = nav.step_west(); // should still be within same chunk
        assert!(matches!(step_west, NavigatorStep::Local { .. }));

        // Step back
        let step_east = nav.step_east();
        assert!(matches!(step_east, NavigatorStep::Local { .. }));
        assert_eq!(nav.current_coord(), start);
    }

    #[test]
    fn step_across_chunk_boundary_triggers_neighbor_chunk() {
        let x = TileChunk::WIDTH_IN_TILES as i32 - 1; // 15
        let y = 5;
        let mut nav = GridNavigator2D::new(WorldGlobalTileCoord::new(x, y));
        let step = nav.step_east(); // crosses into chunk (1, 0)
        assert!(matches!(step, NavigatorStep::NeighborChunk { .. }));
    }

    #[test]
    fn step_across_region_boundary_triggers_neighbor_region() {
        let x = WorldRegion::WIDTH_IN_TILES as i32 - 1; // e.g. 31
        let y = 5;
        let mut nav = GridNavigator2D::new(WorldGlobalTileCoord::new(x, y));
        let step = nav.step_east(); // crosses into region (1, 0)
        assert!(matches!(step, NavigatorStep::NeighborRegion { .. }));
    }

    #[test]
    fn recenter_resets_step_to_local() {
        let x = TileChunk::WIDTH_IN_TILES as i32 - 1;
        let y = TileChunk::HEIGHT_IN_TILES as i32 - 1;
        let mut nav = GridNavigator2D::new(WorldGlobalTileCoord::new(x, y));
        let step = nav.step_east();
        assert!(
            !matches!(step, NavigatorStep::Local { .. }),
            "Expected to leave the chunk"
        );

        nav.recenter(); // updates origin to new coord
        assert!(
            matches!(nav.step(), NavigatorStep::Local { .. }),
            "After recenter, step should be local"
        );
    }
}
