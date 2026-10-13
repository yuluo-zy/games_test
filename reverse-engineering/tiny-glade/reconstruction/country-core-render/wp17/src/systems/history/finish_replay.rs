//! `systems::history::finish_replay` — 回放完成处理（0x1408b36a0，WP17）。
//!
//! O（反编译 0x1408b36a0 + enriched 接口 8ba632a7…）：
//!   - 系统签名：`(ResMut<AppMode>, EventReader<HistoryCmd>,
//!     ResMut<Toolbar>, ResMut<MetaCommander>,
//!     Res<PendingHistoryCommands>, Res<ReplayBuffer>,
//!     Res<ActionSetStates>, Res<RestoreAppTo>,
//!     EventWriter<CmdUnfreezeSheep>)`。
//!   - AppMode==0xd（Replay）：pending 恰 1 条 redo → 写 `CmdUnfreezeSheep`
//!     事件；pending 空 && ReplayBuffer 空 → 回放完成：`RestoreAppTo::get()`
//!     写回 AppMode；恢复模式为游玩范围时 `Toolbar::switch_mode`；
//!     `MetaCommander::peek` 顶部 i64::MIN 哨兵 → log + pop。
//!   - 非 Replay：MetaCommander 顶部哨兵 → log。

use crate::resources::history::replay_cmd_buffer::ReplayBuffer;
use crate::systems::history::boundary::{AppMode, PendingHistoryCommandsFacade};
use crate::systems::history::replay_mode::RestoreAppTo;
use crate::typed_gap::Gap;

/// 绵羊解冻命令（O：`critters::sheep::CmdUnfreezeSheep`；EventWriter 负载）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CmdUnfreezeSheep;

/// MetaCommander 顶部哨兵（O：`-0x7fffffffffffffff`（i64::MIN））。
pub const METACOMMAND_REPLAY_SENTINEL: i64 = i64::MIN;

/// 元命令边界（O：`MetaCommander`；peek/pop）。
#[derive(Clone, Debug, Default)]
pub struct MetaCommanderBoundary {
    pub top: Option<Gap>,
}

impl MetaCommanderBoundary {
    /// O：`peek`（顶部；VecDeque::back 内联）。
    pub fn peek(&self) -> Option<&Gap> {
        self.top.as_ref()
    }
    /// O：`pop`（VecDeque::pop_front 内联）。
    pub fn pop_front(&mut self) {
        self.top = None;
    }
}

/// 工具栏边界（O：`Toolbar`；switch_mode / switch_category）。
#[derive(Clone, Debug, Default)]
pub struct ToolbarBoundary(pub Gap);

impl ToolbarBoundary {
    pub fn switch_mode(&mut self) {}
    pub fn switch_category(&mut self) {}
}

/// 输入动作集边界（O：`ActionSetStates`；pressed 判定）。
#[derive(Clone, Debug, Default)]
pub struct ActionSetStatesBoundary(pub Gap);

/// O：`finish_replay` 0x1408b36a0。骨架实现核心逻辑（bevy 参数边界化）。
pub fn finish_replay(
    app_mode: &mut AppMode,
    history_commands: &[Gap],
    toolbar: &mut ToolbarBoundary,
    meta_commander: &mut MetaCommanderBoundary,
    pending_commands: &PendingHistoryCommandsFacade,
    buffer: &ReplayBuffer,
    restore_app_to: &RestoreAppTo,
    out_unfreeze_sheep: &mut Vec<CmdUnfreezeSheep>,
) {
    let _ = history_commands;
    if app_mode.is_replay() {
        // O：pending 恰 1 条且为 redo → 写 CmdUnfreezeSheep。
        if pending_commands.0.len() == 1 {
            out_unfreeze_sheep.push(CmdUnfreezeSheep);
        }
        // O：pending 空 && ReplayBuffer 空 → 回放完成。
        if pending_commands.is_empty() && buffer.is_empty() {
            if let Some(restore) = restore_app_to.get() {
                *app_mode = restore;
            }
            // O：恢复模式为游玩范围时 switch_mode（uVar8-5 ∈ [0,0xf]）。
            let tag = app_mode.tag();
            if (6..=20).contains(&tag) {
                toolbar.switch_mode();
            }
            // O：MetaCommander 顶部 i64::MIN 哨兵 → log + pop。
            if meta_commander.peek().is_some() {
                // log（“回放完成”）；随后 pop。
                meta_commander.pop_front();
            }
        }
    } else {
        // O：非 Replay：MetaCommander 顶部哨兵 → log。
        let _ = meta_commander.peek();
    }
}
