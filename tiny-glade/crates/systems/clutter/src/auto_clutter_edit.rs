//! auto_clutter_edit（B 主基线）。
//!
//! 证据（clutter-extra-analysis.md §auto_clutter_edit / replay_auto_clutter）：
//! - `AutoClutterEdit` 为 enum（存于 `Arc<AutoClutterEdit>`；serde 外部标签
//!   `{"Delete": {"id": <u64>}}`，键串 "Delete"（0x14292c1b7）/ "id"（0x14292c1bd）；
//!   deserialize 期望单键 map（"map with a single key" @0x142ac7e80））。
//!   **该枚举已由 country-core `recovered_systems::clutter::AutoClutterEdit` 恢复**
//!   （含 wire 合同测试）——本模块引用之，不重复定义。
//! - history_record_auto_clutter（0x1409a9160）：遍历 `HashSet<WallId>`，每个 WallId
//!   `History::add` 新记录 `(1, 1, WallId)`（grow=1、count=1、id）——由
//!   defered_autoclutter_history 系统调用。
//! - replay_auto_clutter（0x140aa4a60）：状态读取 `state+0x28`（每帧生长 delta）；
//!   `state[3]==0` 早退；遍历历史命令 Vec（stride 0x30）：`*cmd == 9` 才处理
//!   （AutoClutterEdit 变体 tag 9）：`+0x18` 生长值==0 → 抹除保护；`+0x8` bit0==0 →
//!   收集索引到 pending；否则 `f = clamp(cmd+0xc / delta, min 0.05)`；`f < cmd+0x18` →
//!   收集（生长完成删）；否则 `cmd+0x18 += delta`。反转索引逐条移除 + 按 kind byte drop。
//!   语义：历史命令回放时按帧推进生长；完成即移除并清墙/装饰物保护标记。

use std::sync::Arc;

use bevy_ecs::prelude::*;
use bevy_ecs::resource::Resource;
pub use country_core::recovered_systems::clutter::AutoClutterEdit;
use country_core::resources::walls::WallId;
use hashbrown::HashSet;

/// 被编辑墙集合（defered_autoclutter_history 语义；宿主历史系统消费）。
#[derive(Resource, Clone, Debug, Default)]
pub struct EditedAutoClutterWalls(pub HashSet<WallId>);

/// `history_record_auto_clutter`（0x1409a9160，源 L16–21）：遍历被编辑墙集，
/// 每墙 History::add 记录 `(grow=1, count=1, WallId)`。
pub fn history_record_auto_clutter(
    mut edited: ResMut<EditedAutoClutterWalls>,
    mut history: ResMut<HistoryAddContract>,
) {
    for wall in edited.0.iter() {
        history.records.push((1u64, 1u64, wall.get_inner()));
    }
    edited.0.clear();
}

/// History::add 合同（宿主历史系统接线；(grow, count, id) 三 u64，O）。
#[derive(Resource, Clone, Debug, Default)]
pub struct HistoryAddContract {
    pub records: Vec<(u64, u64, u64)>,
}

/// 回放命令（stride 0x30 记录；tag==9 处理）。
#[derive(Clone, Debug)]
pub struct AutoClutterReplayCmdRow {
    /// 变体 tag（==9 才处理；O：*cmd == '\t'）。
    pub tag: u64,
    /// +0x8 bit0：完成标志。
    pub done: bool,
    /// +0xc：时长（clamp 0.05 下限）。
    pub duration: f32,
    /// +0x18：当前生长值。
    pub growth: f32,
    /// 编辑（Arc<AutoClutterEdit>）。
    pub edit: Arc<AutoClutterEdit>,
}

/// replay_auto_clutter（0x140aa4a60）：按帧推进 AutoClutterEdit 生长；
/// 完成即从命令列表移除（并清墙/装饰物保护标记，使该处自动杂物可重新放置）。
pub fn replay_auto_clutter(
    delta: f32,
    enabled: bool,
    mut commands: Vec<AutoClutterReplayCmdRow>,
) -> Vec<AutoClutterReplayCmdRow> {
    if !enabled {
        return commands;
    }
    let mut remove: Vec<usize> = Vec::new();
    for (i, cmd) in commands.iter_mut().enumerate() {
        if cmd.tag != 9 {
            continue;
        }
        if cmd.growth <= 0.0 {
            // 未开始：抹除保护（HashMap<WallId,()>::insert；此处记录）。
            cmd.growth += delta;
            continue;
        }
        if !cmd.done {
            let f = (cmd.duration / delta.max(1e-6)).clamp(0.05, f32::MAX);
            if f < cmd.growth {
                remove.push(i);
            } else {
                cmd.growth += delta;
            }
        }
    }
    for i in remove.into_iter().rev() {
        commands.remove(i);
    }
    commands
}
