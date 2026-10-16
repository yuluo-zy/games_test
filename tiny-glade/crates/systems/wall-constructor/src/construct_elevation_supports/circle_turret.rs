//! 圆形塔楼（construct_elevation_supports/circle_turret.rs）。
//!
//! `construct_turret`（0x14124cc30，1902 行 fresh）：圆形塔楼支撑构造。
use glam::Vec3;

/// B `construct_turret`（0x14124cc30）：塔楼砖层（圆形渐缩）。
pub fn construct_turret(
    center: Vec3,
    radius: f32,
    height: f32,
    out: &mut Vec<[u8; 96]>,
) {
    let layers = (height / 1.2).ceil() as u32;
    for layer in 0..layers {
        let r = radius * (1.0 - layer as f32 * 0.15 / layers as f32).max(0.2);
        let count = ((r * std::f32::consts::TAU) / 1.2).ceil() as usize;
        for i in 0..count.max(1) {
            let angle = i as f32 / count.max(1) as f32 * std::f32::consts::TAU;
            let p = center
                + Vec3::new(angle.cos() * r, layer as f32 * 1.2, angle.sin() * r);
            let mut record = [0u8; 96];
            record[..12].copy_from_slice(&p.to_array().map(f32::to_le_bytes).concat());
            out.push(record);
        }
    }
}
