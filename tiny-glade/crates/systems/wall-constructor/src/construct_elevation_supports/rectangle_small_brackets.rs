//! 矩形小/大支架（rectangle_small_brackets.rs / rectangle_large_brackets.rs）。
//!
//! `construct_rectangle_small_brackets`（0x141250da0）/
//! `construct_rectangle_large_brackets`（0x141253fc0）。
use glam::Vec3;

/// B `construct_rectangle_small_brackets`（0x141250da0）。
pub fn construct_rectangle_small_brackets(
    positions: &[Vec3],
    out: &mut Vec<[u8; 96]>,
) {
    for &position in positions {
        let mut record = [0u8; 96];
        record[..12].copy_from_slice(&position.to_array().map(f32::to_le_bytes).concat());
        out.push(record);
    }
}
