//! B `systems/history/replay_color.rs` — 颜色回放命令消费（结构模型）。
//!
//! fresh 证据只覆盖**命令生成**：`generate_replay_cmds_from_edit` tag 9
//! （0x14143fcc0）产出命令 tag 6，布局
//! `{tag@0, flags bit0=timed @+8, duration=0.2 @+0xc, Arc<ColorEdit> @+0x10,
//! elapsed @+0x18, undo @+0x1c}`。颜色应用的 owner（GladeColorCustomization /
//! 各 target 域）在 crate 内没有对应写入面，因此本系统按 history 域既有模式
//! 推进命令并把**完成应用**以事件交给实际 color owner（与 replay_wall 的
//! `WallRestoreColor` 同构）；**消费者本体登记 U**，不声称已写入颜色。
use crate::resources::{
    history::{ReplayBuffer, ReplayCommand},
    time::GameTime,
};
use bevy_ecs::prelude::*;
use std::sync::Arc;

/// 颜色回放应用请求（宿主/颜色域消费；原版颜色 owner 写回面未恢复）。
#[derive(Event, Clone, Debug)]
pub struct ColorHistoryReplayApply {
    pub edit: Arc<crate::resources::history::color_edit::ColorEdit>,
    pub undo: bool,
    /// 0..=1 动画进度（instant 路径为 1.0）。
    pub progress: f32,
}

/// `cubic_in_out`（utils::easing_functions 0x140c961e0）在 wall process 中的
/// 同一曲线：`t<0.5 → 4t³`，否则 `0.5(2t-2)³+1`。
fn cubic_in_out(t: f32) -> f32 {
    if t < 0.5 {
        ((4.0 * t) * t) * t
    } else {
        let x = (t + t) + (-2.0);
        ((0.5 * x) * x) * x + 1.0
    }
}

pub fn replay_color(
    time: Res<GameTime>,
    mut replay: ResMut<ReplayBuffer>,
    mut apply: EventWriter<ColorHistoryReplayApply>,
) {
    let dt = time.delta * replay.speed;
    let Some(bucket) = replay.front_mut_bucket() else {
        return;
    };
    for command in &mut bucket.commands {
        let ReplayCommand::Color(command) = command else {
            continue;
        };
        let progress = if !command.timed {
            1.0
        } else {
            let ratio = (command.elapsed / command.duration).clamp(0.0, 1.0);
            cubic_in_out(ratio)
        };
        apply.write(ColorHistoryReplayApply {
            edit: command.edit.clone(),
            undo: command.undo,
            progress,
        });
        if command.timed {
            command.elapsed += dt;
        }
    }
    // 完成判据与 wall process 相同：timed 命令 elapsed 超过 duration 后移除。
    bucket.commands.retain(|command| {
        !matches!(
            command,
            ReplayCommand::Color(command) if command.timed && command.elapsed > command.duration
        )
    });
}
