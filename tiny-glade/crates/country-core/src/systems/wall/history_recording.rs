//! Wall compound records enter the same history transaction as roof/raster.
use crate::{
    recovered_building::{
        wall_history_edit::{
            WallCompoundHistoryEdit, WallSnapshotError, record_deleted_wall, record_new_wall,
        },
        wall_state::WallId as RecoveredWallId,
    },
    resources::{
        history::{EditType, History},
        walls::{WallId, WallState},
    },
    systems::history::recording::HistoryRecordingFailed,
};
use bevy_ecs::prelude::*;
use std::sync::Arc;

/// Live edit owners emit this once. Replay writes InnerWalls directly and does
/// not emit a second recording request. The color comes from its real owner.
#[derive(Event, Clone, Debug)]
pub struct RecordWallHistory(pub Arc<WallCompoundHistoryEdit>);

impl RecordWallHistory {
    /// Original seed-create followed by full-state growth, preserving both
    /// concurrent-edit groups and every captured source metadata byte.
    pub fn new_wall(
        wall: WallId,
        source: &WallState,
        color_id: u32,
    ) -> Result<Self, WallSnapshotError> {
        record_new_wall(RecoveredWallId(wall.0), source.snapshot(), color_id)
            .map(|edit| Self(Arc::new(edit)))
    }

    /// Pass the actual removed authority returned by InnerWalls::remove. No
    /// public-cache approximation or newly invented color is captured.
    pub fn deleted_wall(wall: WallId, source: &WallState, color_id: u32) -> Self {
        Self(Arc::new(record_deleted_wall(
            RecoveredWallId(wall.0),
            source.snapshot(),
            color_id,
        )))
    }
}

pub fn register_wall_recording_events(world: &mut World) {
    world.init_resource::<Events<RecordWallHistory>>();
}

pub fn record_wall_history<S: Send + Sync + 'static>(
    mut events: EventReader<RecordWallHistory>,
    mut history: ResMut<History<S>>,
    mut failures: EventWriter<HistoryRecordingFailed>,
) {
    for event in events.read() {
        let edit = EditType::Wall(event.0.clone());
        if let Err(error) = history.add(edit.clone()) {
            failures.write(HistoryRecordingFailed { edit, error });
        }
    }
}
