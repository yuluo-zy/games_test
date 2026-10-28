pub mod history_process {
    pub use crate::systems::mode_manager::HistoryCommand as HistoryCmd;

    /// Original arg7 writer: emitted after a queued command is taken and
    /// before cursor selection @140af6017..6058. Native stride8 is EventId
    /// only. HistoryCmd byte3 belongs to the separate PlayReplayAudio writer.
    #[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub struct NewReplayCommandEvent;
    /// B draw_raster emits this payload-free event before the tool's history
    /// callback. Event consumption remains with the history processor.
    #[derive(bevy_ecs::event::Event, Clone, Copy, Debug)]
    pub struct DebugHistoryMsg;
}

pub mod finish_replay;
pub mod recording;
pub mod replay_auto_clutter;
pub mod replay_color;
pub mod replay_decorator;
pub mod replay_mode;
pub mod replay_raster;
pub mod replay_stairs;
pub mod replay_wall;
#[cfg(test)]
mod tests;

use crate::resources::history::History;
use crate::resources::history::HistoryReplayError;
use crate::resources::history::ReplayBucket;
use crate::resources::history::ReplayBuffer;
use crate::resources::history::transactions::HistoryCursorStep;
use crate::systems::mode_manager::HistoryCommand;
use bevy_ecs::prelude::*;
use std::collections::VecDeque;

/// The original command queue persists while a replay bucket is active.
/// Bevy events are only transport into this queue, not the queue's storage.
#[derive(Resource, Default, Debug)]
pub struct PendingHistoryCommands(pub VecDeque<HistoryCommand>);

/// Compatibility adapter for previously captured native EventInstance bytes.
/// The production transport is the four-byte HistoryCmd; it does not contain
/// this sixteen-byte Bevy allocation/bookmark header.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HistoryControlRecord {
    pub bytes: [u8; 16],
}
impl HistoryControlRecord {
    /// 跳表判别字节 @+9。
    pub fn tag(&self) -> u8 {
        self.bytes[9]
    }
    /// 载荷字节 @+0xb（每记录处理结果事件的载荷字节；原版 ==3 跳过）。
    pub fn flag(&self) -> u8 {
        self.bytes[11]
    }
}

/// Legacy captured-EventInstance transport adapter. This is not a second
/// original event stream: B's DebugHistoryMsg parameter is unused in this body.
#[derive(Resource, Default, Debug)]
pub struct PendingHistoryControl(pub VecDeque<HistoryControlRecord>);

/// Compatibility name for the original entry-phase audio notification.
pub use crate::systems::audio::PlayReplayAudio as HistoryControlProcessed;

/// Entry effects from original B's three distinct writers. Native arg6 is
/// PlayReplayAudio, arg7 NewReplayCommandEvent, arg8 InterruptTimelapse.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HistoryCommandEffects {
    pub audio: Option<crate::systems::audio::PlayReplayAudio>,
    pub interrupt_timelapse: bool,
}

/// Queue clearing and animation policy @140af5cfd..5e6c. The fourth byte is
/// the audio kind; value3 suppresses audio only. NewReplay is emitted later,
/// after the actual queue pop, including for quiet load/replay commands.
pub fn accept_history_command_effects(
    command: HistoryCommand,
    pending: &mut PendingHistoryCommands,
    replay: &mut ReplayBuffer,
) -> HistoryCommandEffects {
    let clear = match command.bytes[1] {
        0 => pending.0.back().is_some_and(|last| last.bytes[1] == 1),
        1 => pending.0.back().is_some_and(|last| last.bytes[1] == 0),
        2 => true,
        3 => {
            replay.make_animations_instant();
            true
        }
        _ => false,
    };
    if clear {
        // Native @140af5df8 emits InterruptTimelapse for precisely these
        // branches, then clears the pending queue @5e4d/5e55.
        pending.0.clear();
    } else {
        pending.0.push_back(command);
    }
    HistoryCommandEffects {
        audio: (command.bytes[3] != 3).then_some(crate::systems::audio::PlayReplayAudio {
            kind: command.bytes[3],
        }),
        interrupt_timelapse: clear,
    }
}

/// Existing pure caller adapter over the same entry policy. Runtime uses all
/// effects above; this API returns the entry audio notification only.
pub fn accept_history_command(
    command: HistoryCommand,
    pending: &mut PendingHistoryCommands,
    replay: &mut ReplayBuffer,
) -> Option<crate::systems::audio::PlayReplayAudio> {
    accept_history_command_effects(command, pending, replay).audio
}

