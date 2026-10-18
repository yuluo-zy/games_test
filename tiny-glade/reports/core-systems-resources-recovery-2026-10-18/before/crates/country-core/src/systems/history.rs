pub mod history_process {
    pub use crate::systems::mode_manager::HistoryCommand as HistoryCmd;

    /// Original writer in history_process's SystemParam tuple. The flag is
    /// the fourth byte of HistoryCmd, not an EventInstance header byte.
    #[derive(bevy_ecs::event::Event, Clone, Copy, Debug, PartialEq, Eq)]
    pub struct NewReplayCommandEvent {
        pub flag: u8,
    }
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

/// 第二事件流待处理队列（产生端 U；仅处理端在录）。
#[derive(Resource, Default, Debug)]
pub struct PendingHistoryControl(pub VecDeque<HistoryControlRecord>);

pub use history_process::NewReplayCommandEvent as HistoryControlProcessed;

/// Entry strategy from original B 140af5cfd..5e6c. Queue clearing applies
/// before cursor processing, and all incoming events are visited this frame.
/// Fourth byte3 suppresses only NewReplayCommandEvent. At 140af6046 `shr
/// ebx,8` places byte2 in BL before `and bl,1`; byte3 never becomes a bucket
/// field (the generator consumes only DIL).
pub fn accept_history_command(
    command: HistoryCommand,
    pending: &mut PendingHistoryCommands,
    replay: &mut ReplayBuffer,
) -> Option<history_process::NewReplayCommandEvent> {
    let clear = match command.bytes[1] {
        0 => pending.0.back().is_some_and(|last| last.bytes[1] == 1),
        1 => pending.0.back().is_some_and(|last| last.bytes[1] == 0),
        2 => true,
        3 => {
            replay.make_animations_instant();
            true
        }
        // The fifth original command's cold block is not recovered here.
        // It remains a command, rejected by the cursor consumer, rather than
        // being silently transformed into Undo/Redo.
        _ => false,
    };
    if clear {
        pending.0.clear();
    } else {
        pending.0.push_back(command);
    }
    (command.bytes[3] != 3).then_some(history_process::NewReplayCommandEvent {
        flag: command.bytes[3],
    })
}

/// 第二事件流跳表 0x142ae5420 的**可解码分支**（fresh decomp-0x140af5b30，
/// `switch(*(byte *)(_Src + 9))`）：
/// - case 0：队尾 `HistoryCmd` 第二字节==1（redo）→ 写事件 + 清队（code_r0x000140af5df8）
/// - case 1：队尾第二字节==0（undo）→ 写事件 + 清队
/// - case 2：无条件 → 写事件 + 清队
/// - case 3：`ReplayBuffer::make_animations_instant` + 写事件 + 清队
/// 其余 case（0xb..0x18+：校验/动画即时化/移除）反编译不收敛，登记 U
/// （REBUILD-REMAINDER §2.1）。**默认路径**（未清队）：记录 @+8 的 u32
/// （机器码 `_Src[2]`）压入 [`PendingHistoryCommands`]；每条记录处理后按 @+0xb
/// 字节写事件（==3 跳过），本系统以 [`HistoryControlProcessed`] 承载。
pub fn process_history_control_stream(
    mut pending: ResMut<PendingHistoryCommands>,
    mut controls: ResMut<PendingHistoryControl>,
    mut replay: ResMut<ReplayBuffer>,
    mut processed: EventWriter<HistoryControlProcessed>,
) {
    for record in controls.0.drain(..) {
        let command = HistoryCommand {
            bytes: record.bytes[8..12].try_into().unwrap(),
        };
        if let Some(event) = accept_history_command(command, &mut pending, &mut replay) {
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

pub fn enqueue_history_commands(
    mut events: EventReader<HistoryCommand>,
    mut pending: ResMut<PendingHistoryCommands>,
    mut replay: ResMut<ReplayBuffer>,
    mut processed: EventWriter<history_process::NewReplayCommandEvent>,
) {
    for command in events.read().copied() {
        if let Some(event) = accept_history_command(command, &mut pending, &mut replay) {
            processed.write(event);
        }
    }
}

/// Verified ordinary Undo/Redo branch at B 0x140af5f81..0x140af6134.
/// Native load commands enter through the same four-byte event transport.
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

pub fn register_history_events(world: &mut World) {
    world.init_resource::<PendingHistoryCommands>();
    world.init_resource::<PendingHistoryControl>();
    world.init_resource::<Events<HistoryCommand>>();
    world.init_resource::<Events<HistoryReplayFailed>>();
    world.init_resource::<Events<ReplayBucketFinished>>();
    world.init_resource::<Events<HistoryControlProcessed>>();
    world.init_resource::<Events<replay_raster::RasterReplayFeedback>>();
    world.init_resource::<Events<replay_color::ColorHistoryReplayApply>>();
    world.init_resource::<Events<replay_decorator::DecoratorHistoryReplayApply>>();
    world.init_resource::<Events<replay_mode::ReplayFocusEvent>>();
    world.init_resource::<Events<history_process::DebugHistoryMsg>>();
    world.init_resource::<replay_mode::ReplayFastHoldInput>();
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
            process_history_control_stream,
            process_history_cursor_commands::<S>,
        )
            .chain()
            .in_set(HistoryCpuStage::Cursor),
    );
    schedule.add_systems(replay_stairs::replay_stairs.in_set(HistoryCpuStage::Replay));
    schedule.add_systems(replay_auto_clutter::replay_auto_clutter.in_set(HistoryCpuStage::Replay));
    schedule.add_systems(replay_wall::replay_wall.in_set(HistoryCpuStage::Replay));
    schedule.add_systems(replay_raster::replay_rasters::<S>.in_set(HistoryCpuStage::Replay));
    schedule.add_systems(replay_color::replay_color.in_set(HistoryCpuStage::Replay));
    schedule.add_systems(replay_decorator::replay_decorator.in_set(HistoryCpuStage::Replay));
    schedule.add_systems(replay_mode::replay_mode.in_set(HistoryCpuStage::Replay));
    schedule.add_systems(pop_empty_replay_bucket.in_set(HistoryCpuStage::Pop));
    schedule.add_systems(finish_replay::finish_replay.in_set(HistoryCpuStage::Pop));
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
