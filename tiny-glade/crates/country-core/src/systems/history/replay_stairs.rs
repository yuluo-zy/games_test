//! `replay_stairs.rs` — 楼梯回放系统（B `systems/history/replay_stairs.rs`，1 业务）。
//!
//! `replay_stairs`（0x140af75b0，O）：前桶过滤 tag 10 命令；**双路径**——instant 路径
//! （命令 flags bit0==0，立即处理）与动画路径（bit0==1，门控 `elapsed <= max(duration/speed,
//! 0.05)` 时仅推进）。对 `StairsRemovedSupports` 的增删键为 **全边键 `StairGraphEdge`**；
//! undo 语义由 **payload+0x10 bit0**（SupportsDelete=0 / SupportsRecreate=1）决定：
//! 0 → undo==0 insert / undo!=0 remove_entry；1 与之相反（命令 instant 位不参与）。
//! 查附挂装饰（`DecoratorStorage::lookup_decorator_info` + `+0x41` 字节 `| 0xf8 > 0xf9` 跳过）+
//! `CachedDecoratorTransforms`（键=edge1）→ push `PlayStairsAudioCmd`；完成后逆序移除命令。
//! 章节：`chapters/systems/history/replay_stairs.rs.md` + `history/ALGORITHMS.md` §2.6。
use crate::{
    resources::{
        history::{ReplayBuffer, ReplayCommand},
        stairs::{stairs_removed_supports::StairsRemovedSupports, stairs_state::StairGraphEdge},
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

/// 装饰目标判别字节（DecoratorInfo +0x41）过滤：`| 0xf8 > 0xf9` → 跳过该装饰。
/// fresh 0x140af77da/0x140af784c：`byte [info + 0x41]`（= `DecoratorInfo::extra_style`），
/// 非 dst_payload byte1（+0x11，旧偏移错位）。
fn decorator_skipped(info: &DecoratorInfo) -> bool {
    (info.extra_style() | 0xf8) > 0xf9
}

/// 前桶遍历 + 动画门控 + 边集写入（双路径 undo）+ 附挂装饰/音频事件 + 完成命令逆序移除。
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
        let edge = StairGraphEdge::new(cmd.edge1, cmd.edge2);
        // fresh 0x140af75b0：命令 flags bit0（@+8）只选即时/动画路径；两条路径在
        // 0x140af77b7 / 0x140af7829 都读 payload+0x10 bit0 选 undo 语义：
        //   0（SupportsDelete）：undo==0 → insert、undo!=0 → remove_entry；
        //   1（SupportsRecreate）：undo==0 → remove_entry、undo!=0 → insert。
        if cmd.recreate {
            if !cmd.undo {
                removed.erase(&edge);
            } else {
                removed.record(edge);
            }
        } else if !cmd.undo {
            removed.record(edge);
        } else {
            removed.erase(&edge);
        }
        // 附挂装饰查询：type 字节过滤，命中才携带 decorator 对（@+0x28 语义）。
        let decorator = decorators
            .lookup_decorator_info(DecoratorId(cmd.edge1))
            .filter(|info| !decorator_skipped(info));
        let position = cached.try_position(cmd.edge1).unwrap_or(Vec3::ZERO);
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
            HistoryReplayFailed, ReplayBucketFinished, pop_empty_replay_bucket,
            register_history_events,
        },
    };

    #[test]
    fn animated_redo_records_and_undo_erases_support_edge() {
        // 动画路径（instant=false）：undo==0 → insert、undo!=0 → remove_entry（机器码动画路径）。
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

        let push = |world: &mut World, edge: (u32, u32), undo: bool, instant: bool| {
            world.resource_mut::<ReplayBuffer>().push(ReplayBucket {
                entry_index: 0,
                undo,
                commands: vec![ReplayCommand::Stairs(
                    crate::resources::history::stairs_history_edit::StairsReplayCmd::new(
                        edge.0, edge.1, undo, instant,
                    ),
                )],
                timing: vec![Default::default(); 1],
            });
        };
        let mut schedule = Schedule::default();
        schedule.add_systems((replay_stairs, pop_empty_replay_bucket).chain());
        // 动画门控：required = max(0.2/1.0, 0.05) = 0.2（fresh 生成器命令时长 0.2），dt=0.2。
        // run1: elapsed 0→0.2（<=0.2 先判后加），未处理；run2: 0.2>0.2 假 → 0.4；run3 处理。
        push(&mut world, (3, 4), false, false); // redo: record
        schedule.run(&mut world);
        schedule.run(&mut world);
        schedule.run(&mut world);
        let removed = world.resource::<StairsRemovedSupports>();
        assert!(removed.contains(&StairGraphEdge::new(3, 4)));
        assert!(world.resource::<ReplayBuffer>().is_empty());

        push(&mut world, (3, 4), true, false); // undo: erase
        schedule.run(&mut world);
        schedule.run(&mut world);
        schedule.run(&mut world);
        let removed = world.resource::<StairsRemovedSupports>();
        assert!(!removed.contains(&StairGraphEdge::new(3, 4)));
        assert!(world.resource::<ReplayBuffer>().is_empty());
    }

    #[test]
    fn recreate_kind_reverses_undo_semantics_on_instant_path() {
        // fresh 0x140af75b0：undo 方向由 payload+0x10 bit0 决定（SupportsRecreate=1 反向），
        // instant 路径（命令 flags bit0==0）只影响即时处理。
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
        let mut schedule = Schedule::default();
        schedule.add_systems((replay_stairs, pop_empty_replay_bucket).chain());
        // redo（undo==0，instant，recreate=1）→ remove_entry：边不在存档集。
        world.resource_mut::<ReplayBuffer>().push(ReplayBucket {
            entry_index: 0,
            undo: false,
            commands: vec![ReplayCommand::Stairs(
                crate::resources::history::stairs_history_edit::StairsReplayCmd::new_with_kind(
                    3, 4, false, true, true,
                ),
            )],
            timing: vec![Default::default(); 1],
        });
        schedule.run(&mut world);
        let removed = world.resource::<StairsRemovedSupports>();
        assert!(!removed.contains(&StairGraphEdge::new(3, 4)));
        assert!(world.resource::<ReplayBuffer>().is_empty());
        // undo（undo!=0，instant，recreate=1）→ insert：边进入存档集。
        world.resource_mut::<ReplayBuffer>().push(ReplayBucket {
            entry_index: 0,
            undo: true,
            commands: vec![ReplayCommand::Stairs(
                crate::resources::history::stairs_history_edit::StairsReplayCmd::new_with_kind(
                    3, 4, true, true, true,
                ),
            )],
            timing: vec![Default::default(); 1],
        });
        schedule.run(&mut world);
        let removed = world.resource::<StairsRemovedSupports>();
        assert!(removed.contains(&StairGraphEdge::new(3, 4)));
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
        let mut cmd = crate::resources::history::stairs_history_edit::StairsReplayCmd::new(
            1, 2, false, false,
        );
        cmd.duration = 1.0;
        world.resource_mut::<ReplayBuffer>().push(ReplayBucket {
            entry_index: 0,
            undo: false,
            commands: vec![ReplayCommand::Stairs(cmd)],
            timing: vec![Default::default(); 1],
        });
        let mut schedule = Schedule::default();
        schedule.add_systems((replay_stairs, pop_empty_replay_bucket).chain());
        // required = max(1.0/2.0, 0.05) = 0.5；原版"先判后加"：
        //   run1: 0.0<=0.5 → 0.2；run2: 0.2<=0.5 → 0.4；run3: 0.4<=0.5 → 0.6；均等下一帧。
        schedule.run(&mut world);
        schedule.run(&mut world);
        schedule.run(&mut world);
        assert_eq!(world.resource::<ReplayBuffer>().len(), 1);
        assert!(
            !world
                .resource::<StairsRemovedSupports>()
                .contains(&StairGraphEdge::new(1, 2))
        );
        // run4: 0.6 > 0.5 → 处理，命令移除，空桶弹出。
        schedule.run(&mut world);
        assert!(
            world
                .resource::<StairsRemovedSupports>()
                .contains(&StairGraphEdge::new(1, 2))
        );
        assert!(world.resource::<ReplayBuffer>().is_empty());
    }
}
