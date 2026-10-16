//! 墙裙边（utils/wall_skirts.rs）。
//!
//! `add_skirts`（0x1412221f0，construct_walls 实调；G05 caller 链）。
use glam::Vec3;

/// B `add_skirts`（0x1412221f0）：墙裙边砖（96B 记录）。
/// 结构：沿墙底线每砖间隔放裙边砖（y = 底部 −0.1；数值登记）。
pub fn add_skirts_new(points: &[Vec3], out: &mut Vec<[u8; 96]>) {
    if points.len() < 2 {
        return;
    }
    let count = ((points.len() as f32 * 0.5).ceil() as usize).max(1);
    for i in 0..count {
        let t = i as f32 / count as f32;
        let span = (points.len() - 1) as f32;
        let u = t * span;
        let idx = (u as usize).min(points.len() - 2);
        let frac = u - idx as f32;
        let p = points[idx] + (points[idx + 1] - points[idx]) * frac;
        let mut record = [0u8; 96];
        record[..12].copy_from_slice(&p.to_array().map(f32::to_le_bytes).concat());
        out.push(record);
    }
}
