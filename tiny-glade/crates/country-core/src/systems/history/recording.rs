//! Roof recording writes the same History owner used by mode_manager and
//! replay. UI record notifications are separate from live edit commands so
//! replaying an edit does not record another copy of itself.
use crate::resources::history::EditType;
use crate::resources::history::History;
use crate::resources::history::building_transactions::BuildingEdit;
use crate::resources::history::building_transactions::RoofHistoryEdit;
use crate::resources::history::building_transactions::RoofVisualHistoryRecord;
use crate::resources::history::transactions::HistoryTransactionError;
use crate::resources::roofs::commands::RecordRoofHistory;
use bevy_ecs::prelude::*;

#[derive(Event, Clone, Debug)]
pub struct RecordRoofVisualHistory(pub RoofVisualHistoryRecord);
#[derive(Event, Clone, Copy, Debug)]
pub struct RecordRoofDirectionHistory {
    pub wall_id: u64,
    pub from: u8,
    pub to: u8,
}
#[derive(Event, Clone, Debug)]
pub struct HistoryRecordingFailed {
    pub edit: EditType,
    pub error: HistoryTransactionError,
}

fn record<S>(
    history: &mut History<S>,
    edit: RoofHistoryEdit,
    failures: &mut EventWriter<HistoryRecordingFailed>,
) {
    let edit = EditType::from(edit);
    if let Err(error) = history.add(edit.clone()) {
        failures.write(HistoryRecordingFailed { edit, error });
    }
}
pub fn record_roof_history<S: Send + Sync + 'static>(
    mut events: EventReader<RecordRoofHistory>,
    mut history: ResMut<History<S>>,
    mut failures: EventWriter<HistoryRecordingFailed>,
) {
    for event in events.read() {
        record(&mut history, event.0.clone(), &mut failures);
    }
}
/// Native deferred visual reader consumes only its last unread event.
pub fn record_roof_visual_history<S: Send + Sync + 'static>(
    mut events: EventReader<RecordRoofVisualHistory>,
    mut history: ResMut<History<S>>,
    mut failures: EventWriter<HistoryRecordingFailed>,
) {
    let Some(event) = events.read().last() else {
        return;
    };
    let BuildingEdit::Roof(edit) = event.0.edit() else {
        unreachable!()
    };
    record(&mut history, edit, &mut failures);
}
/// Ridge-direction recording consumes every event in original order.
pub fn record_roof_direction_history<S: Send + Sync + 'static>(
    mut events: EventReader<RecordRoofDirectionHistory>,
    mut history: ResMut<History<S>>,
    mut failures: EventWriter<HistoryRecordingFailed>,
) {
    for event in events.read() {
        record(
            &mut history,
            RoofHistoryEdit::direction(event.wall_id, event.from, event.to),
            &mut failures,
        );
    }
}
pub fn register_recording_events(world: &mut World) {
    world.init_resource::<Events<RecordRoofVisualHistory>>();
    world.init_resource::<Events<RecordRoofDirectionHistory>>();
    world.init_resource::<Events<HistoryRecordingFailed>>();
    world.init_resource::<Events<FinishInputRecordingCmd>>();
    world.init_resource::<Events<crate::systems::ui_clutter::ui_autoclutter::InstaCreateDeco>>();
    world
        .init_resource::<Events<crate::systems::ui_clutter::ui_autoclutter::InstaCreateStairPoint>>(
        );
    world.init_resource::<Events<crate::systems::backend::input::input_record_replay::InputReplayCommand>>();
    world.init_resource::<crate::resources::walls::decorator_storage::DecoratorStorage>();
    world.init_resource::<crate::resources::walls::decorator_archivist::LiveDecoratorArchivist>();
    world.init_resource::<crate::resources::stairs::stairs_state::StairsState>();
    world
        .init_resource::<crate::resources::stairs::stairs_removed_supports::StairsRemovedSupports>(
        );
}

/// Compatibility helper for callers that have already extracted a nonempty
/// decorator/stairs diff. The actual native lifecycle predicates and event
/// readers are implemented below in history_finish_recording_system. Empty
/// diffs alone do not finish this compatibility helper's transaction.
pub fn history_finish_recording<S: Send + Sync + 'static>(
    history: &mut History<S>,
    decorator_diff: &crate::resources::history::decorator_edit::DecoratorHistoryDiffMap,
    stairs_diff: &crate::resources::stairs::stairs_state::StairsHistoryDiff,
    stairs_edges: &mut std::collections::HashSet<
        crate::resources::stairs::stairs_state::StairGraphEdge,
    >,
    verify_stair_checksums: bool,
) -> Option<crate::resources::history::transactions::HistoryCommit> {
    if !history.is_recording() {
        return None;
    }
    let changed = !decorator_diff.is_empty() || !stairs_diff.is_empty();
    if !changed {
        return None;
    }
    history.add_decorator_diff(
        decorator_diff,
        stairs_diff,
        stairs_edges,
        verify_stair_checksums,
    );
    match history.finish_input_recording() {
        Ok(commit) => commit,
        Err(error) => {
            log::error!("history_finish_recording: {error:?}");
            None
        }
    }
}

/// Original FinishInputRecordingCmd is unit: color_menu @142294871..48c9
/// writes only EventId to an eight-byte-stride queue. History reads arrival.
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct FinishInputRecordingCmd;

