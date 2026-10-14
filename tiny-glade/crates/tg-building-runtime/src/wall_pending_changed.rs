//! UNVERIFIED parallel actual PendingWallChanged/deferred-history runtime owner.
//! Upstream freehand UI's next Source is resolved input, not guessed geometry.
use crate::{
    BuildingReport, UpsertRawWall, WallCollectionState,
    history::{BuildingHistory, BuildingHistoryRequests},
    wall_authoring::WallAuthoringState,
};
use bevy_ecs::prelude::*;
use country_core::recovered_building::{
    building_history_transactions::BuildingEdit,
    building_wall_history_edit::{WallAuthoritySnapshot, WallCompoundHistoryEdit},
    building_wall_pending_changed::{
        PendingWallChanged, take_pending_wall_changed, try_pending_wall_changed,
    },
    building_wall_state::WallId,
};
#[derive(Event, Clone)]
pub struct ApplyObservedFreeWallChange {
    pub wall: WallId,
    pub next: WallAuthoritySnapshot,
    pub extend_start: bool,
}
#[derive(Resource, Default)]
pub struct WallPendingChangedState {
    /// Original deferred owner has one pending slot, not a fabricated ledger.
    pub pending: Option<PendingWallChanged>,
    pub updated_this_frame: Vec<WallId>,
    pub recorded_this_frame: usize,
    pub unrecorded_history: Vec<WallCompoundHistoryEdit>,
}
pub fn register(app: &mut bevy_app::App) {
    app.init_resource::<WallPendingChangedState>()
        .add_event::<ApplyObservedFreeWallChange>();
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
    for event in changes.read() {
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
    if let Some(owner) = take_pending_wall_changed(&mut state.pending) {
        if history
            .transactions
            .as_ref()
            .is_some_and(|t| t.is_recording())
        {
            owners
                .0
                .push(BuildingEdit::Wall(std::sync::Arc::new(owner)));
            state.recorded_this_frame = 1;
        } else {
            state.unrecorded_history.push(owner);
        }
    }
}
