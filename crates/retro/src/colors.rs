//! Provides color utility functions.

pub mod palettes;

pub const fn hex_to_u32(hex: &str) -> u32 {
    match u32::from_str_radix(hex, 16) {
        Ok(value) => value,
        Err(_) => 0,
    }
}
