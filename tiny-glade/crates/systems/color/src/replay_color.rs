//! replay_color.rs — 颜色编辑历史回放（B 0x141c7f270，1450B；
//! restorer-color-remaining 2026-10-17 fresh 反编译 `fresh1_0x141c7f270_replay_color.c`）。
//!
//! **判等 / 重建**（逐语句对照 fresh 反编译，解锁条件已满足：core
//! `ReplayCommand::Color(ColorReplayCmd)` + `ColorEdit` 各 target 载荷已落库，
//! 见 `resources/history/replay_cmd_buffer.rs` / `color_edit.rs`）。
//!
//! ## 真实控制流（fresh）
//!
//! 1. `speed = ReplayBuffer.speed`（原生 +0x28）；`front_mut_bucket()`，无桶 return
//!    （行 110-117）。
//! 2. 遍历桶命令（元素 stride **0x30**，行 165：`ptr + len*0x30`）；只处理
//!    `tag(+0) == 6`（Color 命令），其余 `continue`（行 168）。
//! 3. 载荷 `_Dst = *(cmd+0x10)`（`Arc<ColorEdit>` 的 ArcInner；`edit = _Dst+0x10`）。
//!    判别 `sub = edit[+8] - 3`，`if 0xd < sub { sub = 3 }`（行 170-173；
//!    **原版把越界目标夹到 case 3 而非丢弃**——Rust 闭合枚举不可表示，见下）。
//! 4. **派发门**：仅当 `elapsed(cmd+0x18) == 0.0`（且非 NaN）才写事件（行 183/220/249/…）。
//!    case → 目标/写者（fresh 行 181-597；事件记录布局与各 writer 消费面逐字段对上）：
//!
//! | sub | ColorTarget | 事件写者 | 事件载荷 |
//! |---|---|---|---|
//! | 0 | Wall(3) | `SetWallColorIdCmd` | `{id u64 @+8 = edit+0x10, color @+0x10 = undo?from(0x18):to(0x1c), custom @+0x14 = 1}` |
//! | 1 | Gate(4) | `SetGateColorIdCmd` | 同 Wall |
//! | 2 | Roof(5) | `SetRoofColorIdCmd` | 同 Wall |
//! | 3 | FlatRoof(6) | `SetFlatRoofStyleCmd` | stride 0x20 `{id u64 @+8, style 12B @+0x10 = undo?from:to, sound @+0x1c = 1}` |
//! | 4 | Decorator(7) | `SetDecoratorColorIdCmd` | `{id u32 @+8 = edit+0x0c, value u32 @+0xc = undo?from(0x10):to(0x14), custom @+0x10 = 1}` |
//! | 5 | WindowLight(8) | `SetWindowLightCmd` | 同 Decorator |
//! | 6 | Halftimber(9) | `SetHalftimberColorIdCmd` | 同 Wall（u64 id） |
//! | 7 | FlagBunting(10) | `SetFlagConnectionCmd` | stride 0x10 `{id u32 @+8, value u8 @+0xc = undo?from(edit+4):to(edit+5), custom @+0xd = 1}` |
//! | 8 | Brightness(11) | `SetDecoratorBrightnessCmd` | `{id u32 @+8 = edit+0x0c, value u32 @+0xc = undo?from(0x10):to(0x14), custom @+0x10 = 1}` |
//! | 9 | ChimneySmoke(12) | `SetChimneySmokeCmd` | stride 0x10 同 FlagBunting |
//! | 0xa | StairLadder(13) | `SetLadderColorIdCmd` | 同 Decorator |
//! | 0xb | StairRailing(14) | `SetStairRailingCmd` | stride 0x10 同 FlagBunting |
//! | 0xc | MagicColor(15) | `SetMagicColorIdCmd` | 同 Decorator |
//! | 0xd | StairSupport(16) | **无派发**（switch 无 case 0xd，行 597 直接落到时间/移除块） |
//!
//! 所有 case 的 `undo` 方向一致：`undo == 0` 取较后偏移的字段（`to`），
//! `undo != 0` 取较前字段（`from`）（行 186/223/279/328/369/397/433/466/511）。
//! 所有写者 `custom/play_sound = 1`（行 213/297/345/422/525…）。
//!
//! 5. **时间推进 / 移除**（行 598-622，所有 Color 命令共用）：
//!    - `flags.bit0 (cmd+8) == 0` → 立即记移除下标（不推进 elapsed）。
//!    - 否则 `required = duration(cmd+0xc) / speed`；`if required <= 0.1 { required = 0.1 }`
//!      （Ghidra `if (fVar69 <= 0.1)` 语义，NaN 保持 NaN）；`if required < elapsed`
//!      → 记移除；随后**无条件** `elapsed += delta`（`GameTime.delta` @+0x28）。
//! 6. **移除**：`if removed.len() > 1 { 原地反转 }` 后逐个 `Vec::remove`（行 625-694；
//!    后序释放由 Rust `Drop` 自动完成——原生跳表 0x142f203c0 的
//!    `Arc::drop_slow<...>` 族即 `ReplayCommand` 各载荷的 Drop）。
//!
//! ## 登记（非阻塞，U）
//! - 原版 `sub > 0xd` 时 `sub = 3`（把未知 target 当 FlatRoof 派发）：Rust `ColorEdit`
//!   为闭合枚举，不存在该分支；不构造。
//! - 尾部 drop 跳表 PDB 名称（WallReplayCmd/RasterEdit/CrashMiscInfo/…) 为共享释放
//!   glue 的类型恢复结果，Rust `Drop` 语义等价，不手写。
//! - 入口 `*(in_RCX[2]) = in_RCX+0x1c` 为 Bevy 资源变更 tick 机械，不属业务。
//!
//! 证据：`reports/state/color/logic/decomp/fresh1_0x141c7f270_replay_color.c`
//! （worker=shared fresh，2026-10-17）；ReplayBucket 合同见
//! `reports/unlock/replay-decorator/RESTORE.md` §5.4（Color 不需桶 cull 表）。