/// Read the active widget's actual current permission from the two registries
/// already owned by UiState. Neither a missing widget nor an inactive UI can
/// create a recording group.
fn active_widget_records_history(ui: &crate::systems::ui::resources::ui_state::UiState) -> bool {
    // Original active Arc at UiState+0x20 outlives ui_update's frame-map clear.
    if let Some(params) = ui.active_params() {
        return !params.flags.no_history_recording();
    }
    let id = ui.active_widget();
    if let Some(params) = ui.widget(id) {
        return !params.flags.no_history_recording();
    }
    ui.clearing_canvas_params(id)
        .is_some_and(|params| !params.no_history_recording)
}

/// B inline @141734605..63f / @1414e182e..868. The UI producer registers
/// its widget in this frame before this system runs. An explicit start replaces
/// a previous pending group, exactly as the existing History method specifies.
pub fn history_start_recording<S: Send + Sync + 'static>(
    mode: Res<crate::systems::mode_manager::AppMode>,
    ui: Option<Res<crate::systems::ui::resources::ui_state::UiState>>,
    mut history: ResMut<History<S>>,
) {
    let Some(ui) = ui else { return };
    if mode.native_tag() != 13
        && ui.hover_byte() >= 2
        && ui.click_byte() == 3
        && active_widget_records_history(&ui)
    {
        history
            .start_input_recording(mode.native_tag())
            .expect("original history recording start preconditions");
    }
}

/// Actual B history_finish_recording lifecycle. The instant-create payloads
/// belong to their existing creation owner; history only consumes their arrival.
pub fn history_finish_recording_system<S: Send + Sync + 'static>(
    ui: Option<Res<crate::systems::ui::resources::ui_state::UiState>>,
    mut history: ResMut<History<S>>,
    mut storage: ResMut<crate::resources::walls::decorator_storage::DecoratorStorage>,
    mut archivist: ResMut<crate::resources::walls::decorator_archivist::LiveDecoratorArchivist>,
    mut stairs: ResMut<crate::resources::stairs::stairs_state::StairsState>,
    mut supports: ResMut<crate::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,
    mut instant_decorators: EventReader<
        crate::systems::ui_clutter::ui_autoclutter::InstaCreateDeco,
    >,
    mut instant_stairs: EventReader<
        crate::systems::ui_clutter::ui_autoclutter::InstaCreateStairPoint,
    >,
    mut finishes: EventReader<FinishInputRecordingCmd>,
    mut input: EventWriter<crate::systems::backend::input::input_record_replay::InputReplayCommand>,
) {
    // Original returns before all three reader cursors and storage immutability.
    if !history.is_recording() {
        return;
    }
    // Preserve the original first/second reader short circuit. read().count()
    // consumes the entire unread stream, rather than a single event.
    let instant = instant_decorators.read().count() != 0 || instant_stairs.read().count() != 0;
    let ui_finished = ui.as_deref().is_some_and(|ui| {
        ui.hover_byte() > 1 && ui.click_byte() < 2 && active_widget_records_history(ui)
    });
    // Third reader is visited even after an earlier trigger succeeds.
    let forced = finishes.read().count() != 0;
    if instant || ui_finished || forced {
        let decorator_diff = archivist.extract_diff();
        let stairs_diff = stairs.extract_diff_and_make_backup();
        // Reuse the current add_decorator_diff API. Its temporary set mutates
        // only removals; retain those removals in the existing map owner.
        let mut removed_edges = supports.edges.keys().copied().collect();
        history.add_decorator_diff(
            &decorator_diff,
            &stairs_diff,
            &mut removed_edges,
            crate::resources::stairs::stairs_state::VERIFY_STAIR_CHECKSUMS
                .load(std::sync::atomic::Ordering::Relaxed),
        );
        supports
            .edges
            .retain(|edge, _| removed_edges.contains(edge));
        let commit = history
            .finish_input_recording()
            .expect("original history recording commit invariants");
        if commit.is_some() {
            // B finish leaf writes sync niche/+8=1; actual original input
            // consumer dispatches it to on_history_sync(record=true).
            input.write(
                crate::systems::backend::input::input_record_replay::InputReplayCommand::Sync {
                    record: true,
                },
            );
        }
    }
    // B @1409dd9e6 applies this even while an open drag remains uncommitted.
    storage.mark_immutable();
}

/// Necessary local CPU scheduling adapters. Complete native archive restoration
/// remains in the existing upper decorator owner. The frame prefix below calls
/// its already recovered storage-permission method only; it owns no new state.
pub fn add_recording_systems<S: Send + Sync + 'static>(
    schedule: &mut bevy_ecs::schedule::Schedule,
) {
    use crate::systems::{
        backend::input::InputSystemSet,
        history::HistoryCpuStage,
        terrain::{ui_draw_terrain::ui_draw_terrain, ui_edit_terrain::ui_edit_terrain},
    };
    schedule.add_systems(
        (|mut storage: ResMut<crate::resources::walls::decorator_storage::DecoratorStorage>| {
            storage.mark_mutable()
        })
        .before(InputSystemSet::Derive)
        .before(HistoryCpuStage::Cursor)
        .before(ui_draw_terrain)
        .before(ui_edit_terrain)
        .before(history_start_recording::<S>),
    );
    schedule.add_systems(
        history_start_recording::<S>
            .after(ui_draw_terrain)
            .after(ui_edit_terrain)
            .before(crate::systems::ui_clutter::rmb_autoclutter::handle_rmb_autoclutter::<S>)
            .before(HistoryCpuStage::Record),
    );
    schedule.add_systems(
        history_finish_recording_system::<S>
            .after(HistoryCpuStage::Record)
            .before(InputSystemSet::Finish),
    );
}

