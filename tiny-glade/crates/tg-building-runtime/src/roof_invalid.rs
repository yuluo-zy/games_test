//! Actual invalid-notification consumer, between association and deletion.
use crate::{BuildingStage, DeleteRoof, Roof, RoofMarkedForDeletion};
use bevy_ecs::prelude::*;
use country_core::recovered_building::{
    building_history_transactions::RoofHistoryEdit,
    building_roof_invalid_notifications::{InvalidRoofRow, process_invalid_roof_notifications},
    building_wall_state::WallId,
};
#[derive(Event, Clone, Copy)]
pub struct RoofIsInvalid(pub WallId);
#[derive(Event, Clone, Copy)]
pub struct ResetRoofNotifications;
#[derive(Resource, Default)]
pub struct RoofInvalidNotificationState {
    pub emitted_this_frame: Vec<WallId>,
    /// Actual original History.add output, retained until full Wall compound
    /// history can encode/replay its other owner. This is not a commit receipt.
    pub history_pending: Vec<RoofHistoryEdit>,
    /// Only operations produced during this active recording are admitted;
    /// retained observations from earlier unrecorded updates are not replayed.
    pub recording_pending: Vec<RoofHistoryEdit>,
}
pub fn register(app: &mut bevy_app::App) {
    app.init_resource::<RoofInvalidNotificationState>()
        .add_event::<RoofIsInvalid>()
        .add_event::<ResetRoofNotifications>()
        .add_systems(
            bevy_app::Update,
            consume.in_set(BuildingStage::InvalidRoofs),
        );
}
fn consume(
    mut notifications: EventReader<RoofIsInvalid>,
    mut resets: EventReader<ResetRoofNotifications>,
    roofs: Query<(&Roof, Has<RoofMarkedForDeletion>)>,
    mut deletes: EventWriter<DeleteRoof>,
    mut state: ResMut<RoofInvalidNotificationState>,
    history: Res<crate::history::BuildingHistory>,
    bulldoze: Res<crate::wall_bulldoze::WallBulldozeState>,
) {
    state.emitted_this_frame.clear();
    let ids: Vec<_> = notifications.read().map(|event| event.0).collect();
    let reset = resets.read().count() != 0;
    if ids.is_empty() && !reset {
        return;
    }
    let rows: Vec<_> = roofs
        .iter()
        .map(|(roof, marked)| InvalidRoofRow {
            roof: &roof.0,
            marked_for_deletion: marked,
        })
        .collect();
    for deletion in process_invalid_roof_notifications(&rows, &ids, reset) {
        deletes.write(DeleteRoof(deletion.wall_id));
        state.emitted_this_frame.push(deletion.wall_id);
        // CPU graph replay already owns its Roof operation. Retain native
        // invalidation/deletion effects without echoing a fresh transaction.
        // The adapter's final-association pass may see a just-bulldozed source
        // before the Mark stage. Its producer already recorded RoofDelete;
        // preserve invalidation cleanup without creating a duplicate owner.
        if history.replayed_this_frame == 0
            && !bulldoze.roof_owners_this_frame.contains(&deletion.wall_id)
            && let Some(mutation) = deletion.history_mutation
        {
            if history
                .transactions
                .as_ref()
                .is_some_and(|t| t.is_recording())
            {
                state.recording_pending.push(mutation.clone());
            }
            state.history_pending.push(mutation);
        }
    }
}
