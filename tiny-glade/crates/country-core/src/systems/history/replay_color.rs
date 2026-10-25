//! B `systems/history/replay_color.rs` — 颜色回放命令消费与写回。
//!
//! fresh 证据覆盖**命令生成**：`generate_replay_cmds_from_edit` tag 9
//! （0x14143fcc0）产出命令 tag 6，布局
//! `{tag@0, flags bit0=timed @+8, duration=0.2 @+0xc, Arc<ColorEdit> @+0x10,
//! elapsed @+0x18, undo @+0x1c}`。
//!
//! 节奏（O，`system_color::replay_color` 0x141c7f270）：
//! - **只在 `elapsed == 0` 派发一次**（离散 from/to，无 easing/插值）；
//! - instant：派发后立即完成，不推进 elapsed；
//! - timed：`required = max(duration/speed, 0.1)`；`required < elapsed` →
//!   完成；随后**无条件** `elapsed += GameTime.delta`（**不乘 speed**）；
//! - 完成索引 `len > 1` 时先反转再逐个移除。
//!
//! 写回面：`ColorHistoryReplayApply` 由本系统生成、[`apply_color_replay`] 消费：
//! 墙/门/半木/屋顶 CPU 色表 + `DecoratorStorage` 的 `DecoratorInfo` 载荷（色值
//! +0x28 与窗/旗/烟囱/灯/楼梯附加簇）。`system_color` 的 `set_*` 读者不在
//! core 调度内；core 直接写表是本轮的生产路径，plaster/sound/SSBO 副作用
//! 登记为 follow-up（partial，不冒充完成）。
use crate::resources::{
    color_id::{GateColorIds, HalftimberColorIds, MAX_NUMBER_OF_WALLS, RoofColorIds, WallColorIds},
    history::{ReplayBuffer, ReplayCommand, color_edit::ColorEdit},
    roofs::commands::SetFlatRoofStyleCmd,
    roofs::customization::{FlatRoofStyleObserved, FlatRoofStyleRequest},
    time::GameTime,
    walls::{
        decorator::{DecoratorExtraInfo, DecoratorId},
        decorator_storage::DecoratorStorage,
    },
};
use bevy_ecs::prelude::*;
use std::sync::Arc;

/// 颜色回放应用请求：只在原版派发帧（`elapsed == 0`）生成一次。
#[derive(Event, Clone, Debug)]
pub struct ColorHistoryReplayApply {
    pub edit: Arc<crate::resources::history::color_edit::ColorEdit>,
    pub undo: bool,
}

/// timed 完成判据的分母下限（0x3dcccccd = 0.1f）。
const MIN_REQUIRED: f32 = 0.1;

