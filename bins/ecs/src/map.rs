pub mod navigator;
pub mod tile;
pub mod tile_chunk;
pub mod tile_coord;
pub mod tile_instance;
pub mod tile_world;
pub mod world_builder;
pub mod world_region;

/// Integer floor division that works for negative numbers like Rust’s `div_euclid` but returns floor.
#[inline(always)]
const fn div_floor(a: i32, b: i32) -> i32 {
    if a >= 0 {
        a / b
    } else {
        (a - b + 1) / b
    }
}
