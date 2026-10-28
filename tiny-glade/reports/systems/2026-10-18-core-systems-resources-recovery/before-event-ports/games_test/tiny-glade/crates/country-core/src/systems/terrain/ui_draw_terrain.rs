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
//! - Original jump table at0x142adc498 is resolved:0=noop,1=new stroke with live mapped height policy,
//!   2=inside-only continuation plus changed event,3=owned typed history completion.
use bevy_ecs::prelude::*;
use glam::Vec2;
use utils::geometry::Circle2d;

use crate::{
    asset_libraries::ssbo_library::AssetSsboLibrary,
    resources::{
        cursor_terrain_raycast::CursorTerrainRaycast,
        glade_border::{BorderShape, GladeBorder},
        terrain_editing::{StrokeId, TerrainEditingData},
        terrain_height::{TerrainHeightSsbo, TerrainHeightsData},
        terrain_rock_mask::{TerrainRockMaskSsbo, sample_at_world_xz},
    },
    systems::{
        mode_manager::AppMode,
        terrain::{
            RecalculateHeightMapCmd,
            deferred_terrain_history::{PendingTerrainHistory, TerrainEdit},
            draw_terrain_stroke::EvTerrainStrokeChanged,
            ui_edit_terrain::{StairsPlacementBucket, StairsPlacementEntry, TerrainEditUiState},
        },
        ui::resources::ui_state::UiState,
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
/// B state-2 caller 0x140aa025b reads .rdata@0x1429258d8 = 0x3e99999a.
pub const MORPH_SPACING: f32 = 0.3;

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
    /// Host projection of the original editing resource's active draw flag.
    pub active: bool,
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
    mut pending: ResMut<PendingTerrainHistory>,
    mut stairs: ResMut<StairsPlacementBucket>,
    assets: Option<Res<AssetSsboLibrary>>,
    mut terrain_ui: ResMut<TerrainEditUiState>,
) {
    // 门：AppMode::Terrain（O：0x140a9fdf9 `CMP dword [RAX],0xa`）。
    if mode.native_tag() != 10 {
        return;
    }
    // 光标面（O：`tag&1` @+0、`pos.x` @+4、`pos.y` @+8（distance 槽）、`pos.z` @+0xc）。
    let hovered = (hover.tag & 1) != 0;
    let position = hover.position();
    let cursor = Vec2::new(position.x, position.z);
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
        if prev.wrapping_sub(1) < 2 { 3 } else { 0 }
    } else {
        0
    };
    draw_state.state = state;
    // subtype == 3 → drag 距离槽（disasm 0x140aa00a2..0x140aa00c3）。
    if subtype == 3 {
        draw_state.drag_distance = if hovered { distance } else { 0.0 };
    }

    // B0x142adc498: resolved original new/continue/release/noop branches.
    match state {
        1 => {
            if let Ok(Some(id)) = editing.new_stroke(position.to_array()) {
                let library = assets
                    .as_deref()
                    .expect("terrain drawing requires the SSBO library");
                let mouse_index = crate::systems::render_loop::compute_passes::mouse_read_terrain_stroke_index::read_terrain_stroke_index(library);
                let mask = library
                    .get::<TerrainRockMaskSsbo>()
                    .expect("terrain drawing requires TerrainRockMaskSsbo");
                let existing = mouse_index.is_some_and(|index| index != -1);
                let on_rock = existing
                    && sample_at_world_xz(
                        bytemuck::cast_slice(&mask.mapped[..]),
                        cursor.x,
                        cursor.y,
                    ) != 0;
                let height = initial_stroke_height(mouse_index, on_rock, draw_state.drag_distance);
                editing.strokes.get_mut(&id).unwrap().height_ws = height;
                draw_state.active = true;
                terrain_ui.active = 1;
                changed.write(EvTerrainStrokeChanged(StrokeId(id.0)));
                stairs.entries.push(StairsPlacementEntry {
                    tag: 5,
                    decorator_a: position.x.to_bits(),
                    decorator_b: position.y.to_bits(),
                    sub: position.z.to_bits(),
                    value_a: height.to_bits(),
                    ..Default::default()
                });
            }
        }
        2 if inside => {
            if let Ok(stroke) = editing.get_last_mut_stroke() {
                // B 0x140aa0235..0x140aa02c2: the cursor keeps all XYZ
                // coordinates; border failure skips mutation and notification.
                stroke.insert_or_morph_last_point(position.to_array(), MORPH_SPACING);
                let id = StrokeId(
                    editing
                        .next_index
                        .checked_sub(1)
                        .expect("original last terrain stroke counter assertion"),
                );
                // The native caller publishes this even when the point helper
                // returns without changing a point (near duplicate / 1024 cap).
                changed.write(EvTerrainStrokeChanged(id));
            }
        }
        3 => {
            let id = editing.next_index.checked_sub(1).map(StrokeId);
            if let (Some(id), Ok(stroke)) = (id, editing.get_last_mut_stroke()) {
                pending.stage(TerrainEdit::NewTerrainStroke {
                    index: id,
                    height_ws: stroke.height_ws,
                    stroke_pts: stroke.points.clone(),
                });
                stairs.entries.push(StairsPlacementEntry {
                    tag: 0,
                    ..Default::default()
                });
                recalculate.write(RecalculateHeightMapCmd);
            }
            draw_state.active = false;
        }
        _ => {}
    }
}