pub fn replay_color(
    time: Res<GameTime>,
    mut replay: ResMut<ReplayBuffer>,
    mut apply: EventWriter<ColorHistoryReplayApply>,
) {
    let speed = replay.speed;
    let delta = time.delta;
    let Some(bucket) = replay.front_mut_bucket() else {
        return;
    };
    let mut removed: Vec<usize> = Vec::new();
    for (index, command) in bucket.commands.iter_mut().enumerate() {
        let ReplayCommand::Color(command) = command else {
            continue;
        };
        // O：派发门 `elapsed == 0` 且非 NaN（`elapsed == 0.0` 已排除 NaN）。
        if command.elapsed == 0.0 {
            apply.write(ColorHistoryReplayApply {
                edit: command.edit.clone(),
                undo: command.undo,
            });
        }
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
            // O：本路径无条件 `elapsed += GameTime.delta`（speed 只进入分母）。
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

/// 离散选择：`undo` 取 `from`，否则取 `to`（所有 target 一致）。
#[inline]
fn pick<T: Copy>(undo: bool, from: T, to: T) -> T {
    if undo { from } else { to }
}

/// 生产消费者：把一次派发写回真实颜色表/装饰存储。
///
/// owner 资源缺失时该目标的写入跳过（宿主可能未装颜色插件）；其它目标照常
/// 应用，不伪造表值。
pub fn apply_color_replay(
    mut events: EventReader<ColorHistoryReplayApply>,
    mut wall_colors: Option<ResMut<WallColorIds>>,
    mut gate_colors: Option<ResMut<GateColorIds>>,
    mut half_colors: Option<ResMut<HalftimberColorIds>>,
    mut roof_colors: Option<ResMut<RoofColorIds>>,
    mut decorators: Option<ResMut<DecoratorStorage>>,
    mut roof_styles: EventWriter<SetFlatRoofStyleCmd>,
) {
    for event in events.read() {
        match &*event.edit {
            ColorEdit::Wall { wall_id, from, to } => {
                write_color(
                    wall_colors.as_mut().map(|table| &mut table.0),
                    wall_id.0,
                    pick(event.undo, from.0, to.0),
                );
            }
            ColorEdit::Gate { wall_id, from, to } => {
                write_color(
                    gate_colors.as_mut().map(|table| &mut table.0),
                    wall_id.0,
                    pick(event.undo, from.0, to.0),
                );
            }
            ColorEdit::Roof { wall_id, from, to } => {
                write_color(
                    roof_colors.as_mut().map(|table| &mut table.0),
                    wall_id.0,
                    pick(event.undo, from.0, to.0),
                );
            }
            ColorEdit::Halftimber { wall_id, from, to } => {
                write_color(
                    half_colors.as_mut().map(|table| &mut table.0),
                    wall_id.0,
                    pick(event.undo, from.0, to.0),
                );
            }
            // FlatRoof 载荷是 12B 样式记录：交给屋顶 customization owner 的
            // `SetFlatRoofStyleCmd`（与 system_color 的 replay 目标一致；
            // consumer 在 system_roof，不在 core 调度内）。
            ColorEdit::FlatRoof { wall_id, from, to } => {
                let style = pick(event.undo, from, to);
                roof_styles.write(SetFlatRoofStyleCmd(FlatRoofStyleRequest {
                    wall_id: crate::resources::walls::publication_fields::WallId(wall_id.0),
                    style: FlatRoofStyleObserved::from_observed(*style.observed()),
                    observed_play_sound: 1,
                }));
            }
            ColorEdit::Decorator {
                decorator_id,
                from,
                to,
            } => {
                if let Some(storage) = decorators.as_mut() {
                    if let Some(info) = storage.lookup_decorator_info_mut(*decorator_id) {
                        // 与 set_decorator_color_id 同写面：`DecoratorInfo+0x28`。
                        info.u28 = pick(event.undo, from.0, to.0);
                    }
                }
            }
            ColorEdit::WindowLight {
                decorator_id,
                from,
                to,
            } => {
                let value = pick(event.undo, from.0, to.0);
                if let Some(storage) = decorators.as_mut() {
                    if let Some(info) = storage.lookup_decorator_info_mut(*decorator_id) {
                        match info.extra() {
                            DecoratorExtraInfo::Window(mut window) => {
                                window.0 = value;
                                info.set_extra(DecoratorExtraInfo::Window(window));
                            }
                            _ => log::warn!(
                                target: "country_core::systems::history::replay_color",
                                "window-light replay target is not a Window extra; skipped"
                            ),
                        }
                    }
                }
            }
            ColorEdit::Brightness {
                decorator_id,
                from,
                to,
            } => {
                let value = pick(event.undo, *from, *to);
                if let Some(storage) = decorators.as_mut() {
                    if let Some(info) = storage.lookup_decorator_info_mut(*decorator_id) {
                        let mut extra = info.extra();
                        if let DecoratorExtraInfo::Lantern(lantern) = &mut extra {
                            lantern.0 = value;
                            info.set_extra(extra);
                        }
                    }
                }
            }
            ColorEdit::FlagBunting {
                decorator_id,
                from,
                to,
            } => {
                let value = u8::from(pick(event.undo, *from, *to));
                if let Some(storage) = decorators.as_mut() {
                    if let Some(info) = storage.lookup_decorator_info_mut(*decorator_id) {
                        match info.extra() {
                            DecoratorExtraInfo::Flag(mut flag) => {
                                flag.0 = value;
                                info.set_extra(DecoratorExtraInfo::Flag(flag));
                            }
                            _ => log::warn!(
                                target: "country_core::systems::history::replay_color",
                                "flag replay target is not a Flag extra; skipped"
                            ),
                        }
                    }
                }
            }
            ColorEdit::ChimneySmoke {
                decorator_id,
                from,
                to,
            } => {
                let value = u8::from(pick(event.undo, *from, *to));
                if let Some(storage) = decorators.as_mut() {
                    if let Some(info) = storage.lookup_decorator_info_mut(*decorator_id) {
                        match info.extra() {
                            DecoratorExtraInfo::Chimney(mut chimney) => {
                                chimney.smoke = value;
                                info.set_extra(DecoratorExtraInfo::Chimney(chimney));
                            }
                            _ => log::warn!(
                                target: "country_core::systems::history::replay_color",
                                "chimney-smoke replay target is not a Chimney extra; skipped"
                            ),
                        }
                    }
                }
            }
            ColorEdit::StairLadder {
                decorator_id,
                from,
                to,
            } => {
                let value = pick(event.undo, from.0, to.0);
                write_stair_extra(&mut decorators, *decorator_id, 8, &value.to_le_bytes());
            }
            ColorEdit::MagicColor {
                decorator_id,
                from,
                to,
            } => {
                let value = pick(event.undo, from.0, to.0);
                write_stair_extra(&mut decorators, *decorator_id, 12, &value.to_le_bytes());
            }
            ColorEdit::StairRailing {
                decorator_id,
                from,
                to,
            } => {
                let value = pick(event.undo, *from, *to);
                write_stair_extra(&mut decorators, *decorator_id, 0x10, &[value]);
            }
            // fresh switch 无 case 0xd（StairSupport=16）→ 不派发。
            ColorEdit::StairSupport { .. } => {}
        }
    }
}

fn write_color(table: Option<&mut Vec<u32>>, id: u64, color: u32) {
    // 索引 = 目标 id（id→索引在本 crate 为恒等映射；上限 0x5000）。
    let Some(table) = table else {
        return;
    };
    let index = id as usize;
    if index >= MAX_NUMBER_OF_WALLS {
        log::warn!(
            target: "country_core::systems::history::replay_color",
            "color replay target index {index} exceeds MAX_NUMBER_OF_WALLS"
        );
        return;
    }
    if let Some(slot) = table.get_mut(index) {
        *slot = color;
    }
}

fn write_stair_extra(
    decorators: &mut Option<ResMut<'_, DecoratorStorage>>,
    id: DecoratorId,
    offset: usize,
    bytes: &[u8],
) {
    let Some(storage) = decorators.as_mut() else {
        return;
    };
    let Some(info) = storage.lookup_decorator_info_mut(id) else {
        return;
    };
    let mut extra = info.extra();
    let DecoratorExtraInfo::Stair(stair) = &mut extra else {
        return;
    };
    stair.0[offset..offset + bytes.len()].copy_from_slice(bytes);
    info.set_extra(extra);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::history::replay_cmd_buffer::ColorReplayCmd;
    use crate::resources::terrain_editing::StrokeId;
    use crate::resources::walls::WallId;
    use crate::resources::walls::decorator::{DecoratorInfo, FlagExtra, LanternExtra, WindowExtra};
    use crate::resources::walls::decorator_storage::DecoratorStorageAddr;
    use crate::systems::history::register_history_events;

    const TERRAIN: DecoratorStorageAddr = DecoratorStorageAddr::Terrain;

    fn fixture() -> (World, Schedule) {
        let mut world = World::new();
        world.insert_resource(ReplayBuffer::new(1.0));
        world.insert_resource(GameTime {
            delta: 0.1,
            ..Default::default()
        });
        world.init_resource::<Events<ColorHistoryReplayApply>>();
        world.init_resource::<Events<SetFlatRoofStyleCmd>>();
        world.init_resource::<WallColorIds>();
        world.init_resource::<DecoratorStorage>();
        world.init_resource::<Events<crate::systems::history::ReplayBucketFinished>>();
        let mut schedule = Schedule::default();
        schedule.add_systems(
            (
                replay_color,
                apply_color_replay,
                crate::systems::history::pop_empty_replay_bucket,
            )
                .chain(),
        );
        (world, schedule)
    }

    fn place(world: &mut World, id: u32, extra: DecoratorExtraInfo) {
        let mut info = DecoratorInfo {
            decorator_id: id,
            ..DecoratorInfo::default()
        };
        // Extra 与 destination 是独立联合；用真实 owner 写入载荷和 niche。
        info.set_extra(extra);
        world
            .resource_mut::<DecoratorStorage>()
            .apply_decorator_place(TERRAIN, DecoratorId(id), &info);
    }

    fn push(world: &mut World, edit: ColorEdit, undo: bool, instant: bool) {
        world
            .resource_mut::<ReplayBuffer>()
            .push(crate::resources::history::ReplayBucket {
                entry_index: 0,
                undo,
                commands: vec![ReplayCommand::Color(ColorReplayCmd::expanded(
                    Arc::new(edit),
                    undo,
                    instant,
                ))],
                timing: vec![Default::default()],
            });
    }

    fn lantern(world: &World, id: u32) -> f32 {
        world
            .resource::<DecoratorStorage>()
            .lookup_decorator_info(DecoratorId(id))
            .unwrap()
            .extra()
            .as_lantern()
            .0
    }

    #[test]
    fn redo_writes_wall_color_table_and_undo_restores_from_value() {
        let (mut world, mut schedule) = fixture();
        let edit = ColorEdit::Wall {
            wall_id: WallId(7),
            from: StrokeId(3),
            to: StrokeId(12),
        };
        push(&mut world, edit.clone(), false, true);
        schedule.run(&mut world);
        assert_eq!(world.resource::<WallColorIds>().0[7], 12);
        push(&mut world, edit, true, true);
        schedule.run(&mut world);
        assert_eq!(world.resource::<WallColorIds>().0[7], 3);
    }

    #[test]
    fn timed_command_dispatches_exactly_once_and_uses_delta_not_speed() {
        let (mut world, mut schedule) = fixture();
        push(
            &mut world,
            ColorEdit::Brightness {
                decorator_id: DecoratorId(4),
                from: 0.25,
                to: 1.0,
            },
            false,
            false,
        );
        // 第一帧 elapsed==0 → 派发并写终值（无插值）；elapsed += 0.1（不乘 speed）。
        schedule.run(&mut world);
        assert_eq!(world.resource::<Events<ColorHistoryReplayApply>>().len(), 1);
        let command = match &world
            .resource::<ReplayBuffer>()
            .buckets
            .front()
            .unwrap()
            .commands[0]
        {
            ReplayCommand::Color(command) => command.clone(),
            _ => panic!("expected color command"),
        };
        assert!((command.elapsed - 0.1).abs() < 1e-6);
        // 后续帧不再派发。
        world
            .resource_mut::<Events<ColorHistoryReplayApply>>()
            .clear();
        schedule.run(&mut world);
        assert!(
            world
                .resource::<Events<ColorHistoryReplayApply>>()
                .is_empty()
        );
        // required = max(0.2/1.0, 0.1) = 0.2；elapsed 0.2 未过（严格 <）。
        assert_eq!(world.resource::<ReplayBuffer>().len(), 1);
        // elapsed 0.3 仍未过（检查在自增之前）。
        schedule.run(&mut world);
        assert_eq!(world.resource::<ReplayBuffer>().len(), 1);
        // 下一帧 elapsed 0.4 > 0.2 → 移除（同帧 Pop 弹空桶）。
        schedule.run(&mut world);
        assert!(world.resource::<ReplayBuffer>().is_empty());
    }

    #[test]
    fn brightness_is_one_shot_without_interpolation() {
        let (mut world, mut schedule) = fixture();
        place(
            &mut world,
            4,
            DecoratorExtraInfo::Lantern(LanternExtra(0.25)),
        );
        push(
            &mut world,
            ColorEdit::Brightness {
                decorator_id: DecoratorId(4),
                from: 0.25,
                to: 1.0,
            },
            false,
            false,
        );
        schedule.run(&mut world);
        assert_eq!(lantern(&world, 4), 1.0);
    }

    #[test]
    fn floor_uses_point_one_and_speed_only_in_denominator() {
        let (mut world, mut schedule) = fixture();
        world.resource_mut::<ReplayBuffer>().speed = 4.0;
        let mut command = ColorReplayCmd::expanded(
            Arc::new(ColorEdit::Wall {
                wall_id: WallId(1),
                from: StrokeId(1),
                to: StrokeId(2),
            }),
            false,
            false,
        );
        // duration/speed = 0.2/4 = 0.05 ≤ 0.1 → floor 0.1。
        command.duration = 0.2;
        command.elapsed = 0.11;
        world
            .resource_mut::<ReplayBuffer>()
            .push(crate::resources::history::ReplayBucket {
                entry_index: 0,
                undo: false,
                commands: vec![ReplayCommand::Color(command)],
                timing: vec![Default::default()],
            });
        schedule.run(&mut world);
        // 0.1 < 0.11 → 完成并移除（未派发，因为 elapsed != 0）。
        assert!(world.resource::<ReplayBuffer>().is_empty());
        assert!(
            world
                .resource::<Events<ColorHistoryReplayApply>>()
                .is_empty()
        );
    }

    #[test]
    fn flat_roof_style_dispatches_to_roof_owner_event() {
        let (mut world, mut schedule) = fixture();
        let from = crate::resources::roofs::roof_shape::FlatRoofStyle::from_observed([1; 12]);
        let to = crate::resources::roofs::roof_shape::FlatRoofStyle::from_observed([2; 12]);
        push(
            &mut world,
            ColorEdit::FlatRoof {
                wall_id: WallId(8),
                from,
                to,
            },
            false,
            true,
        );
        schedule.run(&mut world);
        let events: Vec<_> = world
            .resource_mut::<Events<SetFlatRoofStyleCmd>>()
            .drain()
            .collect();
        assert_eq!(events.len(), 1);
        assert_eq!(
            events[0].0.wall_id,
            crate::resources::walls::publication_fields::WallId(8)
        );
        assert_eq!(events[0].0.style.bytes(), &[2; 12]);
        assert_eq!(events[0].0.observed_play_sound, 1);
    }

    #[test]
    fn flag_and_window_targets_write_decorator_extra() {
        let (mut world, mut schedule) = fixture();
        place(&mut world, 5, DecoratorExtraInfo::Flag(FlagExtra(0)));
        place(&mut world, 6, DecoratorExtraInfo::Window(WindowExtra(0)));
        push(
            &mut world,
            ColorEdit::FlagBunting {
                decorator_id: DecoratorId(5),
                from: false,
                to: true,
            },
            false,
            true,
        );
        push(
            &mut world,
            ColorEdit::WindowLight {
                decorator_id: DecoratorId(6),
                from: StrokeId(1),
                to: StrokeId(9),
            },
            false,
            true,
        );
        schedule.run(&mut world);
        schedule.run(&mut world);
        let storage = world.resource::<DecoratorStorage>();
        assert_eq!(
            storage
                .lookup_decorator_info(DecoratorId(5))
                .unwrap()
                .extra()
                .as_flag()
                .0,
            1
        );
        assert_eq!(
            storage
                .lookup_decorator_info(DecoratorId(6))
                .unwrap()
                .extra()
                .as_window()
                .0,
            9
        );
    }

    #[test]
    fn apply_event_is_registered_with_history_bundle() {
        let mut world = World::new();
        register_history_events(&mut world);
        assert!(world.contains_resource::<Events<ColorHistoryReplayApply>>());
    }
}