/// Decode old captured EventInstance bytes with the same entry-effects body.
/// Original @+9 is HistoryCmd byte1 after its EventId, not another stream's
/// discriminant. Runtime uses history_process_system with the typed event.
pub fn process_history_control_stream(
    mut pending: ResMut<PendingHistoryCommands>,
    mut controls: ResMut<PendingHistoryControl>,
    mut replay: ResMut<ReplayBuffer>,
    mut processed: EventWriter<HistoryControlProcessed>,
    mut interrupts: EventWriter<crate::systems::timelapse_replay::InterruptTimelapse>,
) {
    for record in controls.0.drain(..) {
        let command = HistoryCommand {
            bytes: record.bytes[8..12].try_into().unwrap(),
        };
        let effects = accept_history_command_effects(command, &mut pending, &mut replay);
        if effects.interrupt_timelapse {
            interrupts.write(crate::systems::timelapse_replay::InterruptTimelapse);
        }
        if let Some(event) = effects.audio {
            processed.write(event);
        }
    }
}

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

/// Isolated transport adapter. It has no History owner and cannot implement
/// the original recording return; runtime uses history_process_system.
pub fn enqueue_history_commands(
    mut events: EventReader<HistoryCommand>,
    mut pending: ResMut<PendingHistoryCommands>,
    mut replay: ResMut<ReplayBuffer>,
    mut audio: EventWriter<crate::systems::audio::PlayReplayAudio>,
    mut interrupts: EventWriter<crate::systems::timelapse_replay::InterruptTimelapse>,
) {
    for command in events.read().copied() {
        let effects = accept_history_command_effects(command, &mut pending, &mut replay);
        if effects.interrupt_timelapse {
            interrupts.write(crate::systems::timelapse_replay::InterruptTimelapse);
        }
        if let Some(event) = effects.audio {
            audio.write(event);
        }
    }
}

/// Original B `history_process` @140af5b30. The entry event iterator and
/// queued replay selection are one system: recording consumes the current
/// event and returns from the whole function, leaving later events unread.
/// With no new event, the original proceeds directly to queue selection.
pub fn history_process_system<S: Send + Sync + 'static>(
    mut history: ResMut<History<S>>,
    mut pending: ResMut<PendingHistoryCommands>,
    mut replay: ResMut<ReplayBuffer>,
    mut events: EventReader<HistoryCommand>,
    _debug: EventReader<history_process::DebugHistoryMsg>,
    mut audio: EventWriter<crate::systems::audio::PlayReplayAudio>,
    mut started: EventWriter<history_process::NewReplayCommandEvent>,
    mut interrupts: EventWriter<crate::systems::timelapse_replay::InterruptTimelapse>,
    bulldoze: Res<crate::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,
    mut failures: EventWriter<HistoryReplayFailed>,
) {
    for command in events.read().copied() {
        // @5ce9 increments this event reader before @5cf3 tests the same
        // pending owner as History::is_recording @140a86970. The taken branch
        // @5ea8 ends at @5f9c, before both entry effects and queue pop.
        if history.is_recording() {
            log::warn!(
                target: "country_core::systems::history::history_process",
                "Can't process History while recording a new edit is active!"
            );
            return;
        }
        let effects = accept_history_command_effects(command, &mut pending, &mut replay);
        if effects.interrupt_timelapse {
            interrupts.write(crate::systems::timelapse_replay::InterruptTimelapse);
        }
        if let Some(event) = effects.audio {
            audio.write(event);
        }
    }
    advance_history_cursor(
        &mut history,
        &mut replay,
        &mut pending,
        &bulldoze,
        &mut failures,
        &mut started,
    );
}

/// Verified ordinary Undo/Redo branch at B 0x140af5f81..0x140af6134.
/// Native load commands enter through the same four-byte event transport.
/// Isolated queue-consumer adapter; runtime registers history_process_system.
pub fn process_history_cursor_commands<S: Send + Sync + 'static>(
    mut history: ResMut<History<S>>,
    mut replay: ResMut<ReplayBuffer>,
    mut pending: ResMut<PendingHistoryCommands>,
    bulldoze: Res<crate::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,
    mut failures: EventWriter<HistoryReplayFailed>,
    mut started: EventWriter<history_process::NewReplayCommandEvent>,
) {
    advance_history_cursor(
        &mut history,
        &mut replay,
        &mut pending,
        &bulldoze,
        &mut failures,
        &mut started,
    );
}