#[cfg(test)]
mod recovered_extend_tests {
    use super::*;
    use crate::systems::{
        terrain::register_terrain_events,
        ui::resources::ui_state::{UiInteraction, WidgetId},
    };

    fn world(position: glam::Vec3, inside: bool) -> World {
        let mut world = World::new();
        register_terrain_events(&mut world);
        world.insert_resource(AppMode::Terrain);
        world.insert_resource(CursorTerrainRaycast {
            tag: 1,
            pos: position,
            ..Default::default()
        });
        world.insert_resource(TerrainHeightsData::<TerrainHeightSsbo>::new());
        let mut editing = tg_terrain::stroke::TerrainEditingData::default();
        editing.new_stroke([0.0; 3]).unwrap();
        editing
            .get_last_mut_stroke()
            .unwrap()
            .points
            .push([0.4, 0.0, 0.0]);
        world.insert_resource(TerrainEditingData(editing));
        world.insert_resource(GladeBorder(BorderShape::Circle(Circle2d::new(
            if inside {
                Vec2::ZERO
            } else {
                Vec2::splat(1000.0)
            },
            100.0,
        ))));
        world.insert_resource(UiState::new(UiInteraction {
            native_tag: 2,
            native_phase: 4,
            widget: WidgetId::clearing_canvas(DRAW_CANVAS_NAME),
        }));
        world.resource_mut::<TerrainDrawState>().state = 1;
        world
    }

    #[test]
    fn continuation_preserves_cursor_height_uses_point_three_spacing_and_notifies() {
        let mut world = world(glam::Vec3::new(0.8, 0.6, 0.1), true);
        let mut schedule = Schedule::default();
        schedule.add_systems(ui_draw_terrain);
        schedule.run(&mut world);
        let stroke = &world.resource::<TerrainEditingData>().strokes[&StrokeId(0)];
        // Existing segment .4 >= original spacing .3: append, rather than morph.
        assert_eq!(stroke.points, [[0.0; 3], [0.4, 0.0, 0.0], [0.8, 0.6, 0.1]]);
        assert_eq!(world.resource::<Events<EvTerrainStrokeChanged>>().len(), 1);
    }

    #[test]
    fn blocked_continuation_keeps_the_stroke_and_emits_no_changed_event() {
        let mut world = world(glam::Vec3::new(0.8, 0.6, 0.1), false);
        let mut schedule = Schedule::default();
        schedule.add_systems(ui_draw_terrain);
        schedule.run(&mut world);
        assert_eq!(
            world.resource::<TerrainEditingData>().strokes[&StrokeId(0)]
                .points
                .len(),
            2
        );
        assert!(
            world
                .resource::<Events<EvTerrainStrokeChanged>>()
                .is_empty()
        );
        assert_eq!(world.resource::<TerrainDrawState>().state, 2);
    }
}

#[cfg(test)]
mod typed_history_release_tests {
    use super::*;
    use crate::resources::history::{EditType, History, transactions::ObservedHistoryLedger};
    use crate::systems::{
        terrain::{deferred_terrain_history::history_record_terrain, register_terrain_events},
        ui::resources::ui_state::{UiInteraction, WidgetId},
    };

