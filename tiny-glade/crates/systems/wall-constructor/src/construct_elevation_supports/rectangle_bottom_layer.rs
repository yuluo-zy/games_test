//! 矩形底部砖层（construct_elevation_supports/rectangle_bottom_layer.rs）。
//!
//! `construct_rect_bottom_layer`（0x14124eb80）；`add_bottom_brick`
//! （0x141fe1de0）与圆形版共用同一体。
//!
//! 2026-10-17 复核：旧实现（宽度/深度各按 1.2 均分网格）为**近似体**，
//! 已移除。fresh（`evidence/elev-0x14124eb80.c`）已解结构：halftimber 早退
//! → halftimber 四角 plank 与普通 5 段边二路 → `ceil(len / 0.69)` 计数 +
//! `jitter=0.32` 宽/长 → `random_splits_into` 逐对起止点 → `add_bottom_brick`。
use glam::Vec3;

/// B `construct_rect_bottom_layer`（0x14124eb80）。**未实现**：见模块文档。
pub fn construct_rect_bottom_layer(
    _min: Vec3,
    _max: Vec3,
    _out: &mut Vec<[u8; 96]>,
) {
    todo!("未实现: construct_rect_bottom_layer 0x14124eb80（halftimber 四角/5 段边二路、0.69 计数、0.32 jitter 未逐语句）")
}
