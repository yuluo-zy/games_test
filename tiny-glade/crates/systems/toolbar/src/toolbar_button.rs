//! toolbar 按钮系统（B 版恢复，restorer-wave3-toolbar-layout，2026-10-17）。
//!
//! 恢复范围（wave3 fresh 反编译 + caller SystemParam 签名，见
//! `reports/wave3/toolbar/RESTORE.md`）：
//! - `toolbar_button_keep_focus`（0x1423414b0）：冷却计时 + 重置 + 保持焦点。
//! - `switch_app_mode_button`（0x142341fe0）/ `switch_toolbar_category_button`
//!   （0x1423427f0）/ `tod_v2_button`（0x142342f30）：点击→状态切换 + `UiPlaySelectAnimation`。
//!
//! 各函数真实 SystemParam 签名（caller `run_unsafe<FunctionSystem<...>>` mangled 名
//! 逐参核对）与 per-widget 更新公式为 **判等/重建**；分歧（未实现）统一登记：
//! - **per-widget 三字节记录 = `UiInteractionState` 组件本身（reviewer-A-toolbar
//!   FIX-T1；descriptor size=3）**：`+0` state（appear 符号）/`+1` keep/`+2` selected，
//!   四系统直接读/写 `interaction.*`（不再有独立 `WidgetByteState`）；bevy `&mut`
//!   自动记 changed-tick（原生 stride-4 u32 写面即此记账，非业务写面）。
//! - `uVar6/uVar5`（`Query+0x14` this_run tick）即 bevy changed-tick 记账（自动等价）。

use bevy_ecs::entity::Entity;
use bevy_ecs::event::{Event, EventReader, EventWriter};
use bevy_ecs::prelude::{Commands, Query, Res, ResMut, With};
use bevy_time::prelude::Time;

use country_core::resources::window::WindowFocusEvent;
use country_core::systems::mode_manager::{RequestAppModeChange, RequestToolbarCategoryChange};

use crate::resouces_and_components::{
    SwitchAppModeButton, SwitchToolbarCategoryButton, TimeOfDayButtonV2,
    ToolbarButtonCooldown, UiInteractionState, UiPlaySelectAnimation,
};

/// 重置全部工具栏按钮冷却事件（O：`toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns`；
/// toolbar_button_keep_focus 消费）。**重建**（I：单元事件）。
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct UiToolbarButtonResetAllCooldowns;

/// O 0x1423414b0 `toolbar::toolbar_button::toolbar_button_keep_focus`。
///
/// 签名（caller mangled 名）：`Query<(&mut ToolbarButtonCooldown, &mut UiInteractionState)>,
/// EventReader<UiToolbarButtonResetAllCooldowns>, Res<Time>`。
///
/// **判等（算法，gap-utils-toolbar 2026-10-17 换真实 keep 字节）**：①keep 字节（stride-3
/// 记录 `+1`）≠0 → 冷却保持 0.3 默认、`any_keep` 置位；=0 → `冷却 -= dt`（clamp ≥0）；
/// ②重置事件非空 → 全部冷却置 0；③`any_keep` 置位 → keep 字节=0 的项冷却置 0；
/// ④冷却>0 的项 keep 字节置 1。**reviewer-A-toolbar FIX-T1**：keep 字节即
/// `UiInteractionState+1`（原生 3B 组件本身），本系统直接读/写 `interaction.keep`
/// （不再有 `WidgetByteState`）；bevy `&mut` 自动记 changed-tick（原生 stride-4 u32
/// 写面即此记账，非业务写面）。
pub fn toolbar_button_keep_focus(
    mut query: Query<(&mut ToolbarButtonCooldown, ComponentWrite<UiInteractionState>)>,
    mut reset_events: EventReader<UiToolbarButtonResetAllCooldowns>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    let mut any_keep = false;
    // 1. 逐项：keep 字节（`interaction.keep`）≠0 → 冷却保持 0.3 默认、any_keep 置位；
    //    =0 → 冷却 -= dt（clamp 0）。
    for (mut cooldown, interaction) in &mut query {
        if interaction.keep != 0 {
            any_keep = true;
            cooldown.remaining = 0.3;
        } else {
            cooldown.remaining = (cooldown.remaining - dt).max(0.0);
        }
    }
    // 2. 重置事件（O 0x142341952：drain + 全部冷却置 0）。
    if !reset_events.is_empty() {
        for (mut cooldown, _interaction) in &mut query {
            cooldown.remaining = 0.0;
        }
        reset_events.clear();
    }
    // 3. any_keep 置位 → keep 字节=0 的项冷却置 0（O 0x142341f3d 区）。
    if any_keep {
        for (mut cooldown, interaction) in &mut query {
            if interaction.keep == 0 {
                cooldown.remaining = 0.0;
            }
        }
    }
    // 4. 冷却>0 的项 keep 字节置 1（O 0x142341bf2..0x142341dc5）。
    for (cooldown, mut interaction) in &mut query {
        if 0.0 < cooldown.remaining {
            interaction.keep = 1;
        }
    }
}

