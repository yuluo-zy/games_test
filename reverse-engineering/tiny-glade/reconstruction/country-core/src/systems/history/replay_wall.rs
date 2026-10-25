//! `systems::history::replay_wall` — 墙回放（0x140951b80，WP17）。
//!
//! O（反编译 0x140951b80 + WallReplayEditCmd::process 0x140abfb30 +
//! _deprecated_morph_curve 0x140952570 + enriched 接口 7d099771…）：
//!   - 系统签名：`(ResMut<History>, ResMut<PendingHistoryCommands>,
//!     ResMut<ReplayBuffer>, EventReader<HistoryCmd>,
//!     EventReader<DebugHistoryMsg>, EventWriter<PlayReplayAudio>,
//!     EventWriter<NewReplayCommandEvent>, EventWriter<InterruptTimelapse>,
//!     Res<AnimateWallBulldozeBuffer>)`。
//!   - 前桶遍历判别 4（WallReplayCmd）；每命令内 `Vec<WallReplayEditCmd>`
//!     （0x18 元素）逐个 `process`；全部 done 时插 `HashMap<WallId,()>`
//!     已改墙集合；instant/动画节奏；完成索引移除。
//!   - `WallReplayCmd::new`（源行 158）/`is_single_adjust_height_cmd`（行 152）；
//!     `WallReplayEditCmd::process_instant`（行 307）/`process_animated`（行 618）。

use crate::resources::history::replay_cmd_buffer::{ReplayBuffer, ReplayCmd, WallReplayCmd, WallReplayEditCmd};
use crate::systems::history::replay_pop_bucket::remove_done_commands;
use crate::resources::history::History;
use crate::typed_gap::Gap;
use std::collections::HashSet;

/// 墙 ID 集合（O：已改墙 HashMap<WallId,()>）。
pub type ChangedWalls = HashSet<u64>;

/// 墙状态域边界（O：InnerWalls / PublicWalls；apply 入口 U）。
#[derive(Clone, Debug, Default)]
pub struct WallStateBoundary(pub Gap);

impl WallStateBoundary {
    /// O：应用一条 WallReplayEditCmd（process 分派表 DAT_142adf408/…4c U）。
    pub fn apply_edit(&mut self, _cmd: &WallReplayEditCmd, _changed: &mut ChangedWalls) {}
}

/// O：`replay_wall` 0x140951b80。骨架实现：消费前桶 WallReplayCmd。
pub fn replay_wall(
    _history: &mut History,
    buffer: &mut ReplayBuffer,
    _dt: f32,
    walls: &mut WallStateBoundary,
) {
    let Some(bucket) = buffer.front_mut_bucket() else {
        return;
    };
    let mut done = Vec::new();
    let mut changed: ChangedWalls = HashSet::new();
    for (i, cmd) in bucket.commands.iter_mut().enumerate() {
        match cmd {
            ReplayCmd::Wall(wall) => {
                process_wall_cmd(wall, walls, &mut changed, _dt, &mut done, i);
            }
            _ => {}
        }
    }
    remove_done_commands(bucket, done);
}

/// 处理一条 WallReplayCmd（O：0x140951b80 内联）。
fn process_wall_cmd(
    wall: &mut WallReplayCmd,
    walls: &mut WallStateBoundary,
    changed: &mut ChangedWalls,
    dt: f32,
    done: &mut Vec<usize>,
    index: usize,
) {
    // O：每 WallReplayEditCmd process；全部 done 时插已改墙集合。
    let mut all_done = true;
    for edit in wall.edits.iter_mut() {
        edit.process();
        if !edit.done {
            all_done = false;
        }
    }
    let _ = dt;
    if wall.instant {
        all_done = true;
    }
    if all_done {
        // O：插已改墙集合（HashMap<WallId,()>）。
        for edit in &wall.edits {
            if edit.edit.is_some() {
                let _ = changed.insert(0);
            }
        }
        done.push(index);
    }
    let _ = walls;
}

/// O：`WallReplayCmd::new` 生成（源行 158；构造自 ConcurrentEdits）。
pub fn build_wall_replay_cmds(_compound: &Gap) -> Vec<WallReplayCmd> {
    Vec::new()
}

/// O：`_deprecated_morph_curve` 0x140952570（源行 875）：已弃用形态曲线应用。
pub fn _deprecated_morph_curve(_wall: u64, _curve: &Gap) {}
