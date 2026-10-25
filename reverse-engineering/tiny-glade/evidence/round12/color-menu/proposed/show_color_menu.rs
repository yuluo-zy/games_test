//! show_color_menu.rs — 颜色菜单显隐（B 0x1422adcf0 hide_color_menu /
//! 0x14229a7f0 show_color_options；restorer-wave2-color 2026-10-17，reviewer
//! wave2 复核后修正 case 0/1 图标载荷）。
//!
//! - `show_color_options`（0x14229a7f0，真实函数体 ~0x38KB）：**重建（结构面 +
//!   case 0/1 图标载荷面）+ 未实现（其余 case/事件写面）**。反编译基 = reviewer
//!   `revieww2color` 专属写 worker fresh clone 全量（`wave2/reviews/color/evidence/
//!   decomp/show_color_options_14229a7f0_full.c`，与 shared read_memory 证据 sha256
//!   逐字节一致）。结构面判等（reviewer 复核通过）：
//!   * 遍历 `ShowColorMenuCmd` 事件（EventInstance stride 0x20；kind 字节
//!     `(byte)*puVar19`、payload `*(*pauVar43 + 8)`）；
//!   * 两段跳表分派（0x14303ca70 相对偏移，base=表地址；kind 0 → 0x14229b725、
//!     1 → 0x14229bde9（roof 色）…40 项已核对）；
//!   * demo 门 `_DEV_OVERRIDE_IS_DEMO_MODE`（0x143634d90，read_memory=0 当前非 demo）；
//!   * `MainCamera::eye_position()` → `ColorRadialMenu::create` + 菜单实体生成 +
//!     事件写者族。
//! - **case 0（0x14229b725）图标载荷（修正后，按 fresh 反编译行 452-1093）**：
//!   非 demo 主路径：`PublicWalls::get()` 空则 return → **主 13 个 kind-3 色
//!   swatch**（bricks_beige/yellow/light/grey/dark ×5 + plaster_colors ×8，
//!   v1c=0,1,3,0xb,2,4,0xc,5,10,8,9,6,7；行 598-677；B-ecstate fresh 反汇编补：
//!   8 个 plaster 条目副值 `+0x24 = [0,7,1,6,4,5,2,3]`，bricks 5 项为 0）→ plaster 条件门
//!   `(iVar9-4U<7)||(iVar9==0xc)`（行 689）追加 4 图标（separator +
//!   plaster_peel_low/medium/high，kind 0xc，v1c=0/0x3f000000/0x3f800000；
//!   行 747-788）→ `*(puStack_250+600)<2` 门（行 793）追加 halftimber/skirt 族
//!   （separator + halftimber(kind 4,b19 2) + skirt(kind 4,b19 1) +
//!   no_skirt(kind 4,b19 0)；行 842-880）→ `Roof::is_flat()` 门（行 884-940）
//!   追加 separator + `new_flat_roof_rim_ty`；非平顶 else 分支（行 941-1088）
//!   5 图标块（separator + skirt/no_skirt + separator + kind-7 30 字符图标，
//!   `ClutterVersion::is_initial` 门）——登记 GAP。
//!   demo 分支（行 1094+）——登记 GAP（demo 门现为 0，非活动路径）。
//! - **case 1（0x14229bde9）roof 色**：11 个 kind-3 roof swatch
//!   （roof_ochre/red/purple/blue/light_blue/turquoise/green_light/green/
//!   dark_wood/dark/grey，v1c=8,0,10,2,9,3,6,4,1,5,7；行 1600-1665）。
//! - **未实现面**：case 2..0x26 逐 case 精确表（装饰器分类
//!   `DecoratorStorage::lookup_decorator_info`+`DecoratorExtraInfo::try_as_*`、
//!   `StairsState::get_all_connected_nodes` 等）、尾部 6+ 个
//!   `Events<(&str, SettingsMenuState)>` / `Events<(DecoratorId,
//!   Vec<PreferredNodeAlignment>)>` 事件写面、`MainCamera::eye_position` SIMD 偏移、
//!   平顶 else 块、demo 分支——登记 GAP。
//! - `hide_color_menu`（0x1422adcf0，5058B）：完整取消/关闭/退场生命周期。

