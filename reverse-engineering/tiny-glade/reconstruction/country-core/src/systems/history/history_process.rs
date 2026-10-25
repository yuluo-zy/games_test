//! `systems::history::history_process` — 提交/撤销/重做驱动（B 骨架对齐，WP13）。
//!
//! O 证据（合同见 `contracts/history.md`；反编译 0x140af5b30 + building-history
//! 已有反编译核验）：
//!   - `HistoryCmd` 4 字节命令字：byte[1]==0 Undo / ==1 Redo / ==4 或 byte[0]==2
//!     Interrupt；每次游标移动后 `replay_edit()`。
//!   - `HistoryCmd::interrupt` 0x140af5b20 构造 `0x02000200`。
//!   - `PendingHistoryCommands` 待处理回放命令。

use crate::resources::history::{History, HistoryMutationToken};
use crate::typed_gap::Gap;

/// 历史命令字（O：4 字节编码；byte[0]=命令族，byte[1]=子命令）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct HistoryCmd(pub u32);

impl HistoryCmd {
    /// O：`interrupt` 0x140af5b20 返回 0x02000200；构造命令，不直接清队列或强制提交。
    pub fn interrupt() -> Self {
        Self(0x0200_0200)
    }
    /// O：Undo 命令字（byte[1]==0）。
    pub fn undo() -> Self {
        Self(0x0000_0000)
    }
    /// O：Redo 命令字（byte[1]==1）。
    pub fn redo() -> Self {
        Self(0x0000_0001)
    }
}

/// 待处理回放命令（O：`new`/`is_empty`；布局 U）。
#[derive(Clone, Debug, Default)]
pub struct PendingHistoryCommands(pub Vec<Gap>);

impl PendingHistoryCommands {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// O：`history_clear_previous_state`（清理回放前状态；语义 U）。
pub fn history_clear_previous_state(_history: &mut History) {}

/// O：`raster_edit_to_replay_cmd`（光栅编辑→回放命令；语义 U）。
pub fn raster_edit_to_replay_cmd(_edit: &Gap) -> Gap {
    Gap
}

/// O：`generate_replay_cmds_from_edit`（按 EditType 展开回放命令；4 实例化
/// replay_edit/path/water/garden）。
pub fn generate_replay_cmds_from_edit(_edit: &Gap) -> Vec<Gap> {
    Vec::new()
}

/// O：`schedule_replay_commands`（+闭包；排程回放命令到 ReplayBuffer）。
pub fn schedule_replay_commands(_commands: &[Gap], _buffer: &mut Gap) {}

/// O：`replay_edit`（撤销/重做目标条目的命令展开；返回命令列表）。
pub fn replay_edit(_edit: &Gap) -> Vec<Gap> {
    Vec::new()
}

/// O：`history_process` 0x140af5b30 — 消费 HistoryCmd 队列；undo/redo 移动游标后
/// replay_edit。此处实现游标规则 + 命令字分派；ReplayBuffer/事件排程为边界。
pub fn process_history_commands(history: &mut History, commands: &[HistoryCmd]) {
    for command in commands {
        let bytes = command.0.to_le_bytes();
        let sub = bytes[1];
        let family = bytes[0];
        if sub == 4 || family == 2 {
            // Interrupt：中断本次批量命令消费。
            return;
        }
        match sub {
            0 => history.undo(),
            1 => history.redo(),
            _ => {
                // 未证子命令：保留队列不消费。
                return;
            }
        }
    }
}

/// O：`HistoryMutationToken` 生产入口（各编辑 owner 提供；此处骨架对齐）。
pub fn begin_mutation() -> HistoryMutationToken {
    HistoryMutationToken { active: true }
}
