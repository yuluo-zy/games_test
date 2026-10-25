//! 矩形大支架（construct_elevation_supports/rectangle_large_brackets.rs）。
//!
//! `construct_rectangle_large_brackets`（0x141253fc0）。
//! 2026-10-17 复核：旧实现（相邻点中点直推 64B）为**近似体**，已移除。
//! 真实为角部斜撑 + 横梁装配，几何参数未逐语句锚定。
use glam::Vec3;

/// B `construct_rectangle_large_brackets`（0x141253fc0）。**未实现**。
pub fn construct_rectangle_large_brackets(_positions: &[Vec3], _out: &mut Vec<[u8; 64]>) {
    todo!("未实现: construct_rectangle_large_brackets 0x141253fc0（角部斜撑+横梁几何参数未逐语句）")
}