/// O 0x142341fe0 `toolbar::toolbar_button::switch_app_mode_button`。
///
/// 签名（caller mangled 名）：`Query<(Entity, &SwitchAppModeButton, &mut UiInteractionState)>,
/// EventReader<UiClickEvent>, EventWriter<RequestAppModeChange>, Res<AppMode>,
/// EventReader<ModeJustChanged>, Commands`。
///
/// **判等（fresh 0x142341fe0 + `0x142341fe0.c` 复读）**：
/// ① 点击阶段：命中 `SwitchAppModeButton` → `RequestAppModeChange(button.mode)`
///    （B 载荷 16B AppMode + EventInstance id，stride 0x18）；
/// ② `ModeJustChanged` 有未读事件时逐项比较按钮目标与 `Res<AppMode>`：外层 tag
///    归一化（<6→0，6..=20→tag-5）+ 各变体 payload 比较，等价于 `AppMode` 的
///    派生相等；写 `interaction.selected`；相等项入队
///    `UiPlaySelectAnimation{0.2,0.2,0}`（命令记录 0x28B：两个 fn 指针 + entity
///    + 12B 载荷）。
///
/// 残余 U：B 事件 id 字段（`uVar3`）由 bevy 自动管理，不单独建模。
pub fn switch_app_mode_button(
    mut query: Query<(Entity, &SwitchAppModeButton, ComponentWrite<UiInteractionState>)>,
    mut click_events: EventReader<crate::ui_interaction::UiClickEvent>,
    mut app_mode_events: EventWriter<RequestAppModeChange>,
    app_mode: Res<country_core::systems::mode_manager::AppMode>,
    mut mode_changed: EventReader<country_core::resources::events::ModeJustChanged>,
    mut commands: Commands,
) {
    // Click phase (O 0x1423422xx..0x142342378): every clicked widget copies its
    // 16-byte switch target into an EventInstance<RequestAppModeChange>
    // (stride 0x18 = event id + 16-byte AppMode payload).
    for click in click_events.read() {
        if let Ok((_, button, _)) = query.get(click.entity) {
            app_mode_events.write(RequestAppModeChange(button.mode));
        }
    }
    // ModeJustChanged phase: the native loop body is entered only when the
    // reader has unread events; it drains them (cursor := end).
    let mut any_mode_change = false;
    for _ in mode_changed.read() {
        any_mode_change = true;
    }
    if !any_mode_change {
        return;
    }
    // Per-widget target/current comparison: normalized outer tags plus the
    // per-variant payload compare (0/1/4/3/6 dispatch) is exactly the derived
    // AppMode equality. Writes +2 selected and, on a match, queues
    // UiPlaySelectAnimation{0.2, 0.2, 0}.
    for (entity, button, mut interaction) in &mut query {
        let selected = button.mode == *app_mode;
        interaction.selected = u8::from(selected);
        if selected {
            commands
                .entity(entity)
                .insert(UiPlaySelectAnimation { a: 0.2, b: 0.2, progress: 0.0 });
        }
    }
}