#[cfg(test)]
mod lifecycle_tests {
    use super::*;
    use crate::resources::{
        cursor_terrain_raycast::CursorTerrainRaycast,
        glade_border::{BorderShape, GladeBorder},
        terrain_editing::{StrokeId, TerrainEditingData},
        walls::decorator_storage::{DecoratorStorage, DecoratorStorageAddr},
        world_raster::DirtyTilePublisher,
        world_raster_defs::WaterRaster,
    };
    use crate::systems::{
        backend::input::input_record_replay::InputReplayCommand,
        history::HistoryCpuStage,
        mode_manager::AppMode,
        terrain::{
            self,
            deferred_terrain_history::{
                PendingTerrainHistory, TerrainEdit, history_record_terrain,
            },
            ui_draw_terrain::{DRAW_CANVAS_NAME, ui_draw_terrain},
        },
        ui::resources::{
            ui_state::{UiInteraction, UiState, WidgetId},
            widget_params::{WidgetFlags, WidgetParams},
        },
    };
    use bevy_ecs::{
        prelude::*,
        system::{IntoSystem, RunSystemOnce, System},
    };
    use glam::{Vec2, Vec3};
    use std::sync::Arc;
    use utils::geometry::Circle2d;

    fn edit() -> EditType {
        EditType::Terrain(Arc::new(TerrainEdit::NewTerrainStroke {
            index: StrokeId(17),
            height_ws: 2.5,
            stroke_pts: vec![[3.0, 0.0, 4.0]],
        }))
    }

    fn state(hover: u8, phase: u8, flags: u8) -> UiState {
        let id = WidgetId(19);
        let ui = UiState::new(UiInteraction {
            native_tag: hover,
            native_phase: phase,
            widget: id,
        });
        let mut params = WidgetParams::default();
        params.flags = WidgetFlags(flags);
        ui.add_with_id(id, params);
        ui
    }