use bevy_ecs::prelude::*;

use country_core::resources::roofs::customization::FlatRoofStyleObserved;
use country_core::resources::roofs::customization::FlatRoofStyleRequest;
use country_core::resources::color_id::{
    SetGateColorIdCmd, SetHalftimberColorIdCmd, SetWallColorIdCmd,
};
use country_core::resources::history::color_edit::ColorEdit;
use country_core::resources::history::replay_cmd_buffer::{ReplayBuffer, ReplayCommand};
use country_core::resources::roofs::commands::SetFlatRoofStyleCmd;
use country_core::resources::time::GameTime;

use crate::{
    SetChimneySmokeCmd, SetDecoratorBrightnessCmd, SetDecoratorColorIdCmd, SetFlagConnectionCmd,
    SetLadderColorIdCmd, SetMagicColorIdCmd, SetRoofColorIdCmd, SetStairRailingCmd,
    SetWindowLightCmd,
};

/// 原生删除判据分母下限（反编译 0x3dcccccd = 0.1f，行 610-612）。
const MIN_REQUIRED: f32 = 0.1;

/// undo 方向选择（O：所有 case `undo==0 → to`、`undo!=0 → from`）。
#[inline]
fn pick<T: Copy>(undo: bool, from: T, to: T) -> T {
    if undo {
        from
    } else {
        to
    }
}

