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
    let trace = std::env::var_os("TG_M2_TRACE_HISTORY").is_some();
    // B @140af5f81..f9a: a live replay bucket OR a live wall bulldoze
    // animation blocks taking the next queued history command.
    if !replay.is_empty() || !bulldoze.0.is_empty() {
        return;
    }
    let Some(command) = pending.0.pop_front() else {
        return;
    };
    if trace {
        eprintln!(
            "M2_HISTORY_CONSUME command={:?} cursor_before={} entries={}",
            command.bytes,
            history.ledger().cursor,
            history.ledger().entries.len(),
        );
    }
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
    // Original replay_edit reads the selected MultiEdit's +18/+20 owners.
    // The latest pending diff on History is not an older entry's payload.
    match history.cursor_step_expanding(step, instant, |index, entry, undo, instant| {
        let aux = entry.auxiliary.for_replay(undo);
        ReplayBucket::expand_with_aux(index, entry, undo, instant, aux.clone())
            .map(|bucket| (bucket, aux))
    }) {
        Ok(Some((bucket, aux))) => replay.push_with_aux(bucket, aux),
        Ok(None) => {}
        Err(error) => {
            failures.write(HistoryReplayFailed {
                command: Some(command),
                error,
            });
        }
    }
    if trace {
        eprintln!(
            "M2_HISTORY_CONSUMED command={:?} cursor_after={} entries={}",
            command.bytes,
            history.ledger().cursor,
            history.ledger().entries.len(),
        );
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
    fn extract_replay_diff(
        &mut self,
        storage: &crate::resources::walls::decorator_storage::DecoratorStorage,
    );
}
impl<C: Send + Sync + 'static> ReplayDiffExtractor
    for crate::resources::walls::decorator_archivist::changes::DecoratorArchivist<C>
{
    fn extract_replay_diff(
        &mut self,
        storage: &crate::resources::walls::decorator_storage::DecoratorStorage,
    ) {
        let _ = self.extract_diff(storage, true);
    }
}

/// B `replay_pop_bucket` 0x140b38250（fresh 逐段）：
///
/// - 前桶**命令非空**：只执行 `DecoratorArchivist::extract_diff`（结果 map 立即
///   drop），不弹桶、不 flush；
/// - 前桶命令空：`ReplayBuffer::pop_front` → 若 aux decorator diff Arc 非空
///   `DecoratorStorage::restore_decorators` → 若 aux stairs diff Arc 非空
///   `StairsState::restore_backup` / `apply_diff` / `iter_edges_and_stubs` /
///   backup GraphMap 的两张 IndexMap clear / `backed_up(+0x178)=0` /
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
        archivist.extract_replay_diff(storage);
        return None;
    }
    let (bucket, aux) = replay.pop_front_bucket()?;
    if let Some(diff) = aux.decorator_diff {
        storage.restore_decorators(&diff, bucket.undo, id_gen, colliders, events);
        // 原生 Arc 递减后 drop。
    }
    if let Some(diff) = aux.stairs_diff {
        stairs.restore_replay_diff(&diff, bucket.undo);
    }
    // ReplayBucket drop 后 archivist 提取并 drop diff map。
    archivist.extract_replay_diff(storage);
    raycast.flush_colliders();
    Some(ReplayBucketFinished {
        entry_index: bucket.entry_index,
        undo: bucket.undo,
    })
}

