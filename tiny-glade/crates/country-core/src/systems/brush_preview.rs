//! `brush_preview.rs` — 笔刷预览（B 0x140865050，O 结构）。
//!
//! 证据：`chapters/systems/brush_preview.rs.md`（terrain-water 域）：
//! - `clone_behavior` @0x140867080：`ComponentCloneBehavior` 判别 2 + 克隆函数指针
//!   0x1412eff70（`lea rdx,[rip+X]; mov eax,2; ret`）；
//! - `register_required_components` @0x140866800：Bevy 派生注册胶水；
//! - `brush_preview` @0x140865050：预览网格跟随光标/笔刷大小更新（26KB）；
//! - `brush_size`（行 94，内联）：`BrushSizes` 读取辅助。
use bevy_ecs::prelude::*;
use glam::Vec3;

use crate::{
    resources::{
        brush_sizes::BrushSizes, cursor_terrain_raycast::CursorTerrainRaycast,
        mouse_terrain_raycast::CursorPositionSS,
    },
    systems::mode_manager::AppMode,
};

/// `BrushPreview` 组件标记（O：`impl$6::clone_behavior` = 判别 2 + 克隆函数指针）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct BrushPreview {
    /// 笔刷大小（世界单位）。
    pub size: f32,
}

/// 克隆行为判别（O：0x140867080 `mov eax,2`）。
pub const CLONE_BEHAVIOR_DISC: u8 = 2;

/// 预览网格跟随光标（O 结构）：读光标 + 当前工具笔刷大小 → 更新预览。
/// 预览网格实体变换由宿主按 [`BrushPreviewUpdate`] 事件应用。
pub fn brush_preview(
    mode: Res<AppMode>,
    brush_sizes: Res<BrushSizes>,
    cursor: Res<CursorPositionSS>,
    raycast: Res<CursorTerrainRaycast>,
    mut updates: EventWriter<BrushPreviewUpdate>,
) {
    // 按当前工具取笔刷大小（O：`brush_size` 行 94 内联读 BrushSizes）。
    let size = match mode.native_tag() {
        7 => brush_sizes.path,    // Path
        15 => brush_sizes.water,  // Water
        17 => brush_sizes.garden, // Garden
        _ => 1.0,
    };
    let position = raycast.position();
    updates.write(BrushPreviewUpdate {
        size,
        position,
        screen: cursor.0,
    });
}

/// 预览更新事件（I：宿主把 size/position 应用到 BrushPreview 网格实体）。
#[derive(Event, Clone, Copy, Debug, PartialEq)]
pub struct BrushPreviewUpdate {
    pub size: f32,
    pub position: Vec3,
    pub screen: Vec2f32,
}

/// 屏幕位置（f32 对）。
pub type Vec2f32 = glam::Vec2;

/// `register_required_components` @0x140866800（O：Bevy 派生注册胶水）。
pub fn register_required_components(_world: &mut World) {}
