//! color_menu.rs — 径向调色菜单交互主系统（B 0x142291230 color_menu /
//! 0x1422979b0 color_button_animations / 0x142297ff0 delete_color_icon_entities；
//! restorer-wave2-color 2026-10-17）。
//!
//! # color_menu（0x142291230，136KB 全量反编译已解）
//!
//! 本轮（RE-ANALYSIS §2.1 + wave2）ReVa 专属写 worker（fresh clone + 600s 定点）
//! 产出全量反编译 `decomp/color_menu_142291230_full.c`（shared DB codelen=0 为
//! overlay 损坏，实现以写 worker clone 为基）。**逐语句判等受 core typed-gap 阻断
//! → 标未实现**；本文件登记完整解出的逻辑与字段布局，供 core 侧 CustomizationIcon
//! 落库后逐语句重建。
//!
//! ## 系统 ABI（Bevy system，~12 参数，RCX/RDX/R8/R9 + stack）
//! - 参数经寄存器 + 栈传入；`in_stack_00000040` = 64 qword 事件写者阵列
//!   （`Events<*>` 0x20B 头 + buffer 0x28/0x30/0x40 光标）。
//! - 事件写者族：6×`Events<(&str, SettingsMenuState)>`（stride 0x18：
//!   {event_id u64, str_ptr u64, enum_tag u32, byte}）、3×`Events<WindowFocusEvent>`
//!   （stride 0x10：{event_id u64, payload u64}）、`Events<(DecoratorId,
//!   Vec<PreferredNodeAlignment>)>`（stride 0x20）、`Events<WriteSettingsFileCmd>`
//!   （stride 8）等。
//!
//! ## CustomizationIcon 0xa0B 图标状态条目（逐图标 state 记录；core typed-gap）
//! | 偏移 | 类型 | 语义 |
//! |---|---|---|
//! | +0x00 | u8 | used/已处理标志（`*local_100 & 1`；`just_pressed` 双动作命中置 1 并保存 +0x60） |
//! | +0x08 | u64 | Option 选中 id（`*(local_100+8)`，`*local_100==0` → None） |
//! | +0x18 | ptr | 图标列表 buffer（`*(local_100+0x18)`，kind@+0x18+idx*0x28） |
//! | +0x20 | u64 | 图标列表 len |
//! | +0x28/+0x2c | u32×2 | 值 A/B（部分菜单态写 0） |
//! | +0x30 | u8 | **menu state byte（主 switch 变量）** |
//! | +0x34 | u32 | payload（如 selected 值） |
//! | +0x38 | u64 | 载荷 str（事件 str_ptr 源） |
//! | +0x40/+0x48 | ptr/len | 菜单图标 buffer + len（`panic_bounds_check` 边界） |
//! | +0x50 | Vec2 | 菜单中心（`*(local_100+0x50)`） |
//! | +0x60 | u64 | 保存态（used 置位时保存） |
//! | +0x68 | u32 | kind/当前值（switch 的 `uVar33` 源） |
//! | +0x6c | u32 | 值 B |
//! | +0x86/0x87/0x89 | u8 | 子状态标志（+0x89=亮度 toggler 门） |
//! | +0x90 | f32 | 半径 |
//!
//! ## 主循环结构
//! 1. 遍历图标实体（`local_4f8` 实体表 → 0xa0 记录 = CustomizationIcon）；`local_189`
//!    （`local_2e0+0x108`）分流两套实体/索引表（stride 0xa0 与 0x48）。
//! 2. **双 `DigitalActionState::just_pressed()`**（0x142291a30/0x142291a4a）：命中且
//!    图标未 used → 置 used、保存 +0x60 态。
//! 3. **选中推进**（`local_4d0[2]==1` 门）：`icons` 列表（+0x18/+0x20）内按
//!    方向步进跳过 kind==5 分隔符，`panic_bounds_check` 保护；写
//!    `local_158[0x20]=1`（高亮）+ 值记录 `local_138[+idx*4]=local_110`。
//! 4. **每图标 widget 交互**（重复块 ×N）：`format_inner`（格式串哈希 0x4303b728/
//!    0x4303b758/0x4303b800/0x4303b898/0x4303b938…）+ `WidgetParams::new()` +
//!    `UiState::add<String>()` + `UiWidgetContext::is_active()/clicked()` +
//!    `DigitalActionState::just_pressed()`；格式串为图标 widget id（如
//!    "color_menu_*"）。
//! 5. **菜单态收尾**（0x142294b44 区）：state 6 → `WindowFocusEvent` 写 +
//!    `History::add` 记录 tag 0xa；state 9 → `SettingsMenuState` 事件写 +
//!    `History::add` tag 0xb（`ColorMenuBrightnessValue::to_f32` 载荷）；state 10 →
//!    `History::add` tag 0xc；`History::try_start_input_recording()` 门 → 否则
//!    `WriteSettingsFileCmd`。
//! 6. **菜单布局**（0x142294f60 区）：`utils::cartesian_to_polar` +
//!    `AngleRange::clamp` + `polar_to_cartesian`，`WidgetParams::new` +
//!    `UiSounds::empty` + `UiSounds::sounds` + `UiState::add<String>`（菜单中心 +
//!    半径 +0x50/+0x90）。
//! 7. **主 switch**（0x1422960f5，`pcVar30=&DAT_14303ba18 + *(int*)…`）——menu
//!    state byte 分派，见下表。
//!
//! ## 主 switch case 表（menu state → 行为）
//! | case | 事件写（Events） | 图标值记录写（local_180[+off]） | History::add 记录 tag |
//! |---|---|---|---|
//! | 0 | `Events<(&str,SettingsMenuState)>` + `PublicWalls::get` 面 | +0x18/+0x1c/+0x42（halftimbered 门） | 3 |
//! | 1 | 同上 | +0x20 | 5 |
//! | 2 | = code_r0x000142295d8c（状态保持） | — | — |
//! | 4 | 同上 + WindowFocusEvent | +0x30 | 8 |
//! | 5 | 同上 | +0x38 | 4 |
//! | 7/8 | 同上 | +0x24（7）/ +0x2c（8） | 9（8 字节记录 `{1,0,..,1,..,0x9}`） |
//! | 0xc | 同上 | +0xc | 0xd |
//! | 0xd | 嵌套 switch（icon 子类型 3/6/9/0xb） | +0x0 / +0x34 / +0x28 / +0x8 | 7 |
//! | 0xe/0xf/0x11/0x12/0x14 | `WriteSettingsFileCmd` 面 | — | — |
//! | 0x10/0x13 | History 直接 add | +0x2c | （pcVar30 载荷） |
//! | 0x15 | — | +0x28（+字节自增） | — |
//! | default | WindowFocusEvent（code_r0x000142295d9b） | — | — |
//!
//! 图标值记录 `local_180`（`*in_R8`，独立于 CustomizationIcon）为 per-kind 值槽：
//! `[0x0, 0x8, 0xc, 0x18, 0x1c, 0x20, 0x24, 0x28, 0x2c, 0x30, 0x34, 0x38, 0x40, 0x42]`。
//! `History::add` 记录布局 `{1, 1, <tag u8 @+3>, <u32/u8 载荷>, …}`，第二参
//! `local_2d8={9,0}` = `EditType::Color` tag 9。
//!
//! **未实现根因**：per-icon 状态机（CustomizationIcon 0xa0 布局）、图标值记录、
//! `Events<(&str, SettingsMenuState)>`、History Color 字节记录、~12 参数系统 ABI
//! 均 core typed-gap → 无法逐语句判等编译。待 core 落库后按本文件布局重建。
//!
//! # color_button_animations / delete_color_icon_entities
//! 同前：`CustomizationIcon+0x18 + icon_index*0x24` 动画进度、`Ui2dExtent` 缩放
//! 数学已解出（删除态 `+= dt/-0.15`，否则 `+= dt*4.0`，clamp 1.0；二次缓动
//! `(1-e)*0.8+e` 混合 + `rot(clamp[0,1])` 缓动 `*0.2+1.0`），core typed-gap 阻断。

