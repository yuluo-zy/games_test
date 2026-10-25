//! replay_tree.rs — 树历史回放（B 0x141bc34f0，O；2026-10-17 fresh 反编译 +
//! wave3 落库）。
//!
//! 语义（O，fresh decomp 0x141bc34f0）：
//! `ReplayBuffer::front_mut_bucket` → 逐 `ReplayCommand`（真实 ReplayCmd
//! stride 0x30，tag==8 为 TreeEdit；共享模型 = `ReplayCommand::Tree` +
//! `ReplayBucket.timing` 并行数组）：
//! - 仅 `elapsed(f32 @+0x18) == 0.0`（非 NaN）时重发命令事件；
//! - `undo`（字节 @+0x1c）判定：Place undo → DeleteTreeCmd；Delete undo →
//!   重建 InsertTreeCmd；Move undo → 旧位置 / 否则新位置；
//! - 时序记账（decomp 行 220-242）：flags @+8 bit0==0 → 记待移除；否则
//!   `delay=cmd@+0xc/speed`（下钳 0.05），`delay < elapsed` → 移除，否则
//!   `elapsed += GameTime.delta`；结束后逆序 `Vec::remove`（行 245-410，
//!   tag switch 释放载荷——共享枚举 Drop 等价）。
//!
//! ## wave3 落库
//! - 时序记账/移除集/降序删除**全实现**（ReplayBucket.timing + advance 语义；
//!   `CommandTiming` 偏移判等见 coregap-1，bit0 极性已按二进制修）。
//! - Move 分支**实现**（MoveTreeCmd 载荷 0x18：index@+0、position@+8；事件槽
//!   0x20 = EventInstance 头 8B + 0x18）。
//! - Delete undo / Place non-undo 的 InsertTreeCmd 重建：共享 InsertTreeCmd
//!   {id, position, scale, prefab_name} 承载 name/position/scale；真实槽另有
//!   height/variant/rotation/flags（stride 0x48）→ **分歧 T2**（tree_edit.rs
//!   由 history 组持有，0x48 未落库）。
//! - Move undo：二进制读 edit+0x14/+0x1c（**旧位置**）→ 现由
//!   `TreeEditData.previous_position` 承载并重发 MoveTreeCmd（FIX-C1 闭合）。

use bevy_ecs::prelude::*;

use country_core::resources::history::replay_cmd_buffer::{ReplayBuffer, ReplayCommand};
use country_core::resources::history::tree_edit::{DeleteTreeCmd, InsertTreeCmd, TreeEditKind};
use country_core::resources::time::GameTime;
use country_core::resources::tree_storage::MoveTreeCmd;

/// `replay_tree` @0x141bc34f0（O 面；wave3 落库时序/Move/撤销面）。
pub fn replay_tree<S: Send + Sync + 'static>(
    mut buffer: ResMut<ReplayBuffer>,
    mut inserts: EventWriter<InsertTreeCmd>,
    mut moves: EventWriter<MoveTreeCmd>,
    mut deletes: EventWriter<DeleteTreeCmd>,
    time: Res<GameTime>,
) {
    let speed = buffer.speed;
    if let Some(bucket) = buffer.front_mut_bucket() {
        // O：待移除索引集（decomp 行 220-242 累计；逆序 Vec::remove）。
        let mut removal: Vec<usize> = Vec::new();
        for (i, command) in bucket.commands.iter_mut().enumerate() {
            let timing = bucket.timing[i];
            if let ReplayCommand::Tree(cmd) = command {
                // O：`elapsed @+0x18 == 0.0 && !NAN` 才执行动画重发（decomp 行
                // 93/145/183；Rust `== 0.0` 对 NaN 为假，等价）。
                if timing.elapsed == 0.0 {
                    match cmd.edit.kind() {
                        TreeEditKind::Delete => {
                            if cmd.undo {
                                // O：撤销删除 = 重建插入（0x141bc34f0 行 106-142）：
                                // 从 edit 读 name/position/height/scale/rotation/
                                // flags/id 组装 0x48B InsertTreeCmd。共享字段
                                // 承载 name/position/scale；height/variant/
                                // rotation/flags → 分歧 T2。
                                inserts.write(InsertTreeCmd {
                                    id: cmd.edit.id(),
                                    position: cmd.edit.edit_position(),
                                    scale: cmd.edit.edit_scale(),
                                    prefab_name: cmd.edit.prefab_name().to_string(),
                                });
                            } else {
                                // O：DeleteTreeCmd 槽 stride 0x10，id = edit+0x18。
                                deletes.write(DeleteTreeCmd { id: cmd.edit.id() });
                            }
                        }
                        TreeEditKind::Move => {
                            // O：MoveTreeCmd 载荷 0x18 {index@+0, position Vec3@+8}
                            // （事件槽 0x20 = EventInstance 头 8 + 0x18）。
                            // 非 undo → 新位置；undo → edit 旧位
                            // （`+0x14/+0x1c` = TreeEditData.previous_position）。
                            let position = if cmd.undo {
                                cmd.edit.previous_position()
                            } else {
                                cmd.edit.edit_position()
                            };
                            moves.write(MoveTreeCmd {
                                index: cmd.edit.id(),
                                position,
                            });
                        }
                        TreeEditKind::Place => {
                            if cmd.undo {
                                // O：撤销放置 = 删除（decomp 行 204-219）。
                                deletes.write(DeleteTreeCmd { id: cmd.edit.id() });
                            } else {
                                // O：重建 0x48B InsertTreeCmd（decomp 行 183-203）。
                                // 共享字段承载 name/position/scale；height/
                                // variant/rotation/flags → 分歧 T2。
                                inserts.write(InsertTreeCmd {
                                    id: cmd.edit.id(),
                                    position: cmd.edit.edit_position(),
                                    scale: cmd.edit.edit_scale(),
                                    prefab_name: cmd.edit.prefab_name().to_string(),
                                });
                            }
                        }
                    }
                }
                // O：时序记账（decomp 行 220-242）。
                if timing.flags & 1 == 0 {
                    // O：flags bit0==0 → 待移除（立即）。
                    removal.push(i);
                } else {
                    // O：flags bit0==1 → 延迟移除 + 推进 elapsed。
                    // `delay = cmd@+0xc / speed`，下钳 0.05。
                    let mut delay = timing.delay / speed;
                    if delay <= 0.05 {
                        delay = 0.05;
                    }
                    let elapsed = timing.elapsed;
                    if delay < elapsed {
                        removal.push(i);
                    }
                    // O：`elapsed += GameTime.delta`（decomp 行 241；
                    // `ReplayBuffer.speed` = `+0x28`，`GameTime.delta` = `+0x28`）。
                    bucket.timing[i].elapsed = elapsed + time.delta;
                }
            }
        }
        // O：移除集逆序 `Vec::remove`（decomp 行 245-410；tag switch 释放载荷
        // = 共享 `ReplayCommand` 枚举 Drop，等价）。
        for i in removal.into_iter().rev() {
            let _ = bucket.commands.remove(i);
            bucket.timing.remove(i);
        }
    }
}