fn advance_history_cursor<S: Send + Sync + 'static>(
    history: &mut History<S>,
    replay: &mut ReplayBuffer,
    pending: &mut PendingHistoryCommands,
    bulldoze: &crate::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer,
    failures: &mut EventWriter<HistoryReplayFailed>,
    started: &mut EventWriter<history_process::NewReplayCommandEvent>,
) {
    // B @140af5f81..f9a: a live replay bucket OR a live wall bulldoze
    // animation blocks taking the next queued history command.
    if !replay.is_empty() || !bulldoze.0.is_empty() {
        return;
    }
    let Some(command) = pending.0.pop_front() else {
        return;
    };
    let bytes = command.bytes;
    if bytes[0] == 2 || bytes[1] == 4 {
        return;
    }
    // Original @6017..6058 occurs before Undo/Redo branch/bounds checks.
    started.write(history_process::NewReplayCommandEvent);
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
    let instant = bytes[2] & 1 != 0;
    // 原生 replay_edit 0x140af6170 的 aux Arc（History +0x18 decorator / +0x20
    // stairs）随桶携带，pop 时由 replay_pop_bucket 恢复。
    let aux = crate::resources::history::replay_cmd_buffer::ReplayBucketAux {
        decorator_diff: history.decorator_diff.clone(),
        stairs_diff: history.stairs_diff.clone(),
    };
    match history.cursor_step_expanding(step, instant, |index, entry, undo, instant| {
        ReplayBucket::expand_with_aux(index, entry, undo, instant, aux.clone())
    }) {
        Ok(Some(bucket)) => replay.push_with_aux(bucket, aux),
        Ok(None) => {}
        Err(error) => {
            failures.write(HistoryReplayFailed {
                command: Some(command),
                error,
            });
        }
    }
}

/// 仅接受空命令桶的 Pop 阶段：弹桶并同步弹出 aux。
/// 完整 `replay_pop_bucket`（装饰/楼梯 diff 恢复 + collision flush）见
/// [`replay_pop_bucket`]。
pub fn pop_empty_replay_bucket(
    mut replay: ResMut<ReplayBuffer>,
    mut finished: EventWriter<ReplayBucketFinished>,
) {
    if replay
        .buckets
        .front()
        .is_some_and(|bucket| bucket.commands.is_empty())
    {
        let (bucket, _aux) = replay.pop_front_bucket().unwrap();
        finished.write(ReplayBucketFinished {
            entry_index: bucket.entry_index,
            undo: bucket.undo,
        });
    }
}

/// 原生 `DecoratorArchivist::extract_diff` 的 history 侧调用面（walls 域类型，
/// 以 trait 解耦，避免 history 依赖其泛型参数）。
pub trait ReplayDiffExtractor {
    fn extract_replay_diff(&mut self);
}
impl<C: Send + Sync + 'static> ReplayDiffExtractor
    for crate::resources::walls::decorator_archivist::changes::DecoratorArchivist<C>
{
    fn extract_replay_diff(&mut self) {
        let _ = self.extract_diff();
    }
}