/// `replay_color` @0x141c7f270（1450B，判等/重建）。
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
    time: Res<GameTime>,
) {
    // O：speed = ReplayBuffer.speed（+0x28）；delta = GameTime.delta（+0x28）。
    let speed = buffer.speed;
    let delta = time.delta;
    // O：front_mut_bucket()（0x1409f1a80）；无桶即 return。
    let Some(bucket) = buffer.front_mut_bucket() else {
        return;
    };
    // O：移除下标收集（原生 usize 数组；`len > 1` 时原地反转后逐个 remove）。
    let mut removed: Vec<usize> = Vec::new();
    for (index, command) in bucket.commands.iter_mut().enumerate() {
        // O：tag(+0) != 6 → skip（行 168）。
        let ReplayCommand::Color(command) = command else {
            continue;
        };
        // O：派发门 elapsed(+0x18) == 0.0 且非 NaN（行 183 等）。
        if command.elapsed == 0.0 {
            match &*command.edit {
                ColorEdit::Wall { wall_id, from, to } => {
                    wall_cmds.write(SetWallColorIdCmd {
                        // 原生把 ColorEdit 的 u64 wall_id 整 qword 写入事件 +8；
                        // 事件字段拆为 id u32 + _gap4 u32（消费面 @+8 读 8 字节）。
                        id: wall_id.0 as u32,
                        _gap4: (wall_id.0 >> 32) as u32,
                        color: pick(command.undo, from.0, to.0),
                        custom: 1,
                        _pad: [0; 3],
                    });
                }
                ColorEdit::Gate { wall_id, from, to } => {
                    gate_cmds.write(SetGateColorIdCmd {
                        id: wall_id.0 as u32,
                        _gap4: (wall_id.0 >> 32) as u32,
                        color: pick(command.undo, from.0, to.0),
                        custom: 1,
                        _pad: [0; 3],
                    });
                }
                ColorEdit::Roof { wall_id, from, to } => {
                    roof_cmds.write(SetRoofColorIdCmd {
                        id: wall_id.0,
                        color: pick(command.undo, from.0, to.0),
                        custom: 1,
                    });
                }
                ColorEdit::FlatRoof { wall_id, from, to } => {
                    // O：事件 stride 0x20，style 12B 由 from/to 的观察字节整体复制，
                    // sound 字节 = 1（行 279-321）。
                    let style = pick(command.undo, from, to);
                    roof_styles.write(SetFlatRoofStyleCmd(FlatRoofStyleRequest {
                        // 登记：core 存在两个同名 u64 newtype——
                        // `resources::walls::WallId`（ColorEdit 载荷）与
                        // `resources::walls::publication_fields::WallId`
                        // （FlatRoofStyleRequest 字段）。wire 同为 u64，显式转换。
                        wall_id: country_core::resources::walls::publication_fields::WallId(
                            wall_id.0,
                        ),
                        style: FlatRoofStyleObserved::from_observed(*style.observed()),
                        observed_play_sound: 1,
                    }));
                }
                ColorEdit::Decorator {
                    decorator_id,
                    from,
                    to,
                } => {
                    decorator_cmds.write(SetDecoratorColorIdCmd {
                        id: decorator_id.0,
                        color: pick(command.undo, from.0, to.0),
                        custom: 1,
                    });
                }
                ColorEdit::WindowLight {
                    decorator_id,
                    from,
                    to,
                } => {
                    window_cmds.write(SetWindowLightCmd {
                        id: decorator_id.0,
                        value: pick(command.undo, from.0, to.0),
                        custom: 1,
                    });
                }
                ColorEdit::Halftimber { wall_id, from, to } => {
                    half_cmds.write(SetHalftimberColorIdCmd {
                        id: wall_id.0 as u32,
                        _gap4: (wall_id.0 >> 32) as u32,
                        color: pick(command.undo, from.0, to.0),
                        custom: 1,
                        _pad: [0; 3],
                    });
                }
                ColorEdit::FlagBunting {
                    decorator_id,
                    from,
                    to,
                } => {
                    flag_cmds.write(SetFlagConnectionCmd {
                        id: decorator_id.0,
                        value: u8::from(pick(command.undo, *from, *to)),
                        custom: 1,
                    });
                }
                ColorEdit::Brightness {
                    decorator_id,
                    from,
                    to,
                } => {
                    // O：事件 +0xc 写 f32 位模式（u32 存储）。
                    brightness_cmds.write(SetDecoratorBrightnessCmd {
                        id: decorator_id.0,
                        value: pick(command.undo, from.to_bits(), to.to_bits()),
                        custom: 1,
                    });
                }
                ColorEdit::ChimneySmoke {
                    decorator_id,
                    from,
                    to,
                } => {
                    smoke_cmds.write(SetChimneySmokeCmd {
                        id: decorator_id.0,
                        value: u8::from(pick(command.undo, *from, *to)),
                        custom: 1,
                    });
                }
                ColorEdit::StairLadder {
                    decorator_id,
                    from,
                    to,
                } => {
                    ladder_cmds.write(SetLadderColorIdCmd {
                        id: decorator_id.0,
                        value: pick(command.undo, from.0, to.0),
                        custom: 1,
                    });
                }
                ColorEdit::StairRailing {
                    decorator_id,
                    from,
                    to,
                } => {
                    railing_cmds.write(SetStairRailingCmd {
                        id: decorator_id.0,
                        value: pick(command.undo, *from, *to),
                        custom: 1,
                    });
                }
                ColorEdit::MagicColor {
                    decorator_id,
                    from,
                    to,
                } => {
                    magic_cmds.write(SetMagicColorIdCmd {
                        id: decorator_id.0,
                        value: pick(command.undo, from.0, to.0),
                        custom: 1,
                    });
                }
                // O：switch 无 case 0xd（StairSupport=16）→ 不派发（行 597）。
                // 原版对 sub>0xd 的 target 夹到 case 3（行 171-173），闭合枚举
                // 不可表示该分支，登记 U。
                ColorEdit::StairSupport { .. } => {}
            }
        }
        // O：时间推进 / 移除（行 598-622）。
        if !command.timed {
            // flags.bit0 == 0 → 立即完成，不推进 elapsed。
            removed.push(index);
        } else {
            let mut required = command.duration / speed;
            if required <= MIN_REQUIRED {
                required = MIN_REQUIRED;
            }
            if required < command.elapsed {
                removed.push(index);
            }
            // O：本路径无条件 elapsed += delta（即使本帧刚判定完成）。
            command.elapsed += delta;
        }
    }
    // O：`if (1 < count)` 时原地反转 → 降序 remove。
    if removed.len() > 1 {
        removed.reverse();
    }
    for index in removed {
        bucket.commands.remove(index);
    }
}

