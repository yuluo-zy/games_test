//! B 0x140aef1f0 terrain editing UI. Original4-item jump table0x142ae404c
//! uses min(subtype.wrapping_sub(2),3). The old supposed subtype6/color controls
//! were a Ghidra function-boundary error:0x140af02d0 belongs to grading_lut's
//! serde field visitor, confirmed by original matching PDB/LLVM.
//! The real case2 produces EditTerrainStrokeCmd, as the original FunctionSystem
//! signature states. Code-folded grow_one labels do not identify its event type.
use bevy_ecs::{prelude::*, system::SystemParam};
use glam::{Vec2, Vec3};

use crate::{
    resources::{
        cursor_terrain_raycast::CursorTerrainRaycast,
        mouse_capture::CursorCaptureCmd,
        onboarding::{NotifyHintSystemTerrain, NotifyHintSystemTerrainSteepness},
        terrain_editing::{StrokeId, TerrainEditingData},
        walls::decorator_archivist::LiveDecoratorArchivist,
        window::CursorMovedScreenSpace,
    },
    systems::{
        arrow_gizmo_animation::EnableArrowGizmoStretch,
        backend::input::mod_key::Modifiers,
        camera_rig::main_camera::MainCameraMoved,
        mode_manager::AppMode,
        render_loop::compute_passes::highlight_active_terrain_stroke::AddTerrainHighlightCmd,
        ui::resources::{
            ui_state::UiState,
            widget_params::{WidgetParams, WidgetVolumeShape},
        },
    },
};

use super::{edit_terrain_stroke::EditTerrainStrokeCmd, terrain_hover_idx::HoveredTerrainStrokeId};
use tg_terrain::edit::{Adjustment, EditCommand};

/// Deprecated host compatibility type. Original terrain editing writes the
/// actual CursorCapture/onboarding/highlight events, not WindowFocus.
#[derive(Event, Clone, Copy, Debug, PartialEq, Eq)]
pub struct TerrainWindowFocus(pub u8);

/// Legacy host-only compatibility record. The former claim that terrain
/// native queue writes were stairs records was withdrawn after matching the
/// original FunctionSystem/consumer identities; current terrain no longer uses it.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct StairsPlacementEntry {
    /// +0x08：tag 判别。
    pub tag: u32,
    /// +0x0c：载荷 a。
    pub decorator_a: u32,
    /// +0x10：载荷 b。
    pub decorator_b: u32,
    /// +0x14：载荷 c。
    pub sub: u32,
    /// +0x18：值 a。
    pub value_a: u32,
    /// +0x1c：值 b（ui_edit case 2 = widget id）。
    pub value_b: u32,
    /// +0x20：幅度 / 判别载荷（f32）。
    pub magnitude: f32,
}

/// Deprecated host compatibility owner. Terrain's native records are carried
/// by the actual audio/edit event types; terrain no longer writes this bucket.
#[derive(Resource, Clone, Debug, Default)]
pub struct StairsPlacementBucket {
    pub entries: Vec<StairsPlacementEntry>,
}

/// The native drawing and editing systems share this one owning pending edit.
/// Keep the previous public name as an alias for existing host code.
pub type TerrainEditHistoryPending = super::deferred_terrain_history::PendingTerrainHistory;

/// 地形编辑历史哨兵（O：case 3 `0x8000000000000001`；区别于 deferred 的 `…0002`）。
pub const TERRAIN_EDIT_SENTINEL: u64 = 0x8000_0000_0000_0001;

