//! B terrain draw state machine. Exact original SystemParam ports supersede
//! folded grow_one aliases: water is WaterRaster, +1e0 belongs to the existing
//! DecoratorArchivist culling policy, release notifies onboarding/tool usage,
//! and state1 sends actual PlayTerrainSound tag5 with pre-clamp desired height.
use bevy_ecs::{prelude::*, system::SystemParam};
use glam::Vec2;
use utils::geometry::Circle2d;

use crate::{
    asset_libraries::ssbo_library::AssetSsboLibrary,
    resources::{
        cursor_terrain_raycast::CursorTerrainRaycast,
        glade_border::{BorderShape, DisplayOutOfBorder, FeedbackInputOutOfBorder, GladeBorder},
        onboarding::{NotifyHintSystemTerrain, NotifyHintSystemToolWasJustUsed},
        terrain_editing::{StrokeId, TerrainEditingData},
        terrain_rock_mask::{TerrainRockMaskSsbo, sample_at_world_xz},
        walls::decorator_archivist::LiveDecoratorArchivist,
        world_raster_defs::WaterRaster,
    },
    systems::{
        mode_manager::AppMode,
        terrain::{
            PlayTerrainSound,
            deferred_terrain_history::{PendingTerrainHistory, TerrainEdit},
            draw_terrain_stroke::EvTerrainStrokeChanged,
        },
        ui::resources::ui_state::UiState,
    },
};

/// 距离门阈值（O：`_UNK_142925870` = 0.5）。
pub const FOCUS_DISTANCE_GATE: f32 = 0.5;
/// 边界检测圆半径（O：反汇编 `MOVSS XMM3,[0x142a16cb8]` = 0.8）。
pub const BORDER_CHECK_RADIUS: f32 = 0.8;
/// B caller passes name length17 in R8, argument0 in R9 and layer1 on stack.
pub const DENY_CANVAS_ARGUMENT: u8 = 0;
pub const DENY_CANVAS_LAYER: u8 = 1;
/// B caller passes name length12 in R8, argument3 in R9 and layer2 on stack.
pub const DRAW_CANVAS_ARGUMENT: u8 = 3;
pub const DRAW_CANVAS_LAYER: u8 = 2;
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

#[derive(SystemParam)]
pub struct TerrainDrawEffects<'w> {
    changed: EventWriter<'w, EvTerrainStrokeChanged>,
    sounds: EventWriter<'w, PlayTerrainSound>,
    terrain_hint: EventWriter<'w, NotifyHintSystemTerrain>,
    used: EventWriter<'w, NotifyHintSystemToolWasJustUsed>,
    outside: EventWriter<'w, DisplayOutOfBorder>,
    feedback: EventWriter<'w, FeedbackInputOutOfBorder>,
}