/// O 0x1423427f0 `toolbar::toolbar_button::switch_toolbar_category_button`。
///
/// 签名（caller mangled 名）：`Query<(Entity, &SwitchToolbarCategoryButton,
/// &mut UiInteractionState)>, EventReader<UiClickEvent>, EventWriter<RequestToolbarCategoryChange>,
/// EventReader<ModeJustChanged>, Res<Toolbar>, Commands`。
///
/// **判等（gap-utils-toolbar 补事件写面；reviewer-A-toolbar FIX-T1）**：点击匹配项 →
/// push `RequestToolbarCategoryChange(WindowFocusEvent(payload))`（B `EventInstance<
/// WindowFocusEvent>` stride 0x10，payload = 按钮 category 字节）；逐项动画插入条件 =
/// **`(ModeJustChanged 事件存在) && (interaction.selected != 1 未选中) &&
/// (toolbar.current == button.category)`**；per-widget `+2` 置选中标志（=
/// `interaction.selected`，**B 组件本身**）。
pub fn switch_toolbar_category_button(
    mut query: Query<(Entity, &SwitchToolbarCategoryButton, ComponentWrite<UiInteractionState>)>,
    mut click_events: EventReader<crate::ui_interaction::UiClickEvent>,
    mut category_events: EventWriter<RequestToolbarCategoryChange>,
    mut mode_changed: EventReader<country_core::resources::events::ModeJustChanged>,
    toolbar: Res<country_core::resources::toolbar::Toolbar>,
    mut commands: Commands,
) {
    // 点击匹配：push `RequestToolbarCategoryChange(WindowFocusEvent)`（O 0x142342855..
    // 0x1423428d8：`EventInstance<WindowFocusEvent>` stride 0x10，payload `uVar2` =
    // 按钮 category 字节）。
    for click in click_events.read() {
        if let Ok((_entity, button, _interaction)) = query.get(click.entity) {
            category_events.write(RequestToolbarCategoryChange(WindowFocusEvent(button.category as u8)));
        }
    }
    // 动画插入两门：①ModeJustChanged 事件存在（读取即推进游标，等价原生
    // 游标移到事件末尾）；②interaction.selected != 1 未选中。
    let mut mode_changed_exists = false;
    for _ in mode_changed.read() {
        mode_changed_exists = true;
    }
    for (entity, button, mut interaction) in &mut query {
        let category_match = toolbar.current == button.category;
        if mode_changed_exists && interaction.selected != 1 && category_match {
            commands
                .entity(entity)
                .insert(UiPlaySelectAnimation { a: 0.1, b: 0.2, ..Default::default() });
        }
        // 写 per-widget +2 = category_match（O `*(bool*)(col+2+id*3) = (current==category)`）。
        interaction.selected = if category_match { 1 } else { 0 };
    }
}

