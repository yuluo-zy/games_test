//! `ui_edit_terrain.rs` — 地形编辑 UI（B 0x140aef1f0，O 门 + 结构）。
//!
//! 证据：`terrain-water/decomp/decomp-0x140aef1f0.txt`（34 KB）、章节、
//! re-analyst-interaction：
//! - 门 `mode == 10`（AppMode::Terrain；AppMode==10 语义名冲突待裁决）；
//! - 每 hovered 笔画按 **16 位脏掩码位图** 遍历（哪些参数变化），生成
//!   `EditTerrainStrokeCmd` / `SetTerrainStrokePtsCmd`；
//! - 维护 [`MouseActivityTimer`] / [`TerrainStrokeParamsCache`]（ui_edit_terrain 自有资源）；
//! - `add<String>`（0x1409231f0）+ 6 处资源写（16B/8B/0x28B 记录）、常量 130.0×2/0x100、
//!   panic@0x142ae3acf（行 370）。
//!
//! widget 注册 / 事件组（CursorCaptureCmd/EnableArrowGizmoStretch/AddTerrainHighlightCmd）
//! 的精确实现待取证（宿主 UI 接线）。
use bevy_ecs::prelude::*;
use glam::Vec2;
use tg_terrain::edit::Adjustment;

use crate::{
    resources::{
        terrain_editing::{StrokeId, TerrainEditingData},
        time::GameTime,
    },
    systems::mode_manager::AppMode,
};

use super::{
    edit_terrain_stroke::{EditTerrainStrokeCmd, SetTerrainStrokePtsCmd},
    terrain_hover_idx::HoveredTerrainStrokeId,
    ui_draw_terrain::TerrainHoverInput,
};

/// 鼠标活动计时器（O：`ui_edit_terrain::MouseActivityTimer::new` 行 211）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct MouseActivityTimer {
    /// 距上次鼠标活动的秒数。
    pub idle_secs: f32,
    /// 最近一次鼠标位置。
    pub last_cursor: Vec2,
}

/// 笔画参数缓存（I：比较新旧参数以计算 16 位脏掩码）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct TerrainStrokeParamsCache {
    /// 当前 hover 笔画 id（-1 哨兵 = 无）。
    pub stroke_id: i32,
    /// 上帧记录的高度。
    pub height: f32,
    /// 上帧记录的 falloff。
    pub falloff: f32,
    /// 上帧记录的点集指纹（len ^ 首点位型；用于脏掩码点集位）。
    pub points_stamp: u64,
}

/// 16 位脏掩码位图（I：re-analyst“每 hovered 笔画按 16 位脏掩码遍历”）。
pub const DIRTY_MASK_HEIGHT: u16 = 1 << 0;
pub const DIRTY_MASK_FALLOFF: u16 = 1 << 1;
pub const DIRTY_MASK_POINTS: u16 = 1 << 2;

/// `ui_edit_terrain` @0x140aef1f0（O 门；按脏掩码生成地形编辑命令）。
pub fn ui_edit_terrain(
    mode: Res<AppMode>,
    time: Res<GameTime>,
    hovered: Res<HoveredTerrainStrokeId>,
    hover: Res<TerrainHoverInput>,
    mut timer: ResMut<MouseActivityTimer>,
    mut cache: ResMut<TerrainStrokeParamsCache>,
    mut terrain: ResMut<TerrainEditingData>,
    mut edits: EventWriter<EditTerrainStrokeCmd>,
    mut point_sets: EventWriter<SetTerrainStrokePtsCmd>,
) {
    if mode.native_tag() != 10 {
        return;
    }
    let cursor = hover.position;
    // 鼠标活动计时（O：自有资源维护）。
    if cursor != timer.last_cursor {
        timer.idle_secs = 0.0;
        timer.last_cursor = cursor;
    } else {
        timer.idle_secs += time.delta;
    }
    // hover 笔画变化 → 重新缓存参数并清脏掩码。
    if hovered.id != cache.stroke_id {
        cache.stroke_id = hovered.id;
        cache.height = 0.0;
        cache.falloff = 0.0;
        cache.points_stamp = 0;
        return;
    }
    if hovered.id == -1 || hovered.tag == 0 {
        return;
    }
    let Some(stroke) = terrain.strokes.get(&StrokeId(hovered.id as u32)) else {
        return;
    };
    // 16 位脏掩码（I）：高度/falloff/点变化。
    let mut dirty: u16 = 0;
    if stroke.height_ws != cache.height {
        dirty |= DIRTY_MASK_HEIGHT;
    }
    if stroke.falloff != cache.falloff {
        dirty |= DIRTY_MASK_FALLOFF;
    }
    let points_stamp = stroke
        .points
        .iter()
        .fold(stroke.points.len() as u64, |acc, p| {
            acc.wrapping_mul(31)
                .wrapping_add(f32::to_bits(p[0]) as u64)
                .wrapping_add(f32::to_bits(p[1]) as u64)
                .wrapping_add(f32::to_bits(p[2]) as u64)
        });
    if points_stamp != cache.points_stamp {
        dirty |= DIRTY_MASK_POINTS;
    }
    if dirty & (DIRTY_MASK_HEIGHT | DIRTY_MASK_FALLOFF) != 0 {
        // 原 `EditCommand` 字段序 `{falloff, height, position, id}`（id@+0x1c）与 tg_terrain
        // `{id, falloff, height, position}` 差异待取证（reviewer 基线）；当前与 tg_terrain
        // 同型生产/消费内部一致。
        edits.write(EditTerrainStrokeCmd(tg_terrain::edit::EditCommand {
            id: StrokeId(hovered.id as u32),
            falloff: if dirty & DIRTY_MASK_FALLOFF != 0 {
                Adjustment::Absolute(stroke.falloff)
            } else {
                Adjustment::Keep
            },
            height: if dirty & DIRTY_MASK_HEIGHT != 0 {
                Adjustment::Absolute(stroke.height_ws)
            } else {
                Adjustment::Keep
            },
            position: [cursor.x, 0.0, cursor.y],
        }));
        cache.height = stroke.height_ws;
        cache.falloff = stroke.falloff;
    }
    if dirty & DIRTY_MASK_POINTS != 0 {
        // 点替换命令（points/position/id；原事件 stride 0x30）。
        point_sets.write(SetTerrainStrokePtsCmd {
            points: stroke.points.clone(),
            position: [cursor.x, 0.0, cursor.y].into(),
            id: StrokeId(hovered.id as u32),
        });
        cache.points_stamp = points_stamp;
    }
    // 常量 130.0×2（采样窗）与 panic@0x142ae3acf（组件表越界）在上游采样/UI 接线，待取证。
    let _ = (&mut timer, &mut terrain);
}
