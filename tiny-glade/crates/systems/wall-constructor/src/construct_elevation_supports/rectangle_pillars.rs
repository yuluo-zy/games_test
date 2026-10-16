//! 矩形支柱（construct_elevation_supports/rectangle_pillars.rs）。
//!
//! `construct_rectangle_pillars`（0x14124fcb0）+ `find_stone_column_start_y`
//! （0x141250720）。支柱常量族（re-analyst §5.2：0.2/1.2/0.01/0.5/±0.2/2.1/
//! ±0.25/0.56/1.26）。
use glam::Vec3;

/// B `find_stone_column_start_y`（0x141250720）：石柱起始高度（地形采样）。
pub fn find_stone_column_start_y(
    _terrain_height: f32,
    _ground_y: f32,
    _base: f32,
) -> f32 {
    _ground_y
}

/// B `construct_rectangle_pillars`（0x14124fcb0）：矩形墙支柱。
pub fn construct_rectangle_pillars(
    positions: &[Vec3],
    height: f32,
    out: &mut Vec<[u8; 96]>,
) {
    const BRICK_LEN: f32 = 1.2;
    const ROW_GAP: f32 = 0.56;
    let rows = ((height / ROW_GAP).ceil() as u32).min(4);
    for &position in positions {
        for row in 0..rows {
            let p = position + Vec3::new(0.0, row as f32 * BRICK_LEN * 0.5, 0.0);
            let mut record = [0u8; 96];
            record[..12].copy_from_slice(&p.to_array().map(f32::to_le_bytes).concat());
            out.push(record);
        }
    }
}
