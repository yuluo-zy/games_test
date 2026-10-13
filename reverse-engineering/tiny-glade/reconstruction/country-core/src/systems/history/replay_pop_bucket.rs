//! `systems::history::replay_pop_bucket` — 队首空桶弹出（0x140b38250，WP17）。
//!
//! O（反编译 0x140b38250 + enriched 接口 be1df256…）：
//!   - 系统签名：`(ResMut<ReplayBuffer>, EventWriter<InputReplayCommand>,
//!     ResMut<DecoratorStorage>, ResMut<DecoratorArchivist>,
//!     ResMut<DecoratorIdGen>, ResMut<StairsState>, ResMut<RaycastWorld>,
//!     ResMut<DecoratorPhysicsColliders>, Res<DeferredDecoratorOpEvents>)`。
//!   - 前桶 `Vec<ReplayCmd> len(+0x10)==0` 时 `ReplayBuffer::pop_front()`：
//!     - 装饰 diff → `DecoratorStorage::restore_decorators<closure_env$0>`
//!       （宿主 = `decorator_edit::send_state_diff_events<closure_env$0>`
//!       0x14017bdae0）。
//!     - 楼梯 diff → `StairsState::restore_backup` + `apply_diff` +
//!       `iter_edges_and_stubs`。
//!     - `DecoratorArchivist::extract_diff()`；
//!       drop_inner_table<(DecoratorStorageAddr, DecoratorStorageHistoryDiff)>。
//!   - 空桶也 `RaycastWorld::flush_colliders()`。

use crate::resources::history::replay_cmd_buffer::{
    ReplayBucket, ReplayBuffer, DecoratorHistoryDiffBoundary, StairsHistoryDiffBoundary,
};
use crate::typed_gap::Gap;

/// 装饰存储边界（O：`DecoratorStorage`；restore_decorators）。
#[derive(Clone, Debug, Default)]
pub struct DecoratorStorageBoundary(pub Gap);

impl DecoratorStorageBoundary {
    /// O：`restore_decorators<C>`（closure 宿主 send_state_diff_events）。
    pub fn restore_decorators(&mut self) {}
}

/// 楼梯状态边界（O：`StairsState`；restore_backup/apply_diff）。
#[derive(Clone, Debug, Default)]
pub struct StairsStateBoundary(pub Gap);

impl StairsStateBoundary {
    pub fn restore_backup(&mut self) {}
    pub fn apply_diff(&mut self) {}
    pub fn iter_edges_and_stubs(&mut self) {}
    pub fn make_backup(&mut self) {}
}

/// 碰撞世界边界（O：`RaycastWorld`；flush_colliders）。
#[derive(Clone, Debug, Default)]
pub struct RaycastWorldBoundary(pub Gap);

impl RaycastWorldBoundary {
    pub fn flush_colliders(&mut self) {}
}

/// 装饰档案边界（O：`DecoratorArchivist`；extract_diff）。
#[derive(Clone, Debug, Default)]
pub struct DecoratorArchivistBoundary(pub Gap);

impl DecoratorArchivistBoundary {
    pub fn extract_diff(&mut self) {}
}

/// 输入回放命令事件（O：`backend::input::input_record_replay::InputReplayCommand`）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct InputReplayCommand(pub u8);

/// 对桶应用装饰/楼梯 diff（O：pop 后处理；独立门禁共享逻辑）。
pub fn apply_bucket_payloads(
    bucket: &ReplayBucket,
    decorator_storage: &mut DecoratorStorageBoundary,
    stairs_state: &mut StairsStateBoundary,
    _raycast_world: &mut RaycastWorldBoundary,
    archivist: &mut DecoratorArchivistBoundary,
) {
    let decorator_diff: Option<&DecoratorHistoryDiffBoundary> = bucket.decorator_diff.as_deref();
    if decorator_diff.is_some() {
        // O：DecoratorStorage::restore_decorators（send_state_diff_events 闭包）。
        decorator_storage.restore_decorators();
    }
    let stairs_diff: Option<&StairsHistoryDiffBoundary> = bucket.stairs_diff.as_deref();
    if stairs_diff.is_some() {
        stairs_state.restore_backup();
        stairs_state.apply_diff();
        stairs_state.iter_edges_and_stubs();
        stairs_state.make_backup();
    }
    // O：DecoratorArchivist::extract_diff + drop_inner_table（收尾）。
    archivist.extract_diff();
}

/// O：`replay_pop_bucket` 0x140b38250。队首桶命令集为空时弹出并应用负载。
pub fn replay_pop_bucket(
    buffer: &mut ReplayBuffer,
    out_input_replay_commands: &mut Vec<InputReplayCommand>,
    decorator_storage: &mut DecoratorStorageBoundary,
    stairs_state: &mut StairsStateBoundary,
    raycast_world: &mut RaycastWorldBoundary,
    decorator_archivist: &mut DecoratorArchivistBoundary,
) {
    let _ = out_input_replay_commands;
    // O：前桶命令集 len(+0x10)==0 → pop_front + 应用负载。
    if buffer
        .front_mut_bucket()
        .is_some_and(|b| b.commands.is_empty())
    {
        let bucket = buffer.pop_front();
        apply_bucket_payloads(
            &bucket,
            &mut *decorator_storage,
            &mut *stairs_state,
            &mut *raycast_world,
            &mut *decorator_archivist,
        );
        raycast_world.flush_colliders();
    }
}

/// 桶清理辅助：从桶命令集移除已完成索引（O：各领域系统尾部 remove 循环）。
pub fn remove_done_commands(bucket: &mut ReplayBucket, mut done: Vec<usize>) {
    done.sort_unstable();
    for idx in done.into_iter().rev() {
        if idx < bucket.commands.len() {
            bucket.commands.remove(idx);
        }
    }
}
