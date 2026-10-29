//! B 版工具栏、MEI、摄影控件和菜单实现。
//!
//! 实际游戏注册由 tiny-glade 的 ToolbarPlugin 和 MEI 宿主负责；本 crate
//! 的 `configure` 是可独立使用的部分配置入口。布局、按钮、自动隐藏、音效、
//! 滑块、菜单与摄影系统的当前实现和原证见各模块及业务恢复台账。
//! CPU 测试覆盖部分交互和原模板链，不能代替完整原行为差分或生产窗口验收。
//! Credits、许可证和反馈按项目范围排除。

pub mod demo_stuff;
pub mod mei;
pub mod menu;
pub mod photomode;
pub mod resouces_and_components;
pub mod toolbar_button;
pub mod toolbar_systems;
pub mod toolbar_update;
pub mod toolbar_startup;
pub mod undo_redo_systems;
pub mod ui_presentation;
pub mod taffy_update;
pub mod ui_interaction;

use bevy_ecs::prelude::*;

pub use mei::core::{GlobalId, MeiContext, Node, NodeContext, NodeContent};
pub use mei::layout::{ChildLayoutInput, ComputedLayout, LayoutStyle, ParentLayout};
pub use mei::systems::{mei_post_update, mei_pre_update};
pub use resouces_and_components::{
    FocusedWidget, FocusedWidgetSubtree, HistoryButton, LogSlider, PerWidgetButtonState, UiNav,
};

/// 为独立 schedule 初始化工具栏资源、事件和局部系统。实际游戏的完整注册
/// 由 tiny-glade 宿主完成，不以此入口代替生产调度。
pub fn configure(world: &mut World, schedule: &mut bevy_ecs::schedule::Schedule) {
    world.init_resource::<MeiContext>();
    world.init_resource::<resouces_and_components::ToolbarFocusTracking>();
    world.init_resource::<resouces_and_components::ToolbarSliderSession>();

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
        // O 0x14233c300 / 0x14233c8b0: select/locked scale animations; their
        // real consumers are TaffyNode+0x10 and the command queue.
        ui_interaction::ui_select_animation,
        ui_interaction::ui_locked_animation,
    ));
}

/// Bevy `App` registration surface for `ToolbarPlugin::build` (O 0x14068d610).
///
/// This installs the real resources/events/components consumed by the toolbar
/// systems. Systems themselves are added by the host plugin
/// (`tiny_glade::toolbar_plugin`) so their `GameSystemSet` ordering stays in
/// the tiny-glade crate (original set numbers 8..43 are U).
pub fn configure_app(app: &mut bevy_app::App) {
    app.init_resource::<MeiContext>();
    app.init_resource::<resouces_and_components::ToolbarFocusTracking>();
    app.init_resource::<resouces_and_components::ToolbarSliderSession>();
    app.init_resource::<toolbar_update::ToolbarSizeMult>();
    // Core-owned in run_game, but idempotent and required by render_taffy's
    // Res<UiNavState> surface when this plugin is the only toolbar entry.
    app.init_resource::<country_core::systems::ui::resources::widget_params::UiNavState>();
    app.init_resource::<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>();

    // Event owners consumed by the button/animation systems.
    app.add_event::<ui_interaction::UiClickEvent>();
    app.add_event::<ui_interaction::UiDragEvent>();
    app.add_event::<ui_interaction::UiJustBecameInFocusEvent>();
    app.add_event::<country_core::resources::toolbar::ToolbarUiIsInFocus>();
    app.add_event::<toolbar_button::UiToolbarButtonResetAllCooldowns>();
    app.add_event::<country_core::resources::events::ModeJustChanged>();
    app.add_event::<country_core::systems::ui::resources::widget_params::PlayUiSound>();
    app.add_event::<country_core::systems::mode_manager::RequestToolbarCategoryChange>();
    app.add_event::<country_core::systems::mode_manager::RequestAppModeChange>();
    app.add_event::<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>();

    // All 24 local toolbar components (native register_required_components).
    resouces_and_components::register_local_components(app.world_mut());

    // Core-owned yet hard prerequisites of render_taffy's SystemParam. Install
    // them only when absent so a later core plugin keeps its own constructor.
    if !app
        .world()
        .contains_resource::<country_core::systems::ui::resources::ui_state::UiState>()
    {
        app.world_mut().insert_resource(
            country_core::systems::ui::resources::ui_state::UiState::at_startup(),
        );
    }
    if !app
        .world()
        .contains_resource::<country_core::resources::mouse_terrain_raycast::CursorPositionSS>()
    {
        app.world_mut().insert_resource(
            country_core::resources::mouse_terrain_raycast::CursorPositionSS(glam::Vec2::ZERO),
        );
    }
}

#[cfg(test)]
mod event_lifecycle_tests {
    use super::*;

    #[test]
    fn app_event_registration_expires_old_clicks_and_ui_sounds() {
        use country_core::systems::ui::resources::widget_params::PlayUiSound;
        let mut app = bevy_app::App::new();
        configure_app(&mut app);
        // Repeated host configuration must retain one event maintenance owner.
        configure_app(&mut app);
        let entity = app.world_mut().spawn_empty().id();
        app.world_mut().send_event(ui_interaction::UiClickEvent { entity });
        app.world_mut().send_event(PlayUiSound::variant4());
        assert_eq!(app.world().resource::<Events<ui_interaction::UiClickEvent>>().len(), 1);
        for _ in 0..3 { app.update(); }
        assert!(app.world().resource::<Events<ui_interaction::UiClickEvent>>().is_empty());
        assert!(app.world().resource::<Events<PlayUiSound>>().is_empty());
    }
}
