//! `ui_draw_terrain.rs` — 地形绘制 UI（B 0x140a9fdc0，O 门 + 结构）。
//!
//! 证据：`terrain-water/decomp/decomp-0x140a9fdc0.txt`、re-analyst-interaction：
//! - 门：`*(int*)*param_1 == 10`（AppMode::Terrain；re-analyst“AppMode==10 语义名冲突
//!   待裁决”，暂按 mode==10）；
//! - 高度采样 `FUN_140a14df0` = `bilinear_sample_at_world_xz<RasterBufferRef<512>>`
//!   （常量 `0x43020000` = 130.0×2、`_UNK_142a7b658` = 栅格引用）；高度 ≤ 0 → 不注册画布；
//! - 悬停/光标有效时注册地形画布 widget + 消费光标增量 → `new_stroke` +
//!   [`EvTerrainStrokeChanged`] 生产。
//!
//! widget 注册细节（UiState::add、名字 hash）与光标→世界变换的精确实现待取证（crate 的
//! UiState 在 systems/ui，宿主负责）。
use bevy_ecs::prelude::*;
use glam::Vec2;

use crate::{
    resources::{
        cursor_terrain_raycast::CursorTerrainRaycast, terrain_editing::{StrokeId, TerrainEditingData},
    },
    systems::{mode_manager::AppMode, terrain::draw_terrain_stroke::EvTerrainStrokeChanged},
};

/// 悬停有效性适配（I：UI 悬停上下文 layout Gap；宿主填充）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct TerrainHoverInput {
    /// 悬停命中（位 0）。
    pub hovered: bool,
    /// 命中位置（世界）。
    pub position: Vec2,
}

/// `ui_draw_terrain` @0x140a9fdc0（O 门；绘制输入 → 新笔画）。
pub fn ui_draw_terrain(
    mode: Res<AppMode>,
    hover: Res<TerrainHoverInput>,
    cursor: Res<CursorTerrainRaycast>,
    mut terrain: ResMut<TerrainEditingData>,
    mut changed: EventWriter<EvTerrainStrokeChanged>,
) {
    // 门：AppMode::Terrain（O：0x140a9fdc0 首指令 `!= 10` 早退）。
    if mode.native_tag() != 10 {
        return;
    }
    let Some(cursor_pos) = cursor_world_pos(&cursor) else {
        return;
    };
    if !hover.hovered {
        return;
    }
    // 点按 → new_stroke（O 生产面：章节“点按 → new_stroke + EvTerrainStrokeChanged”）。
    if let Ok(Some(id)) = terrain.new_stroke([cursor_pos.x, 0.0, cursor_pos.y]) {
        changed.write(EvTerrainStrokeChanged(StrokeId(id.0)));
    }
}

/// 世界光标位置（I：CursorTerrainRaycast 命中时返回世界坐标）。
pub fn cursor_world_pos(cursor: &CursorTerrainRaycast) -> Option<Vec2> {
    let pos = cursor.position();
    (cursor.tag != 0).then_some(Vec2::new(pos.x, pos.z))
}
