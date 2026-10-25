//! B141a18dd0/3254B and141a19a90/4228B. Current owned widgets and canonical
//! events replace native Vec/Arc/queue layouts; all game branches remain.
use bevy_ecs::{prelude::*, system::SystemParam};
use country_core::{
    resources::{
        history::{
            EditType, History,
            building_transactions::RoofVisualHistoryRecord,
            wall_edit::{WallCompoundHistoryEdit, WallHistoryEdit},
        },
        mouse_capture::CursorCaptureCmd,
        onboarding::{NotifyHintSystemRoofShape, NotifyHintSystemShapeEditWasPerformed},
        roofs::{
            Roof, RoofAnimation, RoofTriggerVolume,
            commands::{EditRoofCmd, HighlightRoofCmd},
            geometry::edit_rules::{RoofEditEvent, ScalarEdit, TipEdit},
        },
        walls::{
            HighlightColor, WallId,
            decorator_archivist::{DecoratorArchivalResources, LiveDecoratorArchivist},
            decorator_storage::{DecoratorStorage, DecoratorStorageAddr},
            inner_walls::InnerWalls,
        },
        window::CursorMovedScreenSpace,
    },
    systems::{
        arrow_gizmo_animation::EnableArrowGizmoStretch,
        camera_rig::main_camera::MainCameraMoved,
        history::recording::RecordRoofVisualHistory,
        mode_manager::AppMode,
        ui::resources::{
            ui_state::{DragState, UiState},
            widget_params::{UiSounds, WidgetParams, WidgetVolumeShape},
        },
    },
};
use glam::{Vec2, Vec3};
use utils::geometry::intersect::Triangle;

/// Native cache discriminant2=None,0=parameters,1=position+parameters. Do not
/// replace this snapshot by the currently selected roof on release.
#[derive(Resource, Default, Debug)]
pub struct RoofParamsCacheResource {
    pub before: Option<RoofBeforeEdit>,
}
#[derive(Clone, Debug)]
pub struct RoofBeforeEdit {
    pub wall: WallId,
    pub position: Option<Vec3>,
    pub parameters: [u8; 28],
}
impl RoofParamsCacheResource {
    pub fn remember(&mut self, roof: &Roof, position: Option<Vec3>) {
        self.before = Some(RoofBeforeEdit {
            wall: roof.wall_id(),
            position,
            parameters: roof.observed()[0x24..0x40].try_into().unwrap(),
        });
    }
}
impl RoofBeforeEdit {
    /// Original UCOMISS/JNE/JP comparisons: signed zero is equal; every NaN
    /// comparison differs. Direction and flat-rim bytes are discrete checks.
    pub fn differs(&self, roof: &Roof) -> bool {
        let now = &roof.observed()[0x24..0x40];
        [8, 12, 0, 4, 16, 20].into_iter().any(|o| {
            f32::from_le_bytes(self.parameters[o..o + 4].try_into().unwrap())
                != f32::from_le_bytes(now[o..o + 4].try_into().unwrap())
        }) || self.parameters[24] != now[24]
            || self.parameters[25] != now[25]
    }
    /// Full original152B deferred record, including optional position, all28B
    /// before parameters and the actual88B after Roof. Queue id is host-owned.
    pub fn visual_record(&self, roof: &Roof) -> RoofVisualHistoryRecord {
        let mut record = [0; 152];
        if let Some(p) = self.position {
            record[..4].copy_from_slice(&1u32.to_le_bytes());
            for (i, v) in p.to_array().into_iter().enumerate() {
                record[4 + i * 4..8 + i * 4].copy_from_slice(&v.to_le_bytes());
            }
        }
        record[0x10..0x18].copy_from_slice(&self.wall.0.to_le_bytes());
        record[0x18..0x34].copy_from_slice(&self.parameters);
        record[0x38..0x90].copy_from_slice(roof.observed());
        RoofVisualHistoryRecord::from_observed(record)
    }
}
#[derive(Event, Clone, Copy, Debug)]
pub struct DoUiEditRoof {
    pub entity: Entity,
    pub phase: DragState,
}
#[derive(Default)]
pub struct RoofShapeEditAmount(pub f32);
#[derive(Default)]
pub struct OriginalWallHeight(pub f32);