    #[test]
    fn start_matches_241_original_inline_and_initializer_cases() {
        // Generated from native-history-lifecycle-fixtures.json: mode, hover,
        // phase, flags, existing group, original call to start_input_recording.
        let cases: &[(u32, u8, u8, u8, bool, bool)] = &[
            (0, 0, 0, 0, false, false),
            (0, 0, 0, 1, false, false),
            (0, 0, 0, 4, false, false),
            (0, 0, 0, 5, false, false),
            (0, 0, 1, 0, false, false),
            (0, 0, 1, 1, false, false),
            (0, 0, 1, 4, false, false),
            (0, 0, 1, 5, false, false),
            (0, 0, 2, 0, false, false),
            (0, 0, 2, 1, false, false),
            (0, 0, 2, 4, false, false),
            (0, 0, 2, 5, false, false),
            (0, 0, 3, 0, false, false),
            (0, 0, 3, 1, false, false),
            (0, 0, 3, 4, false, false),
            (0, 0, 3, 5, false, false),
            (0, 0, 4, 0, false, false),
            (0, 0, 4, 1, false, false),
            (0, 0, 4, 4, false, false),
            (0, 0, 4, 5, false, false),
            (0, 1, 0, 0, false, false),
            (0, 1, 0, 1, false, false),
            (0, 1, 0, 4, false, false),
            (0, 1, 0, 5, false, false),
            (0, 1, 1, 0, false, false),
            (0, 1, 1, 1, false, false),
            (0, 1, 1, 4, false, false),
            (0, 1, 1, 5, false, false),
            (0, 1, 2, 0, false, false),
            (0, 1, 2, 1, false, false),
            (0, 1, 2, 4, false, false),
            (0, 1, 2, 5, false, false),
            (0, 1, 3, 0, false, false),
            (0, 1, 3, 1, false, false),
            (0, 1, 3, 4, false, false),
            (0, 1, 3, 5, false, false),
            (0, 1, 4, 0, false, false),
            (0, 1, 4, 1, false, false),
            (0, 1, 4, 4, false, false),
            (0, 1, 4, 5, false, false),
            (0, 2, 0, 0, false, false),
            (0, 2, 0, 1, false, false),
            (0, 2, 0, 4, false, false),
            (0, 2, 0, 5, false, false),
            (0, 2, 1, 0, false, false),
            (0, 2, 1, 1, false, false),
            (0, 2, 1, 4, false, false),
            (0, 2, 1, 5, false, false),
            (0, 2, 2, 0, false, false),
            (0, 2, 2, 1, false, false),
            (0, 2, 2, 4, false, false),
            (0, 2, 2, 5, false, false),
            (0, 2, 3, 0, false, false),
            (0, 2, 3, 1, false, true),
            (0, 2, 3, 4, false, false),
            (0, 2, 3, 5, false, true),
            (0, 2, 4, 0, false, false),
            (0, 2, 4, 1, false, false),
            (0, 2, 4, 4, false, false),
            (0, 2, 4, 5, false, false),
            (0, 3, 0, 0, false, false),
            (0, 3, 0, 1, false, false),
            (0, 3, 0, 4, false, false),
            (0, 3, 0, 5, false, false),
            (0, 3, 1, 0, false, false),
            (0, 3, 1, 1, false, false),
            (0, 3, 1, 4, false, false),
            (0, 3, 1, 5, false, false),
            (0, 3, 2, 0, false, false),
            (0, 3, 2, 1, false, false),
            (0, 3, 2, 4, false, false),
            (0, 3, 2, 5, false, false),
            (0, 3, 3, 0, false, false),
            (0, 3, 3, 1, false, true),
            (0, 3, 3, 4, false, false),
            (0, 3, 3, 5, false, true),
            (0, 3, 4, 0, false, false),
            (0, 3, 4, 1, false, false),
            (0, 3, 4, 4, false, false),
            (0, 3, 4, 5, false, false),
            (10, 0, 0, 0, false, false),
            (10, 0, 0, 1, false, false),
            (10, 0, 0, 4, false, false),
            (10, 0, 0, 5, false, false),
            (10, 0, 1, 0, false, false),
            (10, 0, 1, 1, false, false),
            (10, 0, 1, 4, false, false),
            (10, 0, 1, 5, false, false),
            (10, 0, 2, 0, false, false),
            (10, 0, 2, 1, false, false),
            (10, 0, 2, 4, false, false),
            (10, 0, 2, 5, false, false),
            (10, 0, 3, 0, false, false),
            (10, 0, 3, 1, false, false),
            (10, 0, 3, 4, false, false),
            (10, 0, 3, 5, false, false),
            (10, 0, 4, 0, false, false),
            (10, 0, 4, 1, false, false),
            (10, 0, 4, 4, false, false),
            (10, 0, 4, 5, false, false),
            (10, 1, 0, 0, false, false),
            (10, 1, 0, 1, false, false),
            (10, 1, 0, 4, false, false),
            (10, 1, 0, 5, false, false),
            (10, 1, 1, 0, false, false),
            (10, 1, 1, 1, false, false),
            (10, 1, 1, 4, false, false),
            (10, 1, 1, 5, false, false),
            (10, 1, 2, 0, false, false),
            (10, 1, 2, 1, false, false),
            (10, 1, 2, 4, false, false),
            (10, 1, 2, 5, false, false),
            (10, 1, 3, 0, false, false),
            (10, 1, 3, 1, false, false),
            (10, 1, 3, 4, false, false),
            (10, 1, 3, 5, false, false),
            (10, 1, 4, 0, false, false),
            (10, 1, 4, 1, false, false),
            (10, 1, 4, 4, false, false),
            (10, 1, 4, 5, false, false),
            (10, 2, 0, 0, false, false),
            (10, 2, 0, 1, false, false),
            (10, 2, 0, 4, false, false),
            (10, 2, 0, 5, false, false),
            (10, 2, 1, 0, false, false),
            (10, 2, 1, 1, false, false),
            (10, 2, 1, 4, false, false),
            (10, 2, 1, 5, false, false),
            (10, 2, 2, 0, false, false),
            (10, 2, 2, 1, false, false),
            (10, 2, 2, 4, false, false),
            (10, 2, 2, 5, false, false),
            (10, 2, 3, 0, false, false),
            (10, 2, 3, 1, false, true),
            (10, 2, 3, 4, false, false),
            (10, 2, 3, 5, false, true),
            (10, 2, 4, 0, false, false),
            (10, 2, 4, 1, false, false),
            (10, 2, 4, 4, false, false),
            (10, 2, 4, 5, false, false),
            (10, 3, 0, 0, false, false),
            (10, 3, 0, 1, false, false),
            (10, 3, 0, 4, false, false),
            (10, 3, 0, 5, false, false),
            (10, 3, 1, 0, false, false),
            (10, 3, 1, 1, false, false),
            (10, 3, 1, 4, false, false),
            (10, 3, 1, 5, false, false),
            (10, 3, 2, 0, false, false),
            (10, 3, 2, 1, false, false),
            (10, 3, 2, 4, false, false),
            (10, 3, 2, 5, false, false),
            (10, 3, 3, 0, false, false),
            (10, 3, 3, 1, false, true),
            (10, 3, 3, 4, false, false),
            (10, 3, 3, 5, false, true),
            (10, 3, 4, 0, false, false),
            (10, 3, 4, 1, false, false),
            (10, 3, 4, 4, false, false),
            (10, 3, 4, 5, false, false),
            (13, 0, 0, 0, false, false),
            (13, 0, 0, 1, false, false),
            (13, 0, 0, 4, false, false),
            (13, 0, 0, 5, false, false),
            (13, 0, 1, 0, false, false),
            (13, 0, 1, 1, false, false),
            (13, 0, 1, 4, false, false),
            (13, 0, 1, 5, false, false),
            (13, 0, 2, 0, false, false),
            (13, 0, 2, 1, false, false),
            (13, 0, 2, 4, false, false),
            (13, 0, 2, 5, false, false),
            (13, 0, 3, 0, false, false),
            (13, 0, 3, 1, false, false),
            (13, 0, 3, 4, false, false),
            (13, 0, 3, 5, false, false),
            (13, 0, 4, 0, false, false),
            (13, 0, 4, 1, false, false),
            (13, 0, 4, 4, false, false),
            (13, 0, 4, 5, false, false),
            (13, 1, 0, 0, false, false),
            (13, 1, 0, 1, false, false),
            (13, 1, 0, 4, false, false),
            (13, 1, 0, 5, false, false),
            (13, 1, 1, 0, false, false),
            (13, 1, 1, 1, false, false),
            (13, 1, 1, 4, false, false),
            (13, 1, 1, 5, false, false),
            (13, 1, 2, 0, false, false),
            (13, 1, 2, 1, false, false),
            (13, 1, 2, 4, false, false),
            (13, 1, 2, 5, false, false),
            (13, 1, 3, 0, false, false),
            (13, 1, 3, 1, false, false),
            (13, 1, 3, 4, false, false),
            (13, 1, 3, 5, false, false),
            (13, 1, 4, 0, false, false),
            (13, 1, 4, 1, false, false),
            (13, 1, 4, 4, false, false),
            (13, 1, 4, 5, false, false),
            (13, 2, 0, 0, false, false),
            (13, 2, 0, 1, false, false),
            (13, 2, 0, 4, false, false),
            (13, 2, 0, 5, false, false),
            (13, 2, 1, 0, false, false),
            (13, 2, 1, 1, false, false),
            (13, 2, 1, 4, false, false),
            (13, 2, 1, 5, false, false),
            (13, 2, 2, 0, false, false),
            (13, 2, 2, 1, false, false),
            (13, 2, 2, 4, false, false),
            (13, 2, 2, 5, false, false),
            (13, 2, 3, 0, false, false),
            (13, 2, 3, 1, false, false),
            (13, 2, 3, 4, false, false),
            (13, 2, 3, 5, false, false),
            (13, 2, 4, 0, false, false),
            (13, 2, 4, 1, false, false),
            (13, 2, 4, 4, false, false),
            (13, 2, 4, 5, false, false),
            (13, 3, 0, 0, false, false),
            (13, 3, 0, 1, false, false),
            (13, 3, 0, 4, false, false),
            (13, 3, 0, 5, false, false),
            (13, 3, 1, 0, false, false),
            (13, 3, 1, 1, false, false),
            (13, 3, 1, 4, false, false),
            (13, 3, 1, 5, false, false),
            (13, 3, 2, 0, false, false),
            (13, 3, 2, 1, false, false),
            (13, 3, 2, 4, false, false),
            (13, 3, 2, 5, false, false),
            (13, 3, 3, 0, false, false),
            (13, 3, 3, 1, false, false),
            (13, 3, 3, 4, false, false),
            (13, 3, 3, 5, false, false),
            (13, 3, 4, 0, false, false),
            (13, 3, 4, 1, false, false),
            (13, 3, 4, 4, false, false),
            (13, 3, 4, 5, false, false),
            (10, 2, 3, 1, true, true),
        ];
        for (index, &(mode, hover, phase, flags, was_recording, started)) in
            cases.iter().enumerate()
        {
            let mut world = World::new();
            world.insert_resource(if mode == 13 {
                AppMode::Replay
            } else if mode == 10 {
                AppMode::Terrain
            } else {
                AppMode::default()
            });
            world.insert_resource(state(hover, phase, flags));
            let mut history = History::<()>::empty_glade();
            if was_recording {
                history.start_input_recording(0).unwrap();
                history.add(edit()).unwrap();
            }
            world.insert_resource(history);
            world
                .run_system_once(history_start_recording::<()>)
                .unwrap();
            let history = world.resource::<History<()>>();
            assert_eq!(
                history.is_recording(),
                was_recording || started,
                "native case {index}"
            );
            if started {
                assert!(
                    history.pending().unwrap().edits.is_empty(),
                    "explicit start replaces group {index}"
                );
            }
        }
    }

