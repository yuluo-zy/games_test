//! `ui_draw_terrain.rs` — 地形绘制 UI（B 0x140a9fdc0）。
//!
//! 证据：fresh 反编译 + 反汇编（restorer-terrain-clutter/jobs/decomp/140a9fdc0、
//! disasm-1.txt）。逐语句对照：
//! - 门：`*(int*)*in_RCX != 10 → return`（AppMode::Terrain）；
//! - 高度采样：`bilinear_sample_at_world_xz<RasterBufferRef<512>>`（extent 130）；
//! - **画布 1**（条件 `height > 0 ∧ ((tag&1)==0 ∨ pos.y ≤ 0.5)`）：
//!   `clearing_canvas_w_layer_and_sound("terrain_deny_draw", 0x11, 0)`
//!   （0x140a9fec2 路径；名字串 @0x142adc2df，R8=0x11、R9=0，栈字节 1）；
//! - **画布 2**（无条件）：`clearing_canvas_w_layer_and_sound("terrain_draw", 0x0c, 3)`
//!   （名字串 @0x142adc2f0，R8=0xc、R9B=3，栈字节 2）；
//! - `Circle2d::new(命中位置或 0, 0.8)`（半径常量 @0x142a16cb8 = 0.8）→
//!   `GladeBorder::is_shape_inside`；**非界内**写 8 B unit 事件（原生
//!   `EventInstance<WriteSettingsFileCmd>`，crate 以 [`RecalculateHeightMapCmd`] 承载）；
//! - 非界内且 `widget.dragged() != 2` 再写一条 8 B unit 事件；
//! - subtype = `widget.dragged()`（`[+0x18] ≥ 2 ∧ [+0x38]==widget id` → `[+0x19]`，否则 2），
//!   由 UI 交互（UiState::interaction）驱动，**不是**自造的 pressed 模型；
//! - `uVar10 = min(subtype.wrapping_sub(2), 3)`；u8 回绕状态机：
//!   `uVar10-1 < 2` → `(prev-1) > 1 ? inside : 2`；否则 `uVar10 != 0 ? ((prev-1)<2)*3 : 0`；
//! - 若 subtype == 3：把 `tag&1 ? pos.y : 0` 写入 drag 距离槽（本实现
//!   [`TerrainDrawState::drag_distance`]）；
//! - 末尾 `&DAT_142adc498` 跳表按 state 分派（UNRECOVERED_JUMPTABLE；目标函数未恢复，
//!   state 语义按既有登记：1=起始笔、2=续笔 morph、3=释放读笔、0=无动作）。
use bevy_ecs::prelude::*;
use glam::Vec2;
use utils::geometry::Circle2d;

use crate::{
    resources::{
        cursor_terrain_raycast::CursorTerrainRaycast,
        glade_border::{BorderShape, GladeBorder},
        terrain_editing::{StrokeId, TerrainEditingData},
        terrain_height::{TerrainHeightsData, TerrainHeightSsbo},
    },
    systems::{
        mode_manager::AppMode,
        ui::resources::ui_state::UiState,
        terrain::{draw_terrain_stroke::EvTerrainStrokeChanged, RecalculateHeightMapCmd},
    },
};

/// 距离门阈值（O：`_UNK_142925870` = 0.5）。
pub const FOCUS_DISTANCE_GATE: f32 = 0.5;
/// 边界检测圆半径（O：反汇编 `MOVSS XMM3,[0x142a16cb8]` = 0.8）。
pub const BORDER_CHECK_RADIUS: f32 = 0.8;
/// 画布 1（deny）参数（O：反汇编 R8D=0x11、R9D=0，栈字节 1）。
pub const DENY_CANVAS_ARGUMENT: u8 = 0x11;
pub const DENY_CANVAS_LAYER: u8 = 0;
/// 画布 2（draw）参数（O：反汇编 R8D=0xc、R9B=3，栈字节 2）。
pub const DRAW_CANVAS_ARGUMENT: u8 = 0x0c;
pub const DRAW_CANVAS_LAYER: u8 = 3;
/// 画布名字（O：RNE 直读 @0x142adc2df / @0x142adc2f0）。
pub const DENY_CANVAS_NAME: &str = "terrain_deny_draw";
pub const DRAW_CANVAS_NAME: &str = "terrain_draw";
/// state-2 续笔 morph 间距（0x140a97880 内常量待取证；以 tg_terrain 默认 1.0 接入，I）。
pub const MORPH_SPACING: f32 = 1.0;

/// 悬停有效性适配（I：UI 悬停上下文 layout Gap；ui_edit_terrain 消费）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct TerrainHoverInput {
    /// 悬停命中（位 0）。
    pub hovered: bool,
    /// 命中位置（世界）。
    pub position: Vec2,
    /// UI 交互激活标记（O：ui_edit `*pcVar23` = CursorTerrainRaycast.tag；1 = 命中）。
    pub pressed: bool,
}

