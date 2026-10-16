pub mod history_process {
    /// B draw_raster emits this payload-free event before the tool's history
    /// callback. Event consumption remains with the history processor.
    #[derive(bevy_ecs::event::Event, Clone, Copy, Debug)]
    pub struct DebugHistoryMsg;
}

pub mod finish_replay;
pub mod recording;
pub mod replay_auto_clutter;
pub mod replay_mode;
pub mod replay_raster;
pub mod replay_roof;
pub mod replay_stairs;
pub mod replay_wall;
#[cfg(test)]
mod tests;

use crate::{
    recovered_building::history_transactions::HistoryCursorStep,
    resources::history::{History, HistoryReplayError, ReplayBucket, ReplayBuffer},
    systems::mode_manager::HistoryCommand,
};
use bevy_ecs::prelude::*;
use std::collections::VecDeque;

/// The original command queue persists while a replay bucket is active.
/// Bevy events are only transport into this queue, not the queue's storage.
#[derive(Resource, Default, Debug)]
pub struct PendingHistoryCommands(pub VecDeque<HistoryCommand>);

#[derive(Event, Clone, Debug)]
pub struct HistoryReplayFailed {
    pub command: Option<HistoryCommand>,
    pub error: HistoryReplayError,
}
#[derive(Event, Clone, Copy, Debug)]
pub struct ReplayBucketFinished {
    pub entry_index: usize,
    pub undo: bool,
}
#[derive(SystemSet, Clone, Debug, PartialEq, Eq, Hash)]
pub enum HistoryCpuStage {
    Cursor,
    Replay,
    Apply,
    Pop,
    Record,
}

pub fn enqueue_history_commands(
    mut events: EventReader<HistoryCommand>,
    mut pending: ResMut<PendingHistoryCommands>,
) {
    pending.0.extend(events.read().copied());
}

/// Verified ordinary Undo/Redo branch at B 0x140af5f81..0x140af6134.
/// The separate 16-byte playback/control event stream is not decoded here.
pub fn process_history_cursor_commands<S: Send + Sync + 'static>(
    mut history: ResMut<History<S>>,
    mut replay: ResMut<ReplayBuffer>,
    mut pending: ResMut<PendingHistoryCommands>,
    mut failures: EventWriter<HistoryReplayFailed>,
) {
    if !replay.is_empty() {
        return;
    }
    let Some(command) = pending.0.pop_front() else {
        return;
    };
    let bytes = command.bytes;
    if bytes[0] == 2 || bytes[1] == 4 {
        return;
    }
    let step = match bytes[1] {
        0 => HistoryCursorStep::Undo,
        1 => HistoryCursorStep::Redo,
        _ => {
            failures.write(HistoryReplayFailed {
                command: Some(command),
                error: HistoryReplayError::UnrestoredCommand(bytes),
            });
            return;
        }
    };
    match history.cursor_step_expanding(step, bytes[2] & 1 != 0, ReplayBucket::expand) {
        Ok(Some(bucket)) => replay.push(bucket),
        Ok(None) => {}
        Err(error) => {
            failures.write(HistoryReplayFailed {
                command: Some(command),
                error,
            });
        }
    }
}

/// Only accepted buckets with empty auxiliary owners reach this operation.
/// Full replay_pop_bucket additionally restores decorator/stair diffs and
/// flushes collision: those nonempty owners are rejected before cursor change.
pub fn pop_empty_replay_bucket(
    mut replay: ResMut<ReplayBuffer>,
    mut finished: EventWriter<ReplayBucketFinished>,
) {
    if replay
        .buckets
        .front()
        .is_some_and(|bucket| bucket.commands.is_empty())
    {
        let bucket = replay.buckets.pop_front().unwrap();
        finished.write(ReplayBucketFinished {
            entry_index: bucket.entry_index,
            undo: bucket.undo,
        });
    }
}

pub fn register_history_events(world: &mut World) {
    world.init_resource::<PendingHistoryCommands>();
    world.init_resource::<Events<HistoryCommand>>();
    world.init_resource::<Events<HistoryReplayFailed>>();
    world.init_resource::<Events<ReplayBucketFinished>>();
    world.init_resource::<Events<replay_raster::RasterReplayFeedback>>();
    world.init_resource::<Events<history_process::DebugHistoryMsg>>();
    recording::register_recording_events(world);
    replay_wall::register_wall_replay_events(world);
    crate::systems::roof::register_roof_events(world);
    crate::systems::ui_clutter::registration::register_ui_clutter_resources(world);
    world.init_resource::<crate::resources::stairs::stairs_removed_supports::StairsRemovedSupports>();
}

/// A runnable local CPU order. History<S>, ReplayBuffer speed, and GameTime
/// remain supplied by their actual owners. This is not the original global
/// engine schedule. Owner-specific wall/raster replay systems join Replay.
pub fn add_history_cpu_systems<S: Send + Sync + 'static>(schedule: &mut Schedule) {
    schedule.configure_sets(
        HistoryCpuStage::Replay.before(crate::systems::wall::registration::WallCpuStage::Edit),
    );
    schedule.configure_sets(
        (
            HistoryCpuStage::Cursor,
            HistoryCpuStage::Replay,
            HistoryCpuStage::Apply,
            HistoryCpuStage::Pop,
            HistoryCpuStage::Record,
        )
            .chain(),
    );
    schedule.add_systems(
        (
            enqueue_history_commands,
            process_history_cursor_commands::<S>,
        )
            .chain()
            .in_set(HistoryCpuStage::Cursor),
    );
    schedule.add_systems(replay_roof::replay_roof.in_set(HistoryCpuStage::Replay));
    schedule.add_systems(replay_stairs::replay_stairs.in_set(HistoryCpuStage::Replay));
    schedule.add_systems(replay_auto_clutter::replay_auto_clutter.in_set(HistoryCpuStage::Replay));
    schedule.add_systems(replay_wall::replay_wall.in_set(HistoryCpuStage::Replay));
    schedule.add_systems(replay_raster::replay_rasters::<S>.in_set(HistoryCpuStage::Replay));
    schedule.add_systems(pop_empty_replay_bucket.in_set(HistoryCpuStage::Pop));
    schedule.add_systems(finish_replay::finish_replay.in_set(HistoryCpuStage::Pop));
    crate::systems::ui_clutter::registration::add_ui_clutter_cpu_systems::<S>(schedule);
    schedule.add_systems(
        (
            recording::record_roof_history::<S>,
            recording::record_roof_visual_history::<S>,
            recording::record_roof_direction_history::<S>,
        )
            .chain()
            .in_set(HistoryCpuStage::Record),
    );
}
