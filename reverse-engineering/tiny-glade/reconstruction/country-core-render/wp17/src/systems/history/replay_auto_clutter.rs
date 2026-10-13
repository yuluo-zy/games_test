//! `systems::history::replay_auto_clutter` — 自动杂物回放（0x140aa4a60，WP17）。
//!
//! O（反编译 0x140aa4a60 + enriched 接口 0103bc1d…）：
//!   - 系统签名：`(ResMut<ReplayBuffer>, Res<GameTime>,
//!     ResMut<DeletedAutoClutter>)`。
//!   - 前桶遍历判别 9（AutoClutter）：新命令（elapsed==0）时
//!     `undo==0` → `HashMap<WallId,()>` 插墙；`!=0` → 从 DeletedAutoClutter
//!     erase（`RawTable::erase<(DecoratorId,DecoratorId)>`）。
//!   - instant/动画节奏；`progress = duration/dt` 下限 0.05。

use crate::resources::history::replay_cmd_buffer::{ReplayBuffer, ReplayCmd};
use crate::systems::history::replay_pop_bucket::remove_done_commands;
use crate::resources::history::{DecoratorId, WallId};
use std::collections::HashSet;

/// 已删除自动杂物（O：`DeletedAutoClutter`；WallId 哈希表 + 装饰 erase）。
#[derive(Clone, Debug, Default)]
pub struct DeletedAutoClutter {
    pub walls: HashSet<WallId>,
    pub decorator_pairs: HashSet<(DecoratorId, DecoratorId)>,
}

/// O：`replay_auto_clutter` 0x140aa4a60。骨架实现：消费前桶 AutoClutter 命令。
pub fn replay_auto_clutter(
    buffer: &mut ReplayBuffer,
    _dt: f32,
    deleted: &mut DeletedAutoClutter,
) {
    let Some(bucket) = buffer.front_mut_bucket() else {
        return;
    };
    let mut done = Vec::new();
    for (i, cmd) in bucket.commands.iter_mut().enumerate() {
        match cmd {
            ReplayCmd::AutoClutter(edit) => {
                // O：undo(+0x1c)==0 → 插墙；!=0 → erase 装饰对。
                let undo = false;
                if !undo {
                    let _ = deleted.walls.insert(WallId(0));
                } else {
                    deleted.decorator_pairs.clear();
                }
                let _ = edit;
                done.push(i);
            }
            _ => {}
        }
    }
    remove_done_commands(bucket, done);
}