/// Production ECS consumer of the same front bucket. B @140b38250 always
/// drains the actual decorator archive and flushes the actual collision world
/// after popping; the old local CPU `pop_empty_replay_bucket` omits these.
/// Stair aux uses the same StairsState authority and its petgraph backup.
/// Required owners are checked before popping so errors preserve the payload.
///
/// Host-owned resources (physics colliders / raycast world / archivist / event
/// queue / id generator) are optional: when a host has not assembled them the
/// bucket still pops and decorator state still lands in `DecoratorStorage`;
/// collider removal and archive draining are skipped rather than panicking.
/// A decorator diff without its storage/id generator is an error (payload
/// cannot be written back).
#[cfg(feature = "native-physics")]
pub fn replay_pop_bucket_system(
    mut replay: ResMut<ReplayBuffer>,
    mut storage: Option<ResMut<crate::resources::walls::decorator_storage::DecoratorStorage>>,
    id_gen: Option<ResMut<crate::resources::walls::decorator::DecoratorIdGen>>,
    mut physics: Option<
        ResMut<crate::resources::walls::decorator_collision::DecoratorPhysicsColliders>,
    >,
    events: Option<Res<crate::resources::walls::decorator::DeferredDecoratorOpEvents>>,
    archivist: Option<ResMut<crate::resources::walls::decorator_archivist::LiveDecoratorArchivist>>,
    mut raycast: Option<ResMut<crate::systems::collision::world::RaycastWorld>>,
    stairs: Option<ResMut<crate::resources::stairs::stairs_state::StairsState>>,
    mut finished: EventWriter<ReplayBucketFinished>,
) -> bevy_ecs::error::Result {
    let Some(front) = replay.buckets.front() else {
        return Ok(());
    };
    if archivist.is_some() && storage.is_none() {
        return Err(
            anyhow::anyhow!("decorator archive extraction requires DecoratorStorage").into(),
        );
    }
    if !front.commands.is_empty() {
        if let Some(mut archivist) = archivist {
            drop(
                archivist.extract_diff(storage.as_deref().expect("archive storage checked"), true),
            );
        }
        return Ok(());
    }
    let needs_stairs = replay
        .aux
        .front()
        .is_some_and(|aux| aux.stairs_diff.is_some());
    if needs_stairs && stairs.is_none() {
        return Err(anyhow::anyhow!(
            "B replay_pop_bucket @140b38250: stair diff requires StairsState"
        )
        .into());
    }
    // Validate every owner the aux diff needs BEFORE popping; popping first and
    // then failing would drop the bucket (and its diff) on the error path.
    let needs_decorator = replay
        .aux
        .front()
        .is_some_and(|aux| aux.decorator_diff.is_some());
    if needs_decorator && (storage.is_none() || id_gen.is_none()) {
        return Err(anyhow::anyhow!(
            "B replay_pop_bucket @140b38250: decorator diff requires DecoratorStorage and DecoratorIdGen"
        )
        .into());
    }
    let (bucket, aux) = replay.pop_front_bucket().expect("front bucket exists");
    if let Some(diff) = aux.decorator_diff {
        let (Some(mut storage), Some(mut id_gen)) = (storage.as_deref_mut(), id_gen) else {
            unreachable!("required decorator owners validated before pop");
        };
        // 事件队列缺失时用局部队列承接并告警（不静默丢弃可见性）。
        let fallback = crate::resources::walls::decorator::DeferredDecoratorOpEvents::default();
        let deferred = match events.as_deref() {
            Some(events) => events,
            None => {
                log::warn!(
                    target: "country_core::systems::history",
                    "DeferredDecoratorOpEvents missing; replay_pop_bucket diff events are not queued"
                );
                &fallback
            }
        };
        if let (Some(physics), Some(raycast)) = (physics.as_mut(), raycast.as_mut()) {
            let physics: &mut crate::resources::walls::decorator_collision::DecoratorPhysicsColliders =
                &mut *physics;
            let raycast: &mut crate::systems::collision::world::RaycastWorld = &mut *raycast;
            let mut colliders = crate::resources::walls::decorator_storage::DecoratorColliderMaps {
                world: raycast,
                collision_a: &mut physics.first,
                collision_b: &mut physics.second,
            };
            storage.restore_decorators(&diff, bucket.undo, &mut id_gen, &mut colliders, deferred);
        } else {
            log::warn!(
                target: "country_core::systems::history",
                "collision owners missing; replay_pop_bucket restores decorators without collider removal"
            );
            let mut colliders = NoDecoratorColliderRemoval;
            storage.restore_decorators(&diff, bucket.undo, &mut id_gen, &mut colliders, deferred);
        }
    }
    if let Some(diff) = aux.stairs_diff {
        let mut stairs = stairs.expect("required stair owner validated before pop");
        stairs.restore_replay_diff(&diff, bucket.undo);
    }
    let completed = ReplayBucketFinished {
        entry_index: bucket.entry_index,
        undo: bucket.undo,
    };
    drop(bucket);
    if let Some(mut archivist) = archivist {
        drop(archivist.extract_diff(storage.as_deref().expect("archive storage checked"), true));
    }
    if let Some(mut raycast) = raycast {
        raycast.flush_colliders();
    }
    finished.write(completed);
    Ok(())
}

/// 碰撞 owner 缺失时的空实现（bucket 仍弹出，装饰状态仍写回）。
#[cfg(feature = "native-physics")]
struct NoDecoratorColliderRemoval;
#[cfg(feature = "native-physics")]
impl crate::resources::walls::decorator_storage::DecoratorColliderRemover
    for NoDecoratorColliderRemoval
{
    fn remove_decorator_collider(&mut self, _id: crate::resources::walls::decorator::DecoratorId) {}
}

