//! `replay_stairs.rs` — 楼梯回放系统（B `systems/history/replay_stairs.rs`，1 业务）。
//!
//! `replay_stairs`（0x140af75b0，O）：前桶过滤 tag 10 命令；动画门控
//! `elapsed < max(duration/speed, 下限)` 时仅推进；就绪（或 instant）→
//! undo==0 → hashbrown insert 边入 `StairsRemovedSupports`（键=edge1），
//! undo!=0 → `remove_entry(edge1, edge2)`；查附挂装饰（`DecoratorStorage::lookup_decorator_info`
//! + type 字节 `+0x11 | 0xf8 > 0xf9` 跳过）+ `CachedDecoratorTransforms`（键=edge1）→
//! push `PlayStairsAudioCmd`；完成后逆序移除命令。
//! 章节：`chapters/systems/history/replay_stairs.rs.md` + `history/ALGORITHMS.md` §2.6。
use crate::{
    resources::{
        history::{ReplayBuffer, ReplayCommand},
        stairs::stairs_removed_supports::StairsRemovedSupports,
        time::GameTime,
        walls::{
            cached_decorator_transforms::CachedDecoratorTransforms,
            decorator::{DecoratorId, DecoratorInfo},
            decorator_storage::DecoratorStorage,
        },
    },
    systems::{audio::PlayStairsAudioCmd, history::HistoryReplayFailed},
};
use bevy_ecs::prelude::*;
use glam::Vec3;

/// 装饰目标判别字节（DecoratorInfo +0x11）过滤：`| 0xf8 > 0xf9` → 跳过该装饰。
fn decorator_skipped(info: &DecoratorInfo) -> bool {
    let tag_byte = ((info.dst_payload >> 8) & 0xff) as u8;
    (tag_byte | 0xf8) > 0xf9
}

