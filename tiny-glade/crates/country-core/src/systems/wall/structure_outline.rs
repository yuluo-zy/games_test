//! structure_outline.rs — 结构轮廓命令（组 A restorer 接线；BASELINE 参数面
//! `systems::wall::structure_outline::OutlineWallCmd` / `OutlineStairsCmd`——
//! system_color 的 ui_color_wall_and_roof 0x141c9ab70 / ui_color_windows_and_flags
//! 0x1422b0e00 发射（拾取命中 → 结构轮廓高亮）。载荷语义 U）。

use bevy_ecs::event::Event;

/// 墙轮廓命令（O：`structure_outline::OutlineWallCmd`；载荷语义 U）。
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct OutlineWallCmd {
    /// typed gap。
    pub _gap: u64,
}

/// 楼梯轮廓命令（O：`structure_outline::OutlineStairsCmd`；载荷语义 U）。
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct OutlineStairsCmd {
    /// typed gap。
    pub _gap: u64,
}