/// B `replay_pop_bucket` 0x140b38250（fresh 逐段）：
///
/// - 前桶**命令非空**：只执行 `DecoratorArchivist::extract_diff`（结果 map 立即
///   drop），不弹桶、不 flush；
/// - 前桶命令空：`ReplayBuffer::pop_front` → 若 aux decorator diff Arc 非空
///   `DecoratorStorage::restore_decorators` → 若 aux stairs diff Arc 非空
///   `StairsState::restore_backup` / `apply_diff` / `iter_edges_and_stubs` /
///   两张 IndexMap clear（类型未建模，登记）/ `+0x178=0`（字段未锚定，登记）/
///   `make_backup` → drop ReplayBucket → `extract_diff` + drop map →
///   `RaycastWorld::flush_colliders`。
///
/// 原生 aux 两个 Arc 与桶同生命周期；Rust 侧由 [`ReplayBucketAux`] 并行携带。
pub fn replay_pop_bucket<C, A>(
    replay: &mut ReplayBuffer,
    storage: &mut crate::resources::walls::decorator_storage::DecoratorStorage,
    id_gen: &mut crate::resources::walls::decorator::DecoratorIdGen,
    colliders: &mut C,
    events: &crate::resources::walls::decorator::DeferredDecoratorOpEvents,
    stairs: &mut crate::resources::stairs::stairs_state::StairsState,
    archivist: &mut A,
    raycast: &mut crate::systems::collision::world::RaycastWorld,
) -> Option<ReplayBucketFinished>
where
    C: crate::resources::walls::decorator_storage::DecoratorColliderRemover,
    A: ReplayDiffExtractor,
{
    let front_empty = replay
        .buckets
        .front()
        .is_some_and(|bucket| bucket.commands.is_empty());
    if !front_empty {
        if replay.buckets.is_empty() {
            return None;
        }
        archivist.extract_replay_diff();
        return None;
    }
    let (bucket, aux) = replay.pop_front_bucket()?;
    if let Some(diff) = aux.decorator_diff {
        storage.restore_decorators(&diff, bucket.undo, id_gen, colliders, events);
        // 原生 Arc 递减后 drop。
    }
    if let Some(diff) = aux.stairs_diff {
        stairs.restore_backup();
        stairs.apply_diff(&diff, bucket.undo);
        stairs.iter_edges_and_stubs(|_, _| {});
        // 原生随后 clear 两张 IndexMap：
        //   <DecoratorId, Vec<(DecoratorId, CompactDirection)>> 与
        //   <(SubworldColliderHandle, SubworldColliderHandle), StructuralGraphConnection>。
        // 两类型在 crate 内未建模（stairs/structural_graph 域），登记 U。
        // 然后 stairs +0x178 = 0（字段归属未锚定，登记 U）。
        stairs.make_backup();
    }
    // ReplayBucket drop 后 archivist 提取并 drop diff map。
    archivist.extract_replay_diff();
    raycast.flush_colliders();
    Some(ReplayBucketFinished {
        entry_index: bucket.entry_index,
        undo: bucket.undo,
    })
}

/// Production ECS consumer of the same front bucket. B @140b38250 always
/// drains the actual decorator archive and flushes the actual collision world
/// after popping; the old local CPU `pop_empty_replay_bucket` omits these.
/// Nonempty stair diffs require the unresolved structural-connection owners,
/// so this path rejects them before popping rather than losing their payload.
#[cfg(feature = "native-physics")]
pub fn replay_pop_bucket_system(
    mut replay: ResMut<ReplayBuffer>,
    mut storage: ResMut<crate::resources::walls::decorator_storage::DecoratorStorage>,
    mut id_gen: ResMut<crate::resources::walls::decorator::DecoratorIdGen>,
    mut physics: ResMut<crate::resources::walls::decorator_collision::DecoratorPhysicsColliders>,
    events: Res<crate::resources::walls::decorator::DeferredDecoratorOpEvents>,
    mut archivist: ResMut<crate::resources::walls::decorator_archivist::LiveDecoratorArchivist>,
    mut raycast: ResMut<crate::systems::collision::world::RaycastWorld>,
    mut finished: EventWriter<ReplayBucketFinished>,
) -> bevy_ecs::error::Result {
    let Some(front) = replay.buckets.front() else {
        return Ok(());
    };
    if !front.commands.is_empty() {
        drop(archivist.extract_diff());
        return Ok(());
    }
    if replay
        .aux
        .front()
        .is_some_and(|aux| aux.stairs_diff.is_some())
    {
        return Err(anyhow::anyhow!(
            "B replay_pop_bucket @140b38250: stair diff structural-connection maps are not restored"
        )
        .into());
    }
    let (bucket, aux) = replay.pop_front_bucket().expect("front bucket exists");
    if let Some(diff) = aux.decorator_diff {
        let physics = &mut *physics;
        let mut colliders = crate::resources::walls::decorator_storage::DecoratorColliderMaps {
            world: &mut raycast,
            collision_a: &mut physics.first,
            collision_b: &mut physics.second,
        };
        storage.restore_decorators(&diff, bucket.undo, &mut id_gen, &mut colliders, &events);
    }
    let completed = ReplayBucketFinished {
        entry_index: bucket.entry_index,
        undo: bucket.undo,
    };
    drop(bucket);
    drop(archivist.extract_diff());
    raycast.flush_colliders();
    finished.write(completed);
    Ok(())
}

#[cfg(feature = "native-physics")]
pub fn history_clear_previous_state(
    mut archivist: ResMut<crate::resources::walls::decorator_archivist::LiveDecoratorArchivist>,
    mut events: EventReader<history_process::NewReplayCommandEvent>,
) {
    // Original inline body @141439e7b..ecc clears ONLY the permitted-ID set
    // at DecoratorArchivist+0x30. It does not drain tracked decorators or
    // collision archives (the latter belongs to replay_pop_bucket).
    for _ in events.read() {
        archivist.culling.allowed_ids.clear();
    }
}

