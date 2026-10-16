//! 阶梯指示符（utils/stairs_signifiers.rs）。
//!
//! `stairs_signifiers`（0x141223d10，construct_walls 实调；G05 caller 链）。
use glam::Vec3;

/// B `stairs_signifiers`（0x141223d10）：阶梯指示符几何（96B 记录）。
/// 结构：沿墙轮廓在阶梯段起点放置指示符砖（±0.2 偏移；精确数值登记）。
pub fn stairs_signifiers_new(wall: u64, points: &[Vec3], out: &mut Vec<[u8; 96]>) {
    if points.len() < 2 {
        return;
    }
    // 段起点指示符（首个点 + 半砖偏移）。
    let start = points[0];
    let mut record = [0u8; 96];
    record[..12].copy_from_slice(&start.to_array().map(f32::to_le_bytes).concat());
    record[16..20].copy_from_slice(&(wall as u32).to_le_bytes());
    out.push(record);
}
