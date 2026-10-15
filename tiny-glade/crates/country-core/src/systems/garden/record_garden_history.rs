//! B 0x141695e10 consumes all pending notifications and records slot five once.
use crate::{
    resources::{
        active_raster_stroke::ActiveRasterStrokesStorage,
        history::{EditType, History},
    },
    systems::{history::recording::HistoryRecordingFailed, world_raster::record_raster_history},
};
use bevy_ecs::prelude::*;

#[derive(Event, Clone, Copy, Debug)]
pub struct RecordGardenEditToHistoryCmd;

pub fn record_garden_history<S: Send + Sync + 'static>(
    mut events: EventReader<RecordGardenEditToHistoryCmd>,
    mut strokes: ResMut<ActiveRasterStrokesStorage>,
    mut history: ResMut<History<S>>,
    mut failures: EventWriter<HistoryRecordingFailed>,
) {
    if events.read().last().is_none() {
        return;
    }
    let mut attempted = None;
    if let Err(error) = record_raster_history(&mut strokes, 5, &mut history, |stroke| {
        let edit = EditType::Garden(stroke);
        attempted = Some(edit.clone());
        edit
    }) {
        failures.write(HistoryRecordingFailed {
            edit: attempted.unwrap(),
            error,
        });
    }
}