/// 非 `native-physics` 的等价 CPU Pop：恢复 decorator aux
/// （`restore_decorators`，collider 用 no-op 并登记）、stairs aux
/// （`restore_backup/apply_diff/make_backup`）、`extract_diff`（有 archivist 时）
/// 与 `flush_colliders`（有 raycast 时），然后发出 `ReplayBucketFinished`。
/// 每个构建只允许这一条 Pop 路径（不注册 `pop_empty_replay_bucket`）。
#[cfg(not(feature = "native-physics"))]
pub fn replay_pop_bucket_cpu_system(
    mut replay: ResMut<ReplayBuffer>,
    mut storage: Option<ResMut<crate::resources::walls::decorator_storage::DecoratorStorage>>,
    id_gen: Option<ResMut<crate::resources::walls::decorator::DecoratorIdGen>>,
    events: Option<Res<crate::resources::walls::decorator::DeferredDecoratorOpEvents>>,
    stairs: Option<ResMut<crate::resources::stairs::stairs_state::StairsState>>,
    archivist: Option<ResMut<crate::resources::walls::decorator_archivist::LiveDecoratorArchivist>>,
    raycast: Option<ResMut<crate::systems::collision::world::RaycastWorld>>,
    mut finished: EventWriter<ReplayBucketFinished>,
) -> bevy_ecs::error::Result {
    let Some(front) = replay.buckets.front() else {
        return Ok(());
    };
    if archivist.is_some() && storage.is_none() {
        return Err(
            anyhow::anyhow!("decorator archive extraction requires DecoratorStorage").into(),
        );
    }
    if !front.commands.is_empty() {
        if let Some(mut archivist) = archivist {
            drop(
                archivist.extract_diff(storage.as_deref().expect("archive storage checked"), true),
            );
        }
        return Ok(());
    }
    let needs_decorator = replay
        .aux
        .front()
        .is_some_and(|aux| aux.decorator_diff.is_some());
    let needs_stairs = replay
        .aux
        .front()
        .is_some_and(|aux| aux.stairs_diff.is_some());
    if needs_decorator && (storage.is_none() || id_gen.is_none()) {
        return Err(anyhow::anyhow!(
            "B replay_pop_bucket @140b38250: decorator diff requires DecoratorStorage and DecoratorIdGen"
        )
        .into());
    }
    if needs_stairs && stairs.is_none() {
        return Err(anyhow::anyhow!(
            "B replay_pop_bucket @140b38250: stair diff requires StairsState"
        )
        .into());
    }
    let (bucket, aux) = replay.pop_front_bucket().expect("front bucket exists");
    if let Some(diff) = aux.decorator_diff {
        let (Some(mut storage), Some(mut id_gen)) = (storage.as_deref_mut(), id_gen) else {
            unreachable!("required decorator owners checked above");
        };
        let fallback = crate::resources::walls::decorator::DeferredDecoratorOpEvents::default();
        let deferred = match events.as_deref() {
            Some(events) => events,
            None => {
                log::warn!(
                    target: "country_core::systems::history",
                    "DeferredDecoratorOpEvents missing; CPU replay_pop_bucket diff events are not queued"
                );
                &fallback
            }
        };
        let mut colliders = NoDecoratorColliderRemoval;
        storage.restore_decorators(&diff, bucket.undo, &mut id_gen, &mut colliders, deferred);
    }
    if let Some(diff) = aux.stairs_diff {
        if let Some(mut stairs) = stairs {
            stairs.restore_replay_diff(&diff, bucket.undo);
        }
    }
    let completed = ReplayBucketFinished {
        entry_index: bucket.entry_index,
        undo: bucket.undo,
    };
    drop(bucket);
    if let Some(mut archivist) = archivist {
        drop(archivist.extract_diff(storage.as_deref().expect("archive storage checked"), true));
    }
    if let Some(mut raycast) = raycast {
        raycast.flush_colliders();
    }
    finished.write(completed);
    Ok(())
}