/// DrawState（O：Local；crate 以 Resource 承载以便宿主/测试注入与观测，I；`+0x20..0x40` 8 B 元素 Vec；结尾状态字节回写 `*in_stack_00000068`；
/// `*in_stack_00000070` = drag 距离浮点槽）。
#[derive(Resource, Clone, Debug, Default)]
pub struct TerrainDrawState {
    /// 8 B 元素记录（qword + byte；语义 U，按机器码偏移）。
    pub records: Vec<u64>,
    /// 结尾状态机状态字节 `uVar11`（disasm 0x140aa0099 `MOV byte [RCX],AL`）。
    pub state: u8,
    /// subtype==3 时写入的 drag 距离（disasm 0x140aa00c3 `MOVSS [R14],XMM0`）。
    pub drag_distance: f32,
}

/// `ui_draw_terrain` @0x140a9fdc0（逐语句重建）。
#[allow(clippy::too_many_arguments)]
pub fn ui_draw_terrain(
    mode: Res<AppMode>,
    hover: Res<CursorTerrainRaycast>,
    terrain: Res<TerrainHeightsData<TerrainHeightSsbo>>,
    border: Res<GladeBorder>,
    ui_state: Res<UiState>,
    mut editing: ResMut<TerrainEditingData>,
    mut changed: EventWriter<EvTerrainStrokeChanged>,
    mut recalculate: EventWriter<RecalculateHeightMapCmd>,
    mut draw_state: ResMut<TerrainDrawState>,
) {
    // 门：AppMode::Terrain（O：0x140a9fdf9 `CMP dword [RAX],0xa`）。
    if mode.native_tag() != 10 {
        return;
    }
    // 光标面（O：`tag&1` @+0、`pos.x` @+4、`pos.y` @+8（distance 槽）、`pos.z` @+0xc）。
    let hovered = (hover.tag & 1) != 0;
    let cursor = Vec2::new(hover.pos.x, hover.pos.z);
    let distance = hover.pos.y;
    // 高度采样（O：0x140a9fe9e `bilinear_sample_at_world_xz<RasterBufferRef<512>>`）。
    let height = terrain.sample_at_world_xz(cursor.x, cursor.y);
    // 画布 1：**`height>0 ∧ (¬hovered ∨ dist≤0.5)`**（O：disasm 0x140a9fea3..0x140a9ff07）。
    if height > 0.0 && (!hovered || distance <= FOCUS_DISTANCE_GATE) {
        ui_state.clearing_canvas_w_layer_and_sound(
            DENY_CANVAS_NAME,
            DENY_CANVAS_ARGUMENT,
            DENY_CANVAS_LAYER,
        );
    }
    // 画布 2：**无条件**（O：disasm 0x140a9ff0c..0x140a9ff49，名字 "terrain_draw"）。
    let widget = ui_state.clearing_canvas_w_layer_and_sound(
        DRAW_CANVAS_NAME,
        DRAW_CANVAS_ARGUMENT,
        DRAW_CANVAS_LAYER,
    );
    // Circle2d + 边界判定（O：0x140a9ff8a Circle2d::new，半径 0.8；0x140a9ffa9
    // GladeBorder::is_shape_inside）。
    let center = if hovered { cursor } else { Vec2::ZERO };
    let inside = border.is_shape_inside(BorderShape::Circle(Circle2d::new(
        center,
        BORDER_CHECK_RADIUS,
    )));
    if !inside {
        // 8 B unit 事件（原生 EventInstance<WriteSettingsFileCmd>）。
        recalculate.write(RecalculateHeightMapCmd);
        // 第二条：`dragged() != 2`（disasm 0x140a9ffec..0x140a9fffc）。
        if widget.dragged() != 2 {
            recalculate.write(RecalculateHeightMapCmd);
        }
    }
    // subtype = widget.dragged()（disasm 0x140aa003c..0x140aa004a）。
    let subtype = widget.dragged();
    // `ADD AL,0xfe` + `min(…,3)`（disasm 0x140aa004e..0x140aa005b）。
    let uvar10 = subtype.wrapping_sub(2).min(3);
    let prev = draw_state.state;
    // 状态机（disasm 0x140aa006c..0x140aa0096）。
    let state: u8 = if uvar10.wrapping_sub(1) < 2 {
        if prev.wrapping_sub(1) > 1 {
            u8::from(inside)
        } else {
            2
        }
    } else if uvar10 != 0 {
        if prev.wrapping_sub(1) < 2 {
            3
        } else {
            0
        }
    } else {
        0
    };
    draw_state.state = state;
    // subtype == 3 → drag 距离槽（disasm 0x140aa00a2..0x140aa00c3）。
    if subtype == 3 {
        draw_state.drag_distance = if hovered { distance } else { 0.0 };
    }

    // 末尾跳表 `&DAT_142adc498`（UNRECOVERED_JUMPTABLE；目标函数未恢复）。
    // 已恢复语义：1 = 起始笔 → new_stroke + EvTerrainStrokeChanged；
    // 2 = 续笔 → get_last_mut_stroke + insert_or_morph_last_point；
    // 3 = 释放/读笔（crate 输入模型无 release 事件，登记）；0 = 无动作。
    match state {
        1 => {
            if let Ok(Some(id)) = editing.new_stroke([cursor.x, 0.0, cursor.y]) {
                changed.write(EvTerrainStrokeChanged(StrokeId(id.0)));
            }
        }
        2 => {
            if let Ok(stroke) = editing.get_last_mut_stroke() {
                stroke.insert_or_morph_last_point([cursor.x, 0.0, cursor.y], MORPH_SPACING);
            }
        }
        _ => {}
    }
}