use bevy_ecs::prelude::*;

use country_core::resources::color_icon::{CustomizationIcon, CustomizationIconMarkedForDeletion};

use crate::radial_menu::{ColorRadialMenu, IconInfo, new_flat_roof_rim_ty, new_separator};
use crate::{HideColorMenuCmd, ShowColorMenuCmd};

/// `IconInfo` 色 swatch 图标构造（O：反编译写面 String@0 + kind@+0x18 +
/// v1c@+0x1c；b19@+0x19 / u1a@+0x1a / v20@+0x20 / v24@+0x24 默认 0）。
fn icon_color(name: &str, kind: u8, v1c: u32) -> IconInfo {
    IconInfo {
        name: name.to_string(),
        kind,
        v1c,
        ..IconInfo::default()
    }
}

/// `IconInfo` 带 b19 载荷的构造（O：halftimber/skirt 族 kind 4 + b19 选择位）。
fn icon_b19(name: &str, kind: u8, b19: u8, v1c: u32) -> IconInfo {
    IconInfo {
        name: name.to_string(),
        kind,
        b19,
        v1c,
        ..IconInfo::default()
    }
}

/// case 0 主 13 个 kind-3 色 swatch（O：行 598-677，全 kind 3；5 bricks +
/// 8 plaster_colors）。
fn push_case0_main_swatches(options: &mut Vec<IconInfo>) {
    // O（B-ecstate fresh 反汇编 0x14229d03e/07d/…/204）：
    // 前 6 项 `+0x24 = 0`；后 7 项 plaster_colors 显式写
    // `+0x24 = 7,1,6,4,5,2,3`（对应 v1c = 0xc,5,10,8,9,6,7）。
    // 即 8 个 plaster 条目的 v24 序列 = [0,7,1,6,4,5,2,3]（首个 v1c=4 为 0）。
    for (name, v1c, v24) in [
        ("color_icon/bricks_beige", 0u32, 0u32),
        ("color_icon/bricks_yellow", 1, 0),
        ("color_icon/bricks_light", 3, 0),
        ("color_icon/bricks_grey", 0xb, 0),
        ("color_icon/bricks_dark", 2, 0),
        ("color_icon/plaster_colors", 4, 0),
        ("color_icon/plaster_colors", 0xc, 7),
        ("color_icon/plaster_colors", 5, 1),
        ("color_icon/plaster_colors", 10, 6),
        ("color_icon/plaster_colors", 8, 4),
        ("color_icon/plaster_colors", 9, 5),
        ("color_icon/plaster_colors", 6, 2),
        ("color_icon/plaster_colors", 7, 3),
    ] {
        options.push(IconInfo {
            name: name.to_string(),
            kind: 3,
            v1c,
            v24,
            ..IconInfo::default()
        });
    }
}

/// case 0 plaster 条件块（O：行 689 `(iVar9-4U<7)||(iVar9==0xc)` 门 → 行 747-788
/// 4 图标：separator + plaster_peel_low/medium/high，kind 0xc，
/// v1c=0 / 0x3f000000(0.5f) / 0x3f800000(1.0f)）。
fn push_case0_plaster_peel(options: &mut Vec<IconInfo>) {
    options.push(new_separator());
    options.push(icon_color("color_icon/plaster_peel_low", 0xc, 0));
    options.push(icon_color(
        "color_icon/plaster_peel_medium",
        0xc,
        0x3f000000,
    ));
    options.push(icon_color("color_icon/plaster_peel_high", 0xc, 0x3f800000));
}