    fn first_frame_world() -> World {
        let mut world = World::new();
        terrain::register_terrain_events(&mut world);
        register_recording_events(&mut world);
        world.insert_resource(History::<()>::empty_glade());
        world.insert_resource(AppMode::Terrain);
        // Use the actual input Sync consumer, preserving its existing next-frame
        // Control phase rather than creating a cycle with late history Record.
        crate::systems::backend::input::register_input_events(&mut world);
        world.init_resource::<crate::resources::toolbar::Toolbar>();
        world.init_resource::<crate::systems::camera_rig::main_camera::MainCamera>();
        world.init_resource::<crate::resources::mouse_capture::CursorCapturedPosition>();
        world.insert_resource(CursorTerrainRaycast {
            tag: 1,
            pos: Vec3::new(3.0, -1.0, 4.0),
            ..Default::default()
        });
        world.insert_resource(WaterRaster::new(DirtyTilePublisher::unique()));
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
        let mut library = crate::asset_libraries::ssbo_library::AssetSsboLibrary::new();
        let device = crate::render::test_device::TestDevice(0);
        use crate::{
            resources::terrain_rock_mask::TerrainRockMaskSsbo,
            systems::render_loop::compute_passes::mouse_read_terrain_stroke_index::MouseTerrainRaycastSsbo,
        };
        library.add_new::<MouseTerrainRaycastSsbo>(&device);
        library.add_new::<TerrainRockMaskSsbo>(&device);
        library.get_mut::<MouseTerrainRaycastSsbo>().unwrap().mapped[0].terrain_stroke_index = -1;
        world.insert_resource(library);
        world
    }