/// Native1409a8f80 boop plus1409a8fd0 hover override {tag1,payload2}.
pub(crate) fn roof_widget_sounds() -> UiSounds {
    let mut sounds = UiSounds::empty();
    for (slot, tag) in [1u32, 2, 3, 3].into_iter().enumerate() {
        sounds.slots[slot][..4].copy_from_slice(&tag.to_le_bytes());
    }
    sounds.slots[0][8..16].copy_from_slice(&2u64.to_le_bytes());
    sounds
}

/// Original triangles iterator adds (0*floor,floor,0*floor) per vertex. Keep
/// zero products and per-triangle order, including repeated cap triangles.
pub fn roof_world_triangles(mesh: &RoofTriggerVolume, floor: f32) -> Vec<Triangle> {
    let zero = 0.0 * floor;
    mesh.triangles()
        .iter()
        .map(|t| Triangle {
            points: t.map(|p| Vec3::new(p.x + zero, p.y + floor, p.z + zero)),
        })
        .collect()
}

pub fn ui_edit_roof(
    mode: Res<AppMode>,
    ui: Res<UiState>,
    roofs: Query<(Entity, &Roof, &RoofAnimation, &RoofTriggerVolume)>,
    mut camera: EventReader<MainCameraMoved>,
    mut dispatch: EventWriter<DoUiEditRoof>,
    mut commands: Commands,
) {
    // Camera reader is untouched outside mode0; otherwise it is drained even
    // when the update is suppressed. No selected-Roof-only approximation.
    if mode.native_tag() != 0 {
        return;
    }
    if camera.read().last().is_some() {
        return;
    }
    for (entity, roof, animation, mesh) in &roofs {
        if animation.is_being_animated() {
            continue;
        }
        let params = WidgetParams::new(
            WidgetVolumeShape::default(),
            [1 | ((12 - u64::from(roof.is_pitched())) << 8), 0],
        )
        .triangle_mesh(roof_world_triangles(mesh, roof.floor_y()))
        .sounds(roof_widget_sounds());
        let widget = ui.add(&format!("roof_edit_{}", roof.wall_id()), params);
        if widget.hovered() != 0 {
            let highlight = HighlightRoofCmd::new(roof.wall_id(), HighlightColor(2));
            commands.queue(move |world: &mut World| {
                world.send_event(highlight);
            });
        }
        if u32::from_le_bytes(widget.intersecting()[..4].try_into().unwrap()) == 2 {
            continue;
        }
        dispatch.write(DoUiEditRoof {
            entity,
            phase: widget.into_drag_state(),
        });
        break;
    }
}

#[derive(SystemParam)]
pub struct RoofUiEffects<'w> {
    edit: EventWriter<'w, EditRoofCmd>,
    visual: EventWriter<'w, RecordRoofVisualHistory>,
    capture: EventWriter<'w, CursorCaptureCmd>,
    arrows: EventWriter<'w, EnableArrowGizmoStretch>,
    performed: EventWriter<'w, NotifyHintSystemShapeEditWasPerformed>,
    shape: EventWriter<'w, NotifyHintSystemRoofShape>,
}

/// The complete ordinary drag numeric branch, isolated for an unchanged-native
/// instruction oracle. None means zero/NaN length; it does not consume input.
pub fn drag_edit(roof: &Roof, delta: Vec2, amount: &mut f32) -> Option<(RoofEditEvent, bool, f32)> {
    if !((delta.y * delta.y + delta.x * delta.x).sqrt() > 0.0) {
        return None;
    }
    let vertical = delta.x.abs() < delta.y.abs();
    let scale = f32::from_bits(0xbb23d70a);
    let profile = if vertical {
        delta.x
    } else {
        let d = delta.x * scale;
        d + d
    };
    let height = if vertical { delta.y * scale } else { 0.0 };
    if !vertical {
        *amount = profile.abs() + *amount;
    }
    Some((
        RoofEditEvent {
            profile: ScalarEdit {
                mode: if !vertical && roof.is_pitched() { 2 } else { 0 },
                value: profile,
            },
            height: ScalarEdit {
                mode: if vertical { 2 } else { 0 },
                value: height,
            },
            ridge: ScalarEdit {
                mode: 0,
                value: 0.0,
            },
            eave: ScalarEdit {
                mode: 0,
                value: 0.0,
            },
            wall_id: roof.wall_id().0,
            tip: TipEdit {
                mode: 0,
                value: Vec2::ZERO,
            },
            source_flag: 0,
        },
        *amount > f32::from_bits(0x3e4ccccd),
        height,
    ))
}

