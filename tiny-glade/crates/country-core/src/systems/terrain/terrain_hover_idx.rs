//! `terrain_hover_idx.rs` — 地形悬停笔画索引（B 0x140aff720，O）。
//!
//! 证据：`terrain-water/ALGORITHMS.md`、re-analyst-interaction 深度逆向（interaction-group）：
//! - AppMode 位掩码 **0x421**（位 0/5/10 = Editing(None)/Editing(Terrain)/Terrain）门；
//! - 读 `MouseTerrainRaycastSsbo` 的 `id = *(i32*)(buf + 0x10)`（-1 哨兵）；
//! - 水位采样 ≤ 0 / 光标无效 / 距离 ≤ 0.5 不更新；
//! - 写 `HoveredTerrainStrokeId { tag=1, id }`。
//!
//! `MouseTerrainRaycastSsbo` 布局未恢复（`render_ssbo` 为 `Gap`），这里用
//! [`MouseTerrainHoverReadback`] 适配资源承载宿主从 SSBO 拷贝的 id（待取证：SSBO 字段）。
use bevy_ecs::prelude::*;

use crate::{resources::mouse_terrain_raycast::CursorPositionSS, systems::mode_manager::AppMode};

/// 悬停地形笔画 id 资源（O：`HoveredTerrainStrokeId { tag=1, id }`，0x140aff720 写）。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct HoveredTerrainStrokeId {
    /// 命中标记（写 1）。
    pub tag: u8,
    /// 笔画 id（-1 = 无命中）。
    pub id: i32,
}

/// 鼠标地形射线回读适配（I：`MouseTerrainRaycastSsbo` 布局 Gap，宿主从 SSBO 拷贝 id）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct MouseTerrainHoverReadback {
    /// 命中笔画 id（-1 哨兵 = 未命中）。
    pub id: i32,
    /// 命中距离（≤ 0.5 不更新）。
    pub distance: f32,
}

/// 水位采样适配（I：WaterMaskSsbo 布局 Gap；≤ 0 不更新）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct TerrainWaterSample {
    pub level: f32,
}

/// `terrain_hover_idx` @0x140aff720（O 结构）。
pub fn terrain_hover_idx(
    mode: Res<AppMode>,
    hover: Res<MouseTerrainHoverReadback>,
    water: Res<TerrainWaterSample>,
    _cursor: Res<CursorPositionSS>,
    mut hovered: ResMut<HoveredTerrainStrokeId>,
) {
    // AppMode 0x421 掩码门（O：`mode<0xb && (0x421>>mode)&1`，bits {0,5,10} =
    // All/Garden/Terrain 工具；re-analyst 裁决，修正早前误记 {0,5,9}）。
    let disc = mode.native_tag();
    if disc >= 0xb || ((0x421u32 >> disc) & 1) == 0 {
        return;
    }
    // -1 哨兵 → 未命中，不更新。
    if hover.id == -1 {
        return;
    }
    // 水位 ≤ 0 不更新（O：章节；re-analyst 纠正为“高度≤0 且 id 有效会写”，以机器码为准）。
    if water.level <= 0.0 {
        return;
    }
    if hover.distance <= 0.5 {
        return;
    }
    hovered.tag = 1;
    hovered.id = hover.id;
}