    #[test]
    fn first_actual_draw_registers_permission_then_records_and_commits_on_release() {
        let mut world = first_frame_world();
        let widget = WidgetId::clearing_canvas(DRAW_CANVAS_NAME);
        // This must be absent: pre-registering the canvas would hide a broken
        // start-before-UI schedule and lose the first native drag.
        assert!(
            world
                .resource::<UiState>()
                .clearing_canvas_params(widget)
                .is_none()
        );
        assert!(!world.resource::<History<()>>().is_recording());
        let mut schedule = Schedule::default();
        schedule.add_systems(ui_draw_terrain);
        schedule.add_systems(
            crate::systems::backend::input::input_record_replay::control_input_replay
                .before(ui_draw_terrain),
        );
        schedule.add_systems(history_record_terrain::<()>.in_set(HistoryCpuStage::Record));
        add_recording_systems::<()>(&mut schedule);
        // Exercise a real next-frame storage write after the permission prefix.
        schedule.add_systems(
            (|mut storage: ResMut<DecoratorStorage>| {
                storage
                    .get_or_insert_default(DecoratorStorageAddr::Terrain)
                    .clear_modification_tracking();
            })
            .after(HistoryCpuStage::Cursor)
            .before(ui_draw_terrain),
        );
        schedule.run(&mut world);
        assert!(
            world
                .resource::<UiState>()
                .clearing_canvas_params(widget)
                .is_some()
        );
        assert!(world.resource::<History<()>>().is_recording());
        assert!(
            world
                .resource::<History<()>>()
                .pending()
                .unwrap()
                .edits
                .is_empty()
        );
        assert_eq!(world.resource::<TerrainEditingData>().strokes.len(), 1);

        world
            .resource_mut::<UiState>()
            .replace_interaction(UiInteraction {
                native_tag: 2,
                native_phase: 4,
                widget,
            });
        world.resource_mut::<CursorTerrainRaycast>().pos.x = 4.0;
        schedule.run(&mut world);
        assert!(world.resource::<History<()>>().is_recording());
        assert!(world.resource::<DecoratorStorage>().immutable);
        world
            .resource_mut::<UiState>()
            .replace_interaction(UiInteraction {
                native_tag: 2,
                native_phase: 1,
                widget,
            });
        schedule.run(&mut world);
        let history = world.resource::<History<()>>();
        assert!(!history.is_recording());
        assert_eq!(history.ledger().cursor, 0);
        assert_eq!(history.ledger().entries.len(), 1);
        assert!(
            matches!(&history.ledger().entries[0].edits[..], [EditType::Terrain(edit)] if matches!(edit.as_ref(), TerrainEdit::NewTerrainStroke {index: StrokeId(0), stroke_pts, ..} if stroke_pts == &[[3.0,-1.0,4.0],[4.0,-1.0,4.0]]))
        );
        assert!(world.resource::<PendingTerrainHistory>().is_empty());
        let events = world.resource::<Events<InputReplayCommand>>();
        assert_eq!(
            events
                .get_cursor()
                .read(events)
                .cloned()
                .collect::<Vec<_>>(),
            [InputReplayCommand::Sync { record: true }]
        );
        assert_eq!(world.resource::<crate::systems::backend::input::input_record_replay::InputRecordReplayState>().stream.sync_points().len(), 0);
        // This next start must permit the actual storage write above again,
        // despite finish having marked storage immutable in the previous frame.
        world
            .resource_mut::<UiState>()
            .replace_interaction(UiInteraction {
                native_tag: 2,
                native_phase: 3,
                widget,
            });
        schedule.run(&mut world);
        assert!(world.resource::<History<()>>().is_recording());
        assert_eq!(world.resource::<History<()>>().ledger().entries.len(), 1);
        assert_eq!(world.resource::<crate::systems::backend::input::input_record_replay::InputRecordReplayState>().stream.sync_points().len(), 1);
    }

    #[test]
    fn unstarted_group_preserves_finish_reader_and_empty_finish_emits_no_sync() {
        let mut world = World::new();
        register_recording_events(&mut world);
        world.insert_resource(History::<()>::empty_glade());
        world
            .resource_mut::<Events<FinishInputRecordingCmd>>()
            .send(FinishInputRecordingCmd);
        let mut system = IntoSystem::into_system(history_finish_recording_system::<()>);
        system.initialize(&mut world);
        system.run((), &mut world);
        // Original has no pending group: it has not consumed the event.
        assert!(!world.resource::<DecoratorStorage>().immutable);
        world
            .resource_mut::<History<()>>()
            .start_input_recording(0)
            .unwrap();
        system.run((), &mut world);
        assert!(!world.resource::<History<()>>().is_recording());
        assert!(world.resource::<History<()>>().ledger().entries.is_empty());
        assert!(world.resource::<Events<InputReplayCommand>>().is_empty());
    }
    fn instant_deco() -> crate::systems::ui_clutter::ui_autoclutter::InstaCreateDeco {
        // Exact destination transport from original writer fixture deco[0].
        // This test exercises arrival; placement enum semantics remain open.
        crate::systems::ui_clutter::ui_autoclutter::InstaCreateDeco {
            destination: [0x75dd10c75433c8db, 0xe038d51dca8bf380, 0xecd9fb9341459e0e],
            extra: None,
            rotation: 0.0,
            position: Vec3::ZERO,
            ty: crate::systems::mode_manager::ClutterMeshTy::Bench,
        }
    }
    fn instant_stair() -> crate::systems::ui_clutter::ui_autoclutter::InstaCreateStairPoint {
        // Complete original native writer fixture stair[0], excluding EventId.
        crate::systems::ui_clutter::ui_autoclutter::InstaCreateStairPoint::from_observed([
            0x49, 0xbd, 0xe4, 0x94, 0x63, 0x79, 0x36, 0xe2, 0x9b, 0x35, 0xda, 0xed, 0xb1, 0x4b,
            0xe5, 0x72, 0x28, 0xd0, 0x49, 0x0a, 0x73, 0xe5, 0x46, 0x62, 0x0f, 0xf9, 0xb1, 0xbe,
            0x61, 0x4e, 0xf3, 0x17, 0x68, 0xf6, 0x4e, 0x44, 0x78, 0x9d, 0xdf, 0x00, 0xb9, 0xd0,
            0x4b, 0xed, 0x26, 0xf7, 0x8b, 0xca, 0x43, 0xd0, 0x56, 0xf7, 0xf6, 0x24, 0x34, 0x68,
            0xf9, 0x99, 0xa0, 0x2e, 0xc0, 0x9a, 0xc0, 0x2e, 0x79, 0x85, 0x29, 0x84, 0x4e, 0x90,
            0x6f, 0x05, 0xa2, 0xff, 0x15, 0x87, 0x04, 0xcf, 0x82, 0x2c, 0x03, 0x1a, 0x16, 0x4a,
            0x1f, 0x0f, 0xf1, 0x3c, 0x3a, 0xb3, 0xe0, 0x34, 0xc3, 0x0d, 0xac, 0x36, 0x6d, 0xdf,
            0xf1, 0x5d, 0x3e, 0xc6, 0x84, 0xc4, 0x3f, 0x20, 0x18, 0xa6, 0x70, 0x67, 0x77, 0xc0,
        ])
    }

