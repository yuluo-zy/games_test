//! B `systems/history/replay_decorator.rs` — 装饰回放命令消费（结构模型）。
//!
//! fresh 证据覆盖**命令生成**：`generate_replay_cmds_from_edit` tag 10
//! （DecoratorV5，0x14143fed0）产出命令 tag 7，布局
//! `{tag@0, flags bit0=timed @+8, duration=0.2 @+0xc,
//! Arc<DecoratorHistoryEdit> @+0x10, elapsed @+0x18, undo @+0x1c}`。
//! tag 11（Decorator）在原版生成器里是 `unreachable!()`。
//!
//! 装饰应用的 owner（DecoratorStorage / DecoratorArchivist）在 crate 内由
//! walls 域持有；本系统把完成的 `DecoratorHistoryEdit` 差异交给宿主事件，
//! **消费者本体登记 U**，不声称已改动装饰存储。
use crate::resources::{
    history::{ReplayBuffer, ReplayCommand},
    time::GameTime,
};
use bevy_ecs::prelude::*;
use std::sync::Arc;

/// 装饰回放应用请求（宿主/walls 装饰域消费）。
#[derive(Event, Clone, Debug)]
pub struct DecoratorHistoryReplayApply {
    pub edit: Arc<crate::resources::history::decorator_edit::DecoratorHistoryEdit>,
    pub undo: bool,
    pub progress: f32,
}

fn cubic_in_out(t: f32) -> f32 {
    if t < 0.5 {
        ((4.0 * t) * t) * t
    } else {
        let x = (t + t) + (-2.0);
        ((0.5 * x) * x) * x + 1.0
    }
}

pub fn replay_decorator(
    time: Res<GameTime>,
    mut replay: ResMut<ReplayBuffer>,
    mut apply: EventWriter<DecoratorHistoryReplayApply>,
) {
    let dt = time.delta * replay.speed;
    let Some(bucket) = replay.front_mut_bucket() else {
        return;
    };
    for command in &mut bucket.commands {
        let ReplayCommand::Decorator(command) = command else {
            continue;
        };
        let progress = if !command.timed {
            1.0
        } else {
            let ratio = (command.elapsed / command.duration).clamp(0.0, 1.0);
            cubic_in_out(ratio)
        };
        apply.write(DecoratorHistoryReplayApply {
            edit: command.edit.clone(),
            undo: command.undo,
            progress,
        });
        if command.timed {
            command.elapsed += dt;
        }
    }
    bucket.commands.retain(|command| {
        !matches!(
            command,
            ReplayCommand::Decorator(command)
                if command.timed && command.elapsed > command.duration
        )
    });
}
