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
    mut query: Query<(&mut ToolbarButtonCooldown, &mut UiInteractionState)>,
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
/// **未实现（reviewer-wave3 FIX-B 降级；gap-utils-toolbar 2026-10-17 载荷面闭环）**：
/// country-core `RequestAppModeChange(pub String, pub SettingsMenuState)` 已按 B 载荷
/// `(str, SettingsMenuState)`（stride 0x18）落库（mode_manager 组），本函数 EventWriter
/// 载荷面由此闭环。二进制核心（点击匹配 → push `RequestAppModeChange`；逐项按钮目标与
/// 当前 AppMode/ModeJustChanged 判别比较 0..6 变体；相等 → `insert(UiPlaySelectAnimation
/// {a:0.2,b:0.2})`）**均未落库**——载荷源 U：按钮 &str（B SwitchAppModeButton 组件
/// 16B `{ptr,len}`，country-core `{mode: AppMode,_gap}` 无该字段）与 SettingsMenuState
/// 值（B 自 Events+0x40 读）未取证——本函数保持诚实壳，事件写不伪造。
pub fn switch_app_mode_button(
    _query: Query<(Entity, &SwitchAppModeButton, &mut UiInteractionState)>,
    _click_events: EventReader<crate::ui_interaction::UiClickEvent>,
    _app_mode_events: EventWriter<RequestAppModeChange>,
    _app_mode: Res<country_core::systems::mode_manager::AppMode>,
    _mode_changed: EventReader<country_core::resources::events::ModeJustChanged>,
    _commands: Commands,
) {
    // 未实现（分歧见模块头与 RESTORE.md §switch_app_mode_button）。
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
    mut query: Query<(Entity, &SwitchToolbarCategoryButton, &mut UiInteractionState)>,
    mut click_events: EventReader<crate::ui_interaction::UiClickEvent>,
    mut category_events: EventWriter<RequestToolbarCategoryChange>,
    mode_changed: EventReader<country_core::resources::events::ModeJustChanged>,
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
    // 动画插入两门：①ModeJustChanged 事件存在；②interaction.selected != 1 未选中。
    let mode_changed_exists = !mode_changed.is_empty();
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
///    `UiNavState` +0x10=0（布局 U，登记）；
/// ② click 读尽后处理 **ModeJustChanged**：存在未读事件且**首事件 tag `!= 0xd`** 且
///    `visible==1` → `visible=0`；
/// ③ 逐项写 `interaction.selected = visible`（在 ①② 之后）。
///
/// **分歧（未实现/代理）**：`visible` 真实偏移 +0xa vs country-core `{visible}`@+0；
/// `UiNavState+0x10=0` 写面（UiNavState 布局 U，登记未写）；widget 命中经组件
/// `Query<Entity, With<TimeOfDayButtonV2>>` 代理（B 经 UiNavState hashbrown）。
pub fn tod_v2_button(
    mut query: Query<(&TimeOfDayButtonV2, &mut UiInteractionState)>,
    widget_entities: Query<Entity, With<TimeOfDayButtonV2>>,
    mut click_events: EventReader<crate::ui_interaction::UiClickEvent>,
    mut tod_screen: ResMut<country_core::resources::gui::TimeOfDayScreen>,
    _nav_state: ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,
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
            // toggle 后 visible==1 → 写 UiNavState+0x10=0（O；UiNavState 布局 U，登记未写）。
        }
    }
    // ② FIX-T2：click 之后读 ModeJustChanged 首事件 tag != 0xd && visible → 清 visible。
    if tod_screen.visible
        && mode_changed.read().next().is_some_and(|e| e.tool != 0xd)
    {
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