/// Restore the original wall height policy when crossing flat/pitched0.1.
pub fn crossing_wall_height(original: f32, previous: f32, next: f32) -> Option<f32> {
    if !(original < f32::from_bits(0x3fcccccd)) {
        return None;
    }
    let lower = if previous > 0.1 && next <= 0.1 {
        1.6
    } else if previous <= 0.1 && next > 0.1 {
        0.7
    } else {
        return None;
    };
    let raised = if lower > original { lower } else { original };
    Some(if 14.0 < raised { 14.0 } else { raised })
}

pub fn record_wall_height(history: &mut History, wall: WallId, from: f32, to: f32) {
    let _ = history.add(EditType::Wall(std::sync::Arc::new(
        WallCompoundHistoryEdit {
            groups: vec![vec![WallHistoryEdit::AdjustHeight { wall, from, to }]],
        },
    )));
}

pub fn do_ui_edit_roof(
    roofs: Query<&Roof>,
    mut cache: ResMut<RoofParamsCacheResource>,
    mut archivist: ResMut<LiveDecoratorArchivist>,
    storage: Res<DecoratorStorage>,
    archival: DecoratorArchivalResources,
    mut walls: ResMut<InnerWalls>,
    mut cursor: EventReader<CursorMovedScreenSpace>,
    mut effects: RoofUiEffects,
    mut history: ResMut<History>,
    mut dispatch: EventReader<DoUiEditRoof>,
    mut amount: Local<RoofShapeEditAmount>,
    mut original_height: Local<OriginalWallHeight>,
) {
    let Some(request) = dispatch.read().last() else {
        return;
    };
    let roof = roofs
        .get(request.entity)
        .expect("do_ui_edit_roof requires original entity Roof");
    let wall = roof.wall_id();
    match request.phase.0 {
        2 => {}
        3 => {
            effects.capture.write(CursorCaptureCmd(false));
            cache.remember(roof, None);
            original_height.0 = walls
                .get(wall)
                .expect("roof drag requires wall")
                .snapshot
                .source
                .extra;
            archivist.archive_current_decorators_at_address(
                DecoratorStorageAddr::Wall(wall),
                &storage,
                &archival,
                "edit_roof",
            );
            archivist.decorator_cull_allow_wall(wall, &storage);
            amount.0 = 0.0;
        }
        4 => {
            effects.arrows.write(EnableArrowGizmoStretch);
            let Some(delta) = cursor.read().last() else {
                return;
            };
            let Some((edit, show_hint, height_delta)) = drag_edit(roof, delta.0, &mut amount.0)
            else {
                return;
            };
            if show_hint {
                effects.shape.write(NotifyHintSystemRoofShape { tag: 1 });
            }
            if let Some(height) = crossing_wall_height(
                original_height.0,
                roof.height_01(),
                height_delta + roof.height_01(),
            ) {
                walls.update_wall_state_height(wall, height);
            }
            effects.edit.write(EditRoofCmd(edit));
        }
        _ => {
            effects.capture.write(CursorCaptureCmd(true));
            if let Some(before) = &cache.before {
                if before.differs(roof) {
                    effects
                        .visual
                        .write(RecordRoofVisualHistory(before.visual_record(roof)));
                    let now = walls
                        .get(wall)
                        .expect("roof finish requires wall")
                        .snapshot
                        .source
                        .extra;
                    if original_height.0 != now {
                        record_wall_height(&mut history, wall, original_height.0, now);
                    }
                }
                effects
                    .performed
                    .write(NotifyHintSystemShapeEditWasPerformed);
                effects.shape.write(NotifyHintSystemRoofShape { tag: 0 });
            }
        }
    }
}