/// Existing semantic capture owner: previous height/falloff, drag point and
/// steepness accumulator. This is not a native ABI layout. The legacy active
/// and slot_10 fields remain host compatibility only; native+0x1e0 is the real
/// DecoratorArchivist culling policy and is written through that owner.
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct TerrainEditUiState {
    /// case 1 写入的 hover 点/朝向（cursor 命中时 = `CursorTerrainRaycast` 位置；否则 0）。
    pub hover: Vec3,
    /// Deprecated host-only slot; not an original terrain capture field.
    pub slot_10: f32,
    /// Deprecated host compatibility; original+0x1e0 is archivist.culling.allow_any.
    pub active: u8,
    /// 拖动累加器（O：`*pfVar22`；case 1 清 0、case 2 累加 `|dx*0.002|`）。
    pub accum: f32,
    /// Original drag capture: height then falloff are saved before editing.
    pub previous_height_ws: f32,
    pub previous_falloff: f32,
}

/// Deprecated host compatibility table. The old subtype6/8/12/15 interpretation
/// came from an adjacent grading deserializer, not the terrain UI function.
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct TerrainWidgetResponseTable {
    /// `(&UNK_142ae404d)[uVar56]`（uVar56 = subtype-2 截断索引 0..3）。
    pub slots: [u8; 4],
    /// `*pcVar39`（写 0）。
    pub response: u8,
}

/// cursor-moved 位移（O：ui_edit 首事件队列 stride 0x10，payload Vec2 @+8/+0xc =
/// [`CursorMovedScreenSpace`]）。返回最后一帧未读事件的 x/y。
fn last_cursor_delta(reader: &mut EventReader<CursorMovedScreenSpace>) -> Vec2 {
    reader
        .read()
        .last()
        .map(|event| event.0)
        .unwrap_or(Vec2::ZERO)
}

pub fn terrain_drag_case(subtype: u8) -> u8 {
    subtype.wrapping_sub(2).min(3)
}

/// Native0x140aef4d4 preprocess and case2 command writes. Horizontal wins
/// equal abs deltas; zero/NaN length does not enqueue a parameter command.
pub fn edit_command_for_drag(
    id: StrokeId,
    height: f32,
    position: Vec3,
    delta: Vec2,
) -> Option<EditCommand> {
    let length = (delta.y * delta.y + delta.x * delta.x).sqrt();
    if !(0.0 < length) {
        return None;
    }
    let (falloff, height) = if delta.y.abs() <= delta.x.abs() {
        (Adjustment::Delta(delta.x * 0.002), Adjustment::Keep)
    } else {
        let unit_height = height / 20.0;
        let mut v = unit_height * unit_height - (delta.y * 0.02) / 20.0;
        if v < 0.0 {
            v = 0.0;
        }
        if 1.0 < v {
            v = 1.0;
        }
        (
            Adjustment::Keep,
            Adjustment::Delta(v.sqrt() * 20.0 - height),
        )
    };
    Some(EditCommand {
        id,
        falloff,
        height,
        position: position.to_array(),
    })
}

#[derive(SystemParam)]
pub struct TerrainEditEffects<'w> {
    edits: EventWriter<'w, EditTerrainStrokeCmd>,
    capture: EventWriter<'w, CursorCaptureCmd>,
    arrow: EventWriter<'w, EnableArrowGizmoStretch>,
    highlight: EventWriter<'w, AddTerrainHighlightCmd>,
    terrain_hint: EventWriter<'w, NotifyHintSystemTerrain>,
    steepness: EventWriter<'w, NotifyHintSystemTerrainSteepness>,
}

