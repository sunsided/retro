use crate::LUT_SIZE;
use std::sync::LazyLock;

pub static SIN_LUT: LazyLock<[u8; LUT_SIZE]> = LazyLock::new(make_sin_lut);

fn make_sin_lut() -> [u8; LUT_SIZE] {
    let mut lut = [0u8; LUT_SIZE];
    for (i, entry) in lut.iter_mut().enumerate() {
        let angle = i as f32 * std::f32::consts::TAU / LUT_SIZE as f32;
        *entry = ((angle.sin() * 127.5) + 127.5) as u8;
    }
    lut
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_sin_lut() {
        const LUT_SIZE: usize = 256;
        let lut = make_sin_lut();
        assert_eq!(lut.len(), LUT_SIZE, "LUT size is incorrect");

        for (i, &entry) in lut.iter().enumerate() {
            let angle = i as f32 * std::f32::consts::TAU / LUT_SIZE as f32;
            let expected_value = ((f32::sin(angle) * 127.5) + 127.5) as u8;
            assert_eq!(entry, expected_value, "LUT value mismatch at index {i}");
        }
    }
}
