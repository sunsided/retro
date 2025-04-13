use bitfield_struct::bitfield;

#[bitfield(u32)]
pub struct Tile {
    #[bits(4)]
    pub dungeon: DungeonType,
    #[bits(4)]
    pub biome: BiomeType,
    #[bits(4)]
    pub floor: FloorType,
    #[bits(4)]
    pub wall: WallType,
    #[bits(1)]
    pub wall_north: bool,
    #[bits(1)]
    pub wall_south: bool,
    #[bits(1)]
    pub wall_east: bool,
    #[bits(1)]
    pub wall_west: bool,
    #[bits(2)]
    pub ceiling: CeilingType,
    #[bits(3)]
    pub door: DoorType,
    #[bits(1)]
    pub map_special: bool,
    #[bits(1)]
    pub trigger: bool,
    #[bits(5)]
    pub reserved: u8,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum WallDirection {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DungeonType {
    Outdoors = 0,
    City = 1,
    Basement = 2,
    Cave = 3,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BiomeType {
    Nature = 0,
    Winter = 1,
    Desert = 2,
    Fire = 3,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FloorType {
    Empty = 0,
    Grass = 1,
    Stone = 2,
    Water = 3,
    Lava = 4,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WallType {
    None = 0,
    Brick = 1,
    Metal = 2,
    Door = 3,
    Secret = 4,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CeilingType {
    None = 0,
    CopyFloor = 1,
    CopyWall = 2,
    Special = 3,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DoorType {
    None,
    North,
    East,
    South,
    West,
    Up,
    Down,
}

impl Tile {
    pub fn has_ceiling(&self) -> bool {
        self.ceiling() != CeilingType::None
    }

    pub fn has_ceiling_variant(&self) -> bool {
        self.ceiling() != CeilingType::Special
    }

    pub fn each_wall<F: FnMut(WallDirection)>(&self, mut f: F) {
        if self.wall_north() {
            f(WallDirection::North);
        }
        if self.wall_south() {
            f(WallDirection::South);
        }
        if self.wall_east() {
            f(WallDirection::East);
        }
        if self.wall_west() {
            f(WallDirection::West);
        }
    }
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

impl WallDirection {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::North,
            1 => Self::East,
            2 => Self::South,
            _ => Self::West,
        }
    }
}

impl DungeonType {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::Outdoors,
            1 => Self::City,
            2 => Self::Basement,
            _ => Self::Cave,
        }
    }
}

impl BiomeType {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::Nature,
            1 => Self::Winter,
            2 => Self::Desert,
            _ => Self::Fire,
        }
    }
}

impl FloorType {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::Empty,
            1 => Self::Grass,
            2 => Self::Stone,
            3 => Self::Water,
            _ => Self::Lava,
        }
    }
}

impl WallType {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Brick,
            2 => Self::Metal,
            3 => Self::Door,
            _ => Self::Secret,
        }
    }
}

impl CeilingType {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::None,
            1 => Self::CopyFloor,
            2 => Self::CopyWall,
            _ => Self::Special,
        }
    }
}

impl DoorType {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::None,
            1 => Self::North,
            2 => Self::East,
            3 => Self::South,
            4 => Self::West,
            5 => Self::Up,
            _ => Self::Down,
        }
    }
}
