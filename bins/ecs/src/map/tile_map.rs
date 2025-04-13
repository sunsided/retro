use crate::engine::components::TileData;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TileCoord2D {
    pub x: u16,
    pub y: u16,
}

pub type TileCoordX = u16;
pub type TileCoordY = u16;
pub type TileCoordZ = u8;

#[derive(Debug, Clone)]
pub struct SparseTileMap {
    pub base: HashMap<TileCoord2D, TileData>,
    pub stacks: HashMap<TileCoord2D, HashMap<TileCoordZ, TileData>>, // (x, y) → z → tile
}

impl SparseTileMap {
    pub fn new() -> Self {
        Self {
            base: HashMap::new(),
            stacks: HashMap::new(),
        }
    }

    pub fn set(&mut self, x: TileCoordX, y: TileCoordY, z: TileCoordZ, tile: TileData) {
        if z == 0 {
            self.base.insert(TileCoord2D { x, y }, tile);
        } else {
            self.stacks
                .entry(TileCoord2D { x, y })
                .or_default()
                .insert(z, tile);
        }
    }

    pub fn get(&self, x: u16, y: u16, z: u8) -> Option<TileData> {
        if z == 0 {
            self.base.get(&TileCoord2D { x, y }).copied()
        } else {
            self.stacks
                .get(&TileCoord2D { x, y })
                .and_then(|map| map.get(&z))
                .copied()
        }
    }

    pub fn all_instances(&self) -> impl Iterator<Item = (u16, u16, u8, TileData)> + '_ {
        let base_iter = self
            .base
            .iter()
            .map(|(&TileCoord2D { x, y }, &tile)| (x, y, 0, tile));
        let stack_iter = self
            .stacks
            .iter()
            .flat_map(|(&TileCoord2D { x, y }, levels)| {
                levels.iter().map(move |(&z, &tile)| (x, y, z, tile))
            });
        base_iter.chain(stack_iter)
    }
}
