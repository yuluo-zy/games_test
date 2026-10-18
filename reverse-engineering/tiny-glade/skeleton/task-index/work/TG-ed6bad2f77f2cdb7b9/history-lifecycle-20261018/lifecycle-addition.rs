/// Original FinishInputRecordingCmd is unit: color_menu @142294871..48c9
/// writes only EventId to an eight-byte-stride queue. History reads arrival.
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct FinishInputRecordingCmd;

/// Read the active widget's actual current permission from the two registries
/// already owned by UiState. Neither a missing widget nor an inactive UI can
/// create a recording group.
fn active_widget_records_history(ui: &crate::systems::ui::resources::ui_state::UiState) -> bool {
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
    if mode.native_tag() != 13 && ui.hover_byte() >= 2
        && ui.click_byte() == 3 && active_widget_records_history(&ui)
    {
        history.start_input_recording(mode.native_tag())
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
    mut instant_decorators: EventReader<crate::systems::ui_clutter::ui_autoclutter::InstaCreateDeco>,
    mut instant_stairs: EventReader<crate::systems::ui_clutter::ui_autoclutter::InstaCreateStairPoint>,
    mut finishes: EventReader<FinishInputRecordingCmd>,
    mut input: EventWriter<crate::systems::backend::input::input_record_replay::InputReplayCommand>,
) {
    // Original returns before all three reader cursors and storage immutability.
    if !history.is_recording() { return; }
    // Preserve the original first/second reader short circuit. read().count()
    // consumes the entire unread stream, rather than a single event.
    let instant = instant_decorators.read().count() != 0
        || instant_stairs.read().count() != 0;
    let ui_finished = ui.as_deref().is_some_and(|ui| ui.hover_byte() > 1
        && ui.click_byte() < 2 && active_widget_records_history(ui));
    // Third reader is visited even after an earlier trigger succeeds.
    let forced = finishes.read().count() != 0;
    if instant || ui_finished || forced {
        let decorator_diff = archivist.extract_diff();
        let stairs_diff = stairs.extract_diff_and_make_backup();
        // Reuse the current add_decorator_diff API. Its temporary set mutates
        // only removals; retain those removals in the existing map owner.
        let mut removed_edges = supports.edges.keys().copied().collect();
        history.add_decorator_diff(&decorator_diff, &stairs_diff, &mut removed_edges,
            crate::resources::stairs::stairs_state::VERIFY_STAIR_CHECKSUMS
                .load(std::sync::atomic::Ordering::Relaxed));
        supports.edges.retain(|edge, _| removed_edges.contains(edge));
        let commit = history.finish_input_recording()
            .expect("original history recording commit invariants");
        if commit.is_some() {
            // B finish leaf writes sync niche/+8=1; actual original input
            // consumer dispatches it to on_history_sync(record=true).
            input.write(crate::systems::backend::input::input_record_replay::InputReplayCommand::Sync { record: true });
        }
    }
    // B @1409dd9e6 applies this even while an open drag remains uncommitted.
    storage.mark_immutable();
}

/// Necessary local CPU scheduling adapters. Complete native archive restoration
/// remains in the existing upper decorator owner. The frame prefix below calls
/// its already recovered storage-permission method only; it owns no new state.
pub fn add_recording_systems<S: Send + Sync + 'static>(schedule: &mut bevy_ecs::schedule::Schedule) {
    use crate::systems::{backend::input::InputSystemSet, history::HistoryCpuStage, terrain::{ui_draw_terrain::ui_draw_terrain, ui_edit_terrain::ui_edit_terrain}};
    schedule.add_systems(
        (|mut storage: ResMut<crate::resources::walls::decorator_storage::DecoratorStorage>| storage.mark_mutable())
            .before(InputSystemSet::Derive)
            .before(HistoryCpuStage::Cursor)
            .before(ui_draw_terrain).before(ui_edit_terrain)
            .before(history_start_recording::<S>),
    );
    schedule.add_systems(history_start_recording::<S>
        .after(ui_draw_terrain).after(ui_edit_terrain)
        .before(crate::systems::ui_clutter::rmb_autoclutter::handle_rmb_autoclutter::<S>)
        .before(HistoryCpuStage::Record));
    schedule.add_systems(history_finish_recording_system::<S>
        .after(HistoryCpuStage::Record).before(InputSystemSet::Finish));
}
