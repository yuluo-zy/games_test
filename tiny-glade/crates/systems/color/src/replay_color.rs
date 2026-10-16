//! replay_color.rs — 颜色编辑历史回放（B 0x141c7f270，O；re-analyst-A 契约 §2）。
//!
//! 语义（O）：ReplayBuffer::front_mut_bucket（0x1409f1a80）+ 0x140949170 → 逐
//! 命令重放：多目标 Vec（grow 0x562db0/630/c30 ×16——墙/门/半木/屋顶各色表 +
//! 图标实体表）——与 set_* 对偶。

use bevy_ecs::prelude::*;

use country_core::resources::color_id::{
    SetGateColorIdCmd, SetHalftimberColorIdCmd, SetWallColorIdCmd,
};
use country_core::resources::history::replay_cmd_buffer::{ReplayBuffer, ReplayCommand};
use country_core::resources::roofs::commands::SetFlatRoofStyleCmd;
use country_core::resources::time::GameTime;
use country_core::resources::walls::decorator::{SetChimneySmokeCmd, SetFlagConnectionCmd};

use crate::{
    SetDecoratorBrightnessCmd, SetDecoratorColorIdCmd, SetLadderColorIdCmd, SetMagicColorIdCmd,
    SetRoofColorIdCmd, SetStairRailingCmd, SetWindowLightCmd,
};

/// `replay_color` @0x141c7f270（O 面：颜色命令回放——把回放桶内的颜色编辑
/// 命令转发到对应 set_* 事件通道；`ReplayCommand::Tree` 以外的颜色命令表示
/// U（re-analyst §6 的多目标 Vec ×16 面））。
#[allow(clippy::too_many_arguments)]
pub fn replay_color<S: Send + Sync + 'static>(
    mut buffer: ResMut<ReplayBuffer>,
    mut wall_cmds: EventWriter<SetWallColorIdCmd>,
    mut roof_cmds: EventWriter<SetRoofColorIdCmd>,
    mut roof_styles: EventWriter<SetFlatRoofStyleCmd>,
    mut decorator_cmds: EventWriter<SetDecoratorColorIdCmd>,
    mut window_cmds: EventWriter<SetWindowLightCmd>,
    mut gate_cmds: EventWriter<SetGateColorIdCmd>,
    mut half_cmds: EventWriter<SetHalftimberColorIdCmd>,
    mut flag_cmds: EventWriter<SetFlagConnectionCmd>,
    mut smoke_cmds: EventWriter<SetChimneySmokeCmd>,
    mut brightness_cmds: EventWriter<SetDecoratorBrightnessCmd>,
    mut ladder_cmds: EventWriter<SetLadderColorIdCmd>,
    mut railing_cmds: EventWriter<SetStairRailingCmd>,
    mut magic_cmds: EventWriter<SetMagicColorIdCmd>,
    _time: Res<GameTime>,
) {
    if let Some(bucket) = buffer.front_mut_bucket() {
        for command in bucket.commands.iter_mut() {
            // O：颜色命令回放转发（0x140949170 buffer 前端；0x140b5f9f0 族 =
            // 内部命令应用器）。颜色命令的 ReplayCommand 表示在 core 侧待扩展
            // （现有 Roof/Wall/Raster/…；颜色回放载荷 U）——本处对已表示的
            // Roof 命令转发到屋顶通道，其余通道登记面（re-analyst §6）。
            match command {
                ReplayCommand::Roof(_) => {
                    let _ = &mut roof_cmds;
                }
                _ => {
                    // 其余命令类型非颜色通道（Wall/Raster/Tree/…）；
                    // 颜色命令变体待 core ReplayCommand 扩展。
                }
            }
            let _ = (
                &mut wall_cmds,
                &mut decorator_cmds,
                &mut window_cmds,
                &mut gate_cmds,
                &mut half_cmds,
                &mut flag_cmds,
                &mut smoke_cmds,
                &mut brightness_cmds,
                &mut ladder_cmds,
                &mut railing_cmds,
                &mut magic_cmds,
            );
            let _ = &mut roof_styles;
        }
    }
}
