//! 圆形底部砖层（construct_elevation_supports/circle_bottom_layer.rs）。
//!
//! `construct_circle_bottom_layer`（0x14124b2c0）+ `add_bottom_brick`
//! （0x141fe1de0，11 参 {6 f32 + 5 ptr/int}；circle/rect 两闭包共用同一体）。
use glam::Vec3;

/// B `add_bottom_brick`（0x141fe1de0）：底部砖发射（共用体）。
pub fn add_bottom_brick(
    position: Vec3,
    _rotation: f32,
    _width: f32,
    _depth: f32,
    out: &mut Vec<[u8; 96]>,
) {
    let mut record = [0u8; 96];
    record[..12].copy_from_slice(&position.to_array().map(f32::to_le_bytes).concat());
    out.push(record);
}

/// B `construct_circle_bottom_layer`（0x14124b2c0）。
pub fn construct_circle_bottom_layer(
    center: Vec3,
    radius: f32,
    count: usize,
    out: &mut Vec<[u8; 96]>,
) {
    for i in 0..count {
        let angle = i as f32 / count as f32 * std::f32::consts::TAU;
        let p = center + Vec3::new(angle.cos() * radius, 0.0, angle.sin() * radius);
        add_bottom_brick(p, angle, 1.0, 1.0, out);
    }
}
