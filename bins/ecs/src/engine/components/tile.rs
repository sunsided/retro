#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct TileData(pub u32);

impl TileData {
    pub fn floor_layer(&self) -> u8 {
        (self.0 & 0xFF) as u8
    }

    pub fn wall_layer(&self) -> u8 {
        ((self.0 >> 8) & 0xFF) as u8
    }

    pub fn top_layer(&self) -> u8 {
        ((self.0 >> 16) & 0xFF) as u8
    }

    pub fn flags(&self) -> TileFlags {
        TileFlags::from_bits_truncate((self.0 >> 24) as u8)
    }

    pub fn middle_layer(&self) -> u32 {
        self.wall_layer() as u32
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum FloorType {
    Empty = 0,
    Grass = 1,
    Stone = 2,
    Water = 3,
    Lava = 4,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum WallType {
    None = 0,
    Brick = 1,
    Metal = 2,
    Door = 3,
    Secret = 4,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum TopType {
    None = 0,
    Tower = 1,
    Tree = 2,
    Crystal = 3,
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct TileFlags: u8 {
        const NONE      = 0b0000_0000;
        const DOOR      = 0b0000_0001;
        const TRIGGER   = 0b0000_0010;
        const HIDDEN    = 0b0000_0100;
        const RESERVED  = 0b0000_1000;
    }
}

#[macro_export]
macro_rules! tile {
    (
        floor: $floor:expr,
        wall: $wall:expr,
        top: $top:expr,
        flags: $flags:expr $(,)?
    ) => {
        TileData(
            (($flags.bits() as u32) << 24) |
            (($top as u32) << 16) |
            (($wall as u32) << 8) |
            ($floor as u32)
        )
    };

    (
        floor: $floor:expr,
        wall: $wall:expr,
        top: $top:expr $(,)?
    ) => {
        tile!(floor: $floor, wall: $wall, top: $top, flags: TileFlags::NONE)
    };

    (
        floor: $floor:expr,
        wall: $wall:expr $(,)?
    ) => {
        tile!(floor: $floor, wall: $wall, top: TopType::None, flags: TileFlags::NONE)
    };
}

impl std::fmt::Display for TileData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Tile(floor: {}, wall: {}, top: {}, flags: {:?})",
            self.floor_layer(),
            self.wall_layer(),
            self.top_layer(),
            self.flags(),
        )
    }
}