/// case 0 halftimber/skirt 条件块（O：行 793 `*(puStack_250+600)<2` 门 → 行 842-880
/// 4 图标：separator + halftimber(kind 4,b19 2) + skirt(kind 4,b19 1) +
/// no_skirt(kind 4,b19 0)）。
fn push_case0_halftimber_skirt(options: &mut Vec<IconInfo>) {
    options.push(new_separator());
    options.push(icon_b19("color_icon/halftimber", 4, 2, 0));
    options.push(icon_b19("color_icon/skirt", 4, 1, 0));
    options.push(icon_b19("color_icon/no_skirt", 4, 0, 0));
}

/// case 0（0x14229b725）plaster/wall 颜色选项（O：行 452-1093 非 demo 主路径）。
///
/// 门参数：`is_plaster_wall` = `(iVar9-4U<7)||(iVar9==0xc)`（行 688-689，iVar9 源
/// `local_180 = *in_RDX`（Res<WallColorIds> 面）+ `get_instruction()` 索引——GAP，
/// 以参数承载）；`halftimber_count` = `*(puStack_250+600)`（行 793，<2 门）；
/// `has_flat_roof` = `Roof::is_flat()`（行 897）。平顶 else 块（行 941-1088）与
/// demo 分支（行 1094+）登记 GAP，不构造。
fn build_plaster_wall_roof_options(
    is_plaster_wall: bool,
    halftimber_count: u32,
    has_flat_roof: bool,
) -> Vec<IconInfo> {
    let mut options: Vec<IconInfo> = Vec::new();
    // O：主 13 个 kind-3 色 swatch（首图标 = bricks_beige，非分隔符）。
    push_case0_main_swatches(&mut options);
    // O：plaster 门 → separator + plaster_peel_low/medium/high。
    if is_plaster_wall {
        push_case0_plaster_peel(&mut options);
    }
    // O：halftimber/skirt 门（count<2）→ separator + halftimber/skirt/no_skirt。
    if halftimber_count < 2 {
        push_case0_halftimber_skirt(&mut options);
        if has_flat_roof {
            // O：`Roof::is_flat()` 门（行 884-940）→ separator + flat_roof_rim_ty。
            options.push(new_separator());
            options.push(new_flat_roof_rim_ty());
        } else {
            // —— GAP：非平顶 else 块（行 941-1088）5 图标（separator + skirt(4,1)
            // + no_skirt(4,0) + separator + kind-7 30 字符，ClutterVersion 门），登记。
        }
    }
    options
}

/// case 1（0x14229bde9）roof 色选项（O：行 1600-1665，11 个 kind-3 roof swatch；
/// 0x14303c2a1/0x14303c36f/0x14303c356 read_memory 确证串）。
fn build_roof_color_options(options: &mut Vec<IconInfo>) {
    for (name, v) in [
        ("color_icon/roof_ochre", 8u32),
        ("color_icon/roof_red", 0),
        ("color_icon/roof_purple", 10),
        ("color_icon/roof_blue", 2),
        ("color_icon/roof_light_blue", 9),
        ("color_icon/roof_turquoise", 3),
        ("color_icon/roof_green_light", 6),
        ("color_icon/roof_green", 4),
        ("color_icon/roof_dark_wood", 1),
        ("color_icon/roof_dark", 5),
        ("color_icon/roof_grey", 7),
    ] {
        options.push(icon_color(name, 3, v));
    }
}

/// 选项 kind → 径向图标列表（O：show_color_options 0x14229a7f0 的 `switch(*puVar19)`
/// case 0..0x26。case 0/1 载荷已重建；case 2..0x26 逐 case 精确表登记 GAP）。
fn options_for_kind(
    kind: u32,
    is_plaster_wall: bool,
    halftimber_count: u32,
    has_flat_roof: bool,
) -> Vec<IconInfo> {
    match kind {
        0 => build_plaster_wall_roof_options(is_plaster_wall, halftimber_count, has_flat_roof),
        // O：case 1（0x14229bde9）= roof 色 11 swatch（fresh radare 0x14229d754 复核）。
        1 => {
            let mut options: Vec<IconInfo> = Vec::new();
            build_roof_color_options(&mut options);
            options
        }
        // —— case 2..0x26：逐 case 精确串/值表未实现（GAP，B-ecstate §8.4-2）。
        // 不构造近似表（此前 `_ =>` 回落 roof 色系错误近似，已移除）；结构已证：
        // 每个 case 为 1..40 个 `IconInfo`（kind 3 颜色 / 0xc 材质 / 5 分隔 / 8 类型），
        // 值来自 `DecoratorStorage`/`DecoratorExtraInfo`/`StairsState` 数据面。
        _ => Vec::new(),
    }
}