#[cfg(test)]
mod original_drag_oracle_tests {
    use super::*;
    // Frozen unchanged-RNE oracle input/output. No implementation-generated
    // expected values: native_drag_oracle.py supplies these event stores.
    const CASES: &[([u32; 5], [u32; 4], u32, Option<u32>, bool)] = &[
        (
            [1028443341, 0, 0, 1044549468, 1056964608],
            [4294967295, 0, 0, 0],
            1044549468,
            None,
            false,
        ),
        (
            [1028443341, 0, 0, 1044549468, 1070386381],
            [4294967295, 0, 0, 0],
            1044549468,
            None,
            false,
        ),
        (
            [1028443341, 1092616192, 1065353216, 1044549468, 1056964608],
            [0, 0, 0, 0],
            1047904911,
            None,
            true,
        ),
        (
            [1028443341, 1092616192, 1065353216, 1044549468, 1070386381],
            [0, 0, 0, 0],
            1047904911,
            None,
            true,
        ),
        (
            [1028443341, 3240099840, 1065353216, 1044549468, 1056964608],
            [0, 0, 0, 0],
            1047904911,
            None,
            true,
        ),
        (
            [1028443341, 3240099840, 1065353216, 1044549468, 1070386381],
            [0, 0, 0, 0],
            1047904911,
            None,
            true,
        ),
        (
            [1028443341, 1065353216, 1092616192, 1044549468, 1056964608],
            [0, 0, 2, 3167538380],
            1044549468,
            None,
            false,
        ),
        (
            [1028443341, 1065353216, 1092616192, 1044549468, 1070386381],
            [0, 0, 2, 3167538380],
            1044549468,
            None,
            false,
        ),
        (
            [1028443341, 1065353216, 3240099840, 1044549468, 1056964608],
            [0, 0, 2, 1020054732],
            1044549468,
            None,
            false,
        ),
        (
            [1028443341, 1065353216, 3240099840, 1044549468, 1070386381],
            [0, 0, 2, 1020054732],
            1044549468,
            None,
            false,
        ),
        (
            [1028443341, 1092616192, 1092616192, 1044549468, 1056964608],
            [0, 0, 0, 0],
            1047904911,
            None,
            true,
        ),
        (
            [1028443341, 1092616192, 1092616192, 1044549468, 1070386381],
            [0, 0, 0, 0],
            1047904911,
            None,
            true,
        ),
        (
            [1028443341, 2143289344, 1065353216, 1044549468, 1056964608],
            [4294967295, 0, 0, 0],
            1044549468,
            None,
            false,
        ),
        (
            [1028443341, 2143289344, 1065353216, 1044549468, 1070386381],
            [4294967295, 0, 0, 0],
            1044549468,
            None,
            false,
        ),
        (
            [1028443341, 2139095040, 1065353216, 1044549468, 1056964608],
            [0, 0, 0, 0],
            2139095040,
            None,
            true,
        ),
        (
            [1028443341, 2139095040, 1065353216, 1044549468, 1070386381],
            [0, 0, 0, 0],
            2139095040,
            None,
            true,
        ),
        (
            [1036831949, 0, 0, 1044549468, 1056964608],
            [4294967295, 0, 0, 0],
            1044549468,
            None,
            false,
        ),
        (
            [1036831949, 0, 0, 1044549468, 1070386381],
            [4294967295, 0, 0, 0],
            1044549468,
            None,
            false,
        ),
        (
            [1036831949, 1092616192, 1065353216, 1044549468, 1056964608],
            [0, 0, 0, 0],
            1047904911,
            None,
            true,
        ),
        (
            [1036831949, 1092616192, 1065353216, 1044549468, 1070386381],
            [0, 0, 0, 0],
            1047904911,
            None,
            true,
        ),
        (
            [1036831949, 3240099840, 1065353216, 1044549468, 1056964608],
            [0, 0, 0, 0],
            1047904911,
            None,
            true,
        ),
        (
            [1036831949, 3240099840, 1065353216, 1044549468, 1070386381],
            [0, 0, 0, 0],
            1047904911,
            None,
            true,
        ),
        (
            [1036831949, 1065353216, 1092616192, 1044549468, 1056964608],
            [0, 0, 2, 3167538380],
            1044549468,
            None,
            false,
        ),
        (
            [1036831949, 1065353216, 1092616192, 1044549468, 1070386381],
            [0, 0, 2, 3167538380],
            1044549468,
            None,
            false,
        ),
        (
            [1036831949, 1065353216, 3240099840, 1044549468, 1056964608],
            [0, 0, 2, 1020054732],
            1044549468,
            Some(1060320051),
            false,
        ),
        (
            [1036831949, 1065353216, 3240099840, 1044549468, 1070386381],
            [0, 0, 2, 1020054732],
            1044549468,
            None,
            false,
        ),
        (
            [1036831949, 1092616192, 1092616192, 1044549468, 1056964608],
            [0, 0, 0, 0],
            1047904911,
            None,
            true,
        ),
        (
            [1036831949, 1092616192, 1092616192, 1044549468, 1070386381],
            [0, 0, 0, 0],
            1047904911,
            None,
            true,
        ),
        (
            [1036831949, 2143289344, 1065353216, 1044549468, 1056964608],
            [4294967295, 0, 0, 0],
            1044549468,
            None,
            false,
        ),
        (
            [1036831949, 2143289344, 1065353216, 1044549468, 1070386381],
            [4294967295, 0, 0, 0],
            1044549468,
            None,
            false,
        ),
        (
            [1036831949, 2139095040, 1065353216, 1044549468, 1056964608],
            [0, 0, 0, 0],
            2139095040,
            None,
            true,
        ),
        (
            [1036831949, 2139095040, 1065353216, 1044549468, 1070386381],
            [0, 0, 0, 0],
            2139095040,
            None,
            true,
        ),
        (
            [1036831950, 0, 0, 1044549468, 1056964608],
            [4294967295, 0, 0, 0],
            1044549468,
            None,
            false,
        ),
        (
            [1036831950, 0, 0, 1044549468, 1070386381],
            [4294967295, 0, 0, 0],
            1044549468,
            None,
            false,
        ),
        (
            [1036831950, 1092616192, 1065353216, 1044549468, 1056964608],
            [2, 3175926988, 0, 0],
            1047904911,
            None,
            true,
        ),
        (
            [1036831950, 1092616192, 1065353216, 1044549468, 1070386381],
            [2, 3175926988, 0, 0],
            1047904911,
            None,
            true,
        ),
        (
            [1036831950, 3240099840, 1065353216, 1044549468, 1056964608],
            [2, 1028443340, 0, 0],
            1047904911,
            None,
            true,
        ),
        (
            [1036831950, 3240099840, 1065353216, 1044549468, 1070386381],
            [2, 1028443340, 0, 0],
            1047904911,
            None,
            true,
        ),
        (
            [1036831950, 1065353216, 1092616192, 1044549468, 1056964608],
            [0, 0, 2, 3167538380],
            1044549468,
            Some(1070386381),
            false,
        ),
        (
            [1036831950, 1065353216, 1092616192, 1044549468, 1070386381],
            [0, 0, 2, 3167538380],
            1044549468,
            None,
            false,
        ),
        (
            [1036831950, 1065353216, 3240099840, 1044549468, 1056964608],
            [0, 0, 2, 1020054732],
            1044549468,
            None,
            false,
        ),
        (
            [1036831950, 1065353216, 3240099840, 1044549468, 1070386381],
            [0, 0, 2, 1020054732],
            1044549468,
            None,
            false,
        ),
        (
            [1036831950, 1092616192, 1092616192, 1044549468, 1056964608],
            [2, 3175926988, 0, 0],
            1047904911,
            None,
            true,
        ),
        (
            [1036831950, 1092616192, 1092616192, 1044549468, 1070386381],
            [2, 3175926988, 0, 0],
            1047904911,
            None,
            true,
        ),
        (
            [1036831950, 2143289344, 1065353216, 1044549468, 1056964608],
            [4294967295, 0, 0, 0],
            1044549468,
            None,
            false,
        ),
        (
            [1036831950, 2143289344, 1065353216, 1044549468, 1070386381],
            [4294967295, 0, 0, 0],
            1044549468,
            None,
            false,
        ),
        (
            [1036831950, 2139095040, 1065353216, 1044549468, 1056964608],
            [2, 4286578688, 0, 0],
            2139095040,
            None,
            true,
        ),
        (
            [1036831950, 2139095040, 1065353216, 1044549468, 1070386381],
            [2, 4286578688, 0, 0],
            2139095040,
            None,
            true,
        ),
        (
            [1056964608, 0, 0, 1044549468, 1056964608],
            [4294967295, 0, 0, 0],
            1044549468,
            None,
            false,
        ),
        (
            [1056964608, 0, 0, 1044549468, 1070386381],
            [4294967295, 0, 0, 0],
            1044549468,
            None,
            false,
        ),
        (
            [1056964608, 1092616192, 1065353216, 1044549468, 1056964608],
            [2, 3175926988, 0, 0],
            1047904911,
            None,
            true,
        ),
        (
            [1056964608, 1092616192, 1065353216, 1044549468, 1070386381],
            [2, 3175926988, 0, 0],
            1047904911,
            None,
            true,
        ),
        (
            [1056964608, 3240099840, 1065353216, 1044549468, 1056964608],
            [2, 1028443340, 0, 0],
            1047904911,
            None,
            true,
        ),
        (
            [1056964608, 3240099840, 1065353216, 1044549468, 1070386381],
            [2, 1028443340, 0, 0],
            1047904911,
            None,
            true,
        ),
        (
            [1056964608, 1065353216, 1092616192, 1044549468, 1056964608],
            [0, 0, 2, 3167538380],
            1044549468,
            None,
            false,
        ),
        (
            [1056964608, 1065353216, 1092616192, 1044549468, 1070386381],
            [0, 0, 2, 3167538380],
            1044549468,
            None,
            false,
        ),
        (
            [1056964608, 1065353216, 3240099840, 1044549468, 1056964608],
            [0, 0, 2, 1020054732],
            1044549468,
            None,
            false,
        ),
        (
            [1056964608, 1065353216, 3240099840, 1044549468, 1070386381],
            [0, 0, 2, 1020054732],
            1044549468,
            None,
            false,
        ),
        (
            [1056964608, 1092616192, 1092616192, 1044549468, 1056964608],
            [2, 3175926988, 0, 0],
            1047904911,
            None,
            true,
        ),
        (
            [1056964608, 1092616192, 1092616192, 1044549468, 1070386381],
            [2, 3175926988, 0, 0],
            1047904911,
            None,
            true,
        ),
        (
            [1056964608, 2143289344, 1065353216, 1044549468, 1056964608],
            [4294967295, 0, 0, 0],
            1044549468,
            None,
            false,
        ),
        (
            [1056964608, 2143289344, 1065353216, 1044549468, 1070386381],
            [4294967295, 0, 0, 0],
            1044549468,
            None,
            false,
        ),
        (
            [1056964608, 2139095040, 1065353216, 1044549468, 1056964608],
            [2, 4286578688, 0, 0],
            2139095040,
            None,
            true,
        ),
        (
            [1056964608, 2139095040, 1065353216, 1044549468, 1070386381],
            [2, 4286578688, 0, 0],
            2139095040,
            None,
            true,
        ),
        (
            [2143289344, 0, 0, 1044549468, 1056964608],
            [4294967295, 0, 0, 0],
            1044549468,
            None,
            false,
        ),
        (
            [2143289344, 0, 0, 1044549468, 1070386381],
            [4294967295, 0, 0, 0],
            1044549468,
            None,
            false,
        ),
        (
            [2143289344, 1092616192, 1065353216, 1044549468, 1056964608],
            [2, 3175926988, 0, 0],
            1047904911,
            None,
            true,
        ),
        (
            [2143289344, 1092616192, 1065353216, 1044549468, 1070386381],
            [2, 3175926988, 0, 0],
            1047904911,
            None,
            true,
        ),
        (
            [2143289344, 3240099840, 1065353216, 1044549468, 1056964608],
            [2, 1028443340, 0, 0],
            1047904911,
            None,
            true,
        ),
        (
            [2143289344, 3240099840, 1065353216, 1044549468, 1070386381],
            [2, 1028443340, 0, 0],
            1047904911,
            None,
            true,
        ),
        (
            [2143289344, 1065353216, 1092616192, 1044549468, 1056964608],
            [0, 0, 2, 3167538380],
            1044549468,
            None,
            false,
        ),
        (
            [2143289344, 1065353216, 1092616192, 1044549468, 1070386381],
            [0, 0, 2, 3167538380],
            1044549468,
            None,
            false,
        ),
        (
            [2143289344, 1065353216, 3240099840, 1044549468, 1056964608],
            [0, 0, 2, 1020054732],
            1044549468,
            None,
            false,
        ),
        (
            [2143289344, 1065353216, 3240099840, 1044549468, 1070386381],
            [0, 0, 2, 1020054732],
            1044549468,
            None,
            false,
        ),
        (
            [2143289344, 1092616192, 1092616192, 1044549468, 1056964608],
            [2, 3175926988, 0, 0],
            1047904911,
            None,
            true,
        ),
        (
            [2143289344, 1092616192, 1092616192, 1044549468, 1070386381],
            [2, 3175926988, 0, 0],
            1047904911,
            None,
            true,
        ),
        (
            [2143289344, 2143289344, 1065353216, 1044549468, 1056964608],
            [4294967295, 0, 0, 0],
            1044549468,
            None,
            false,
        ),
        (
            [2143289344, 2143289344, 1065353216, 1044549468, 1070386381],
            [4294967295, 0, 0, 0],
            1044549468,
            None,
            false,
        ),
        (
            [2143289344, 2139095040, 1065353216, 1044549468, 1056964608],
            [2, 4286578688, 0, 0],
            2139095040,
            None,
            true,
        ),
        (
            [2143289344, 2139095040, 1065353216, 1044549468, 1070386381],
            [2, 4286578688, 0, 0],
            2139095040,
            None,
            true,
        ),
        (
            [1056964608, 1065353216, 0, 0, 1056964608],
            [2, 3148076810, 0, 0],
            1000593162,
            None,
            false,
        ),
        (
            [1056964608, 1065353216, 0, 1044885012, 1056964608],
            [2, 3148076810, 0, 0],
            1045220556,
            None,
            false,
        ),
        (
            [1056964608, 1065353216, 0, 1045220557, 1056964608],
            [2, 3148076810, 0, 0],
            1045556101,
            None,
            true,
        ),
        (
            [1056964608, 1065353216, 0, 1045220558, 1056964608],
            [2, 3148076810, 0, 0],
            1045556102,
            None,
            true,
        ),
        (
            [1028443341, 0, 3267887104, 0, 1056964608],
            [0, 0, 2, 1048576000],
            0,
            Some(1060320051),
            false,
        ),
        (
            [1056964608, 0, 1148846080, 0, 1056964608],
            [0, 0, 2, 3223322624],
            0,
            Some(1070386381),
            false,
        ),
        (
            [1056964608, 1065353216, 0, 2143289344, 1056964608],
            [2, 3148076810, 0, 0],
            2143289344,
            None,
            false,
        ),
    ];
    #[test]
    fn ordinary_roof_drag_matches_original_instructions() {
        for (index, (input, event, amount_bits, height, hint)) in CASES.iter().enumerate() {
            let [roof_height, dx, dy, start_amount, original] = input.map(f32::from_bits);
            let mut bytes = [0; 88];
            bytes[..8].copy_from_slice(&91u64.to_le_bytes());
            bytes[0x30..0x34].copy_from_slice(&roof_height.to_le_bytes());
            let roof = Roof::from_observed(bytes);
            let mut amount = start_amount;
            let got = drag_edit(&roof, Vec2::new(dx, dy), &mut amount);
            assert_eq!(amount.to_bits(), *amount_bits, "amount case {index}");
            if event[0] == u32::MAX {
                assert!(got.is_none(), "no edit case {index}");
                assert!(height.is_none());
            } else {
                let (edit, got_hint, delta) = got.expect("native emitted edit");
                assert_eq!(
                    [edit.profile.mode, edit.height.mode],
                    [event[0], event[2]],
                    "modes case {index}"
                );
                if edit.profile.mode != 0 {
                    assert_eq!(
                        edit.profile.value.to_bits(),
                        event[1],
                        "profile case {index}"
                    );
                }
                if edit.height.mode != 0 {
                    assert_eq!(edit.height.value.to_bits(), event[3], "height case {index}");
                }
                assert_eq!(got_hint, *hint, "hint case {index}");
                assert_eq!(
                    crossing_wall_height(original, roof_height, delta + roof_height)
                        .map(f32::to_bits),
                    *height,
                    "wall target case {index}"
                );
                assert_eq!(edit.wall_id, 91);
                assert_eq!([edit.ridge.mode, edit.eave.mode, edit.tip.mode], [0; 3]);
                assert_eq!(edit.source_flag, 0);
            }
        }
    }
    #[test]
    fn parameter_release_comparison_retains_nan_and_signed_zero_rules() {
        let mut bytes = [0; 88];
        bytes[..8].copy_from_slice(&7u64.to_le_bytes());
        let roof = Roof::from_observed(bytes);
        let mut cache = RoofParamsCacheResource::default();
        cache.remember(&roof, None);
        let mut after = roof.clone();
        after.observed_mut()[0x24..0x28].copy_from_slice(&(-0.0f32).to_le_bytes());
        assert!(!cache.before.as_ref().unwrap().differs(&after));
        after.observed_mut()[0x24..0x28].copy_from_slice(&f32::NAN.to_le_bytes());
        assert!(cache.before.as_ref().unwrap().differs(&after));
        cache.remember(&after, None);
        assert!(cache.before.as_ref().unwrap().differs(&after));
    }
}