/// O 0x142342f30 `toolbar::toolbar_button::tod_v2_button`。
///
/// 签名（caller mangled 名）：`Query<(&TimeOfDayButtonV2, &mut UiInteractionState)>,
/// EventReader<UiClickEvent>, ResMut<TimeOfDayScreen>, ResMut<UiNavState>,
/// EventReader<ModeJustChanged>, Commands`。
///
/// **重建（核心）+ FIX-T2（reviewer-A-toolbar 顺序/事件源修正）**：
/// ① 先处理 **UiClickEvent**：实体命中 TimeOfDayButtonV2 widget → `visible ^= 1` +
///    insert `UiPlaySelectAnimation{a:0.1,b:0.2}`；若 toggle 后 visible==1 → 写
///    `UiNavState` +0x10 的头 8B 清零，保留焦点载荷；
/// ② click 读尽后处理 **ModeJustChanged**：存在未读事件且**末事件 tag `!= 0xd`** 且
///    `visible==1` → `visible=0`；
/// ③ 逐项写 `interaction.selected = visible`（在 ①② 之后）。
///
/// **分歧（未实现/代理）**：`visible` 真实偏移 +0xa vs country-core `{visible}`@+0；
/// widget 命中经组件
/// `Query<Entity, With<TimeOfDayButtonV2>>` 代理（B 经 UiNavState hashbrown）。
pub fn tod_v2_button(
    mut query: Query<(&TimeOfDayButtonV2, ComponentWrite<UiInteractionState>)>,
    widget_entities: Query<Entity, With<TimeOfDayButtonV2>>,
    mut click_events: EventReader<crate::ui_interaction::UiClickEvent>,
    mut tod_screen: ResMut<country_core::resources::gui::TimeOfDayScreen>,
    mut nav_state: ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,
    mut mode_changed: EventReader<country_core::resources::events::ModeJustChanged>,
    mut commands: Commands,
) {
    // ① click 处理：命中 widget → visible ^= 1 + 动画。
    for click in click_events.read() {
        if widget_entities.contains(click.entity) {
            tod_screen.visible ^= true;
            commands
                .entity(click.entity)
                .insert(UiPlaySelectAnimation { a: 0.1, b: 0.2, ..Default::default() });
            // B 0x14234316b writes one qword, not the following payload.
            if tod_screen.visible {
                nav_state.focus.tag = 0;
                nav_state.focus._pad = 0;
            }
        }
    }
    // B 0x14234322b..26c selects the tail of the newer nonempty event buffer,
    // otherwise the older buffer. Consume all unread events and use the last.
    let last_tool = mode_changed.read().last().map(|event| event.current.native_tag());
    if tod_screen.visible && last_tool.is_some_and(|tool| tool != 0xd) {
        tod_screen.visible = false;
    }
    // ③ 逐项写 per-widget +2 = visible（在 ①② 之后；B `*(char*)(col+2+id*3)=visible`）。
    for (_button, mut interaction) in &mut query {
        interaction.selected = if tod_screen.visible { 1 } else { 0 };
    }
}

/// 未实现（诚实）入口：ui_sounds / ui_slider 等交互系统在各自模块登记，
/// 本模块只落按钮族（toolbar_button 模块）。
#[allow(dead_code)]
const _TOOLBAR_BUTTON_SCOPE: () = ();

#[cfg(test)]
mod native_switch_app_mode_tests {
    use super::*;
    use bevy_ecs::event::Events;
    use bevy_ecs::prelude::{Schedule, World};
    use country_core::resources::events::ModeJustChanged;
    use country_core::systems::mode_manager::AppMode;

    #[test]
    fn click_writes_target_mode_and_mode_change_selects_matching_button() {
        let mut world = World::new();
        world.init_resource::<Events<crate::ui_interaction::UiClickEvent>>();
        world.init_resource::<Events<RequestAppModeChange>>();
        world.init_resource::<Events<ModeJustChanged>>();
        world.insert_resource(AppMode::Path);
        let entity = world
            .spawn((
                SwitchAppModeButton { mode: AppMode::Path },
                UiInteractionState::default(),
            ))
            .id();
        let mut schedule = Schedule::default();
        schedule.add_systems(switch_app_mode_button);
        world
            .resource_mut::<Events<crate::ui_interaction::UiClickEvent>>()
            .send(crate::ui_interaction::UiClickEvent { entity });
        schedule.run(&mut world);
        let events = world.resource::<Events<RequestAppModeChange>>();
        let mut cursor = events.get_cursor();
        assert_eq!(
            cursor.read(events).map(|e| e.0).collect::<Vec<_>>(),
            vec![AppMode::Path]
        );
        // No ModeJustChanged yet: selected stays untouched.
        assert_eq!(world.get::<UiInteractionState>(entity).unwrap().selected, 0);
        world
            .resource_mut::<Events<ModeJustChanged>>()
            .send(ModeJustChanged {
                current: AppMode::Path,
                previous: AppMode::default(),
            });
        schedule.run(&mut world);
        world.flush();
        assert_eq!(world.get::<UiInteractionState>(entity).unwrap().selected, 1);
        assert!(world.get::<UiPlaySelectAnimation>(entity).is_some());
    }
}

use country_core::render::extract::ComponentWrite;
