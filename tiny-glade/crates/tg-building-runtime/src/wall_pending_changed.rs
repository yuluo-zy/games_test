//! Applies explicit freehand source changes and records their previous/next states.
//! The upstream UI supplies the complete next authority snapshot.
use crate::{
    BuildingReport, UpsertRawWall, WallCollectionState,
    history::{BuildingHistory, BuildingHistoryRequests},
    wall_authoring::WallAuthoringState,
};
use bevy_ecs::prelude::*;
use country_core::resources::history::building_transactions::BuildingEdit;
use country_core::resources::history::wall_edit::WallAuthoritySnapshot;
use country_core::resources::history::wall_edit::WallCompoundHistoryEdit;
use country_core::systems::wall::wall_state::pending_changed::PendingWallChanged;
use country_core::systems::wall::wall_state::pending_changed::take_pending_wall_changed;
use country_core::systems::wall::wall_state::pending_changed::try_pending_wall_changed;
use country_core::resources::walls::publication_fields::WallId;
#[derive(Event, Clone)]
pub struct ApplyObservedFreeWallChange {
    pub wall: WallId,
    pub next: WallAuthoritySnapshot,
    pub extend_start: bool,
}
#[derive(Resource, Default)]
pub struct WallPendingChangedState {
    /// The current event is drained before accepting the next change.
    pub pending: Option<PendingWallChanged>,
    pub updated_this_frame: Vec<WallId>,
    pub recorded_this_frame: usize,
    pub unrecorded_history: Vec<WallCompoundHistoryEdit>,
}
pub fn register(app: &mut bevy_app::App) {
    app.init_resource::<WallPendingChangedState>()
        .add_event::<ApplyObservedFreeWallChange>();
}
fn record_pending(
    state: &mut WallPendingChangedState,
    recording: bool,
    owners: &mut BuildingHistoryRequests,
) {
    if let Some(owner) = take_pending_wall_changed(&mut state.pending) {
        if recording {
            owners
                .0
                .push(BuildingEdit::Wall(std::sync::Arc::new(owner)));
            state.recorded_this_frame += 1;
        } else {
            state.unrecorded_history.push(owner);
        }
    }
}

pub(crate) fn consume(
    mut changes: EventReader<ApplyObservedFreeWallChange>,
    mut state: ResMut<WallPendingChangedState>,
    collection: Res<WallCollectionState>,
    mut authoring: ResMut<WallAuthoringState>,
    history: Res<BuildingHistory>,
    mut owners: ResMut<BuildingHistoryRequests>,
    mut upsert: EventWriter<UpsertRawWall>,
    mut report: ResMut<BuildingReport>,
) {
    state.updated_this_frame.clear();
    state.recorded_this_frame = 0;
    let recording = history
        .transactions
        .as_ref()
        .is_some_and(|t| t.is_recording());
    record_pending(&mut state, recording, &mut owners);
    for event in changes.read() {
        if !crate::raw_source_is_valid(&event.next.source) {
            report.faults.push(format!(
                "Pending wall change {} has invalid next authority",
                event.wall.0
            ));
            continue;
        }
        if authoring.removed_sources.contains(&event.wall) {
            report.faults.push(format!(
                "Pending wall change {} targets a source removed this frame",
                event.wall.0
            ));
            continue;
        }
        let Some(raw) = authoring
            .staged_sources
            .get(&event.wall)
            .or_else(|| collection.raw_sources.get(&event.wall))
        else {
            report.faults.push(format!(
                "Pending wall change {} requires actual previous RawSource",
                event.wall.0
            ));
            continue;
        };
        let Some(metadata) = authoring.history_metadata.get(&event.wall).copied() else {
            report.faults.push(format!(
                "Pending wall change {} requires actual previous metadata",
                event.wall.0
            ));
            continue;
        };
        let previous = WallAuthoritySnapshot {
            source: raw.clone(),
            metadata,
        };
        match try_pending_wall_changed(event.wall, previous, event.next.clone(), event.extend_start)
        {
            Ok(pending) => {
                state.pending = pending;
                record_pending(&mut state, recording, &mut owners);
                authoring
                    .history_metadata
                    .insert(event.wall, event.next.metadata);
                authoring.updated_sources.insert(event.wall);
                authoring
                    .staged_sources
                    .insert(event.wall, event.next.source.clone());
                upsert.write(UpsertRawWall {
                    wall: event.wall,
                    source: event.next.source.clone(),
                });
                state.updated_this_frame.push(event.wall);
            }
            Err(error) => report
                .faults
                .push(format!("Pending wall change {}: {error:?}", event.wall.0)),
        }
    }
}
