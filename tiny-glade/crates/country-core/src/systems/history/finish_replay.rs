//! B `systems/history/finish_replay.rs` — 回放自然结束（RE-HISTORY-WRAPUP §3，O）。
//!
//! finish_replay：ReplayBuffer 与待处理命令双空 → `RestoreAppTo` 写回 AppMode →
//! MetaCommander 哨兵弹栈 → 恢复工具栏 → `CmdUnfreezeSheep`
//! （"History replay has naturally finished. Unblocking game commander."
//! @0x142a9f208）。CmdUnfreezeSheep 属 sheep 域（未建），登记为宿主消费。
use crate::{
    resources::history::ReplayBuffer,
    systems::history::PendingHistoryCommands,
};
use bevy_ecs::prelude::*;

/// 回放自然结束判定：ReplayBuffer 与待处理命令双空（O）。
pub fn is_replay_naturally_finished(replay: &ReplayBuffer, pending: &PendingHistoryCommands) -> bool {
    replay.is_empty() && pending.0.is_empty()
}

/// `finish_replay` 系统：双空时 log
/// "History replay has naturally finished. Unblocking game commander."
/// RestoreAppTo 写回 AppMode / MetaCommander 哨兵弹栈 / 工具栏恢复 / CmdUnfreezeSheep
/// 由宿主（mode_manager/sheep/工具栏域）接线。
pub fn finish_replay(
    replay: Res<ReplayBuffer>,
    pending: Res<PendingHistoryCommands>,
) {
    if !is_replay_naturally_finished(&replay, &pending) {
        return;
    }
    log::info!("History replay has naturally finished. Unblocking game commander.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finish_requires_both_queues_empty() {
        let replay = ReplayBuffer::new(1.0);
        let pending = PendingHistoryCommands::default();
        assert!(is_replay_naturally_finished(&replay, &pending));
        let mut pending = pending;
        pending.0.push_back(crate::systems::mode_manager::HistoryCommand {
            bytes: [0; 4],
        });
        assert!(!is_replay_naturally_finished(&replay, &pending));
    }
}
