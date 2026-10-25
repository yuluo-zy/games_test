//! B `systems/history/finish_replay.rs` — 回放自然结束（RE-HISTORY-WRAPUP §3，O）。
//!
//! finish_replay（0x1408b36a0）：AppMode==0xd 且 ReplayBuffer 与待处理命令双空 →
//! `RestoreAppTo` 写回 AppMode → 恢复工具栏（`Toolbar::switch_mode`；CloneClutter
//! 无工具栏类别 → 直写 `Editing` 类别，原版 `+0x18=7`）→ MetaCommander 哨兵
//! （tag `0x8000000000000001`，非 `EMPTY_SENTINEL`）→ log
//! "History replay has naturally finished. Unblocking game commander."
//! → pop_front。哨兵检查在回放/非回放两条路径都会执行。
//! `CmdUnfreezeSheep` 属 sheep 域（未建），以 log 文案承载，登记为宿主消费。
use crate::{
    meta_commander::MetaCommander,
    resources::{history::ReplayBuffer, toolbar::{Toolbar, ToolbarCategory}},
    systems::{
        history::PendingHistoryCommands,
        mode_manager::{AppMode, RestoreAppTo},
    },
};
use bevy_ecs::prelude::*;

/// MetaCommander 队首的回放自然结束哨兵 tag（finish_replay 0x1408b36a0
/// 检查 `!= -0x7fffffffffffffff`，即 `== 0x8000000000000001`）。
pub const REPLAY_FINISHED_SENTINEL: u64 = 0x8000_0000_0000_0001;

/// 回放自然结束判定：AppMode 为 Replay（fresh 0x1408b36a0：顶层 `*AppMode == 0xd`
/// 即写回；队列检查只门控内联的清理循环，不门控 AppMode 写回）。
pub fn is_replay_naturally_finished(
    mode: AppMode,
    replay: &ReplayBuffer,
    pending: &PendingHistoryCommands,
) -> bool {
    let _ = (replay, pending);
    mode.native_tag() == 13
}

/// `finish_replay` 系统（0x1408b36a0）：
/// AppMode==Replay → RestoreAppTo 写回 AppMode + 工具栏恢复；
/// 随后 MetaCommander 队首哨兵 → log + pop。
pub fn finish_replay(
    mut mode: ResMut<AppMode>,
    restore: ResMut<RestoreAppTo>,
    mut toolbar: ResMut<Toolbar>,
    mut commander: ResMut<MetaCommander>,
    replay: Res<ReplayBuffer>,
    pending: Res<PendingHistoryCommands>,
) {
    if is_replay_naturally_finished(*mode, &replay, &pending) {
        let restore_to = restore.0;
        *mode = restore_to;
        // fresh：`tag ∈ 6..=20` → `switch_mode`，其中 tag==20（CloneClutter，
        // 无工具栏类别）走直写；tag ∉ 6..=20 同样直写。
        let tag = restore_to.native_tag();
        if (6..=19).contains(&tag) {
            toolbar.switch_mode(&restore_to);
        } else {
            // 原版直写 toolbar `+0x18 = 7`（Editing 类别）。
            toolbar.current = ToolbarCategory::Editing;
        }
    }
    if commander
        .peek()
        .is_some_and(|command| command.tag == REPLAY_FINISHED_SENTINEL)
    {
        log::info!("History replay has naturally finished. Unblocking game commander.");
        commander.consume();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        meta_commander::MetaCommand,
        resources::history::{ReplayBucket, ReplayBuffer},
        systems::mode_manager::EditSubject,
    };

    #[test]
    fn finish_requires_replay_mode_only() {
        // fresh 0x1408b36a0：顶层只看 `*AppMode == 0xd`；队列状态只门控内联清理循环。
        let replay = ReplayBuffer::new(1.0);
        let pending = PendingHistoryCommands::default();
        assert!(is_replay_naturally_finished(AppMode::Replay, &replay, &pending));
        assert!(!is_replay_naturally_finished(
            AppMode::LoadGame,
            &replay,
            &pending
        ));
        let mut pending = pending;
        pending.0.push_back(crate::systems::mode_manager::HistoryCommand {
            bytes: [0; 4],
        });
        assert!(is_replay_naturally_finished(
            AppMode::Replay,
            &replay,
            &pending
        ));
        let mut replay = replay;
        replay.push(ReplayBucket {
            entry_index: 0,
            undo: false,
            commands: vec![],
            timing: Vec::new(),
        });
        assert!(is_replay_naturally_finished(
            AppMode::Replay,
            &replay,
            &PendingHistoryCommands::default()
        ));
    }

    #[test]
    fn sentinel_is_popped_by_finish_replay() {
        let mut world = World::new();
        world.insert_resource(AppMode::Replay);
        world.insert_resource(RestoreAppTo(AppMode::Editing(EditSubject::None)));
        world.init_resource::<Toolbar>();
        world.init_resource::<MetaCommander>();
        world.insert_resource(ReplayBuffer::new(1.0));
        world.init_resource::<PendingHistoryCommands>();
        world
            .resource_mut::<MetaCommander>()
            .extend_script([MetaCommand {
                tag: REPLAY_FINISHED_SENTINEL,
                payloads: [0; 6],
            }]);
        let mut schedule = Schedule::default();
        schedule.add_systems(finish_replay);
        schedule.run(&mut world);
        assert!(world.resource::<MetaCommander>().is_empty());
        assert_eq!(
            world.resource::<AppMode>().native_tag(),
            AppMode::Editing(EditSubject::None).native_tag()
        );
    }
}