/// 前桶遍历 + 动画门控 + 边集写入 + 附挂装饰/音频事件 + 完成命令逆序移除。
pub fn replay_stairs(
    time: Res<GameTime>,
    mut replay: ResMut<ReplayBuffer>,
    mut removed: ResMut<StairsRemovedSupports>,
    decorators: Res<DecoratorStorage>,
    cached: Res<CachedDecoratorTransforms>,
    mut audio: EventWriter<PlayStairsAudioCmd>,
    _failures: EventWriter<HistoryReplayFailed>,
) {
    let dt = time.delta;
    let speed = replay.speed;
    let Some(bucket) = replay.front_mut_bucket() else {
        return;
    };
    let mut done = Vec::new();
    for (index, command) in bucket.commands.iter_mut().enumerate() {
        let ReplayCommand::Stairs(cmd) = command else {
            continue;
        };
        if !cmd.advance(dt, speed) {
            continue;
        }
        if !cmd.undo {
            removed.record(cmd.edge1, cmd.edge2);
        } else {
            removed.erase(cmd.edge1);
        }
        // 附挂装饰查询：type 字节过滤，命中才携带 decorator 对（@+0x28 语义）。
        let decorator = decorators
            .lookup_decorator_info(DecoratorId(cmd.edge1))
            .filter(|info| !decorator_skipped(info));
        let position = cached
            .try_position(cmd.edge1)
            .unwrap_or(Vec3::ZERO);
        audio.write(PlayStairsAudioCmd {
            kind: 0,
            position,
            values: [0.0; 4],
            tail: [0.0; 2],
            bytes: [0; 3],
        });
        let _ = decorator; // 事件载荷精确映射 U（音频域 PlayStairsAudioCmd 消费端待接线）。
        cmd.finish();
        done.push(index);
    }
    for &index in done.iter().rev() {
        // 已完成命令逆序移除（memmove 压缩语义）。
        bucket.commands.remove(index);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        resources::{
            history::{ReplayBucket, ReplayBuffer},
            stairs::stairs_removed_supports::StairsRemovedSupports,
            time::GameTime,
        },
        systems::history::{
            pop_empty_replay_bucket, HistoryReplayFailed, ReplayBucketFinished,
            register_history_events,
        },
    };

    #[test]
    fn redo_records_and_undo_erases_support_edge() {
        let mut world = World::new();
        register_history_events(&mut world);
        world.init_resource::<StairsRemovedSupports>();
        world.init_resource::<crate::resources::walls::decorator_storage::DecoratorStorage>();
        world.init_resource::<crate::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>();
        world.init_resource::<Events<PlayStairsAudioCmd>>();
        world.init_resource::<Events<HistoryReplayFailed>>();
        world.init_resource::<Events<ReplayBucketFinished>>();
        world.insert_resource(ReplayBuffer::new(1.0));
        world.insert_resource(GameTime {
            delta: 0.2,
            ..Default::default()
        });

        let push = |world: &mut World, edge: (u32, u32), undo: bool| {
            world.resource_mut::<ReplayBuffer>().push(ReplayBucket {
                entry_index: 0,
                undo,
                commands: vec![ReplayCommand::Stairs(
                    crate::resources::history::stairs_history_edit::StairsReplayCmd::new(
                        edge.0, edge.1, undo, true,
                    ),
                )],
            });
        };
        // 完成命令由 replay_stairs 移除；空桶由 pop_empty_replay_bucket 弹出
        // （与 add_history_cpu_systems 的真实接线一致）。
        let mut schedule = Schedule::default();
        schedule.add_systems((replay_stairs, pop_empty_replay_bucket).chain());
        push(&mut world, (3, 4), false); // redo: record
        schedule.run(&mut world);
        let removed = world.resource::<StairsRemovedSupports>();
        assert!(removed.contains(3));
        assert_eq!(removed.edges.get(&3), Some(&4));
        assert!(world.resource::<ReplayBuffer>().is_empty());

        push(&mut world, (3, 4), true); // undo: erase
        schedule.run(&mut world);
        let removed = world.resource::<StairsRemovedSupports>();
        assert!(!removed.contains(3));
        assert!(world.resource::<ReplayBuffer>().is_empty());
    }

    #[test]
    fn animation_gating_advances_then_processes() {
        let mut world = World::new();
        register_history_events(&mut world);
        world.init_resource::<StairsRemovedSupports>();
        world.init_resource::<crate::resources::walls::decorator_storage::DecoratorStorage>();
        world.init_resource::<crate::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>();
        world.init_resource::<Events<PlayStairsAudioCmd>>();
        world.init_resource::<Events<HistoryReplayFailed>>();
        world.init_resource::<Events<ReplayBucketFinished>>();
        world.insert_resource(ReplayBuffer::new(2.0));
        world.insert_resource(GameTime {
            delta: 0.2,
            ..Default::default()
        });
        let mut cmd = crate::resources::history::stairs_history_edit::StairsReplayCmd::new(1, 2, false, false);
        cmd.duration = 1.0;
        world.resource_mut::<ReplayBuffer>().push(ReplayBucket {
            entry_index: 0,
            undo: false,
            commands: vec![ReplayCommand::Stairs(cmd)],
        });
        let mut schedule = Schedule::default();
        schedule.add_systems((replay_stairs, pop_empty_replay_bucket).chain());
        // required = max(1.0/2.0, 0.1) = 0.5；原版"先判后加"：
        //   run1: 0.0<=0.5 → 0.2；run2: 0.2<=0.5 → 0.4；run3: 0.4<=0.5 → 0.6；均等下一帧。
        schedule.run(&mut world);
        schedule.run(&mut world);
        schedule.run(&mut world);
        assert_eq!(world.resource::<ReplayBuffer>().len(), 1);
        assert!(!world.resource::<StairsRemovedSupports>().contains(1));
        // run4: 0.6 > 0.5 → 处理，命令移除，空桶弹出。
        schedule.run(&mut world);
        assert!(world.resource::<StairsRemovedSupports>().contains(1));
        assert!(world.resource::<ReplayBuffer>().is_empty());
    }
}
