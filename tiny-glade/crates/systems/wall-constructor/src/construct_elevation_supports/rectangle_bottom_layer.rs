//! 矩形底部砖层（construct_elevation_supports/rectangle_bottom_layer.rs）。
//!
//! `construct_rect_bottom_layer`（0x14124eb80）；`add_bottom_brick`
//! （0x141fe1de0）与圆形版共用同一体。
use glam::Vec3;

/// B `construct_rect_bottom_layer`（0x14124eb80）：矩形墙底砖层。
pub fn construct_rect_bottom_layer(
    min: Vec3,
    max: Vec3,
    out: &mut Vec<[u8; 96]>,
) {
    let width = (max.x - min.x).abs();
    let depth = (max.z - min.z).abs();
    let count_x = ((width / 1.2).ceil() as usize).max(1);
    let count_z = ((depth / 1.2).ceil() as usize).max(1);
    for ix in 0..count_x {
        for iz in 0..count_z {
            let p = Vec3::new(
                min.x + (ix as f32 + 0.5) * width / count_x as f32,
                min.y,
                min.z + (iz as f32 + 0.5) * depth / count_z as f32,
            );
            let mut record = [0u8; 96];
            record[..12].copy_from_slice(&p.to_array().map(f32::to_le_bytes).concat());
            out.push(record);
        }
    }
}