/// All native cancellation writers, kept in one SystemParam to preserve the
/// original event ownership while avoiding a flattened function parameter list.
#[derive(bevy_ecs::system::SystemParam)]
pub struct ColorMenuRestoreEvents<'w> {
    pub walls: EventWriter<'w, country_core::resources::color_id::SetWallColorIdCmd>,
    pub roofs: EventWriter<'w, crate::SetRoofColorIdCmd>,
    pub decorators: EventWriter<'w, crate::SetDecoratorColorIdCmd>,
    pub ladders: EventWriter<'w, crate::SetLadderColorIdCmd>,
    pub brightness: EventWriter<'w, crate::SetDecoratorBrightnessCmd>,
    pub gates: EventWriter<'w, country_core::resources::color_id::SetGateColorIdCmd>,
    pub halftimber: EventWriter<'w, country_core::resources::color_id::SetHalftimberColorIdCmd>,
    pub styles: EventWriter<'w, country_core::systems::wall::SetWallStyleCmd>,
    pub flags: EventWriter<'w, crate::SetFlagConnectionCmd>,
    pub window_lights: EventWriter<'w, crate::SetWindowLightCmd>,
    pub rims: EventWriter<'w, country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd>,
    pub chimneys: EventWriter<'w, crate::SetChimneySmokeCmd>,
    pub magic: EventWriter<'w, crate::SetMagicColorIdCmd>,
    pub flat_styles: EventWriter<'w, crate::SetFlatRoofStyleCmd>,
    pub plaster: EventWriter<'w, country_core::systems::wall::SetPlasterPeelAmountCmd>,
}
impl ColorMenuRestoreEvents<'_> {
    /// B1422ae3e4 jump table0x14303cad8, targets0..12. The stored initial
    /// snapshot is restored on cancel; current preview values are not committed.
    pub fn restore(&mut self, menu: &ColorRadialMenu) {
        use country_core::resources::roofs::customization::{
            FlatRoofRimRequest, FlatRoofStyleObserved, FlatRoofStyleRequest,
        };
        use country_core::resources::walls::WallId;
        let target = menu.target;
        let initial = menu.initial;
        let wall = WallId(target.id);
        let id = target.payload;
        match target.kind {
            0 => {
                self.walls
                    .write(country_core::resources::color_id::SetWallColorIdCmd {
                        id: target.id as u32,
                        _gap4: (target.id >> 32) as u32,
                        color: initial.color,
                        custom: 0,
                        _pad: [0; 3],
                    });
                self.styles
                    .write(country_core::systems::wall::SetWallStyleCmd {
                        wall_id: wall,
                        from: initial.wall_style,
                        to: initial.wall_style,
                        record_history: 0,
                        play_sound: 0,
                    });
                self.rims.write(
                    country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd(
                        FlatRoofRimRequest {
                            wall_id: country_core::resources::walls::publication_fields::WallId(
                                target.id,
                            ),
                            from: initial.flat_roof_rim_style,
                            to: initial.flat_roof_rim_style,
                            observed_record_history: 0,
                        },
                    ),
                );
                self.plaster
                    .write(country_core::systems::wall::SetPlasterPeelAmountCmd {
                        wall_id: wall,
                        from: initial.plaster_peel,
                        to: initial.plaster_peel,
                        record_history: 0,
                    });
            }
            1 => {
                self.roofs.write(crate::SetRoofColorIdCmd {
                    id: target.id,
                    color: initial.color,
                    custom: 0,
                });
            }
            2 => {
                self.flat_styles
                    .write(crate::SetFlatRoofStyleCmd(FlatRoofStyleRequest {
                        wall_id: country_core::resources::walls::publication_fields::WallId(
                            target.id,
                        ),
                        style: FlatRoofStyleObserved::from_observed(initial.flat_roof_style),
                        observed_play_sound: 0,
                    }));
            }
            3 => {
                self.decorators.write(crate::SetDecoratorColorIdCmd {
                    id,
                    color: initial.color,
                    custom: 0,
                });
            }
            4 => {
                self.window_lights.write(crate::SetWindowLightCmd {
                    id,
                    value: initial.color,
                    custom: 0,
                });
            }
            5 => {
                self.gates
                    .write(country_core::resources::color_id::SetGateColorIdCmd {
                        id: target.id as u32,
                        _gap4: (target.id >> 32) as u32,
                        color: initial.color,
                        custom: 0,
                        _pad: [0; 3],
                    });
            }
            6 => {
                self.decorators.write(crate::SetDecoratorColorIdCmd {
                    id,
                    color: initial.color,
                    custom: 0,
                });
                self.flags.write(crate::SetFlagConnectionCmd {
                    id,
                    value: initial.flag_connection,
                    custom: 0,
                    _pad: [0; 2],
                });
            }
            9 => {
                self.decorators.write(crate::SetDecoratorColorIdCmd {
                    id,
                    color: initial.color,
                    custom: 0,
                });
                self.brightness.write(crate::SetDecoratorBrightnessCmd {
                    id,
                    value: initial.brightness.to_bits(),
                    custom: 0,
                });
            }
            10 => {
                self.decorators.write(crate::SetDecoratorColorIdCmd {
                    id,
                    color: initial.color,
                    custom: 0,
                });
                self.chimneys.write(crate::SetChimneySmokeCmd {
                    id,
                    value: initial.chimney_smoke,
                    custom: 0,
                    _pad: [0; 2],
                });
            }
            11 | 12 => {
                if target.kind == 11 {
                    self.decorators.write(crate::SetDecoratorColorIdCmd {
                        id,
                        color: initial.color,
                        custom: 0,
                    });
                } else {
                    self.ladders.write(crate::SetLadderColorIdCmd {
                        id,
                        value: initial.color,
                        custom: 0,
                    });
                }
                self.magic.write(crate::SetMagicColorIdCmd {
                    id,
                    value: initial.magic_color,
                    custom: 0,
                });
            }
            _ => {
                self.halftimber
                    .write(country_core::resources::color_id::SetHalftimberColorIdCmd {
                        id: target.id as u32,
                        _gap4: (target.id >> 32) as u32,
                        color: initial.color,
                        custom: 0,
                        _pad: [0; 3],
                    });
            }
        }
    }
}

