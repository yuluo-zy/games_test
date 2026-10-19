//! `replay_decorator.rs` — 装饰回放（0x14080c8d0，fresh Ghidra shared 反编译
//! `work/wave3-decorator-visual/jobs/decomp_0x14080c8d0.c`，997B）。
//!
//! 参数面（O，寄存器）：in_RCX = ResMut<GameTime>（delta `*+0x28`）、in_RDX =
//! ResMut<ReplayBuffer>（speed `*+0x28`、tick 回写 +0x10/+0x1c）、in_R8 =
//! ResMut<DecoratorArchivist>（cull-allow 集合 + tick 回写 +0x10/+0x1c）。
//!
//! 行为（O，fresh）：
//! 1. 取 ReplayBuffer 前端桶（`front_mut_bucket`）；
//! 2. 取桶 `+0x18/+0x20` 的 **`Arc<[DecoratorId]>` cull-allow id 表**（指针置
//!    0 = 取出），逐 id 入 `DecoratorArchivist.culling.allowed_ids` → Arc drop；
//! 3. 扫描命令（桶 `+0x8` 指针 / `+0x10` 长度，**每条 0x30B**，布局
//!    `{tag u8@0, flags@+8, duration f32@+0xc, payload@+0x10, elapsed
//!    f32@+0x18}`；**跳过非 tag-7 命令**，只处理 tag==7 装饰命令）：
//!    - `flags bit0==0` → 立即完成（记移除索引）；
//!    - 否则 `required = max(duration/speed, 0.05)`（**0.05 为 O floor**）；
//!      `required < elapsed` → 完成记索引；
//!    - 未完成命令 `elapsed += delta`（**不乘 speed**）。
//! 4. 完成索引**反转**后逐条 `Vec::remove`（0x30B memmove 右移）——core
//!    `ReplayBucket::remove_command` 同步压缩 timing。
//!
//! **core 缺口已闭合**（CONTRACT §0.4）：`ReplayCommand::Decorator(
//! DecoratorReplayCmd)`、`ReplayBucketAux.cull_allow`、`ReplayBucket.timing`
//! 均已在 core；本系统按 core 类型真实接线，不再返回 None/空壳。
//! 装饰存储的真正恢复在 Pop 阶段（`restore_decorators(diff, undo)`），不在
//! 本系统——原版 0x14080c8d0 不写 `DecoratorStorage`。
use bevy_ecs::prelude::*;

use country_core::resources::history::replay_cmd_buffer::{DecoratorReplayCmd, ReplayCommand};
use country_core::resources::history::ReplayBuffer;
use country_core::resources::time::GameTime;
use country_core::resources::walls::decorator_archivist::LiveDecoratorArchivist;

/// 原生 timed 完成判据分母下限（0x3dcccccd/2 语义：0.05f）。
pub const MIN_REQUIRED: f32 = 0.05;

/// `replay_decorator` @0x14080c8d0（O）。
pub fn replay_decorator(
    time: Res<GameTime>,
    mut replay: ResMut<ReplayBuffer>,
    mut archivist: ResMut<LiveDecoratorArchivist>,
) {
    // O：桶 cull-allow 表 take 语义（原生 `*(bucket+0x18) = 0`）。
    let cull = replay
        .aux
        .front_mut()
        .and_then(|aux| aux.cull_allow.take());
    if let Some(ids) = cull {
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
        // 机器码只处理 tag==7（装饰命令）；其余命令原样保留。
        let ReplayCommand::Decorator(command) = command else {
            continue;
        };
        // flags bit0==0 → 立即完成。
        if !command.timed {
            removed.push(index);
        } else {
            // required = max(duration/speed, 0.05)；`required < elapsed` 完成。
            let mut required = command.duration / speed;
            if required <= MIN_REQUIRED {
                required = MIN_REQUIRED;
            }
            if required < command.elapsed {
                removed.push(index);
            }
            // O：无条件推进（即使本帧刚判定完成），delta 不乘 speed。
            command.elapsed += delta;
        }
    }
    if removed.len() > 1 {
        removed.reverse();
    }
    for index in removed {
        // core 同步压缩 commands/timing（禁止直接 commands.remove）。
        bucket.remove_command(index);
    }
}

/// 从 core ReplayCommand 提取装饰命令（O：tag-7 判别）。
pub fn decorator_cmd_of(command: &ReplayCommand) -> Option<&DecoratorReplayCmd> {
    match command {
        ReplayCommand::Decorator(cmd) => Some(cmd),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use country_core::resources::history::decorator_edit::DecoratorV5Edit;
    use country_core::resources::history::replay_cmd_buffer::{ReplayBucket, ReplayBucketAux};
    use country_core::resources::walls::decorator::DecoratorId;
    use std::sync::Arc;

    fn fixture() -> (World, Schedule) {
        let mut world = World::new();
        world.insert_resource(ReplayBuffer::new(1.0));
        world.insert_resource(GameTime {
            delta: 0.1,
            ..Default::default()
        });
        world.init_resource::<LiveDecoratorArchivist>();
        // 空桶弹出由 core `pop_empty_replay_bucket` 承担（原生 replay 链）。
        world.init_resource::<bevy_ecs::event::Events<
            country_core::systems::history::ReplayBucketFinished,
        >>();
        let mut schedule = Schedule::default();
        schedule.add_systems((
            replay_decorator,
            country_core::systems::history::pop_empty_replay_bucket,
        )
            .chain());
        (world, schedule)
    }

    fn push(world: &mut World, elapsed: f32, instant: bool, aux: ReplayBucketAux) {
        let mut command = DecoratorReplayCmd::expanded(
            Arc::new(DecoratorV5Edit::default()),
            false,
            instant,
        );
        command.elapsed = elapsed;
        world.resource_mut::<ReplayBuffer>().push_with_aux(
            ReplayBucket {
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
        // elapsed 0.25 > 0.2 → 完成并移除（timing 同步压缩）。
        schedule.run(&mut world);
        assert!(world.resource::<ReplayBuffer>().is_empty());
    }
}
