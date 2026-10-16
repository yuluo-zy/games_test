//! replay_tree.rs — 树历史回放（B 0x141bc34f0，O；re-analyst-A 契约 §2）。
//!
//! 语义（O）：`ReplayBuffer::front_mut_bucket`（0x1409f1a80）读回放桶 →
//! `ReplayCommand::Tree` 命令分派（变体 0=放置/1=移动/2=删除 → InsertTreeCmd /
//! MoveTreeCmd / DeleteTreeCmd 事件；0x140b5cfc0/0x140b5d010/0x140b5f9f0 =
//! 内部命令应用器族）。撤销放置 = 删除（I：EditType 无 Delete 变体——
//! 0x58B 变体 2 以 undo+Place 落地，登记待补证）。

use bevy_ecs::prelude::*;

use country_core::resources::history::replay_cmd_buffer::{ReplayBuffer, ReplayCommand};
use country_core::resources::history::tree_edit::{DeleteTreeCmd, InsertTreeCmd, TreeEditKind};
use country_core::resources::time::GameTime;
use country_core::resources::tree_storage::MoveTreeCmd;

/// `replay_tree` @0x141bc34f0（O 面）。
pub fn replay_tree<S: Send + Sync + 'static>(
    mut buffer: ResMut<ReplayBuffer>,
    mut inserts: EventWriter<InsertTreeCmd>,
    mut moves: EventWriter<MoveTreeCmd>,
    mut deletes: EventWriter<DeleteTreeCmd>,
    _time: Res<GameTime>,
) {
    if let Some(bucket) = buffer.front_mut_bucket() {
        // O：桶前端命令分派（命令应用器族 0x140b5cfc0/0x140b5d010/0x140b5f9f0；
        // 树命令记录 0x58B {1,1,变体,树 id} 字段 U——以 TreeReplayCmd 语义落地）。
        for command in bucket.commands.iter_mut() {
            if let ReplayCommand::Tree(cmd) = command {
                match cmd.edit.kind() {
                    TreeEditKind::Place => {
                        if cmd.undo {
                            // 撤销放置 = 删除（I，见模块头）。
                            deletes.write(DeleteTreeCmd { id: cmd.edit.id() });
                        } else {
                            inserts.write(InsertTreeCmd {
                                id: cmd.edit.id(),
                                position: cmd.edit.edit_position(),
                                scale: cmd.edit.edit_scale(),
                                prefab_name: cmd.edit.prefab_name().to_string(),
                            });
                        }
                    }
                    TreeEditKind::Move => {
                        moves.write(MoveTreeCmd { _gap: cmd.edit.id() });
                    }
                    TreeEditKind::Delete => {
                        deletes.write(DeleteTreeCmd { id: cmd.edit.id() });
                    }
                }
            }
        }
    }
}