/// B1422adcf0. The latest command decides cancellation; keyboard ui_cancel18
/// supplies true only when there is no command. Menu despawn is immediate, while
/// icon entities retain their original animated deletion lifetime.
#[allow(clippy::too_many_arguments)]
pub fn hide_color_menu(
    mut cmds: EventReader<HideColorMenuCmd>,
    mut sounds: EventWriter<country_core::systems::ui::resources::widget_params::PlayUiSound>,
    menus: Query<(Entity, &ColorRadialMenu)>,
    icons: Query<(Entity, &CustomizationIcon)>,
    actions: Res<country_core::input::action_set::ActionSetStates>,
    mut commands: Commands,
    mut restoration: ColorMenuRestoreEvents,
) {
    let cancel = match cmds.read().last() {
        Some(cmd) => cmd.cancel,
        None if actions.digital[18].just_pressed() => true,
        None => return,
    };
    for (entity, menu) in &menus {
        if cancel {
            restoration.restore(menu);
            sounds.write(
                country_core::systems::ui::resources::widget_params::PlayUiSound {
                    kind: 9,
                    ..Default::default()
                },
            );
        }
        commands.entity(entity).despawn();
    }
    for (entity, _) in &icons {
        commands
            .entity(entity)
            .insert(CustomizationIconMarkedForDeletion);
    }
}

