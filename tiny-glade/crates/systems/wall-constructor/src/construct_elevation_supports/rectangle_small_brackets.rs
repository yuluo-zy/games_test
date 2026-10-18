//! 矩形小支架（rectangle_small_brackets.rs）。
//!
//! `construct_rectangle_small_brackets`（0x141250da0，7701B）。
//! 2026-10-17 复核：旧实现（逐点直推 96B）为**近似体**，已移除。
//! 真实逻辑（fresh 已取，1137 行 decomp）含托架几何参数/墙高门控/每角
//! 支架装配，未逐语句锚定。
use glam::Vec3;

/// B `construct_rectangle_small_brackets`（0x141250da0）。**未实现**。
pub fn construct_rectangle_small_brackets(
    _positions: &[Vec3],
    _out: &mut Vec<[u8; 96]>,
) {
    todo!("未实现: construct_rectangle_small_brackets 0x141250da0（1137 行：托架几何参数/门控/角部装配未逐语句）")
}