use bevy_ecs::prelude::*;

use country_core::resources::color_icon::{CustomizationIcon, CustomizationIconMarkedForDeletion};

/// 菜单交互状态（O：`Local<Option<(usize,usize)>>`——上次选中图标/菜单对）。
#[derive(Default)]
pub struct ColorMenuSelection(pub Option<(usize, usize)>);

/// B1422979b0: animate the actual 0x24-byte component, not the unrelated
/// 0xa0-byte radial-menu owner. Both marked/unmarked branches are preserved.
pub fn color_button_animations(
    mut icons: Query<(
        &mut CustomizationIcon,
        &mut country_core::components::Ui2dExtent,
        &mut country_core::components::UiDrawOrder,
        Option<&CustomizationIconMarkedForDeletion>,
    )>,
    time: Res<country_core::resources::time::GameTime>,
) {
    for (mut icon, mut extent, mut order, marked) in icons.iter_mut() {
        animate_icon(
            &mut icon,
            &mut extent,
            &mut order,
            marked.is_some(),
            time.delta,
        );
    }
}

fn native_min(a: f32, b: f32) -> f32 {
    if a < b { a } else { b }
}
fn native_max(a: f32, b: f32) -> f32 {
    if a > b { a } else { b }
}

fn animate_icon(
    icon: &mut CustomizationIcon,
    extent: &mut country_core::components::Ui2dExtent,
    order: &mut country_core::components::UiDrawOrder,
    deleting: bool,
    delta: f32,
) {
    use utils::easing_functions::EasingFunction;
    icon.alpha_18 += if deleting { delta / -0.15 } else { delta * 4.0 };
    order.0 = u64::from(!deleting);
    icon.alpha_18 = native_min(icon.alpha_18, 1.0);
    let progress = native_min(1.0, native_max(0.0, icon.alpha_18));
    let eased = EasingFunction::BackOut.apply(progress);
    extent.size = icon.size * eased;
    if deleting {
        return;
    }
    // Native evaluates BackOut again before the position interpolation.
    let eased = EasingFunction::BackOut.apply(progress);
    let mix = (1.0 - eased) * 0.8 + eased;
    extent.pos = icon.position * mix + icon.initial_position * (1.0 - mix);
    // This is a per-frame increment in the original, not multiplied by dt.
    let step = if icon.hovered != 0 { 0.15 } else { -0.15 };
    icon.hover_progress = native_min(1.0, native_max(0.0, icon.hover_progress + step));
    let eased = EasingFunction::CubicOut.apply(icon.hover_progress);
    let scale = (1.0 - eased) * 0.0 + eased * 0.2 + 1.0;
    extent.size *= scale;
}