    #[test]
    fn finish_matches_86_original_lifecycle_cases() {
        // hover, phase, flags, arrivals[Deco,Stair,Finish], pending, edit_count,
        // original recording_after, ledger_len, storage_immutable, sync_count.
        let cases: &[(
            u8,
            u8,
            u8,
            [usize; 3],
            bool,
            usize,
            bool,
            usize,
            bool,
            usize,
        )] = &[
            (0, 0, 0, [0, 0, 0], true, 1, true, 0, true, 0),
            (0, 0, 1, [0, 0, 0], true, 1, true, 0, true, 0),
            (0, 0, 4, [0, 0, 0], true, 1, true, 0, true, 0),
            (0, 0, 5, [0, 0, 0], true, 1, true, 0, true, 0),
            (0, 1, 0, [0, 0, 0], true, 1, true, 0, true, 0),
            (0, 1, 1, [0, 0, 0], true, 1, true, 0, true, 0),
            (0, 1, 4, [0, 0, 0], true, 1, true, 0, true, 0),
            (0, 1, 5, [0, 0, 0], true, 1, true, 0, true, 0),
            (0, 2, 0, [0, 0, 0], true, 1, true, 0, true, 0),
            (0, 2, 1, [0, 0, 0], true, 1, true, 0, true, 0),
            (0, 2, 4, [0, 0, 0], true, 1, true, 0, true, 0),
            (0, 2, 5, [0, 0, 0], true, 1, true, 0, true, 0),
            (0, 3, 0, [0, 0, 0], true, 1, true, 0, true, 0),
            (0, 3, 1, [0, 0, 0], true, 1, true, 0, true, 0),
            (0, 3, 4, [0, 0, 0], true, 1, true, 0, true, 0),
            (0, 3, 5, [0, 0, 0], true, 1, true, 0, true, 0),
            (0, 4, 0, [0, 0, 0], true, 1, true, 0, true, 0),
            (0, 4, 1, [0, 0, 0], true, 1, true, 0, true, 0),
            (0, 4, 4, [0, 0, 0], true, 1, true, 0, true, 0),
            (0, 4, 5, [0, 0, 0], true, 1, true, 0, true, 0),
            (1, 0, 0, [0, 0, 0], true, 1, true, 0, true, 0),
            (1, 0, 1, [0, 0, 0], true, 1, true, 0, true, 0),
            (1, 0, 4, [0, 0, 0], true, 1, true, 0, true, 0),
            (1, 0, 5, [0, 0, 0], true, 1, true, 0, true, 0),
            (1, 1, 0, [0, 0, 0], true, 1, true, 0, true, 0),
            (1, 1, 1, [0, 0, 0], true, 1, true, 0, true, 0),
            (1, 1, 4, [0, 0, 0], true, 1, true, 0, true, 0),
            (1, 1, 5, [0, 0, 0], true, 1, true, 0, true, 0),
            (1, 2, 0, [0, 0, 0], true, 1, true, 0, true, 0),
            (1, 2, 1, [0, 0, 0], true, 1, true, 0, true, 0),
            (1, 2, 4, [0, 0, 0], true, 1, true, 0, true, 0),
            (1, 2, 5, [0, 0, 0], true, 1, true, 0, true, 0),
            (1, 3, 0, [0, 0, 0], true, 1, true, 0, true, 0),
            (1, 3, 1, [0, 0, 0], true, 1, true, 0, true, 0),
            (1, 3, 4, [0, 0, 0], true, 1, true, 0, true, 0),
            (1, 3, 5, [0, 0, 0], true, 1, true, 0, true, 0),
            (1, 4, 0, [0, 0, 0], true, 1, true, 0, true, 0),
            (1, 4, 1, [0, 0, 0], true, 1, true, 0, true, 0),
            (1, 4, 4, [0, 0, 0], true, 1, true, 0, true, 0),
            (1, 4, 5, [0, 0, 0], true, 1, true, 0, true, 0),
            (2, 0, 0, [0, 0, 0], true, 1, true, 0, true, 0),
            (2, 0, 1, [0, 0, 0], true, 1, false, 1, true, 1),
            (2, 0, 4, [0, 0, 0], true, 1, true, 0, true, 0),
            (2, 0, 5, [0, 0, 0], true, 1, false, 1, true, 1),
            (2, 1, 0, [0, 0, 0], true, 1, true, 0, true, 0),
            (2, 1, 1, [0, 0, 0], true, 1, false, 1, true, 1),
            (2, 1, 4, [0, 0, 0], true, 1, true, 0, true, 0),
            (2, 1, 5, [0, 0, 0], true, 1, false, 1, true, 1),
            (2, 2, 0, [0, 0, 0], true, 1, true, 0, true, 0),
            (2, 2, 1, [0, 0, 0], true, 1, true, 0, true, 0),
            (2, 2, 4, [0, 0, 0], true, 1, true, 0, true, 0),
            (2, 2, 5, [0, 0, 0], true, 1, true, 0, true, 0),
            (2, 3, 0, [0, 0, 0], true, 1, true, 0, true, 0),
            (2, 3, 1, [0, 0, 0], true, 1, true, 0, true, 0),
            (2, 3, 4, [0, 0, 0], true, 1, true, 0, true, 0),
            (2, 3, 5, [0, 0, 0], true, 1, true, 0, true, 0),
            (2, 4, 0, [0, 0, 0], true, 1, true, 0, true, 0),
            (2, 4, 1, [0, 0, 0], true, 1, true, 0, true, 0),
            (2, 4, 4, [0, 0, 0], true, 1, true, 0, true, 0),
            (2, 4, 5, [0, 0, 0], true, 1, true, 0, true, 0),
            (3, 0, 0, [0, 0, 0], true, 1, true, 0, true, 0),
            (3, 0, 1, [0, 0, 0], true, 1, false, 1, true, 1),
            (3, 0, 4, [0, 0, 0], true, 1, true, 0, true, 0),
            (3, 0, 5, [0, 0, 0], true, 1, false, 1, true, 1),
            (3, 1, 0, [0, 0, 0], true, 1, true, 0, true, 0),
            (3, 1, 1, [0, 0, 0], true, 1, false, 1, true, 1),
            (3, 1, 4, [0, 0, 0], true, 1, true, 0, true, 0),
            (3, 1, 5, [0, 0, 0], true, 1, false, 1, true, 1),
            (3, 2, 0, [0, 0, 0], true, 1, true, 0, true, 0),
            (3, 2, 1, [0, 0, 0], true, 1, true, 0, true, 0),
            (3, 2, 4, [0, 0, 0], true, 1, true, 0, true, 0),
            (3, 2, 5, [0, 0, 0], true, 1, true, 0, true, 0),
            (3, 3, 0, [0, 0, 0], true, 1, true, 0, true, 0),
            (3, 3, 1, [0, 0, 0], true, 1, true, 0, true, 0),
            (3, 3, 4, [0, 0, 0], true, 1, true, 0, true, 0),
            (3, 3, 5, [0, 0, 0], true, 1, true, 0, true, 0),
            (3, 4, 0, [0, 0, 0], true, 1, true, 0, true, 0),
            (3, 4, 1, [0, 0, 0], true, 1, true, 0, true, 0),
            (3, 4, 4, [0, 0, 0], true, 1, true, 0, true, 0),
            (3, 4, 5, [0, 0, 0], true, 1, true, 0, true, 0),
            (0, 2, 0, [1, 0, 0], true, 1, false, 1, true, 1),
            (0, 2, 0, [0, 1, 0], true, 1, false, 1, true, 1),
            (0, 2, 0, [0, 0, 1], true, 1, false, 1, true, 1),
            (0, 2, 0, [2, 3, 1], true, 1, false, 1, true, 1),
            (0, 2, 0, [2, 3, 1], false, 1, false, 0, false, 0),
            (0, 2, 0, [0, 0, 1], true, 0, false, 0, true, 0),
        ];
        for (
            index,
            &(hover, phase, flags, events, pending, edits, after, ledger, immutable, syncs),
        ) in cases.iter().enumerate()
        {
            let mut world = World::new();
            register_recording_events(&mut world);
            world.insert_resource(state(hover, phase, flags));
            let mut history = History::<()>::empty_glade();
            if pending {
                history.start_input_recording(0).unwrap();
                for _ in 0..edits {
                    history.add(edit()).unwrap();
                }
            }
            world.insert_resource(history);
            for _ in 0..events[0] {
                world.send_event(instant_deco());
            }
            for _ in 0..events[1] {
                world.send_event(instant_stair());
            }
            for _ in 0..events[2] {
                world.send_event(FinishInputRecordingCmd);
            }
            world
                .run_system_once(history_finish_recording_system::<()>)
                .unwrap();
            assert_eq!(
                world.resource::<History<()>>().is_recording(),
                after,
                "native case {index}"
            );
            assert_eq!(
                world.resource::<History<()>>().ledger().entries.len(),
                ledger,
                "native case {index}"
            );
            assert_eq!(
                world.resource::<DecoratorStorage>().immutable,
                immutable,
                "native case {index}"
            );
            assert_eq!(
                world.resource::<Events<InputReplayCommand>>().len(),
                syncs,
                "native case {index}"
            );
        }
    }

