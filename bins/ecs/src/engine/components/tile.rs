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

    pub fn has_wall_north(&self) -> bool {
        self.flags().contains(TileFlags::WALL_N)
    }

    pub fn has_wall_south(&self) -> bool {
        self.flags().contains(TileFlags::WALL_S)
    }

    pub fn has_wall_east(&self) -> bool {
        self.flags().contains(TileFlags::WALL_E)
    }

    pub fn has_wall_west(&self) -> bool {
        self.flags().contains(TileFlags::WALL_W)
    }

    pub fn each_wall<F: FnMut(WallDirection)>(&self, mut f: F) {
        if self.has_wall_north() {
            f(WallDirection::North);
        }
        if self.has_wall_south() {
            f(WallDirection::South);
        }
        if self.has_wall_east() {
            f(WallDirection::East);
        }
        if self.has_wall_west() {
            f(WallDirection::West);
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum WallDirection {
    North,
    East,
    South,
    West,
}

impl WallDirection {
    pub fn rotation_y_radians(&self) -> f32 {
        use std::f32::consts::FRAC_PI_2;
        match self {
            WallDirection::North => std::f32::consts::PI,
            WallDirection::East => FRAC_PI_2,
            WallDirection::South => 0.0,
            WallDirection::West => FRAC_PI_2 * 3.0,
        }
    }

    pub fn offset(&self) -> (i32, i32) {
        match self {
            WallDirection::North => (0, -1),
            WallDirection::East => (1, 0),
            WallDirection::South => (0, 1),
            WallDirection::West => (-1, 0),
        }
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

        const WALL_N    = 0b0001_0000;
        const WALL_E    = 0b0010_0000;
        const WALL_S    = 0b0100_0000;
        const WALL_W    = 0b1000_0000;
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
