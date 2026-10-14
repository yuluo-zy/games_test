//! UNVERIFIED parallel UI80 → finish/Enclosure history/cancel runtime bridge.
use crate::{
    BuildingReport, WallCollectionState,
    history::{BuildingHistory, BuildingHistoryRequests},
    wall_authoring::WallAuthoringState,
    wall_decorator_archivist::{WallDecoratorArchiveState, restore_for_cancel},
};
use bevy_ecs::prelude::*;
use country_core::recovered_building::{
    building_history_transactions::BuildingEdit,
    building_wall_history_edit::WallCompoundHistoryEdit,
    building_wall_rectangle_edit::{RectangleEditState, finish_rectangle_edit_decision},
    building_wall_state::WallId,
};
#[derive(Event, Clone, Copy)]
pub struct ObserveRectangleEditUi(pub [u8; 80]);
#[derive(Event, Clone, Copy)]
pub struct FinishRectangleEdit;
#[derive(Event, Clone, Copy)]
pub struct RectangleEditActiveSnapshot(pub [u8; 80]);
#[derive(Event, Clone, Copy)]
pub struct RectangleEditFinished(pub WallId);
#[derive(Event, Clone, Copy)]
pub struct RectangleEditEmptyNotification;
#[derive(Resource, Default)]
pub struct WallRectangleEditState {
    pub ui: Option<RectangleEditState>,
    pub finish_pending: bool,
    /// Adapter retries remain bound to their original observed UI state.
    pending_ui: Option<RectangleEditState>,
    queued_ui: Option<RectangleEditState>,
    pub completed_this_frame: Vec<WallId>,
    /// Old observations are retained but never admitted by a later Start.
    pub unrecorded_history: Vec<WallCompoundHistoryEdit>,
    pub pending_reason: Option<String>,
}
pub fn register(app: &mut bevy_app::App) {
    app.init_resource::<WallRectangleEditState>()
        .add_event::<ObserveRectangleEditUi>()
        .add_event::<FinishRectangleEdit>()
        .add_event::<RectangleEditActiveSnapshot>()
        .add_event::<RectangleEditFinished>()
        .add_event::<RectangleEditEmptyNotification>();
}
pub(crate) fn consume_and_finish(
    mut ui: EventReader<ObserveRectangleEditUi>,
    mut finish: EventReader<FinishRectangleEdit>,
    mut state: ResMut<WallRectangleEditState>,
    mut archive: ResMut<WallDecoratorArchiveState>,
    authoring: Res<WallAuthoringState>,
    collection: Res<WallCollectionState>,
    history: Res<BuildingHistory>,
    mut owners: ResMut<BuildingHistoryRequests>,
    mut snapshots: EventWriter<RectangleEditActiveSnapshot>,
    mut finished: EventWriter<RectangleEditFinished>,
    mut empty: EventWriter<RectangleEditEmptyNotification>,
    mut report: ResMut<BuildingReport>,
) {
    state.completed_this_frame.clear();
    state.pending_reason = None;
    for event in ui.read() {
        if state.finish_pending && state.pending_ui.is_some() {
            state.queued_ui = Some(RectangleEditState(event.0));
        } else {
            state.ui = Some(RectangleEditState(event.0));
        }
    }
    if finish.read().count() != 0 {
        state.finish_pending = true;
    }
    if state.finish_pending && state.pending_ui.is_none() {
        state.pending_ui = state.ui.clone();
    }
    let Some(observed) = state.pending_ui.as_ref().or(state.ui.as_ref()) else {
        if state.finish_pending {
            state.pending_reason = Some("Rectangle finish needs actual UiState80".into());
        }
        return;
    };
    let wall = observed.wall();
    let source = wall.and_then(|wall| {
        if authoring.removed_sources.contains(&wall) {
            None
        } else {
            authoring
                .staged_sources
                .get(&wall)
                .or_else(|| collection.raw_sources.get(&wall))
        }
    });
    let mut next = observed.clone();
    let decision = match finish_rectangle_edit_decision(&mut next, state.finish_pending, source) {
        Ok(out) => out,
        Err(error) => {
            let fault = format!("Rectangle finish pending: {error:?}");
            state.pending_reason = Some(fault.clone());
            report.faults.push(fault);
            return;
        }
    };
    if let Some(snapshot) = decision.active_state_snapshot {
        snapshots.write(RectangleEditActiveSnapshot(snapshot));
    }
    if !state.finish_pending {
        return;
    }
    if decision.restore_archived_decorators
        && let Err(error) = restore_for_cancel(&mut archive)
    {
        let fault = format!("Rectangle cancel pending: {error:?}");
        state.pending_reason = Some(fault.clone());
        report.faults.push(fault);
        return;
    }
    if let Some(owner) = decision.history {
        if history
            .transactions
            .as_ref()
            .is_some_and(|t| t.is_recording())
        {
            owners
                .0
                .push(BuildingEdit::Wall(std::sync::Arc::new(owner)));
        } else {
            state.unrecorded_history.push(owner);
        }
    }
    if decision.empty_notification {
        empty.write(RectangleEditEmptyNotification);
    }
    if let Some(wall) = decision.wall_notification {
        finished.write(RectangleEditFinished(wall));
    }
    if let Some(wall) = wall {
        state.completed_this_frame.push(wall);
    }
    state.ui = state.queued_ui.take().or(Some(next));
    state.pending_ui = None;
    state.finish_pending = false;
}
