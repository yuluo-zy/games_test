use super::{
    DeletedAutoClutter,
    defered_autoclutter_history::{RecordAutoClutterHistory, defered_autoclutter_history},
    edit_autoclutter::{DeleteAutoClutterCmd, delete_auto_clutter},
};
use crate::systems::history::{HistoryCpuStage, recording::HistoryRecordingFailed};
use bevy_ecs::prelude::*;

pub fn register_ui_clutter_resources(world: &mut World) {
    world.init_resource::<DeletedAutoClutter>();
    world.init_resource::<Events<DeleteAutoClutterCmd>>();
    world.init_resource::<Events<RecordAutoClutterHistory>>();
    world.init_resource::<Events<HistoryRecordingFailed>>();
}

/// Local resolved-ID edit/record order; caller maintains events and supplies
/// History<S>. The global original UI schedule has not been reconstructed.
pub fn add_ui_clutter_cpu_systems<S: Send + Sync + 'static>(schedule: &mut Schedule) {
    schedule.add_systems(
        (delete_auto_clutter::<S>, defered_autoclutter_history::<S>)
            .chain()
            .in_set(HistoryCpuStage::Record),
    );
}
