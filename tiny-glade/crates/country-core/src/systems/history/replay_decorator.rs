//! B `systems/history/replay_decorator.rs` — 装饰回放命令消费。
//!
//! fresh 证据覆盖**命令生成**：`generate_replay_cmds_from_edit` tag 10
//! （DecoratorV5，0x14143fed0）产出命令 tag 7，布局
//! `{tag@0, flags bit0=timed @+8, duration=0.2 @+0xc,
//! Arc<DecoratorHistoryEdit> @+0x10, elapsed @+0x18, undo @+0x1c}`。
//! tag 11（Decorator）在原版生成器里是 `unreachable!()`。
//!
//! 原版 `system_decorator::replay_decorator` 0x14080c8d0 **不写
//! `DecoratorStorage`**。它只做两件事：
//! 1. 取桶携带的 cull-allow id 表（原生 bucket `+0x18/+0x20` 的
//!    `Arc<[DecoratorId]>`，**take** 语义）逐 id 插入
//!    `DecoratorArchivist.culling.allowed_ids`；None 不插入（清空由
//!    `history_clear_previous_state` 在 NewReplayCommandEvent 时完成）。
//! 2. 扫描 tag-7 命令：instant 立即完成；timed `required=max(duration/speed,
//!    0.05)`，`required < elapsed` 完成；随后无条件 `elapsed += GameTime.delta`
//!    （不乘 speed）。完成后索引反转、逐个移除。
//!
//! 装饰存储的真正恢复在 Pop 阶段（`restore_decorators(diff, reverse=undo)`），
//! 不在本系统。`DecoratorHistoryReplayApply` 事件已删除，避免回放中重复写存储。
use crate::resources::{
    history::{ReplayBuffer, ReplayCommand},
    time::GameTime,
    walls::decorator_archivist::LiveDecoratorArchivist,
};
use bevy_ecs::prelude::*;

/// timed 完成判据的分母下限（0x3dcccccd/2 语义：0.05f）。
const MIN_REQUIRED: f32 = 0.05;

pub fn replay_decorator(
    time: Res<GameTime>,
    mut replay: ResMut<ReplayBuffer>,
    archivist: Option<ResMut<LiveDecoratorArchivist>>,
) {
    // O：桶 cull-allow 表 take 语义（原生 `*(bucket+0x18) = 0`）。
    let cull = replay.aux.front_mut().and_then(|aux| aux.cull_allow.take());
    if let (Some(ids), Some(mut archivist)) = (cull, archivist) {
        for id in ids.iter() {
            archivist.culling.allowed_ids.insert(id.0);
        }
    }
    let speed = replay.speed;
    let delta = time.delta;
    let Some(bucket) = replay.front_mut_bucket() else {
        return;
    };
    let mut removed: Vec<usize> = Vec::new();
    for (index, command) in bucket.commands.iter_mut().enumerate() {
        let ReplayCommand::Decorator(command) = command else {
            continue;
        };
        if !command.timed {
            removed.push(index);
        } else {
            let mut required = command.duration / speed;
            if required <= MIN_REQUIRED {
                required = MIN_REQUIRED;
            }
            if required < command.elapsed {
                removed.push(index);
            }
            // O：无条件推进，即使本帧刚判定完成。
            command.elapsed += delta;
        }
    }
    if removed.len() > 1 {
        removed.reverse();
    }
    for index in removed {
        bucket.remove_command(index);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::history::decorator_edit::{DecoratorHistoryEdit, DecoratorV5Edit};
    use crate::resources::history::replay_cmd_buffer::{DecoratorReplayCmd, ReplayBucketAux};
    use crate::resources::walls::decorator::DecoratorId;
    use crate::resources::walls::decorator_archivist::LiveDecoratorArchivist;
    use std::sync::Arc;

    fn fixture() -> (World, Schedule) {
        let mut world = World::new();
        world.insert_resource(ReplayBuffer::new(1.0));
        world.insert_resource(GameTime {
            delta: 0.1,
            ..Default::default()
        });
        world.init_resource::<LiveDecoratorArchivist>();
        world.init_resource::<Events<crate::systems::history::ReplayBucketFinished>>();
        let mut schedule = Schedule::default();
        schedule.add_systems(
            (
                replay_decorator,
                crate::systems::history::pop_empty_replay_bucket,
            )
                .chain(),
        );
        (world, schedule)
    }

    fn push(world: &mut World, elapsed: f32, instant: bool, aux: ReplayBucketAux) {
        let mut command =
            DecoratorReplayCmd::expanded(Arc::new(DecoratorV5Edit::default()), false, instant);
        command.elapsed = elapsed;
        world.resource_mut::<ReplayBuffer>().push_with_aux(
            crate::resources::history::ReplayBucket {
                entry_index: 0,
                undo: false,
                commands: vec![ReplayCommand::Decorator(command)],
                timing: vec![Default::default()],
            },
            aux,
        );
    }

    #[test]
    fn cull_allow_is_taken_into_archivist_allowed_set() {
        let (mut world, mut schedule) = fixture();
        let aux = ReplayBucketAux {
            cull_allow: Some(Arc::from(vec![DecoratorId(4), DecoratorId(9)])),
            ..Default::default()
        };
        push(&mut world, 0.0, true, aux);
        schedule.run(&mut world);
        let archivist = world.resource::<LiveDecoratorArchivist>();
        assert!(archivist.culling.allowed_ids.contains(&4));
        assert!(archivist.culling.allowed_ids.contains(&9));
    }

    #[test]
    fn timed_completion_uses_floor_and_frame_delta() {
        let (mut world, mut schedule) = fixture();
        // required = max(0.2/1.0, 0.05) = 0.2；elapsed 0.15 未完成 → +0.1。
        push(&mut world, 0.15, false, ReplayBucketAux::default());
        schedule.run(&mut world);
        assert_eq!(world.resource::<ReplayBuffer>().len(), 1);
        // elapsed 0.25 > 0.2 → 完成并移除。
        let command = match &world
            .resource::<ReplayBuffer>()
            .buckets
            .front()
            .unwrap()
            .commands[0]
        {
            ReplayCommand::Decorator(command) => command.clone(),
            _ => panic!("expected decorator command"),
        };
        assert!((command.elapsed - 0.25).abs() < 1e-6);
        schedule.run(&mut world);
        assert!(world.resource::<ReplayBuffer>().is_empty());
    }

    #[test]
    fn replay_does_not_touch_decorator_storage() {
        let (mut world, mut schedule) = fixture();
        world.init_resource::<crate::resources::walls::decorator_storage::DecoratorStorage>();
        let aux = ReplayBucketAux {
            decorator_diff: Some(Arc::new(Default::default())),
            ..Default::default()
        };
        push(&mut world, 0.0, true, aux);
        schedule.run(&mut world);
        // 命令完成只移除；存储由 Pop 的 restore_decorators 恢复。
        assert!(world.resource::<ReplayBuffer>().is_empty());
        assert!(
            world
                .resource::<crate::resources::walls::decorator_storage::DecoratorStorage>()
                .per_wall
                .is_empty()
        );
    }
}