    #[test]
    fn release_records_an_owned_typed_stroke_once_and_resets_pending() {
        let mut world = World::new();
        register_terrain_events(&mut world);
        world.insert_resource(AppMode::Terrain);
        world.insert_resource(CursorTerrainRaycast::default());
        world.insert_resource(TerrainHeightsData::<TerrainHeightSsbo>::new());
        let mut editing = tg_terrain::stroke::TerrainEditingData::default();
        editing.new_stroke([1.0, 2.0, 3.0]).unwrap();
        editing
            .get_last_mut_stroke()
            .unwrap()
            .points
            .push([4.0, 5.0, 6.0]);
        world.insert_resource(TerrainEditingData(editing));
        world.insert_resource(GladeBorder(BorderShape::Circle(Circle2d::new(
            Vec2::ZERO,
            100.0,
        ))));
        world.insert_resource(UiState::new(UiInteraction {
            native_tag: 2,
            native_phase: 5,
            widget: WidgetId::clearing_canvas(DRAW_CANVAS_NAME),
        }));
        world.resource_mut::<TerrainDrawState>().state = 2;
        world.resource_mut::<TerrainDrawState>().active = true;
        let mut history: History = History::from_observed(ObservedHistoryLedger {
            entries: vec![],
            cursor: -1,
            undo_anchor: -1,
            resolved_snapshots: vec![],
            observed_count_58: 0,
            mutation_nonce: 0,
        });
        history.start_input_recording(7).unwrap();
        world.insert_resource(history);
        let mut schedule = Schedule::default();
        schedule.add_systems((ui_draw_terrain, history_record_terrain::<()>).chain());
        schedule.run(&mut world);
        assert!(world.resource::<PendingTerrainHistory>().is_empty());
        assert!(!world.resource::<TerrainDrawState>().active);
        let recorded = &world.resource::<History>().pending().unwrap().edits;
        assert_eq!(recorded.len(), 1);
        assert!(
            matches!(&recorded[0], EditType::Terrain(edit) if matches!(edit.as_ref(), TerrainEdit::NewTerrainStroke { index: StrokeId(0), height_ws, stroke_pts } if *height_ws == 2.5 && *stroke_pts == [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]))
        );
        schedule.run(&mut world);
        assert_eq!(
            world.resource::<History>().pending().unwrap().edits.len(),
            1
        );
    }
}

/// B0x140aa0138..0x140aa0534 initial height policy. Equal signed-zero values
/// choose the drag value, and NaN chooses the finite minimum as in MAXSS.
pub fn initial_stroke_height(mouse_index: Option<i32>, on_rock: bool, drag_height: f32) -> f32 {
    let existing = mouse_index.is_some_and(|index| index != -1);
    let minimum = if existing { 0.0 } else { 2.5 };
    let desired = if existing && !on_rock {
        drag_height + 1.5
    } else {
        drag_height
    };
    let height = if desired.is_nan() || desired < minimum {
        minimum
    } else {
        desired
    };
    if height > 20.0 { 20.0 } else { height }
}

#[cfg(test)]
pub(super) fn register_test_buffers(world: &mut World) {
    let library = AssetSsboLibrary::new();
    let device = crate::render::test_device::TestDevice(0);
    library.add_new::<crate::systems::render_loop::compute_passes::mouse_read_terrain_stroke_index::MouseTerrainRaycastSsbo>(&device);
    library.add_new::<TerrainRockMaskSsbo>(&device);
    world.insert_resource(library);
}

