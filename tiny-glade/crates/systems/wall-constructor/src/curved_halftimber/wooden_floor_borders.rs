//! 木地板边条（curved_halftimber/wooden_floor_borders.rs）。
//!
//! `wooden_floor_borders`（0x141244310）：半木构木地板边界条生成。
use glam::Vec3;

/// B 0x141244310：沿地板轮廓生成木边条（64B 记录族）。
pub fn wooden_floor_borders(
    points: &[Vec3],
    _floor_y: f32,
    out: &mut Vec<[u8; 64]>,
) {
    for w in points.windows(2) {
        let mut record = [0u8; 64];
        let mid = (w[0] + w[1]) * 0.5;
        record[0..12].copy_from_slice(&mid.to_array().map(f32::to_le_bytes).concat());
        out.push(record);
    }
}
