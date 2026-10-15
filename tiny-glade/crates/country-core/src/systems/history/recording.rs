//! Roof recording writes the same History owner used by mode_manager and
//! replay. UI record notifications are separate from live edit commands so
//! replaying an edit does not record another copy of itself.
use crate::{
    recovered_building::{
        building_history_transactions::{BuildingEdit, RoofHistoryEdit, RoofVisualHistoryRecord},
        history_transactions::HistoryTransactionError,
    },
    resources::{
        history::{EditType, History},
        roofs::commands::RecordRoofHistory,
    },
};
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
}
