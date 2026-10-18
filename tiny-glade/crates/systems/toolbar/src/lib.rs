//! toolbar（B 版恢复，restorer-interaction-toolbar）。
//!
//! 恢复范围（本批，判等/重建见各文件注释与 RESTORE.md）：
//! - `mei::core`：节点引擎叶子（safe_frame/new_at_pos/Node 方法/NodeContext 系列/
//!   process_input_action_handlers，GlobalId 隐式派生）。
//! - `mei::layout`：盒模型 LayoutStyle/horizontal/ComputedLayout::center/place_childen
//!   分派。
//! - `mei::config`：load_font_table/read_node_config_from_file（NodeConfig RON 合同 U）。
//! - `resouces_and_components`：LogSlider::param_to_value。
//!
//! wave2（restorer-wave2-toolbar，2026-10-17）：`reports/wave2/toolbar/RESTORE.md`
//! 已对 RE-ANALYSIS 的 6 个 toolbar 函数做 fresh 复核，全部标"未实现（重建阻塞）"。
//!
//! wave2b（restorer-wave2b-toolbar-types，2026-10-17）：按反编译布局落库 toolbar 本地
//! 组件类型（TaffyNode/UiScaleWithUserSetting/UiAnimationState/UiMouseInteractable/
//! UiInteractionState/UiShowIfParentIsInFocus/ShowOnlyInFpvAndFree/ToolbarItemKind）并
//! 实现 6 函数。`place_childen_impl` **签名裁决**（以反汇编为准改
//! `ParentLayout`/`ChildLayoutInput`）+ 核心算法重建；`add_toolbar_item` 控制流判等
//! （kind 载荷未实现）；toolbar_alignment 前置重建；toolbar_auto_hide/ui_sounds/ui_slider
//! 未实现（分歧见 `reports/wave2b/toolbar/RESTORE.md`）。
//!
//! wave3（restorer-wave3-toolbar-layout，2026-10-17）：`UiAnimationState`/`UiInteractionState`
//! 布局按 **ui_animation_state_update 0x14233b3f0 fresh 反编译**校正（16B
//! `{appear,select,_gap4,state}` / 4B `{state,_pad,flag,_pad}`）；增补按钮族组件与事件
//! （ToolbarButtonCooldown/UiLocked/SwitchAppModeButton/SwitchToolbarCategoryButton/
//! UiSlider/PhotomodeButton/TimeOfDayButtonV2/UiPlaySelectAnimation + UiClickEvent/
//! UiDragEvent/UiJustBecameInFocusEvent/UiToolbarButtonResetAllCooldowns）；`toolbar_button`
//! 模块落 4 按钮系统（重建，per-widget 容器/事件载荷分歧见 RESTORE.md §类型/§各函数）；
//! `toolbar_alignment` 落 `taffy_mut_style` 骨架 + 分派结构（setter 目标 U）；toolbar_auto_hide/
//! ui_sounds/ui_slider 未实现（分歧见 RESTORE.md）。
//!
//! 未实现（分歧见 RESTORE.md 全符号清单）：
//! - ToolbarPlugin::build 0x14068d610 / Ui2dPlugin::build 0x14068dc70（9 事件注册 +
//!   Startup/Update 系统集合，未落库系统占位）——本 lib 以 `configure` 表达可落部分
//!   （MeiContext 资源）。
//! - mei widgets/tweaker/decorate/audio、菜单 menu/*、拍照 photomode/*、文本 text、
//!   taffy_layout/undo_redo_layout、toolbar_systems 分派 case 体、ui_interaction
//!   截断 case、toolbar_button/toolbar_update/ui_slider、demo_stuff、
//!   third_party_licenses_server（taffy/tiny_http/image 依赖已入 workspace，
//!   类型/接线仍缺，见 wave2b RESTORE.md）。
//!
//! restorer-mei-body（2026-10-17）：`mei_pre_update`/`mei_post_update` 主体与
//! `render_taffy_ui_to_widgets` 按 fresh 反编译重建（判等/重建/未实现逐项见
//! `reports/mei/body/RESTORE.md`）；`closure_env 1/3..9` 无 B 单态、case 4/6
//! 二跳索引语义未闭合，均登记未实现。
//!
//! restorer-mei-final（2026-10-18）：`mei_pre_update` 的焦点门控动作分发/
//! 父子链重建/clicked 焦点二项、`mei_post_update` 的 `update_nav`（方向输入、
//! 默认焦点、父链兄弟搜索、`iter_subtree` 两个闭包）与焦点位写、
//! `render_taffy_ui_to_widgets` 的焦点交换/keep 面/共享 click+drag 事件写面
//! 均按逐指令判等落库；z 比较器经 `insertion_sort_shift_left` 反汇编定案为
//! `f32::total_cmp` 升序。残余未实现/未取证见 `reports/mei/final/RESTORE.md`。