#[cfg(not(feature = "native-physics"))]
struct NoDecoratorColliderRemoval;
#[cfg(not(feature = "native-physics"))]
impl crate::resources::walls::decorator_storage::DecoratorColliderRemover
    for NoDecoratorColliderRemoval
{
    fn remove_decorator_collider(&mut self, _id: crate::resources::walls::decorator::DecoratorId) {}
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
    world.init_resource::<Events<replay_mode::ReplayFocusEvent>>();
    world.init_resource::<Events<history_process::DebugHistoryMsg>>();
    world.init_resource::<crate::systems::timelapse_replay::ReplaySpeedMultiplierOverride>();
    world.init_resource::<crate::input::action_set::MainActionSet>();
    world.init_resource::<replay_mode::HoldToFastReplay>();
    world.init_resource::<replay_mode::ReplayModeElapsedTime>();
    world.init_resource::<Events<crate::systems::mode_manager::AppModeChanged>>();
    world.init_resource::<Events<finish_replay::CmdUnfreezeSheep>>();
    world.init_resource::<Events<crate::systems::mode_manager::ClearCanvasCmd>>();
    recording::register_recording_events(world);
    replay_wall::register_wall_replay_events(world);
    // Color replay consumer target tables (same CPU tables as system_color's
    // `set_*_color_id`). `init_resource` is idempotent when the color plugin
    // already installed them.
    world.init_resource::<crate::resources::color_id::WallColorIds>();
    world.init_resource::<crate::resources::color_id::GateColorIds>();
    world.init_resource::<crate::resources::color_id::HalftimberColorIds>();
    world.init_resource::<crate::resources::color_id::RoofColorIds>();
    crate::resources::roofs::register_roof_events(world);
    // 读档 Roofs 段在 PublicWalls 发布前记录待重建关联；发布后由
    // `rebuild_pending_roof_associations` 一次成功重建（避免恒 gap）。
    world.init_resource::<crate::systems::save::snapshot::PendingRoofAssociationRebuild>();
    world.init_resource::<crate::resources::roofs::associations::RoofAssociations>();
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
    // Original clears the archivist permitted-ID set when a new replay command
    // starts (NewReplayCommandEvent is written by history_process before the
    // queue pop), so it must run after the cursor stage and before consumers.
    #[cfg(feature = "native-physics")]
    schedule.add_systems(
        history_clear_previous_state
            .after(history_process_system::<S>)
            .run_if(
                bevy_ecs::schedule::common_conditions::resource_exists::<
                    crate::resources::walls::decorator_archivist::LiveDecoratorArchivist,
                >,
            )
            .in_set(HistoryCpuStage::Cursor),
    );
    schedule.add_systems(replay_stairs::replay_stairs.in_set(HistoryCpuStage::Replay));
    schedule.add_systems(replay_auto_clutter::replay_auto_clutter.in_set(HistoryCpuStage::Replay));
    schedule.add_systems(replay_wall::replay_wall.in_set(HistoryCpuStage::Replay));
    schedule.add_systems(replay_raster::replay_rasters::<S>.in_set(HistoryCpuStage::Replay));
    schedule.add_systems(replay_color::replay_color.in_set(HistoryCpuStage::Replay));
    schedule.add_systems(replay_decorator::replay_decorator.in_set(HistoryCpuStage::Replay));
    // Real consumers of the replay command effects: write tables/storage/audio.
    // Decorator storage is NOT written here; Pop's `restore_decorators` owns it.
    schedule.add_systems(
        (
            replay_color::apply_color_replay.after(replay_color::replay_color),
            replay_wall::apply_wall_replay_color.after(replay_wall::replay_wall),
            replay_wall::apply_wall_replay_audio.after(replay_wall::replay_wall),
        )
            .in_set(HistoryCpuStage::Replay),
    );
    schedule.add_systems(
        replay_mode::replay_mode
            .after(crate::systems::mode_manager::ModeChangePublication)
            .in_set(HistoryCpuStage::Replay),
    );
    // 读档 RoofAssociations 的延后重建：等墙发布阶段填充 PublicWalls 行。
    schedule.add_systems(
        crate::systems::save::snapshot::rebuild_pending_roof_associations
            .after(crate::systems::wall::registration::WallCpuStage::Publish),
    );
    // Exactly one pop path: the production aux/collision-aware pop under
    // native-physics; the local empty-bucket pop otherwise.
    #[cfg(feature = "native-physics")]
    schedule.add_systems(replay_pop_bucket_system.in_set(HistoryCpuStage::Pop));
    #[cfg(not(feature = "native-physics"))]
    schedule.add_systems(replay_pop_bucket_cpu_system.in_set(HistoryCpuStage::Pop));
    #[cfg(feature = "native-physics")]
    schedule.add_systems(
        finish_replay::finish_replay
            .after(replay_pop_bucket_system)
            .in_set(HistoryCpuStage::Pop),
    );
    #[cfg(not(feature = "native-physics"))]
    schedule.add_systems(
        finish_replay::finish_replay
            .after(replay_pop_bucket_cpu_system)
            .in_set(HistoryCpuStage::Pop),
    );
    crate::systems::ui_clutter::registration::add_ui_clutter_cpu_systems::<S>(schedule);
    recording::add_recording_systems::<S>(schedule);
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

pub mod signifier_replay_active;