/// `ui_edit_terrain` @0x140aef1f0（逐语句重建）。
#[allow(clippy::too_many_arguments)]
pub fn ui_edit_terrain(
    mode: Res<AppMode>,
    cursor: Res<CursorTerrainRaycast>,
    hovered: Res<HoveredTerrainStrokeId>,
    ui_state: Res<UiState>,
    terrain: Res<TerrainEditingData>,
    mut cursor_moved: EventReader<CursorMovedScreenSpace>,
    mut pending: ResMut<TerrainEditHistoryPending>,
    mut effects: TerrainEditEffects,
    mut ui: ResMut<TerrainEditUiState>,
    modifiers: Res<Modifiers>,
    mut camera_moved: EventReader<MainCameraMoved>,
    mut archivist: ResMut<LiveDecoratorArchivist>,
) {
    if mode.native_tag() != 10 {
        return;
    }
    // hovered 笔画 id（O：`if (*in_stack_00000078 != 0) iVar37 = *(int*)(...+8)`；
    // 资源 tag 非零才读，id@+8）。
    let hovered_id: i32 = if hovered.tag != 0 {
        hovered.id as i32
    } else {
        -1
    };
    // Original arg12 tuple@0: hovered terrain notification tag1.
    if hovered_id != -1 && cursor.tag == 1 {
        effects
            .terrain_hint
            .write(NotifyHintSystemTerrain { tag: 1 });
    }
    // 最后一帧 cursor-moved（O：首事件队列，取最后未读；fVar75/fVar76）。
    let delta = last_cursor_delta(&mut cursor_moved);
    let camera_did_move = camera_moved.read().last().is_some();

    // 逐笔画 widget 注册 + 各自的 subtype switch（O：`do { … } while (puVar48 != 0)`；
    // `UiState::add` 返回值即该 widget 的上下文）。
    let stroke_ids: Vec<StrokeId> = terrain.strokes.keys().copied().collect();
    for id in stroke_ids {
        let ty = if !modifiers.disable_snapping() {
            (hovered_id != id.0 as i32) as u8 * 3 + 4
        } else {
            // Original modifier byte0 disables the hovered-ID edit shape.
            7
        };
        let widget = ui_state.add(
            &format!("terrain_edit_{}__canvas", id.0),
            WidgetParams::new(
                WidgetVolumeShape {
                    tag: ty,
                    layer: 0,
                    size: Vec2::splat(130.0),
                },
                [0; 2],
            ),
        );
        if widget.clicked() && hovered_id != -1 && !camera_did_move {
            effects.highlight.write(AddTerrainHighlightCmd(hovered_id));
        }
        let subtype = widget.dragged();
        match terrain_drag_case(subtype) {
            1 => {
                // Capture cursor, cache current parameters/point, allow decorator culling.
                effects.capture.write(CursorCaptureCmd(false));
                ui.hover = cursor.position();
                if let Some(stroke) = terrain.strokes.get(&id) {
                    ui.previous_height_ws = stroke.height_ws;
                    ui.previous_falloff = stroke.falloff;
                }
                archivist.decorator_cull_allow_any();
                ui.accum = 0.0;
            }
            2 => {
                effects.arrow.write(EnableArrowGizmoStretch);
                if let Some(stroke) = terrain.strokes.get(&id) {
                    if let Some(command) =
                        edit_command_for_drag(id, stroke.height_ws, ui.hover, delta)
                    {
                        effects.edits.write(EditTerrainStrokeCmd(command));
                        if delta.y.abs() <= delta.x.abs() {
                            ui.accum += (delta.x * 0.002).abs();
                            if 0.3 < ui.accum {
                                effects
                                    .steepness
                                    .write(NotifyHintSystemTerrainSteepness { tag: 1 });
                            }
                        }
                    }
                } else {
                    log::error!(
                        "Was requested to edit a terrain stroke {} that doesnt exist",
                        id.0
                    );
                }
            }
            3 => {
                effects.capture.write(CursorCaptureCmd(true));
                effects
                    .steepness
                    .write(NotifyHintSystemTerrainSteepness { tag: 0 });
                let stroke = terrain
                    .strokes
                    .get(&id)
                    .expect("original terrain editing widget stroke assertion");
                pending.stage(tg_terrain::schema::TerrainEdit::EditTerrainStroke {
                    index: id,
                    prev_height_ws: ui.previous_height_ws,
                    new_height_ws: stroke.height_ws,
                    prev_falloff: ui.previous_falloff,
                    new_falloff: stroke.falloff,
                });
            }
            _ => {
                // case 0（subtype 2）及其余：无动作。
            }
        }
    }
}

