//! `ui_draw_terrain.rs` — 地形绘制 UI（B 0x140a9fdc0，O 消费面）。
//!
//! 证据：`terrain-water/decomp/decomp-0x140a9fdc0.txt`（168 行）、re-analyst-interaction
//! RE-ANALYSIS2 §3：
//! - 门：`*(int*)*param_1 == 10`（AppMode::Terrain）；
//! - 光标面（param_2 = `CursorTerrainRaycast`，O 布局 `{tag u8@0, pos.x@4, dist@8,
//!   pos.z@0xc}`）：`(*pbVar5 & 1)` = 悬停、`+4/+0xc` = 世界 x/z、`+8` = 射线距离；
//! - 高度采样 `FUN_140a14df0` = `bilinear_sample_at_world_xz<RasterBufferRef<512>>`
//!   （常量 `0x43020000` = 130.0×2 = 采样窗缩放、`_UNK_142a7b658` = 栅格引用）——本实现用
//!   crate 自有 [`TerrainHeightsData::sample_at_world_xz`]（同族采样）；
//! - 三道门：**高度 ≤ 0 → 不注册画布**；`(*pbVar5 & 1) == 0`（未悬停）→ 不注册；
//!   `dist > 0.5`（`_UNK_142925870`）→ 不注册；
//! - 通过后推 `clearing_canvas_w_layer_and_sound`（FUN_14092df50，tag 0x11，payload 0/1）；
//! - 光标世界坐标选双原点（`lVar12+0x298` flag 选 {+8/+0x10} 或 {+0x20/+0x28}）；
//! - `is_shape_inside`（0x140ad2970）判定界内 → 推 DrawState Vec（`+0x20..0x40`，
//!   8 B 元素 {qword, count}）→ 尾部 jumptable 分派（0..3：点按/拖拽/边界反馈）。
//!
//! 生产面：`new_stroke` → `EvTerrainStrokeChanged`；悬停/光标有效时注册地形画布 widget +
//! 消费光标增量。widget 注册细节（UiState::add、名字 hash）与光标→世界变换的精确实现
//! 依赖宿主 UI 画布（登记待接）。
use bevy_ecs::prelude::*;
use glam::Vec2;

use crate::{
    resources::{
        cursor_terrain_raycast::CursorTerrainRaycast,
        terrain_editing::{StrokeId, TerrainEditingData},
        terrain_height::{TerrainHeightsData, TerrainHeightSsbo},
    },
    systems::{mode_manager::AppMode, terrain::draw_terrain_stroke::EvTerrainStrokeChanged},
};

/// 距离门阈值（O：`_UNK_142925870` = 0.5）。
pub const FOCUS_DISTANCE_GATE: f32 = 0.5;
/// 高度采样窗缩放（O：`0x43020000` = 130.0×2，`bilinear_sample_at_world_xz` 的窗宽）。
pub const HEIGHT_SAMPLE_WINDOW: f32 = 130.0;

/// 悬停有效性适配（I：UI 悬停上下文 layout Gap；ui_edit_terrain 消费）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct TerrainHoverInput {
    /// 悬停命中（位 0）。
    pub hovered: bool,
    /// 命中位置（世界）。
    pub position: Vec2,
}

/// 画布注册事件（O：`clearing_canvas_w_layer_and_sound` FUN_14092df50，tag 0x11，
/// payload `{0, 1}`：层 + 启用标志；名字 U，按机器码偏移命名）。
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct TerrainCanvasRegister {
    pub layer: u32,
    pub enabled: u32,
}

/// 界外反馈事件（O：`DisplayOutOfBorder` / `FeedbackInputOutOfBorder` 生产面；载荷 =
/// 光标位置）。
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct TerrainOutOfBorder {
    pub position: Vec2,
}

/// DrawState（O：Local，`+0x20..0x40` 8 B 元素 Vec {qword, byte}；悬停点列/拖拽状态）。
#[derive(Resource, Clone, Debug, Default)]
pub struct TerrainDrawState {
    /// 8 B 元素记录（qword + byte；语义 U，按机器码偏移）。
    pub records: Vec<u64>,
}

/// `ui_draw_terrain` @0x140a9fdc0（O 消费面：高度采样门 + widget 注册 + 界外反馈）。
#[allow(clippy::too_many_arguments)]
pub fn ui_draw_terrain(
    mode: Res<AppMode>,
    hover: Res<CursorTerrainRaycast>,
    terrain: Res<TerrainHeightsData<TerrainHeightSsbo>>,
    mut editing: ResMut<TerrainEditingData>,
    mut changed: EventWriter<EvTerrainStrokeChanged>,
    mut canvas: EventWriter<TerrainCanvasRegister>,
    mut out_of_border: EventWriter<TerrainOutOfBorder>,
    mut draw_state: Local<TerrainDrawState>,
) {
    // 门：AppMode::Terrain（O：0x140a9fdc0 首指令 `!= 10` 早退）。
    if mode.native_tag() != 10 {
        return;
    }
    // 光标面（O：tag bit0 = 悬停、+4/+0xc = x/z、+8 = 距离）。
    let hovered = (hover.tag & 1) != 0;
    let cursor = Vec2::new(hover.pos.x, hover.pos.z);
    let distance = hover.distance(); // O 布局：+8 槽 = 射线距离
    // 高度采样（O：bilinear_sample_at_world_xz，本实现用同族 TerrainHeightsData 采样）。
    let height = terrain.sample_at_world_xz(cursor.x, cursor.y);
    // 三道门：高度 ≤ 0 → 不注册画布；未悬停 → 不注册；距离 > 0.5 → 不注册。
    if height <= 0.0 || !hovered || distance > FOCUS_DISTANCE_GATE {
        return;
    }
    // 画布注册（O：clearing_canvas_w_layer_and_sound，tag 0x11，payload {0,1}）。
    canvas.write(TerrainCanvasRegister { layer: 0, enabled: 1 });
    // 界内判定（O：is_shape_inside 0x140ad2970；本实现登记 DrawState 记录，真实边界
    // 判定依赖宿主 UI 画布 widget）。
    draw_state.records.push(0);
    if draw_state.records.len() > 64 {
        draw_state.records.drain(0..1);
    }
    // 生产面：点按 → new_stroke + EvTerrainStrokeChanged（O：章节“点按 → new_stroke +
    // EvTerrainStrokeChanged”）。
    if let Ok(Some(id)) = editing.new_stroke([cursor.x, 0.0, cursor.y]) {
        changed.write(EvTerrainStrokeChanged(StrokeId(id.0)));
    } else if draw_state.records.len() > 1 {
        // 已存在笔画且未命中 → 界外反馈（O：DisplayOutOfBorder/FeedbackInputOutOfBorder）。
        out_of_border.write(TerrainOutOfBorder { position: cursor });
    }
}