/// `show_color_options` @0x14229a7f0（真实 ~0x38KB；结构面 + case 0/1 载荷面
/// 已重建，case 2..0x26/事件写面 GAP）。
#[allow(clippy::too_many_arguments)]
pub fn show_color_options<S: Send + Sync + 'static>(
    mut cmds: EventReader<ShowColorMenuCmd>,
    wall_colors: Res<country_core::resources::color_id::WallColorIds>,
    _gate_colors: Res<country_core::resources::color_id::GateColorIds>,
    _half_colors: Res<country_core::resources::color_id::HalftimberColorIds>,
    roofs: Query<&country_core::resources::roofs::roof_shape::Roof>,
    _decorators: Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,
    _walls: Res<country_core::resources::walls::public_walls::PublicWalls>,
    _config: Res<country_core::game_config::GameConfig>,
    camera: Res<country_core::systems::camera_rig::main_camera::MainCamera>,
    mut _nav: ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,
    _cursor: Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,
    _customization: Res<
        country_core::resources::glade_color_customization::GladeColorCustomization,
    >,
    mut commands: Commands,
) {
    // O：demo 门 = `_DEV_OVERRIDE_IS_DEMO_MODE`（0x143634d90 静态字节，非资源；
    // read_memory 现为 0）。GAP：core 未暴露该全局，以 `false` 近似（登记）。
    let demo_mode = false;
    // O：roof is_flat 门（行 884-940；GAP：逐实体精确遍历为高层抽象，以任一平顶为门）。
    let has_flat_roof = roofs.iter().any(|r| r.is_flat());
    for cmd in cmds.read() {
        // O：plaster 门 iVar9 源 = Res<WallColorIds> + get_instruction() 索引
        // （GAP：索引源未定，以遍历任一 ∈{4..10,12} 近似登记）。
        let is_plaster_wall = wall_colors
            .0
            .iter()
            .any(|id| (4..=10).contains(id) || *id == 0xc);
        // O：halftimber 门 `*(puStack_250+600)<2`（GAP：puStack_250 源为 core
        // HalftimberColorIds/墙目录面，以 0 近似登记——近似不入行为判等）。
        let halftimber_count = 0u32;
        // O：构建该 kind 的图标选项列表。
        let options = options_for_kind(
            u32::from(cmd.kind),
            is_plaster_wall,
            halftimber_count,
            has_flat_roof,
        );
        // O：`ColorRadialMenu::create`——center 源自 `MainCamera::eye_position()`
        // + 偏移（GAP：SIMD 逐分量偏移未对齐 core 类型，以 eye_position().truncate()
        // 近似登记），icons = 上述列表。
        let center = camera.eye_position().truncate();
        let radius = 1.0;
        let menu = ColorRadialMenu::create(center, radius, options);
        // O：菜单实体生成（create 后写 Events 面 + 菜单实体登记）。
        commands.spawn((menu, CustomizationIcon::default()));
    }
    let _ = demo_mode;
}