#[cfg(test)]
mod full_wall_id_tests {
    use super::*;
    use country_core::resources::{
        history::{ReplayBucket, replay_cmd_buffer::ColorReplayCmd},
        terrain_editing::StrokeId,
        walls::WallId,
    };
    use std::sync::Arc;
    #[test]
    fn existing_replay_retains_both_words_for_all_three_wall_families() {
        for (kind, undo) in [
            (0, false),
            (0, true),
            (1, false),
            (1, true),
            (2, false),
            (2, true),
        ] {
            let id = WallId(0x1234_5678_0000_0007);
            let from = StrokeId(3);
            let to = StrokeId(12);
            let edit = match kind {
                0 => ColorEdit::Wall {
                    wall_id: id,
                    from,
                    to,
                },
                1 => ColorEdit::Gate {
                    wall_id: id,
                    from,
                    to,
                },
                _ => ColorEdit::Halftimber {
                    wall_id: id,
                    from,
                    to,
                },
            };
            let mut buffer = ReplayBuffer::new(1.0);
            buffer.push(ReplayBucket {
                entry_index: 0,
                undo,
                commands: vec![ReplayCommand::Color(ColorReplayCmd::expanded(
                    Arc::new(edit),
                    undo,
                    true,
                ))],
                timing: vec![Default::default()],
            });
            let mut world = World::new();
            crate::register_color_events(&mut world);
            world.insert_resource(buffer);
            world.insert_resource(GameTime::default());
            let mut schedule = Schedule::default();
            schedule.add_systems(replay_color::<()>);
            schedule.run(&mut world);
            let (actual, color, custom) = match kind {
                0 => {
                    let events = world.resource::<Events<SetWallColorIdCmd>>();
                    let mut cursor = events.get_cursor();
                    let cmd = cursor.read(events).next().unwrap();
                    (cmd.wall_id(), cmd.color, cmd.custom)
                }
                1 => {
                    let events = world.resource::<Events<SetGateColorIdCmd>>();
                    let mut cursor = events.get_cursor();
                    let cmd = cursor.read(events).next().unwrap();
                    (cmd.wall_id(), cmd.color, cmd.custom)
                }
                _ => {
                    let events = world.resource::<Events<SetHalftimberColorIdCmd>>();
                    let mut cursor = events.get_cursor();
                    let cmd = cursor.read(events).next().unwrap();
                    (cmd.wall_id(), cmd.color, cmd.custom)
                }
            };
            assert_eq!(actual, id);
            assert_eq!(color, if undo { 3 } else { 12 });
            assert_eq!(custom, 1);
        }
    }
}