#[cfg(test)]
mod original_edit_producer_tests {
    use super::*;
    #[test]
    fn original_dispatch_all_bytes() {
        let expected: [u8; 256] = [
            3, 3, 0, 1, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
            3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
            3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
            3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
            3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
            3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
            3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
            3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
            3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
        ];
        for (value, expected) in expected.into_iter().enumerate() {
            assert_eq!(terrain_drag_case(value as u8), expected);
        }
    }
    #[test]
    fn original_case2_0() {
        let output = edit_command_for_drag(
            StrokeId(17),
            f32::from_bits(0),
            Vec3::new(1., 2., 3.),
            Vec2::new(f32::from_bits(0), f32::from_bits(0)),
        );
        assert!(output.is_none());
    }
    #[test]
    fn original_case2_1() {
        let output = edit_command_for_drag(
            StrokeId(17),
            f32::from_bits(1075838976),
            Vec3::new(1., 2., 3.),
            Vec2::new(f32::from_bits(0), f32::from_bits(0)),
        );
        assert!(output.is_none());
    }
    #[test]
    fn original_case2_2() {
        let output = edit_command_for_drag(
            StrokeId(17),
            f32::from_bits(1101004800),
            Vec3::new(1., 2., 3.),
            Vec2::new(f32::from_bits(0), f32::from_bits(0)),
        );
        assert!(output.is_none());
    }
    #[test]
    fn original_case2_3() {
        let output = edit_command_for_drag(
            StrokeId(17),
            f32::from_bits(0),
            Vec3::new(1., 2., 3.),
            Vec2::new(f32::from_bits(1092616192), f32::from_bits(1065353216)),
        );
        let output = output.unwrap();
        assert_eq!(output.id, StrokeId(17));
        assert_eq!(
            output.position.map(f32::to_bits),
            [1065353216, 1073741824, 1077936128]
        );
        let Adjustment::Delta(value) = output.falloff else {
            panic!("expected native Delta");
        };
        assert_eq!(value.to_bits(), 1017370379);
        assert_eq!(output.height, Adjustment::Keep);
    }
    #[test]
    fn original_case2_4() {
        let output = edit_command_for_drag(
            StrokeId(17),
            f32::from_bits(1075838976),
            Vec3::new(1., 2., 3.),
            Vec2::new(f32::from_bits(1092616192), f32::from_bits(1065353216)),
        );
        let output = output.unwrap();
        assert_eq!(output.id, StrokeId(17));
        assert_eq!(
            output.position.map(f32::to_bits),
            [1065353216, 1073741824, 1077936128]
        );
        let Adjustment::Delta(value) = output.falloff else {
            panic!("expected native Delta");
        };
        assert_eq!(value.to_bits(), 1017370379);
        assert_eq!(output.height, Adjustment::Keep);
    }
    #[test]
    fn original_case2_5() {
        let output = edit_command_for_drag(
            StrokeId(17),
            f32::from_bits(1101004800),
            Vec3::new(1., 2., 3.),
            Vec2::new(f32::from_bits(1092616192), f32::from_bits(1065353216)),
        );
        let output = output.unwrap();
        assert_eq!(output.id, StrokeId(17));
        assert_eq!(
            output.position.map(f32::to_bits),
            [1065353216, 1073741824, 1077936128]
        );
        let Adjustment::Delta(value) = output.falloff else {
            panic!("expected native Delta");
        };
        assert_eq!(value.to_bits(), 1017370379);
        assert_eq!(output.height, Adjustment::Keep);
    }
    #[test]
    fn original_case2_6() {
        let output = edit_command_for_drag(
            StrokeId(17),
            f32::from_bits(0),
            Vec3::new(1., 2., 3.),
            Vec2::new(f32::from_bits(3240099840), f32::from_bits(1065353216)),
        );
        let output = output.unwrap();
        assert_eq!(output.id, StrokeId(17));
        assert_eq!(
            output.position.map(f32::to_bits),
            [1065353216, 1073741824, 1077936128]
        );
        let Adjustment::Delta(value) = output.falloff else {
            panic!("expected native Delta");
        };
        assert_eq!(value.to_bits(), 3164854027);
        assert_eq!(output.height, Adjustment::Keep);
    }
    #[test]
    fn original_case2_7() {
        let output = edit_command_for_drag(
            StrokeId(17),
            f32::from_bits(1075838976),
            Vec3::new(1., 2., 3.),
            Vec2::new(f32::from_bits(3240099840), f32::from_bits(1065353216)),
        );
        let output = output.unwrap();
        assert_eq!(output.id, StrokeId(17));
        assert_eq!(
            output.position.map(f32::to_bits),
            [1065353216, 1073741824, 1077936128]
        );
        let Adjustment::Delta(value) = output.falloff else {
            panic!("expected native Delta");
        };
        assert_eq!(value.to_bits(), 3164854027);
        assert_eq!(output.height, Adjustment::Keep);
    }
    #[test]
    fn original_case2_8() {
        let output = edit_command_for_drag(
            StrokeId(17),
            f32::from_bits(1101004800),
            Vec3::new(1., 2., 3.),
            Vec2::new(f32::from_bits(3240099840), f32::from_bits(1065353216)),
        );
        let output = output.unwrap();
        assert_eq!(output.id, StrokeId(17));
        assert_eq!(
            output.position.map(f32::to_bits),
            [1065353216, 1073741824, 1077936128]
        );
        let Adjustment::Delta(value) = output.falloff else {
            panic!("expected native Delta");
        };
        assert_eq!(value.to_bits(), 3164854027);
        assert_eq!(output.height, Adjustment::Keep);
    }
    #[test]
    fn original_case2_9() {
        let output = edit_command_for_drag(
            StrokeId(17),
            f32::from_bits(0),
            Vec3::new(1., 2., 3.),
            Vec2::new(f32::from_bits(1065353216), f32::from_bits(1092616192)),
        );
        let output = output.unwrap();
        assert_eq!(output.id, StrokeId(17));
        assert_eq!(
            output.position.map(f32::to_bits),
            [1065353216, 1073741824, 1077936128]
        );
        assert_eq!(output.falloff, Adjustment::Keep);
        let Adjustment::Delta(value) = output.height else {
            panic!("expected native Delta");
        };
        assert_eq!(value.to_bits(), 0);
    }
    #[test]
    fn original_case2_10() {
        let output = edit_command_for_drag(
            StrokeId(17),
            f32::from_bits(1075838976),
            Vec3::new(1., 2., 3.),
            Vec2::new(f32::from_bits(1065353216), f32::from_bits(1092616192)),
        );
        let output = output.unwrap();
        assert_eq!(output.id, StrokeId(17));
        assert_eq!(
            output.position.map(f32::to_bits),
            [1065353216, 1073741824, 1077936128]
        );
        assert_eq!(output.falloff, Adjustment::Keep);
        let Adjustment::Delta(value) = output.height else {
            panic!("expected native Delta");
        };
        assert_eq!(value.to_bits(), 3212836864);
    }
    #[test]
    fn original_case2_11() {
        let output = edit_command_for_drag(
            StrokeId(17),
            f32::from_bits(1101004800),
            Vec3::new(1., 2., 3.),
            Vec2::new(f32::from_bits(1065353216), f32::from_bits(1092616192)),
        );
        let output = output.unwrap();
        assert_eq!(output.id, StrokeId(17));
        assert_eq!(
            output.position.map(f32::to_bits),
            [1065353216, 1073741824, 1077936128]
        );
        assert_eq!(output.falloff, Adjustment::Keep);
        let Adjustment::Delta(value) = output.height else {
            panic!("expected native Delta");
        };
        assert_eq!(value.to_bits(), 3184349440);
    }
    #[test]
    fn original_case2_12() {
        let output = edit_command_for_drag(
            StrokeId(17),
            f32::from_bits(0),
            Vec3::new(1., 2., 3.),
            Vec2::new(f32::from_bits(1065353216), f32::from_bits(3240099840)),
        );
        let output = output.unwrap();
        assert_eq!(output.id, StrokeId(17));
        assert_eq!(
            output.position.map(f32::to_bits),
            [1065353216, 1073741824, 1077936128]
        );
        assert_eq!(output.falloff, Adjustment::Keep);
        let Adjustment::Delta(value) = output.height else {
            panic!("expected native Delta");
        };
        assert_eq!(value.to_bits(), 1073741824);
    }
    #[test]
    fn original_case2_13() {
        let output = edit_command_for_drag(
            StrokeId(17),
            f32::from_bits(1075838976),
            Vec3::new(1., 2., 3.),
            Vec2::new(f32::from_bits(1065353216), f32::from_bits(3240099840)),
        );
        let output = output.unwrap();
        assert_eq!(output.id, StrokeId(17));
        assert_eq!(
            output.position.map(f32::to_bits),
            [1065353216, 1073741824, 1077936128]
        );
        assert_eq!(output.falloff, Adjustment::Keep);
        let Adjustment::Delta(value) = output.height else {
            panic!("expected native Delta");
        };
        assert_eq!(value.to_bits(), 1060346260);
    }
    #[test]
    fn original_case2_14() {
        let output = edit_command_for_drag(
            StrokeId(17),
            f32::from_bits(1101004800),
            Vec3::new(1., 2., 3.),
            Vec2::new(f32::from_bits(1065353216), f32::from_bits(3240099840)),
        );
        let output = output.unwrap();
        assert_eq!(output.id, StrokeId(17));
        assert_eq!(
            output.position.map(f32::to_bits),
            [1065353216, 1073741824, 1077936128]
        );
        assert_eq!(output.falloff, Adjustment::Keep);
        let Adjustment::Delta(value) = output.height else {
            panic!("expected native Delta");
        };
        assert_eq!(value.to_bits(), 0);
    }
    #[test]
    fn original_case2_15() {
        let output = edit_command_for_drag(
            StrokeId(17),
            f32::from_bits(0),
            Vec3::new(1., 2., 3.),
            Vec2::new(f32::from_bits(1073741824), f32::from_bits(1073741824)),
        );
        let output = output.unwrap();
        assert_eq!(output.id, StrokeId(17));
        assert_eq!(
            output.position.map(f32::to_bits),
            [1065353216, 1073741824, 1077936128]
        );
        let Adjustment::Delta(value) = output.falloff else {
            panic!("expected native Delta");
        };
        assert_eq!(value.to_bits(), 998445679);
        assert_eq!(output.height, Adjustment::Keep);
    }
    #[test]
    fn original_case2_16() {
        let output = edit_command_for_drag(
            StrokeId(17),
            f32::from_bits(1075838976),
            Vec3::new(1., 2., 3.),
            Vec2::new(f32::from_bits(1073741824), f32::from_bits(1073741824)),
        );
        let output = output.unwrap();
        assert_eq!(output.id, StrokeId(17));
        assert_eq!(
            output.position.map(f32::to_bits),
            [1065353216, 1073741824, 1077936128]
        );
        let Adjustment::Delta(value) = output.falloff else {
            panic!("expected native Delta");
        };
        assert_eq!(value.to_bits(), 998445679);
        assert_eq!(output.height, Adjustment::Keep);
    }
    #[test]
    fn original_case2_17() {
        let output = edit_command_for_drag(
            StrokeId(17),
            f32::from_bits(1101004800),
            Vec3::new(1., 2., 3.),
            Vec2::new(f32::from_bits(1073741824), f32::from_bits(1073741824)),
        );
        let output = output.unwrap();
        assert_eq!(output.id, StrokeId(17));
        assert_eq!(
            output.position.map(f32::to_bits),
            [1065353216, 1073741824, 1077936128]
        );
        let Adjustment::Delta(value) = output.falloff else {
            panic!("expected native Delta");
        };
        assert_eq!(value.to_bits(), 998445679);
        assert_eq!(output.height, Adjustment::Keep);
    }
    #[test]
    fn ui_edit_dispatch_reaches_the_existing_parameter_consumer() {
        use crate::systems::terrain::{
            delayed_terrain_changed_msg::ScheduledTerrainReadback,
            edit_terrain_stroke::edit_terrain_stroke, register_terrain_events,
        };
        use crate::systems::ui::resources::ui_state::{UiInteraction, WidgetId};
        let mut world = World::new();
        register_terrain_events(&mut world);
        world.insert_resource(AppMode::Terrain);
        world.insert_resource(CursorTerrainRaycast::default());
        let mut data = tg_terrain::stroke::TerrainEditingData::default();
        data.new_stroke([0.; 3]).unwrap();
        world.insert_resource(TerrainEditingData(data));
        world.insert_resource(UiState::new(UiInteraction {
            native_tag: 2,
            native_phase: 4,
            widget: WidgetId::from_name("terrain_edit_0__canvas"),
        }));
        world.insert_resource(ScheduledTerrainReadback::default());
        world.insert_resource(crate::resources::time::GameTime::default());
        world.resource_mut::<TerrainEditUiState>().hover = Vec3::new(1., 2., 3.);
        world.send_event(CursorMovedScreenSpace(Vec2::new(10., 1.)));
        let mut schedule = Schedule::default();
        schedule.add_systems((ui_edit_terrain, edit_terrain_stroke).chain());
        schedule.run(&mut world);
        assert_eq!(world.resource::<Events<EditTerrainStrokeCmd>>().len(), 1);
        assert!(world.resource::<TerrainEditingData>().strokes[&StrokeId(0)].falloff > 0.4);
        assert_eq!(
            world
                .resource::<ScheduledTerrainReadback>()
                .pending
                .load(std::sync::atomic::Ordering::Relaxed),
            1
        );

        let widget = WidgetId::from_name("terrain_edit_0__canvas");
        world
            .resource_mut::<UiState>()
            .replace_interaction(UiInteraction {
                native_tag: 2,
                native_phase: 3,
                widget,
            });
        schedule.run(&mut world);
        assert!(world.resource::<LiveDecoratorArchivist>().culling.allow_any);
        assert_eq!(world.resource::<TerrainEditUiState>().active, 0);
        world
            .resource_mut::<UiState>()
            .replace_interaction(UiInteraction {
                native_tag: 2,
                native_phase: 4,
                widget,
            });
        world.send_event(CursorMovedScreenSpace(Vec2::new(200., 0.)));
        schedule.run(&mut world);
        world
            .resource_mut::<UiState>()
            .replace_interaction(UiInteraction {
                native_tag: 2,
                native_phase: 5,
                widget,
            });
        schedule.run(&mut world);
        let capture = world.resource::<Events<CursorCaptureCmd>>();
        let mut reader = capture.get_cursor();
        assert_eq!(
            reader
                .read(capture)
                .map(|event| event.0)
                .collect::<Vec<_>>(),
            [false, true]
        );
        let steep = world.resource::<Events<NotifyHintSystemTerrainSteepness>>();
        let mut reader = steep.get_cursor();
        assert_eq!(
            reader
                .read(steep)
                .map(|event| event.tag)
                .collect::<Vec<_>>(),
            [1, 0]
        );
        assert_eq!(world.resource::<Events<EnableArrowGizmoStretch>>().len(), 2);
        assert!(matches!(
            world.resource::<TerrainEditHistoryPending>().edit.as_ref(),
            Some(tg_terrain::schema::TerrainEdit::EditTerrainStroke {
                index: StrokeId(0),
                ..
            })
        ));
    }
}
