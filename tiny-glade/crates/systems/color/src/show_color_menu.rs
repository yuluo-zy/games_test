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
//! - `hide_color_menu`（0x1422adcf0，2408B）：**未实现**。

use bevy_ecs::prelude::*;

use country_core::resources::color_icon::{CustomizationIcon, CustomizationIconMarkedForDeletion};

use crate::radial_menu::{new_flat_roof_rim_ty, new_separator, ColorRadialMenu, IconInfo};
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
    options.push(icon_color("color_icon/plaster_peel_medium", 0xc, 0x3f000000));
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

/// `hide_color_menu` @0x1422adcf0（2408B，未实现——事件/动作面+图标子类型
/// switch 的写者映射依赖反编译与 core 目录，登记）。
#[allow(clippy::too_many_arguments)]
pub fn hide_color_menu(
    mut cmds: EventReader<HideColorMenuCmd>,
    mut _sounds: EventWriter<country_core::systems::ui::resources::widget_params::PlayUiSound>,
    menus: Query<(Entity, &ColorRadialMenu)>,
    icons: Query<(Entity, &CustomizationIcon)>,
    _actions: Res<country_core::input::action_set::ActionSetStates>,
    mut commands: Commands,
) {
    // FIX-2（reviewer-state-color）：原按 HideColorMenuCmd 存在（或
    // `DigitalActionState::just_pressed()`）**命中才动**；无命令/动作则不动。
    // just_pressed 的具体输入动作名 U（GAP）——本实现以事件存在为门控。
    if cmds.read().next().is_some() {
        for (entity, _menu) in menus.iter() {
            commands.entity(entity).despawn();
        }
        for (entity, _icon) in icons.iter() {
            commands
                .entity(entity)
                .insert(CustomizationIconMarkedForDeletion)
                .despawn();
        }
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
    _customization: Res<country_core::resources::glade_color_customization::GladeColorCustomization>,
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
        let is_plaster_wall = wall_colors.0.iter().any(|id| (4..=10).contains(id) || *id == 0xc);
        // O：halftimber 门 `*(puStack_250+600)<2`（GAP：puStack_250 源为 core
        // HalftimberColorIds/墙目录面，以 0 近似登记——近似不入行为判等）。
        let halftimber_count = 0u32;
        // O：构建该 kind 的图标选项列表。
        let options = options_for_kind(cmd.kind, is_plaster_wall, halftimber_count, has_flat_roof);
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