/// `ui_draw_terrain` @0x140a9fdc0（逐语句重建）。
#[allow(clippy::too_many_arguments)]
pub fn ui_draw_terrain(
    mode: Res<AppMode>,
    hover: Res<CursorTerrainRaycast>,
    water: Res<WaterRaster>,
    border: Res<GladeBorder>,
    ui_state: Res<UiState>,
    mut editing: ResMut<TerrainEditingData>,
    mut effects: TerrainDrawEffects,
    mut draw_state: ResMut<TerrainDrawState>,
    mut pending: ResMut<PendingTerrainHistory>,
    assets: Option<Res<AssetSsboLibrary>>,
    mut archivist: ResMut<LiveDecoratorArchivist>,
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
    let height = water
        .0
        .as_ref()
        .bilinear_sample_at_world_xz(cursor.x, cursor.y, 130.0, 130.0);
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
        effects.outside.write(DisplayOutOfBorder);
        // 第二条：`dragged() != 2`（disasm 0x140a9ffec..0x140a9fffc）。
        if widget.dragged() != 2 {
            effects.feedback.write(FeedbackInputOutOfBorder);
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
                // B aa010f..aa011d: preserve the raw drag distance before any
                // SSBO lookup or clamp/addition for the actual stroke height.
                editing.cursor_height_override = Some(draw_state.drag_distance);
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
                archivist.decorator_cull_allow_any();
                effects
                    .changed
                    .write(EvTerrainStrokeChanged(StrokeId(id.0)));
                effects.sounds.write(PlayTerrainSound::CreateChunk {
                    position,
                    height: initial_stroke_desired_height(
                        mouse_index,
                        on_rock,
                        draw_state.drag_distance,
                    ),
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
                effects.changed.write(EvTerrainStrokeChanged(id));
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
                effects
                    .terrain_hint
                    .write(NotifyHintSystemTerrain { tag: 0 });
                effects.used.write(NotifyHintSystemToolWasJustUsed);
            }
            // B aa02e1 jumps here when get_last_mut_stroke is missing too.
            // aa06eb clears the same editing owner's Option after release events.
            editing.cursor_height_override = None;
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
        world.insert_resource(WaterRaster::new(
            crate::resources::world_raster::DirtyTilePublisher::unique(),
        ));
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
        world.insert_resource(WaterRaster::new(
            crate::resources::world_raster::DirtyTilePublisher::unique(),
        ));
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
        let mut history: History<()> = History::<()>::from_observed(ObservedHistoryLedger {
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
        let recorded = &world.resource::<History<()>>().pending().unwrap().edits;
        assert_eq!(recorded.len(), 1);
        assert!(
            matches!(&recorded[0], EditType::Terrain(edit) if matches!(edit.as_ref(), TerrainEdit::NewTerrainStroke { index: StrokeId(0), height_ws, stroke_pts } if *height_ws == 2.5 && *stroke_pts == [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]))
        );
        schedule.run(&mut world);
        assert_eq!(
            world.resource::<History<()>>().pending().unwrap().edits.len(),
            1
        );
    }
}

/// B0x140aa0138..0x140aa0534 initial height policy. Equal signed-zero values
/// choose the drag value, and NaN chooses the finite minimum as in MAXSS.
pub fn initial_stroke_desired_height(
    mouse_index: Option<i32>,
    on_rock: bool,
    drag_height: f32,
) -> f32 {
    if mouse_index.is_some_and(|id| id != -1) && !on_rock {
        drag_height + 1.5
    } else {
        drag_height
    }
}

pub fn initial_stroke_height(mouse_index: Option<i32>, on_rock: bool, drag_height: f32) -> f32 {
    let existing = mouse_index.is_some_and(|index| index != -1);
    let minimum = if existing { 0.0 } else { 2.5 };
    let desired = initial_stroke_desired_height(mouse_index, on_rock, drag_height);
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
        world.insert_resource(WaterRaster::new(
            crate::resources::world_raster::DirtyTilePublisher::unique(),
        ));
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
        let events = world.resource::<Events<PlayTerrainSound>>();
        let mut reader = events.get_cursor();
        assert!(
            matches!(reader.read(events).last(),Some(PlayTerrainSound::CreateChunk{height,..})if *height==0.5)
        );
        assert!(world.resource::<LiveDecoratorArchivist>().culling.allow_any);
        assert!(world.resource::<TerrainDrawState>().active);
        assert_eq!(world.resource::<TerrainEditingData>().cursor_height_override, Some(-1.0));

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
        assert_eq!(world.resource::<TerrainEditingData>().cursor_height_override, Some(-1.0));
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
        assert_eq!(world.resource::<TerrainEditingData>().cursor_height_override, None);
        assert!(
            matches!(world.resource::<PendingTerrainHistory>().edit.as_ref(),
            Some(TerrainEdit::NewTerrainStroke { index: StrokeId(0), height_ws, stroke_pts })
            if *height_ws == 0.5 && *stroke_pts == [[3.0, -1.0, 4.0], [4.0, -1.0, 4.0]])
        );
        assert_eq!(world.resource::<Events<NotifyHintSystemTerrain>>().len(), 1);
        assert_eq!(
            world
                .resource::<Events<NotifyHintSystemToolWasJustUsed>>()
                .len(),
            1
        );
    }
}

#[cfg(test)]
mod original_audio_height_tests {
    use super::*;
    #[test]
    fn original_state1_audio_uses_pre_clamp_desired_bits() {
        assert_eq!(
            initial_stroke_desired_height(None, false, f32::from_bits(3212836864)).to_bits(),
            3212836864
        );
        assert_eq!(
            initial_stroke_desired_height(None, false, f32::from_bits(0)).to_bits(),
            0
        );
        assert_eq!(
            initial_stroke_desired_height(None, false, f32::from_bits(1065353216)).to_bits(),
            1065353216
        );
        assert_eq!(
            initial_stroke_desired_height(None, false, f32::from_bits(1092616192)).to_bits(),
            1092616192
        );
        assert_eq!(
            initial_stroke_desired_height(None, false, f32::from_bits(1103626240)).to_bits(),
            1103626240
        );
        assert_eq!(
            initial_stroke_desired_height(None, false, f32::from_bits(2143289344)).to_bits(),
            2143289344
        );
        assert_eq!(
            initial_stroke_desired_height(None, true, f32::from_bits(3212836864)).to_bits(),
            3212836864
        );
        assert_eq!(
            initial_stroke_desired_height(None, true, f32::from_bits(0)).to_bits(),
            0
        );
        assert_eq!(
            initial_stroke_desired_height(None, true, f32::from_bits(1065353216)).to_bits(),
            1065353216
        );
        assert_eq!(
            initial_stroke_desired_height(None, true, f32::from_bits(1092616192)).to_bits(),
            1092616192
        );
        assert_eq!(
            initial_stroke_desired_height(None, true, f32::from_bits(1103626240)).to_bits(),
            1103626240
        );
        assert_eq!(
            initial_stroke_desired_height(None, true, f32::from_bits(2143289344)).to_bits(),
            2143289344
        );
        assert_eq!(
            initial_stroke_desired_height(Some(-1), false, f32::from_bits(3212836864)).to_bits(),
            3212836864
        );
        assert_eq!(
            initial_stroke_desired_height(Some(-1), false, f32::from_bits(0)).to_bits(),
            0
        );
        assert_eq!(
            initial_stroke_desired_height(Some(-1), false, f32::from_bits(1065353216)).to_bits(),
            1065353216
        );
        assert_eq!(
            initial_stroke_desired_height(Some(-1), false, f32::from_bits(1092616192)).to_bits(),
            1092616192
        );
        assert_eq!(
            initial_stroke_desired_height(Some(-1), false, f32::from_bits(1103626240)).to_bits(),
            1103626240
        );
        assert_eq!(
            initial_stroke_desired_height(Some(-1), false, f32::from_bits(2143289344)).to_bits(),
            2143289344
        );
        assert_eq!(
            initial_stroke_desired_height(Some(-1), true, f32::from_bits(3212836864)).to_bits(),
            3212836864
        );
        assert_eq!(
            initial_stroke_desired_height(Some(-1), true, f32::from_bits(0)).to_bits(),
            0
        );
        assert_eq!(
            initial_stroke_desired_height(Some(-1), true, f32::from_bits(1065353216)).to_bits(),
            1065353216
        );
        assert_eq!(
            initial_stroke_desired_height(Some(-1), true, f32::from_bits(1092616192)).to_bits(),
            1092616192
        );
        assert_eq!(
            initial_stroke_desired_height(Some(-1), true, f32::from_bits(1103626240)).to_bits(),
            1103626240
        );
        assert_eq!(
            initial_stroke_desired_height(Some(-1), true, f32::from_bits(2143289344)).to_bits(),
            2143289344
        );
        assert_eq!(
            initial_stroke_desired_height(Some(7), false, f32::from_bits(3212836864)).to_bits(),
            1056964608
        );
        assert_eq!(
            initial_stroke_desired_height(Some(7), false, f32::from_bits(0)).to_bits(),
            1069547520
        );
        assert_eq!(
            initial_stroke_desired_height(Some(7), false, f32::from_bits(1065353216)).to_bits(),
            1075838976
        );
        assert_eq!(
            initial_stroke_desired_height(Some(7), false, f32::from_bits(1092616192)).to_bits(),
            1094189056
        );
        assert_eq!(
            initial_stroke_desired_height(Some(7), false, f32::from_bits(1103626240)).to_bits(),
            1104412672
        );
        assert_eq!(
            initial_stroke_desired_height(Some(7), false, f32::from_bits(2143289344)).to_bits(),
            2143289344
        );
        assert_eq!(
            initial_stroke_desired_height(Some(7), true, f32::from_bits(3212836864)).to_bits(),
            3212836864
        );
        assert_eq!(
            initial_stroke_desired_height(Some(7), true, f32::from_bits(0)).to_bits(),
            0
        );
        assert_eq!(
            initial_stroke_desired_height(Some(7), true, f32::from_bits(1065353216)).to_bits(),
            1065353216
        );
        assert_eq!(
            initial_stroke_desired_height(Some(7), true, f32::from_bits(1092616192)).to_bits(),
            1092616192
        );
        assert_eq!(
            initial_stroke_desired_height(Some(7), true, f32::from_bits(1103626240)).to_bits(),
            1103626240
        );
        assert_eq!(
            initial_stroke_desired_height(Some(7), true, f32::from_bits(2143289344)).to_bits(),
            2143289344
        );
    }
}

#[cfg(test)]
mod native_canvas_argument_tests {
    use super::*;
    use crate::systems::{
        terrain::register_terrain_events,
        ui::resources::ui_state::{UiInteraction, WidgetId},
    };

    #[test]
    fn actual_terrain_ui_uses_original_argument_and_layer_ports() {
        // Original callers 140a9fefb..ff49 and constructor 14092df72/e0bd/e0ca:
        // R8 is the string length. R9 and stack are distinct argument/layer.
        for (wet, hit, distance, deny_expected) in [
            (false, false, 0.0, false),
            (true, false, 0.0, true),
            (true, true, 0.5, true),
            (true, true, 0.6, false),
        ] {
            let mut world = World::new();
            register_terrain_events(&mut world);
            world.insert_resource(AppMode::Terrain);
            world.insert_resource(CursorTerrainRaycast {
                tag: u32::from(hit),
                pos: glam::Vec3::new(0.0, distance, 0.0),
                ..Default::default()
            });
            let mut water =
                WaterRaster::new(crate::resources::world_raster::DirtyTilePublisher::unique());
            if wet {
                water.write_composite(Some(&vec![255; 512 * 512]));
            }
            world.insert_resource(water);
            world.insert_resource(TerrainEditingData(
                tg_terrain::stroke::TerrainEditingData::default(),
            ));
            world.insert_resource(GladeBorder(BorderShape::Circle(Circle2d::new(
                Vec2::ZERO,
                100.0,
            ))));
            world.insert_resource(UiState::new(UiInteraction {
                native_tag: 0,
                native_phase: 2,
                widget: WidgetId::clearing_canvas(DRAW_CANVAS_NAME),
            }));
            let mut schedule = Schedule::default();
            schedule.add_systems(ui_draw_terrain);
            schedule.run(&mut world);
            let ui = world.resource::<UiState>();
            let draw = ui
                .clearing_canvas_params(WidgetId::clearing_canvas(DRAW_CANVAS_NAME))
                .unwrap();
            assert_eq!((draw.argument_79, draw.layer), (3, 2));
            let deny = ui.clearing_canvas_params(WidgetId::clearing_canvas(DENY_CANVAS_NAME));
            assert_eq!(deny.is_some(), deny_expected);
            if let Some(deny) = deny {
                assert_eq!((deny.argument_79, deny.layer), (0, 1));
            }
        }
    }
}

#[cfg(test)]
mod cursor_plane_lifecycle_tests {
    use super::*;
    use crate::systems::{
        terrain::register_terrain_events,
        ui::resources::ui_state::{UiInteraction, WidgetId},
    };

    fn world(distance: f32, phase: u8) -> World {
        let mut world = World::new();
        register_terrain_events(&mut world);
        world.insert_resource(AppMode::Terrain);
        world.insert_resource(CursorTerrainRaycast {
            tag: 1,
            pos: glam::Vec3::new(0.0, distance, 0.0),
            ..Default::default()
        });
        world.insert_resource(WaterRaster::new(
            crate::resources::world_raster::DirtyTilePublisher::unique(),
        ));
        world.insert_resource(TerrainEditingData(
            tg_terrain::stroke::TerrainEditingData::default(),
        ));
        world.insert_resource(GladeBorder(BorderShape::Circle(Circle2d::new(
            Vec2::ZERO, 100.0,
        ))));
        world.insert_resource(UiState::new(UiInteraction {
            native_tag: 2,
            native_phase: phase,
            widget: WidgetId::clearing_canvas(DRAW_CANVAS_NAME),
        }));
        world
    }

    #[test]
    fn start_sets_raw_plane_before_a_later_missing_ssbo_failure() {
        for bits in [(-1.0_f32).to_bits(), 25.0_f32.to_bits(), 0x8000_0000, 0x7fc0_1234] {
            let mut world = world(f32::from_bits(bits), 3);
            let mut schedule = Schedule::default();
            schedule.add_systems(ui_draw_terrain);
            // The original mutates the plane first; preserve that observable
            // mutation even when the later required library/SSBO read fails.
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                schedule.run(&mut world);
            }));
            assert!(result.is_err());
            let editing = world.resource::<TerrainEditingData>();
            assert_eq!(editing.cursor_height_override.map(f32::to_bits), Some(bits));
            assert_eq!(editing.strokes[&StrokeId(0)].height_ws, 2.5);
        }
    }

    #[test]
    fn rejected_creation_leaves_the_previous_plane_unchanged() {
        let mut world = world(12.0, 3);
        {
            let mut editing = world.resource_mut::<TerrainEditingData>();
            editing.next_index = tg_terrain::stroke::ID_COUNTER_LIMIT;
            editing.cursor_height_override = Some(7.25);
        }
        let mut schedule = Schedule::default();
        schedule.add_systems(ui_draw_terrain);
        schedule.run(&mut world); // no assets: the rejected creation must skip the read.
        assert_eq!(world.resource::<TerrainEditingData>().cursor_height_override, Some(7.25));
        assert!(world.resource::<TerrainEditingData>().strokes.is_empty());
    }

    #[test]
    fn release_without_a_last_stroke_still_clears_the_plane() {
        let mut world = world(12.0, 5);
        world.resource_mut::<TerrainDrawState>().state = 2;
        world.resource_mut::<TerrainEditingData>().cursor_height_override = Some(7.25);
        let mut schedule = Schedule::default();
        schedule.add_systems(ui_draw_terrain);
        schedule.run(&mut world);
        assert_eq!(world.resource::<TerrainEditingData>().cursor_height_override, None);
        assert!(world.resource::<Events<NotifyHintSystemTerrain>>().is_empty());
        assert!(world.resource::<Events<NotifyHintSystemToolWasJustUsed>>().is_empty());
    }
}