    #[test]
    fn instant_short_circuit_preserves_stair_reader_but_always_consumes_finish_reader() {
        let mut world = World::new();
        register_recording_events(&mut world);
        let mut history = History::<()>::empty_glade();
        history.start_input_recording(0).unwrap();
        history.add(edit()).unwrap();
        world.insert_resource(history);
        world.send_event(instant_deco());
        world.send_event(instant_stair());
        world.send_event(FinishInputRecordingCmd);
        let mut system = IntoSystem::into_system(history_finish_recording_system::<()>);
        system.initialize(&mut world);
        system.run((), &mut world);
        assert_eq!(world.resource::<History<()>>().ledger().entries.len(), 1);
        // Reader2 remains unread on the first frame and commits this new group.
        world
            .resource_mut::<History<()>>()
            .start_input_recording(0)
            .unwrap();
        world.resource_mut::<History<()>>().add(edit()).unwrap();
        system.run((), &mut world);
        assert_eq!(world.resource::<History<()>>().ledger().entries.len(), 2);
        // Reader3 was consumed despite Reader1's trigger; it cannot finish this.
        world
            .resource_mut::<History<()>>()
            .start_input_recording(0)
            .unwrap();
        system.run((), &mut world);
        assert!(world.resource::<History<()>>().is_recording());
        assert_eq!(world.resource::<Events<InputReplayCommand>>().len(), 2);
    }
}
