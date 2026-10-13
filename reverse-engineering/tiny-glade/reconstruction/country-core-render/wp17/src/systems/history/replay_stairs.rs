//! `systems::history::replay_stairs` — 楼梯回放（0x140af75b0，WP17）。
//!
//! O（反编译 0x140af75b0 + enriched 接口 f762bb08…）：
//!   - 系统签名：`(ResMut<ReplayBuffer>, Res<GameTime>,
//!     ResMut<StairsRemovedSupports>, EventWriter<PlayStairsAudioCmd>,
//!     Res<CachedDecoratorTransforms>, Res<DecoratorStorage>)`。
//!   - 前桶遍历判别 10（Stairs）：`undo==0` → `HashMap<StairGraphEdge,()>`
//!     insert；`!=0` → `RawTable::remove_entry<StairGraphEdge>`。
//!   - `DecoratorStorage::lookup_decorator_info` 查附挂装饰；
//!     `CachedDecoratorTransforms` 哈希检索取边装饰变换；发 PlayStairsAudioCmd。
//!   - instant/动画节奏。

use crate::resources::history::replay_cmd_buffer::{ReplayBuffer, ReplayCmd};
use crate::resources::history::{DecoratorId, StairGraphEdge};
use crate::systems::history::replay_pop_bucket::remove_done_commands;
use crate::typed_gap::Gap;
use std::collections::HashSet;

/// 楼梯移除支撑（O：`StairsRemovedSupports`）。
#[derive(Clone, Debug, Default)]
pub struct StairsRemovedSupports(pub HashSet<StairGraphEdge>);

/// 装饰缓存变换边界（O：`CachedDecoratorTransforms`；哈希检索）。
#[derive(Clone, Debug, Default)]
pub struct CachedDecoratorTransforms(pub Gap);

/// 播放楼梯音频命令（O：`PlayStairsAudioCmd`）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PlayStairsAudioCmd;

/// 楼梯状态域边界（O：`StairsState`；边集）。
#[derive(Clone, Debug, Default)]
pub struct StairsGraphBoundary {
    pub edges: HashSet<StairGraphEdge>,
    /// O：装饰附挂（DecoratorId）。
    pub decorator: Option<DecoratorId>,
}

/// O：`replay_stairs` 0x140af75b0。骨架实现：消费前桶 Stairs 命令。
pub fn replay_stairs(
    buffer: &mut ReplayBuffer,
    _dt: f32,
    removed_supports: &mut StairsRemovedSupports,
    out_audio: &mut Vec<PlayStairsAudioCmd>,
    graph: &mut StairsGraphBoundary,
    _cached_transforms: &CachedDecoratorTransforms,
) {
    let Some(bucket) = buffer.front_mut_bucket() else {
        return;
    };
    let mut done = Vec::new();
    for (i, cmd) in bucket.commands.iter_mut().enumerate() {
        match cmd {
            ReplayCmd::Stairs(stairs) => {
                let _ = stairs;
                // O：undo==0 → insert 边；!=0 → remove_entry。
                let undo = false;
                let edge = StairGraphEdge((0, 0));
                if !undo {
                    let _ = graph.edges.insert(edge);
                    let _ = removed_supports.0.insert(edge);
                } else {
                    graph.edges.remove(&edge);
                    removed_supports.0.remove(&edge);
                }
                // O：DecoratorStorage::lookup_decorator_info + 缓存变换检索 +
                // PlayStairsAudioCmd（EventWriter）。
                out_audio.push(PlayStairsAudioCmd);
                done.push(i);
            }
            _ => {}
        }
    }
    remove_done_commands(bucket, done);
}
