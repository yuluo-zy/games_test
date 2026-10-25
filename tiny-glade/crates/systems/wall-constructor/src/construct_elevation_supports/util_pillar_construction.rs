//! 支柱构造工具（util_pillar_construction.rs / util_cross_brick_pillar.rs）。
//!
//! `construct_wooden_pillar_w_stone_base`（0x141fe2150/0x141fe3240）：
//! 木柱 + 石基（0.2/1.2/0.01/0.5/±0.2/2.1/±0.25/0.56/1.26 常量族）。
//! `construct_crossbrick_pillar`（0x142418f50，15.9KB fresh）：十字砖柱
//! （closure env<construct_wooden_pillar_w_stone_base>）。
//!
//! 2026-10-17 复核：旧实现（0.56 等分 + ±0.2 十字）为**近似体**，已移除。
//! 真实逻辑（fresh 已取）含：高度钳 1.1、`len>1.25` 石基、0.3 斜撑、
//! 0x18B 记录二路；`construct_brick_columns` 走 FrozenMap(0x3a)→writer→
//! 0x40B 表；`construct_crossbrick_pillar` 的 count=`ceil((top-bottom)/
//! (size*0.4))` + `random_splits` + 四边 + `xz_rotation_bad`。
use glam::Vec3;

/// B `construct_wooden_pillar_w_stone_base`（0x141fe2150）。**未实现**。
pub fn construct_wooden_pillar_w_stone_base(_base: Vec3, _height: f32, _out: &mut Vec<[u8; 96]>) {
    todo!(
        "未实现: construct_wooden_pillar_w_stone_base 0x141fe2150（1.1 钳/1.25 石基/0.3 斜撑/0x18B 二路未逐语句）"
    )
}

/// B `construct_brick_columns`（0x141fe3240）。**未实现**。
pub fn construct_brick_columns(_positions: &[Vec3], _height: f32, _out: &mut Vec<[u8; 96]>) {
    todo!("未实现: construct_brick_columns 0x141fe3240（FrozenMap(0x3a)→writer→0x40B 表未逐语句）")
}

/// B `construct_crossbrick_pillar`（0x142418f50）。**未实现**。
pub fn construct_crossbrick_pillar(_base: Vec3, _height: f32, _out: &mut Vec<[u8; 96]>) {
    todo!(
        "未实现: construct_crossbrick_pillar 0x142418f50（count=ceil((top-bottom)/(size*0.4))/random_splits/四边/xz_rotation_bad 未逐语句）"
    )
}
