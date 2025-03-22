use crate::LUT_SIZE;

pub fn make_sin_lut() -> [u8; LUT_SIZE] {
    let mut lut = [0u8; LUT_SIZE];
    for i in 0..LUT_SIZE {
        let angle = i as f32 * std::f32::consts::TAU / LUT_SIZE as f32;
        lut[i] = ((f32::sin(angle) * 127.5) + 127.5) as u8;
    }
    lut
}
