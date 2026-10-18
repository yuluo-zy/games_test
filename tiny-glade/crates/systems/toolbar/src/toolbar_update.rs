//! `toolbar::toolbar_update` 资源/系统面（依赖线使能，2026-10-17）。
//!
//! 本批只落 `ToolbarSizeMult` 资源（`render_taffy_ui_to_widgets` 13 参之一的
//! `Res<ToolbarSizeMult>`）。其余 `toolbar_update` 系统
//! （`toolbar_button_size_and_padding_update` 0x141eaec50、
//! `taffy_update_drawables` 0x142340d20、`taffy_finalize_element_size` 0x142340890）
//! 仍未实现（见 RESTORE.md）。

use bevy_ecs::prelude::Resource;

/// 工具栏尺寸倍率（O：`toolbar::toolbar_update::ToolbarSizeMult`；
/// `render_taffy_ui_to_widgets` 0x142338990 fresh 反编译中作为 `f32` 读入
/// （`fVar26 = *local_208`，用于 `rect = (len + scale·10·mult) · 0.5` 的扩张）。
/// **重建（I）**：单 f32 载荷；字段名 U。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct ToolbarSizeMult {
    /// 尺寸倍率（f32@+0）。
    pub multiplier: f32,
}