/// `color_menu` @0x142291230（136KB 全量反编译已解——本文件模块头登记完整结构/
/// 字段布局/switch 表；**未实现**：per-icon 状态机依赖 core `CustomizationIcon`
/// 0xa0 布局 + `Events<(&str,SettingsMenuState)>` + History Color 字节记录 +
/// ~12 参数系统 ABI，均为 typed-gap，无法逐语句判等编译）。
///
/// 签名保持 PDB 候选面（icons/menus/Local/sounds/history）；未构造近似控制流——
/// 反编译证明真实逻辑以 per-icon 字节状态机为主，typed 面仅能表达外环。
#[allow(clippy::too_many_arguments)]
pub fn color_menu<S: Send + Sync + 'static>(
    mut icons: Query<(
        &mut CustomizationIcon,
        Option<&country_core::resources::color_icon::ColorIcon>,
        Option<&country_core::resources::color_icon::WallStyleIcon>,
        &country_core::components::Ui2dExtent,
        &mut country_core::components::Ui2dImage,
    )>,
    mut menus: Query<(Entity, &mut crate::radial_menu::ColorRadialMenu)>,
    mut selection: Local<ColorMenuSelection>,
    mut sounds: EventWriter<country_core::systems::ui::resources::widget_params::PlayUiSound>,
    _history: ResMut<country_core::resources::history::History<S>>,
) {
    // 未实现：反编译主循环 = 逐图标 0xa0 状态机（menu state switch 0..0x15、
    // 双 just_pressed 门、选中推进、widget 交互、History/事件写）。
    // 保持读取面（避免未使用告警），不构造近似控制流。
    let _ = (
        icons.iter_mut().count(),
        menus.iter_mut().count(),
        selection.0,
        &mut sounds,
    );
    selection.0 = None;
}

/// B142297ff0: delete marked entities only when the animation is finished.
pub fn delete_color_icon_entities(
    icons: Query<(
        Entity,
        &CustomizationIcon,
        &CustomizationIconMarkedForDeletion,
    )>,
    mut commands: Commands,
) {
    for (entity, icon, _) in icons.iter() {
        // UCOMISS 0,alpha; JB keeps positive or unordered values.
        if icon.alpha_18 <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

#[cfg(test)]
mod animation_tests {
    use super::*;
    #[test]
    fn retained_icon_animates_then_deletes_without_overwriting_exit_position() {
        use country_core::components::{Ui2dExtent, UiDrawOrder};
        let mut icon = CustomizationIcon {
            size: glam::Vec2::splat(30.),
            position: glam::Vec2::new(100., 200.),
            initial_position: glam::Vec2::ZERO,
            hovered: 1,
            ..Default::default()
        };
        let mut extent = Ui2dExtent::default();
        let mut order = UiDrawOrder::default();
        animate_icon(&mut icon, &mut extent, &mut order, false, 0.25);
        assert_eq!(icon.alpha_18, 1.);
        assert_eq!(order.0, 1);
        assert_eq!(icon.hover_progress, 0.15);
        assert_eq!(extent.pos, icon.position);
        assert!(extent.size.x > 30.);
        let position = extent.pos;
        animate_icon(&mut icon, &mut extent, &mut order, true, 0.15);
        assert_eq!(order.0, 0);
        assert_eq!(icon.alpha_18, 0.);
        assert_eq!(extent.pos, position);
        // Native BackOut keeps sin(pi)'s floating-point endpoint residue.
        assert!(extent.size.abs().max_element() < 0.0001);
        let mut world = World::new();
        let exiting = world.spawn((icon, CustomizationIconMarkedForDeletion)).id();
        let pending = world
            .spawn((
                CustomizationIcon {
                    alpha_18: 0.1,
                    ..Default::default()
                },
                CustomizationIconMarkedForDeletion,
            ))
            .id();
        let mut schedule = Schedule::default();
        schedule.add_systems(delete_color_icon_entities);
        schedule.run(&mut world);
        assert!(world.get_entity(exiting).is_err());
        assert!(world.get_entity(pending).is_ok());
    }
}
