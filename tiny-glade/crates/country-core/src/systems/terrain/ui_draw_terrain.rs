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
//! - 画布注册三道门（decomp 行 73-84）：**高度 ≤ 0 → 不注册**；未悬停 → 不注册；
//!   `dist > 0.5`（`_UNK_142925870`）→ 不注册；
//! - 通过后推 `clearing_canvas_w_layer_and_sound`（FUN_14092df50，tag 0x11，payload 0/1）；
//! - **tag 0xc 界外反馈事件无条件发射**（decomp 行 97：`FUN_14092df50(..,&UNK_142adc2f0,0xc,
//!   3,2,..)`，与画布注册无关）；
//! - `is_shape_inside`（0x140ad2970）判定界内 → 非界内推 DrawState Vec（`+0x20..0x40`，
//!   8 B 元素 {qword, count}）；
//! - 结尾状态机（decomp 行 134-165）：事件 subtype `cVar9`（默认 2，tag-0xc 事件队列
//!   `+0x19` 字节）+ 上帧状态 `prev` + `bVar8` 折叠为状态 `uVar11`（0..3），
//!   经 `&UNK_142adc498` 跳表分派。**跳表目标函数未恢复**（Ghidra
//!   UNRECOVERED_JUMPTABLE）；已恢复的语义：`cVar9==3`（点按）写 stroke id，且点按分支
//!   才启动笔画（`new_stroke` + `EvTerrainStrokeChanged`）。分支 0/1/3 行为待取证。
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
/// state-2 续笔 morph 间距（0x140a97880 内 `movss xmm2,[rip+…]` 常量待取证；以
/// tg_terrain 默认 1.0 接入，标注 I——① T3 jumptable 契约）。
pub const MORPH_SPACING: f32 = 1.0;

/// 悬停有效性适配（I：UI 悬停上下文 layout Gap；ui_edit_terrain 消费）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct TerrainHoverInput {
    /// 悬停命中（位 0）。
    pub hovered: bool,
    /// 命中位置（世界）。
    pub position: Vec2,
    /// 点按状态（O：事件 subtype 3 = 点按；供状态机判定"点按分支才启动笔画"）。
    pub pressed: bool,
}

/// 画布注册事件（O：`clearing_canvas_w_layer_and_sound` FUN_14092df50，tag 0x11，
/// payload `{0, 1}`：层 + 启用标志；名字 U，按机器码偏移命名）。
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct TerrainCanvasRegister {
    pub layer: u32,
    pub enabled: u32,
}

/// 界外反馈事件（O：`FeedbackInputOutOfBorder` 生产面；tag 0xc **无条件**发射，载荷 =
/// `{3, 2}`）。当前事件 struct 载荷语义 I，仅登记 tag-0xc 生产面。
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct TerrainOutOfBorder {
    /// O：事件队列 push 的载荷首字节（3）。
    pub kind: u32,
    /// O：事件队列 push 的载荷次字节（2）。
    pub sub: u32,
}

/// DrawState（O：Local，`+0x20..0x40` 8 B 元素 Vec {qword, count}；悬停点列/拖拽状态）。
#[derive(Resource, Clone, Debug, Default)]
pub struct TerrainDrawState {
    /// 8 B 元素记录（qword + byte；语义 U，按机器码偏移）。
    pub records: Vec<u64>,
    /// 结尾状态机状态字节 `uVar11`（decomp 行 154 写回 Local `*in_stack_00000068`）。
    pub state: u8,
}

/// `ui_draw_terrain` @0x140a9fdc0（O 消费面：高度采样门 + widget 注册 + 界外反馈 + 状态机）。
#[allow(clippy::too_many_arguments)]
pub fn ui_draw_terrain(
    mode: Res<AppMode>,
    hover: Res<CursorTerrainRaycast>,
    terrain: Res<TerrainHeightsData<TerrainHeightSsbo>>,
    input: Res<TerrainHoverInput>,
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
    // 画布注册三道门（O：decomp 行 73-84）：高度 ≤ 0 → 不注册；未悬停 → 不注册；
    // 距离 > 0.5 → 不注册。
    if height <= 0.0 || !hovered || distance > FOCUS_DISTANCE_GATE {
        // 未通过画布注册门：跳过注册（decomp `goto code_r0x000140a9ff0c`）。
    } else {
        // 画布注册（O：clearing_canvas_w_layer_and_sound，tag 0x11，payload {0,1}）。
        canvas.write(TerrainCanvasRegister { layer: 0, enabled: 1 });
    }
    // **tag 0xc 界外反馈事件无条件发射**（O：decomp 行 97，与画布注册无关；
    // 载荷 {3, 2} = FUN_14092df50 参数）。
    out_of_border.write(TerrainOutOfBorder { kind: 3, sub: 2 });

    // 点按/光标增量：`is_shape_inside` 判定（O：FUN_140ad2970）。非界内 →
    // 推 DrawState 记录（8 B 元素 {qword, count}，decomp 行 111-120；元素语义 U）。
    let inside = input.hovered && input.pressed;
    if !inside {
        draw_state.records.push(0);
        if draw_state.records.len() > 64 {
            draw_state.records.drain(0..1);
        }
    }

    // 结尾状态机（O：decomp 行 134-165）。事件 subtype：点按 = 3，其余默认 2。
    let cvar9: u8 = if input.pressed { 3 } else { 2 };
    let uvar10: u8 = cvar9.wrapping_sub(2).min(3);
    let prev = draw_state.state;
    let state: u8 = if (1..3).contains(&uvar10) {
        // uVar10 ∈ {1,2}：prev > 2 → bVar8（inside），否则 2。
        if prev > 2 {
            u8::from(inside)
        } else {
            2
        }
    } else if uvar10 != 0 {
        // uVar10 == 3：prev ≤ 2 → 3，否则 0。
        if prev <= 2 {
            3
        } else {
            0
        }
    } else {
        0
    };
    draw_state.state = state;

    // 点按分支才启动笔画（O：章节"点按 → new_stroke + EvTerrainStrokeChanged"；
    // `cVar9 == 3` 写 stroke id，decomp 行 155-162）。
    if cvar9 == 3 && inside {
        if let Ok(Some(id)) = editing.new_stroke([cursor.x, 0.0, cursor.y]) {
            changed.write(EvTerrainStrokeChanged(StrokeId(id.0)));
        }
    }

    // ① T3 契约 t3-ui_draw_terrain-jumptable §2：结尾状态机跳表 &UNK_142adc498 分派
    // state 0..3（0x140aa076d / 0x140aa00e9 / 0x140aa0235 / 0x140aa02c7）。
    // - state 0：函数 epilogue → 无动作（`_ => {}` 等价）。
    // - state 1：点按/new_stroke（上方 cvar9==3 分支，O 已落地）。
    // - state 2：移动/续笔 → get_last_mut_stroke + insert_or_morph_last_point
    //   （0x140924900 / 0x140a97880，续笔 morph 落点；morph 间距常量
    //   `movss xmm2,[rip+…]` 待取证，以 tg_terrain 默认 1.0 接入标注 I）。
    // - state 3：释放/读笔 → get_last_mut_stroke + 点集遍历（12B/点 = Vec3）。
    //   当前输入模型（TerrainHoverInput）无 release 事件，state 3 不可达，语义待取证登记。
    if state == 2 {
        if let Ok(stroke) = editing.get_last_mut_stroke() {
            stroke.insert_or_morph_last_point([cursor.x, 0.0, cursor.y], MORPH_SPACING);
        }
    }
}