#[cfg(test)]
mod original_initial_height_tests {
    use super::*;
    #[test]
    fn original_b_initial_height_0() {
        assert_eq!(
            initial_stroke_height(None, false, f32::from_bits(3212836864)).to_bits(),
            1075838976
        );
    }
    #[test]
    fn original_b_initial_height_1() {
        assert_eq!(
            initial_stroke_height(None, false, f32::from_bits(0)).to_bits(),
            1075838976
        );
    }
    #[test]
    fn original_b_initial_height_2() {
        assert_eq!(
            initial_stroke_height(None, false, f32::from_bits(1065353216)).to_bits(),
            1075838976
        );
    }
    #[test]
    fn original_b_initial_height_3() {
        assert_eq!(
            initial_stroke_height(None, false, f32::from_bits(1092616192)).to_bits(),
            1092616192
        );
    }
    #[test]
    fn original_b_initial_height_4() {
        assert_eq!(
            initial_stroke_height(None, false, f32::from_bits(1103626240)).to_bits(),
            1101004800
        );
    }
    #[test]
    fn original_b_initial_height_5() {
        assert_eq!(
            initial_stroke_height(None, false, f32::from_bits(2143289344)).to_bits(),
            1075838976
        );
    }
    #[test]
    fn original_b_initial_height_6() {
        assert_eq!(
            initial_stroke_height(None, true, f32::from_bits(3212836864)).to_bits(),
            1075838976
        );
    }
    #[test]
    fn original_b_initial_height_7() {
        assert_eq!(
            initial_stroke_height(None, true, f32::from_bits(0)).to_bits(),
            1075838976
        );
    }
    #[test]
    fn original_b_initial_height_8() {
        assert_eq!(
            initial_stroke_height(None, true, f32::from_bits(1065353216)).to_bits(),
            1075838976
        );
    }
    #[test]
    fn original_b_initial_height_9() {
        assert_eq!(
            initial_stroke_height(None, true, f32::from_bits(1092616192)).to_bits(),
            1092616192
        );
    }
    #[test]
    fn original_b_initial_height_10() {
        assert_eq!(
            initial_stroke_height(None, true, f32::from_bits(1103626240)).to_bits(),
            1101004800
        );
    }
    #[test]
    fn original_b_initial_height_11() {
        assert_eq!(
            initial_stroke_height(None, true, f32::from_bits(2143289344)).to_bits(),
            1075838976
        );
    }
    #[test]
    fn original_b_initial_height_12() {
        assert_eq!(
            initial_stroke_height(Some(-1), false, f32::from_bits(3212836864)).to_bits(),
            1075838976
        );
    }
    #[test]
    fn original_b_initial_height_13() {
        assert_eq!(
            initial_stroke_height(Some(-1), false, f32::from_bits(0)).to_bits(),
            1075838976
        );
    }
    #[test]
    fn original_b_initial_height_14() {
        assert_eq!(
            initial_stroke_height(Some(-1), false, f32::from_bits(1065353216)).to_bits(),
            1075838976
        );
    }
    #[test]
    fn original_b_initial_height_15() {
        assert_eq!(
            initial_stroke_height(Some(-1), false, f32::from_bits(1092616192)).to_bits(),
            1092616192
        );
    }
    #[test]
    fn original_b_initial_height_16() {
        assert_eq!(
            initial_stroke_height(Some(-1), false, f32::from_bits(1103626240)).to_bits(),
            1101004800
        );
    }
    #[test]
    fn original_b_initial_height_17() {
        assert_eq!(
            initial_stroke_height(Some(-1), false, f32::from_bits(2143289344)).to_bits(),
            1075838976
        );
    }
    #[test]
    fn original_b_initial_height_18() {
        assert_eq!(
            initial_stroke_height(Some(-1), true, f32::from_bits(3212836864)).to_bits(),
            1075838976
        );
    }
    #[test]
    fn original_b_initial_height_19() {
        assert_eq!(
            initial_stroke_height(Some(-1), true, f32::from_bits(0)).to_bits(),
            1075838976
        );
    }
    #[test]
    fn original_b_initial_height_20() {
        assert_eq!(
            initial_stroke_height(Some(-1), true, f32::from_bits(1065353216)).to_bits(),
            1075838976
        );
    }
    #[test]
    fn original_b_initial_height_21() {
        assert_eq!(
            initial_stroke_height(Some(-1), true, f32::from_bits(1092616192)).to_bits(),
            1092616192
        );
    }
    #[test]
    fn original_b_initial_height_22() {
        assert_eq!(
            initial_stroke_height(Some(-1), true, f32::from_bits(1103626240)).to_bits(),
            1101004800
        );
    }
    #[test]
    fn original_b_initial_height_23() {
        assert_eq!(
            initial_stroke_height(Some(-1), true, f32::from_bits(2143289344)).to_bits(),
            1075838976
        );
    }
    #[test]
    fn original_b_initial_height_24() {
        assert_eq!(
            initial_stroke_height(Some(7), false, f32::from_bits(3212836864)).to_bits(),
            1056964608
        );
    }
    #[test]
    fn original_b_initial_height_25() {
        assert_eq!(
            initial_stroke_height(Some(7), false, f32::from_bits(0)).to_bits(),
            1069547520
        );
    }
    #[test]
    fn original_b_initial_height_26() {
        assert_eq!(
            initial_stroke_height(Some(7), false, f32::from_bits(1065353216)).to_bits(),
            1075838976
        );
    }
    #[test]
    fn original_b_initial_height_27() {
        assert_eq!(
            initial_stroke_height(Some(7), false, f32::from_bits(1092616192)).to_bits(),
            1094189056
        );
    }
    #[test]
    fn original_b_initial_height_28() {
        assert_eq!(
            initial_stroke_height(Some(7), false, f32::from_bits(1103626240)).to_bits(),
            1101004800
        );
    }
    #[test]
    fn original_b_initial_height_29() {
        assert_eq!(
            initial_stroke_height(Some(7), false, f32::from_bits(2143289344)).to_bits(),
            0
        );
    }
    #[test]
    fn original_b_initial_height_30() {
        assert_eq!(
            initial_stroke_height(Some(7), true, f32::from_bits(3212836864)).to_bits(),
            0
        );
    }
    #[test]
    fn original_b_initial_height_31() {
        assert_eq!(
            initial_stroke_height(Some(7), true, f32::from_bits(0)).to_bits(),
            0
        );
    }
    #[test]
    fn original_b_initial_height_32() {
        assert_eq!(
            initial_stroke_height(Some(7), true, f32::from_bits(1065353216)).to_bits(),
            1065353216
        );
    }
    #[test]
    fn original_b_initial_height_33() {
        assert_eq!(
            initial_stroke_height(Some(7), true, f32::from_bits(1092616192)).to_bits(),
            1092616192
        );
    }
    #[test]
    fn original_b_initial_height_34() {
        assert_eq!(
            initial_stroke_height(Some(7), true, f32::from_bits(1103626240)).to_bits(),
            1101004800
        );
    }
    #[test]
    fn original_b_initial_height_35() {
        assert_eq!(
            initial_stroke_height(Some(7), true, f32::from_bits(2143289344)).to_bits(),
            0
        );
    }

