//! `systems::history::replay_terrain` — 地形回放（0x140995150，WP17）。
//!
//! O（反编译 0x140995150 + TerrainReplayCmd::forward 0x140994f30 / backward
//! 0x140995040 + enriched 接口 7e9c5a15…）：
//!   - 系统签名：`(Res<GameTime>, ResMut<TerrainEditingData>,
//!     ResMut<ReplayBuffer>, EventWriter<EditTerrainStrokeCmd>,
//!     EventWriter<EvTerrainStrokeChanged>, EventWriter<SetTerrainStrokePtsCmd>)`。
//!   - 前桶遍历判别 3（Terrain）；`TerrainEdit` 状态写入 `TerrainEditingData`
//!     （HashMap<TerrainStrokeId, TerrainStroke>）；动画 `1 - t^3`（ease_out）；
//!     `undo==0` → `TerrainEdit::reapply()`；`!=0` → `revert()`。
//!   - `TerrainReplayCmd::forward`：`duration = clamp(0.4, Σ点距*0.01, 3.0)`，
//!     无点 0.3。

use crate::resources::history::replay_cmd_buffer::{ReplayBuffer, ReplayCmd};
use crate::resources::history::terrain_edit::TerrainEdit;
use crate::systems::history::replay_pop_bucket::remove_done_commands;
use crate::typed_gap::Gap;
use std::collections::HashMap;

/// 地形笔刷状态域边界（O：`TerrainEditingData`；HashMap<TerrainStrokeId,
/// TerrainStroke>）。
#[derive(Clone, Debug, Default)]
pub struct TerrainEditingDataBoundary(pub HashMap<u64, Gap>);

/// O：`TerrainReplayCmd::forward`（0x140994f30）：构造前向回放命令。
pub struct TerrainReplayCmd {
    /// O：payload（Arc<Option<TerrainEdit>> 边界）。
    pub edit: Option<TerrainEdit>,
    /// O：duration。
    pub duration: f32,
    /// O：方向（forward/backward）。
    pub forward: bool,
}

impl TerrainReplayCmd {
    /// O：`new`（源行 50）。
    pub fn new(edit: TerrainEdit) -> Self {
        let duration = edit.duration();
        Self {
            edit: Some(edit),
            duration,
            forward: true,
        }
    }
    /// O：`forward` 0x140994f30（写 duration；`+0x8=2` 方向位）。
    pub fn forward(&mut self, edit: TerrainEdit) {
        self.edit = Some(edit);
        self.duration = self
            .edit
            .as_ref()
            .map(TerrainEdit::duration)
            .unwrap_or(0.3);
        self.forward = true;
    }
    /// O：`backward` 0x140995040。
    pub fn backward(&mut self, edit: TerrainEdit) {
        self.edit = Some(edit);
        self.duration = self
            .edit
            .as_ref()
            .map(TerrainEdit::duration)
            .unwrap_or(0.3);
        self.forward = false;
    }
}

/// O：`ease_out`（源行 168）：`1 - (1-t)^3`（反编译 0x140995150 显示
/// `1 - t^3` 变体；U 系数歧义）。
pub fn ease_out(t: f32) -> f32 {
    1.0 - (1.0 - t).powi(3)
}

/// 动画进度判定（O：`progress = elapsed/duration`，下限 0.05）。
pub fn animation_progress(elapsed: f32, duration: f32) -> f32 {
    let progress = if duration > 0.0 { elapsed / duration } else { 1.0 };
    progress.clamp(0.0, 1.0)
}

/// O：`replay_terrain` 0x140995150。骨架实现：消费前桶 Terrain 命令。
pub fn replay_terrain(
    buffer: &mut ReplayBuffer,
    _dt: f32,
    editing: &mut TerrainEditingDataBoundary,
) {
    let Some(bucket) = buffer.front_mut_bucket() else {
        return;
    };
    let mut done = Vec::new();
    for (i, cmd) in bucket.commands.iter_mut().enumerate() {
        match cmd {
            ReplayCmd::Terrain(edit) => {
                // O：新命令启动时把 TerrainEdit 状态写入 TerrainEditingData。
                if let Some(t) = &**edit {
                    let _ = editing.0.insert(0, Gap);
                    // O：undo(+0x1c)==0 → reapply；!=0 → revert。
                    t.reapply();
                }
                done.push(i);
            }
            _ => {}
        }
    }
    remove_done_commands(bucket, done);
}