pub fn register_history_events(world: &mut World) {
    world.init_resource::<PendingHistoryCommands>();
    world.init_resource::<PendingHistoryControl>();
    world.init_resource::<crate::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>();
    world.init_resource::<Events<HistoryCommand>>();
    world.init_resource::<Events<HistoryReplayFailed>>();
    world.init_resource::<Events<ReplayBucketFinished>>();
    world.init_resource::<Events<HistoryControlProcessed>>();
    world.init_resource::<Events<history_process::NewReplayCommandEvent>>();
    world.init_resource::<Events<crate::systems::timelapse_replay::InterruptTimelapse>>();
    world.init_resource::<Events<replay_raster::RasterReplayFeedback>>();
    world.init_resource::<Events<replay_color::ColorHistoryReplayApply>>();
    world.init_resource::<Events<replay_decorator::DecoratorHistoryReplayApply>>();
    world.init_resource::<Events<replay_mode::ReplayFocusEvent>>();
    world.init_resource::<Events<history_process::DebugHistoryMsg>>();
    world.init_resource::<replay_mode::ReplayFastHoldInput>();
    world.init_resource::<crate::input::action_set::MainActionSet>();
    world.init_resource::<replay_mode::HoldToFastReplay>();
    world.init_resource::<replay_mode::ReplayModeElapsedTime>();
    world.init_resource::<Events<crate::systems::mode_manager::AppModeChanged>>();
    world.init_resource::<Events<finish_replay::CmdUnfreezeSheep>>();
    world.init_resource::<Events<crate::systems::mode_manager::ClearCanvasCmd>>();
    recording::register_recording_events(world);
    replay_wall::register_wall_replay_events(world);
    crate::resources::roofs::register_roof_events(world);
    crate::systems::ui_clutter::registration::register_ui_clutter_resources(world);
    world
        .init_resource::<crate::resources::stairs::stairs_removed_supports::StairsRemovedSupports>(
        );
}

/// A runnable local CPU order. History<S>, ReplayBuffer speed, and GameTime
/// remain supplied by their actual owners. This is not the original global
/// engine schedule. Owner-specific wall/raster replay systems join Replay.
pub fn add_history_cpu_systems<S: Send + Sync + 'static>(schedule: &mut Schedule) {
    // Consume this frame's resolved/replayed actions before advancing its input
    // replay cursor. These are local adapter data dependencies, not a recovered
    // claim about the original game's complete global schedule.
    schedule.configure_sets(
        HistoryCpuStage::Cursor.after(crate::systems::backend::input::InputSystemSet::Derive),
    );
    schedule.configure_sets(
        HistoryCpuStage::Record.before(crate::systems::backend::input::InputSystemSet::Finish),
    );
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
    schedule.add_systems(history_process_system::<S>.in_set(HistoryCpuStage::Cursor));
    schedule.add_systems(replay_stairs::replay_stairs.in_set(HistoryCpuStage::Replay));
    schedule.add_systems(replay_auto_clutter::replay_auto_clutter.in_set(HistoryCpuStage::Replay));
    schedule.add_systems(replay_wall::replay_wall.in_set(HistoryCpuStage::Replay));
    schedule.add_systems(replay_raster::replay_rasters::<S>.in_set(HistoryCpuStage::Replay));
    schedule.add_systems(replay_color::replay_color.in_set(HistoryCpuStage::Replay));
    schedule.add_systems(replay_decorator::replay_decorator.in_set(HistoryCpuStage::Replay));
    schedule.add_systems(
        replay_mode::replay_mode
            .after(crate::systems::mode_manager::ModeChangePublication)
            .in_set(HistoryCpuStage::Replay),
    );
    schedule.add_systems(pop_empty_replay_bucket.in_set(HistoryCpuStage::Pop));
    schedule.add_systems(
        finish_replay::finish_replay
            .after(pop_empty_replay_bucket)
            .in_set(HistoryCpuStage::Pop),
    );
    crate::systems::ui_clutter::registration::add_ui_clutter_cpu_systems::<S>(schedule);
    schedule.add_systems(
        (
            recording::record_roof_history::<S>,
            recording::record_roof_visual_history::<S>,
            recording::record_roof_direction_history::<S>,
            crate::systems::wall::history_recording::record_wall_history::<S>,
        )
            .chain()
            .in_set(HistoryCpuStage::Record),
    );
}

pub mod wall_replay_interpreter;

pub mod wall_replay_transition;