    #[test]
    fn live_mapped_start_continuation_and_typed_release_preserve_height() {
        use crate::systems::render_loop::compute_passes::mouse_read_terrain_stroke_index::MouseTerrainRaycastSsbo;
        use crate::systems::{
            terrain::register_terrain_events,
            ui::resources::ui_state::{UiInteraction, WidgetId},
        };
        let mut world = World::new();
        register_terrain_events(&mut world);
        register_test_buffers(&mut world);
        world
            .resource_mut::<AssetSsboLibrary>()
            .get_mut::<MouseTerrainRaycastSsbo>()
            .unwrap()
            .mapped[0]
            .terrain_stroke_index = 7;
        world.insert_resource(AppMode::Terrain);
        world.insert_resource(CursorTerrainRaycast {
            tag: 1,
            pos: glam::Vec3::new(3.0, -1.0, 4.0),
            ..Default::default()
        });
        world.insert_resource(TerrainHeightsData::<TerrainHeightSsbo>::new());
        world.insert_resource(TerrainEditingData(
            tg_terrain::stroke::TerrainEditingData::default(),
        ));
        world.insert_resource(GladeBorder(BorderShape::Circle(Circle2d::new(
            Vec2::ZERO,
            100.0,
        ))));
        world.insert_resource(UiState::new(UiInteraction {
            native_tag: 2,
            native_phase: 3,
            widget: WidgetId::clearing_canvas(DRAW_CANVAS_NAME),
        }));
        let mut schedule = Schedule::default();
        schedule.add_systems(ui_draw_terrain);
        schedule.run(&mut world);
        assert_eq!(
            world.resource::<TerrainEditingData>().strokes[&StrokeId(0)].height_ws,
            0.5
        );
        assert_eq!(world.resource::<Events<EvTerrainStrokeChanged>>().len(), 1);
        let placements = &world.resource::<StairsPlacementBucket>().entries;
        assert_eq!(placements.len(), 1);
        assert_eq!(placements[0].tag, 5);
        assert_eq!(placements[0].value_a, 0.5f32.to_bits());
        assert_eq!(world.resource::<TerrainEditUiState>().active, 1);
        assert!(world.resource::<TerrainDrawState>().active);

        let widget = WidgetId::clearing_canvas(DRAW_CANVAS_NAME);
        world
            .resource_mut::<UiState>()
            .replace_interaction(UiInteraction {
                native_tag: 2,
                native_phase: 4,
                widget,
            });
        world.resource_mut::<CursorTerrainRaycast>().pos.x = 4.0;
        schedule.run(&mut world);
        assert_eq!(world.resource::<Events<EvTerrainStrokeChanged>>().len(), 2);
        assert_eq!(world.resource::<TerrainDrawState>().state, 2);
        assert_eq!(
            world.resource::<TerrainEditingData>().strokes[&StrokeId(0)].points,
            [[3.0, -1.0, 4.0], [4.0, -1.0, 4.0]]
        );

        world
            .resource_mut::<UiState>()
            .replace_interaction(UiInteraction {
                native_tag: 2,
                native_phase: 5,
                widget,
            });
        schedule.run(&mut world);
        assert!(!world.resource::<TerrainDrawState>().active);
        assert!(
            matches!(world.resource::<PendingTerrainHistory>().edit.as_ref(),
            Some(TerrainEdit::NewTerrainStroke { index: StrokeId(0), height_ws, stroke_pts })
            if *height_ws == 0.5 && *stroke_pts == [[3.0, -1.0, 4.0], [4.0, -1.0, 4.0]])
        );
        assert_eq!(
            world
                .resource::<StairsPlacementBucket>()
                .entries
                .last()
                .unwrap()
                .tag,
            0
        );
    }
}
