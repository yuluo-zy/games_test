//! 矩形大支架（rectangle_large_brackets.rs）。
//!
//! `construct_rectangle_large_brackets`（0x141253fc0）。
use glam::Vec3;

/// B `construct_rectangle_large_brackets`（0x141253fc0）：大支架（角部
/// 斜撑 + 横梁；64B Beam 族）。
pub fn construct_rectangle_large_brackets(
    positions: &[Vec3],
    out: &mut Vec<[u8; 64]>,
) {
    for w in positions.windows(2) {
        let mut record = [0u8; 64];
        let mid = (w[0] + w[1]) * 0.5;
        record[..12].copy_from_slice(&mid.to_array().map(f32::to_le_bytes).concat());
        out.push(record);
    }
}