pub mod mei;
pub mod menu;
pub mod photomode;
pub mod resouces_and_components;
pub mod toolbar_button;
pub mod toolbar_systems;
pub mod toolbar_update;
pub mod taffy_update;
pub mod ui_interaction;

use bevy_ecs::prelude::*;

pub use mei::core::{GlobalId, MeiContext, Node, NodeContext, NodeContent};
pub use mei::layout::{ChildLayoutInput, ComputedLayout, LayoutStyle, ParentLayout};
pub use mei::systems::{mei_post_update, mei_pre_update};
pub use resouces_and_components::{
    FocusedWidget, FocusedWidgetSubtree, HistoryButton, LogSlider, PerWidgetButtonState, UiNav,
};

/// ToolbarPlugin::build（O 0x14068d610）的可落部分（glue-legitimate，不重建）：
/// `insert_resource<MeiContext>` + 按钮族事件/组件注册 + 四系统接线（reviewer-A-toolbar
/// FIX-T1）。完整 build（TypeId 查重注册 9 事件 + Startup text_startup_system + Update
/// 系统全集）未实现的其余系统/事件见 RESTORE.md。
pub fn configure(world: &mut World, schedule: &mut bevy_ecs::schedule::Schedule) {
    world.init_resource::<MeiContext>();

    // FIX-T1：按钮族事件注册（B ToolbarPlugin::build 注册面）。
    world.init_resource::<Events<ui_interaction::UiClickEvent>>();
    world.init_resource::<Events<ui_interaction::UiDragEvent>>();
    world.init_resource::<Events<ui_interaction::UiJustBecameInFocusEvent>>();
    world.init_resource::<Events<toolbar_button::UiToolbarButtonResetAllCooldowns>>();
    world.init_resource::<Events<country_core::resources::events::ModeJustChanged>>();
    world.init_resource::<
        Events<country_core::systems::ui::resources::widget_params::PlayUiSound>,
    >();
    world.init_resource::<
        Events<country_core::systems::mode_manager::RequestToolbarCategoryChange>,
    >();

    // FIX-T1：按钮族组件注册（原生 register_component 面；Query 可用实体匹配）。
    let _ = world.register_component::<resouces_and_components::UiInteractionState>();
    let _ = world.register_component::<resouces_and_components::UiAnimationState>();
    let _ = world.register_component::<resouces_and_components::ToolbarButtonCooldown>();
    let _ = world.register_component::<resouces_and_components::SwitchToolbarCategoryButton>();
    let _ = world.register_component::<resouces_and_components::TimeOfDayButtonV2>();

    // FIX-T1：四系统接入 schedule（当前无可运行等价面的核心缺口）。
    schedule.add_systems((
        toolbar_button::toolbar_button_keep_focus,
        toolbar_button::switch_toolbar_category_button,
        toolbar_button::tod_v2_button,
        ui_interaction::ui_animation_state_update,
    ));
}