#[cfg(test)]
mod native_hide_tests {
    use super::*;
    use crate::radial_menu::{ColorMenuTarget, ColorSelectionSnapshot};
    use country_core::resources::color_id::*;
    use country_core::systems::ui::resources::widget_params::PlayUiSound;
    fn setup() -> World {
        let mut world = World::new();
        crate::register_color_events(&mut world);
        world.init_resource::<country_core::input::action_set::ActionSetStates>();
        world
    }
    #[test]
    fn cancellation_restores_every_native_target_then_keeps_exit_animation() {
        let mut world = setup();
        let icon = world
            .spawn(CustomizationIcon {
                alpha_18: 1.,
                ..Default::default()
            })
            .id();
        for kind in 0..13 {
            world.spawn(ColorRadialMenu {
                target: ColorMenuTarget {
                    kind,
                    variant: 0,
                    reserved: 0,
                    payload: 67,
                    id: 0x100000009,
                },
                initial: ColorSelectionSnapshot {
                    color: 5,
                    brightness: 4.,
                    plaster_peel: 0.5,
                    magic_color: 8,
                    wall_style: 2,
                    flat_roof_rim_style: 1,
                    flag_connection: 1,
                    chimney_smoke: 1,
                    ..Default::default()
                },
                ..Default::default()
            });
        }
        world
            .resource_mut::<Events<HideColorMenuCmd>>()
            .send(HideColorMenuCmd { cancel: true });
        let mut schedule = Schedule::default();
        schedule.add_systems(hide_color_menu);
        schedule.run(&mut world);
        assert_eq!(world.query::<&ColorRadialMenu>().iter(&world).count(), 0);
        assert!(
            world
                .get::<CustomizationIconMarkedForDeletion>(icon)
                .is_some()
        );
        assert_eq!(world.get::<CustomizationIcon>(icon).unwrap().alpha_18, 1.);
        assert_eq!(world.resource::<Events<PlayUiSound>>().len(), 13);
        assert_eq!(world.resource::<Events<SetWallColorIdCmd>>().len(), 1);
        assert_eq!(
            world.resource::<Events<crate::SetRoofColorIdCmd>>().len(),
            1
        );
        assert_eq!(
            world.resource::<Events<crate::SetFlatRoofStyleCmd>>().len(),
            1
        );
        assert_eq!(
            world
                .resource::<Events<crate::SetDecoratorColorIdCmd>>()
                .len(),
            5
        );
        assert_eq!(
            world.resource::<Events<crate::SetWindowLightCmd>>().len(),
            1
        );
        assert_eq!(world.resource::<Events<SetGateColorIdCmd>>().len(), 1);
        assert_eq!(world.resource::<Events<SetHalftimberColorIdCmd>>().len(), 2);
        assert_eq!(
            world
                .resource::<Events<crate::SetFlagConnectionCmd>>()
                .len(),
            1
        );
        assert_eq!(
            world
                .resource::<Events<crate::SetDecoratorBrightnessCmd>>()
                .len(),
            1
        );
        assert_eq!(
            world.resource::<Events<crate::SetChimneySmokeCmd>>().len(),
            1
        );
        assert_eq!(
            world.resource::<Events<crate::SetMagicColorIdCmd>>().len(),
            2
        );
        assert_eq!(
            world.resource::<Events<crate::SetLadderColorIdCmd>>().len(),
            1
        );
        assert_eq!(
            world
                .resource::<Events<country_core::systems::wall::SetWallStyleCmd>>()
                .len(),
            1
        );
        assert_eq!(
            world
                .resource::<Events<country_core::systems::wall::SetPlasterPeelAmountCmd>>()
                .len(),
            1
        );
        assert_eq!(world.resource::<Events<country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd>>().len(), 1);
        let wall = world
            .resource::<Events<SetWallColorIdCmd>>()
            .iter_current_update_events()
            .next()
            .unwrap();
        assert_eq!(wall.wall_id().0, 0x100000009);
        assert_eq!((wall.color, wall.custom), (5, 0));
    }
    #[test]
    fn latest_false_command_overrides_keyboard_cancel_without_restoring_values() {
        let mut world = setup();
        let menu = world.spawn(ColorRadialMenu::default()).id();
        world
            .resource_mut::<country_core::input::action_set::ActionSetStates>()
            .digital[18]
            .phase = country_core::input::action_set::DigitalActionPhase::JustPressed;
        let mut cmds = world.resource_mut::<Events<HideColorMenuCmd>>();
        cmds.send(HideColorMenuCmd { cancel: true });
        cmds.send(HideColorMenuCmd { cancel: false });
        let mut schedule = Schedule::default();
        schedule.add_systems(hide_color_menu);
        schedule.run(&mut world);
        assert!(world.get_entity(menu).is_err());
        assert!(world.resource::<Events<SetWallColorIdCmd>>().is_empty());
        assert!(world.resource::<Events<PlayUiSound>>().is_empty());
    }
}
