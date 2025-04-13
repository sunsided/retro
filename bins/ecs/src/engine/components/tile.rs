use crate::map::tile::Tile;
use bitfield_struct::bitfield;
use std::ops::Deref;

#[derive(Debug, Copy, Clone)]
pub struct TileData {
    pub tile: Tile,
}

impl Deref for TileData {
    type Target = Tile;

    fn deref(&self) -> &Self::Target {
        &self.tile
    }
}

impl From<Tile> for TileData {
    fn from(tile: Tile) -> Self {
        Self { tile }
    }
}

impl From<TileData> for Tile {
    fn from(tile_data: TileData) -> Self {
        tile_data.tile
    }
}
