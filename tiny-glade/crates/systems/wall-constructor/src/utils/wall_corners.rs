//! 墙角砖（utils/wall_corners.rs）。
//!
//! `add_wall_corners`（0x141215540，construct_walls 实调；G05 caller 链）。
use glam::Vec3;

/// B `add_wall_corners`（0x141215540）：墙角砖（96B 记录）。
/// 结构：曲线首尾端点各放一块转角砖（方向 = 首段/尾段切线；数值登记）。
pub fn add_wall_corners_new(points: &[Vec3], out: &mut Vec<[u8; 96]>) {
    if points.len() < 2 {
        return;
    }
    for corner in [points[0], points[points.len() - 1]] {
        let mut record = [0u8; 96];
        record[..12].copy_from_slice(&corner.to_array().map(f32::to_le_bytes).concat());
        out.push(record);
    }
}
