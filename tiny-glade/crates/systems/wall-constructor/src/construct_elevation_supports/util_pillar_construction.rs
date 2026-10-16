//! 支柱构造工具（util_pillar_construction.rs / util_cross_brick_pillar.rs）。
//!
//! `construct_wooden_pillar_w_stone_base`（0x141fe2150/0x141fe3240）：
//! 木柱 + 石基（0.2/1.2/0.01/0.5/±0.2/2.1/±0.25/0.56/1.26 常量族）。
//! `construct_crossbrick_pillar`（0x142418f50，15.9KB fresh）：十字砖柱
//! （closure env<construct_wooden_pillar_w_stone_base>）。
use glam::Vec3;

/// B `construct_wooden_pillar_w_stone_base`（0x141fe2150）。
pub fn construct_wooden_pillar_w_stone_base(
    base: Vec3,
    height: f32,
    out: &mut Vec<[u8; 96]>,
) {
    let stone_rows = (height / 0.56).ceil() as u32;
    for row in 0..stone_rows {
        let p = base + Vec3::new(0.0, row as f32 * 0.56, 0.0);
        let mut record = [0u8; 96];
        record[..12].copy_from_slice(&p.to_array().map(f32::to_le_bytes).concat());
        out.push(record);
    }
}

/// B `construct_brick_columns`（0x141fe3240）。
pub fn construct_brick_columns(
    positions: &[Vec3],
    height: f32,
    out: &mut Vec<[u8; 96]>,
) {
    for &position in positions {
        construct_wooden_pillar_w_stone_base(position, height, out);
    }
}

/// B `construct_crossbrick_pillar`（0x142418f50）：十字砖柱（每层 ±0.2
/// 十字偏移）。
pub fn construct_crossbrick_pillar(
    base: Vec3,
    height: f32,
    out: &mut Vec<[u8; 96]>,
) {
    const CROSS: f32 = 0.2;
    let rows = (height / 0.56).ceil() as u32;
    for row in 0..rows {
        for offset in [Vec3::new(CROSS, 0.0, 0.0), Vec3::new(-CROSS, 0.0, 0.0), Vec3::new(0.0, 0.0, CROSS), Vec3::new(0.0, 0.0, -CROSS)] {
            let p = base + offset + Vec3::new(0.0, row as f32 * 0.56, 0.0);
            let mut record = [0u8; 96];
            record[..12].copy_from_slice(&p.to_array().map(f32::to_le_bytes).concat());
            out.push(record);
        }
    }
}
