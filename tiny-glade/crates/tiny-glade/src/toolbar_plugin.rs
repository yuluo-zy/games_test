//! Recovered toolbar registration. Original constructors assign interaction to
//! Update/set7, gameplay toolbar logic to set8, demo policy to set36, and text /
//! Taffy finalization to set44. Exact native members and tuple boundaries are in
//! phase2-toolbar-layout/native-schedule-config-names.json.
//!
//! Text/Taffy set44 and renderer-scale set7 are already installed by mei_host.
//! This entry installs recovered members once. Picking consumes the previous
//! frame's widget registry before MEI and toolbar producers register this frame.
use bevy_app::{App, Plugin, Update};
use bevy_ecs::schedule::IntoScheduleConfigs;
use crate::run_game::GameSystemSet;

pub struct ToolbarPlugin;
impl Plugin for ToolbarPlugin {
    fn build(&self, app: &mut App) { build(app); }
}

/// 原14068dd10..dd96的摄影宿主组；生产入口和调度验收复用同一注册体。
pub fn install_photomode_host_systems(app: &mut App) {
    use country_core::systems::backend::photomode as photo;
    app.add_systems(Update, (
        photo::update_photomode_visibility,
        photo::process_photomode_transitions,
        toolbar::photomode::timelapse::restore_after_timelapse,
        photo::finish_photomode_frame,
    ).chain_ignore_deferred().in_set(GameSystemSet(8)).run_if(
        bevy_state::condition::in_state(country_core::app_loading_state::AppLoadingState::Game)));
}

/// 原set10的模式请求消费者，生产宿主与工具栏App验收共用。
pub fn install_mode_consumers(app: &mut App) {
    use country_core::{resources::history::{EditType, MaybePendingSnapshotId},
        systems::mode_manager};
    app.add_systems(Update, (
        mode_manager::synchronize_mode_manager_ui_context,
        mode_manager::mode_manager::<EditType, MaybePendingSnapshotId>,
        country_core::systems::timelapse_replay::start_timelapse,
        mode_manager::send_mode_just_changed_event.in_set(mode_manager::ModeChangePublication),
    ).chain_ignore_deferred().in_set(GameSystemSet(10)));
}

pub fn build(app: &mut App) {
    use toolbar::{ui_interaction as ui, ui_presentation as presentation,
        toolbar_button as button, toolbar_systems as systems,
        toolbar_update as sizing, photomode as photo, undo_redo_systems as undo};
    toolbar::configure_app(app);
    // 原版 GUI 的 Local 状态由当前会话持有；Reload 时与 MEI 一起释放。
    app.init_resource::<crate::gui::gui_tod::TodButtonState>();
    app.init_resource::<toolbar::demo_stuff::demo_show_full_glade_size::TextAnchors>();
    app.add_systems(bevy_app::Startup, |mut commands: bevy_ecs::prelude::Commands| {
        commands.insert_resource(photo::PhotomodeScreenshotFolder(
            paths::config().join("screenshots")));
    });
    // B country-core/src/systems/ui/signifiers owns these implementations;
    // keep only their host set6 registration in this existing adapter.
    use country_core::systems::ui::signifiers::{
        animation::{signifier_animation, signifier_animation_t},
        signifier_create::signifier_create,
        signifier_mark_for_delete::{despawn_marked, signfier_mark_for_delete},
    };
    app.add_systems(Update, (
        signifier_create, signfier_mark_for_delete, signifier_animation_t,
        signifier_animation, despawn_marked,
    ).chain_ignore_deferred().in_set(GameSystemSet(6))
        .after(country_core::systems::ui::ui_update::ui_update)
        .before(toolbar::mei::systems::mei_pre_update));
    // The same dispatcher consumes toolbar sounds for either real host audio
    // owner. Never consume the visual signifier stream as an audio substitute.
    app.add_systems(bevy_app::Startup,
        country_core::systems::audio::signifier_audio::signifier_audio_init_system::<
            crate::gameplay_startup::GameplayAudioStudio>);
    app.add_systems(Update,
        country_core::systems::audio::signifier_audio::signifier_audio_system::<
            crate::gameplay_startup::GameplayAudioStudio>.in_set(GameSystemSet(38)));
    app.add_event::<ui::UiClickEvent>();
    app.add_event::<ui::UiDragEvent>();
    app.add_event::<ui::UiJustBecameInFocusEvent>();
    app.add_event::<button::UiToolbarButtonResetAllCooldowns>();
    app.add_systems(bevy_state::state::OnEnter(
        country_core::app_loading_state::AppLoadingState::Game),
        toolbar::toolbar_startup::toolbar_setup);
    app.add_systems(bevy_state::state::OnExit(
        country_core::app_loading_state::AppLoadingState::Game),
        clear_toolbar_session);

    // Original set6: hover_intersection -> ui_update -> mei_pre_update.
    // Keep this on the gameplay App: the separate loading App has no camera
    // or world picker. Do not move producers ahead of the registry consumer.
    app.add_systems(Update, (
        country_core::systems::ui::hover_intersection::hover_intersection,
        country_core::systems::ui::ui_update::ui_update,
    ).chain_ignore_deferred().in_set(GameSystemSet(6))
        .before(toolbar::mei::systems::mei_pre_update));

    // Preserve native nested tuples: these members are not internally chained.
    app.add_systems(Update, (
        ui::hide_ui,
        ui::render_taffy_ui_to_widgets,
        ui::ui_focus_parent_if_self_in_focus,
        button::toolbar_button_keep_focus,
        ui::ui_send_focus_begin_events,
        ui::ui_notify_hints_if_toolbar_in_focus,
        (ui::ui_show_if_parent_in_focus, ui::ui_animation_state_update,
            presentation::ui_sounds),
        (presentation::ui_focus_animation, presentation::ui_appear_animation,
            ui::ui_select_animation, ui::ui_locked_animation),
        ui::ui_nav_update,
    ).chain_ignore_deferred().in_set(GameSystemSet(7))
        .after(crate::mei_host::update_text_renderer_scale));

    let game = || bevy_state::condition::in_state(
        country_core::app_loading_state::AppLoadingState::Game);
    app.add_systems(Update, (
        sizing::toolbar_size_mult_update,
        systems::toolbar_steal_mouse_input,
        (button::switch_app_mode_button, button::switch_toolbar_category_button,
            button::tod_v2_button),
    ).chain_ignore_deferred().in_set(GameSystemSet(8)).run_if(game())
        .before(country_core::systems::backend::photomode::update_photomode_visibility));
    // B 原 Demo 提示属于 set36；菜单/摄影门禁仍由原函数自行判定。
    app.add_event::<country_core::resources::glade_border::MaxDemoElevationReached>();
    // B Ui2dPlugin::build 0x14068d89f..0x14068d8f6 checks the
    // Events<ShowDemoBorderOverlay> TypeId and registers it when absent.
    app.add_event::<country_core::resources::glade_border::ShowDemoBorderOverlay>();
    app.add_systems(Update, (
        toolbar::demo_stuff::demo_show_full_glade_size::demo_show_full_glade_size,
        toolbar::demo_stuff::demo_show_max_elevation::demo_show_max_elevation,
    ).in_set(GameSystemSet(36)).run_if(game()));
    // do_photomode's existing host implementation is split into ordered
    // systems. Preserve that implementation and place its recovered button /
    // slider companions before the final previous-state copy.
    app.add_systems(Update, photo::photomode_focus
        .in_set(GameSystemSet(8)).run_if(game())
        .after(country_core::systems::backend::photomode::process_photomode_transitions)
        .after(photo::timelapse::restore_after_timelapse)
        .before(photo::photomode_buttons)
        .before(country_core::systems::backend::photomode::finish_photomode_frame));
    app.add_systems(Update, (photo::photomode_buttons, photo::ui_slider_reset)
        .in_set(GameSystemSet(8)).run_if(game())
        .after(photo::timelapse::restore_after_timelapse)
        .before(country_core::systems::backend::photomode::finish_photomode_frame));
    app.add_systems(Update, (
        (systems::toolbar_auto_hide, undo::undo_redo_auto_hide),
        (systems::toolbar_alignment, undo::undo_redo_alignment),
        ui::sort_contextual_buttons_draw_order,
        sizing::toolbar_button_size_and_padding_update,
        photo::ui_slider,
    ).chain_ignore_deferred().in_set(GameSystemSet(8)).run_if(game())
        .after(country_core::systems::backend::photomode::finish_photomode_frame));
}

/// Host lifecycle repair: the native picker owns captured Arcs across missing
/// frame registrations, but a direct session Reload must release that ownership.
/// This is deliberately separate from the original ui_update algorithm: no
/// synthetic Escape/Shutdown and no manufactured physical input edges.
pub fn clear_toolbar_session(world: &mut bevy_ecs::world::World) {
    use country_core::{
        resources::mouse_capture::{OsCursorVisibilityCmd, OsCursorVisibilityCmdOrigin},
        systems::{brush_preview::OverwriteBrushPreview, ui::resources::{
            ui_state::{DragState, UiInteraction, UiState, WidgetId},
            widget_params::UiNavState,
        }},
    };
    use bevy_ecs::event::Events;
    if let Some(mut ui) = world.get_resource_mut::<UiState>() {
        ui.clear_widgets();
        ui.set_hover_intersections(Vec::new());
        let mut inactive = [0; 56];
        inactive[..4].copy_from_slice(&2u32.to_le_bytes());
        ui.replace_interaction_full(UiInteraction {
            native_tag: 0, native_phase: DragState::NONE.0, widget: WidgetId(0),
        }, None, inactive);
    }
    if let Some(mut navigation) = world.get_resource_mut::<UiNavState>() {
        *navigation = UiNavState::default();
    }
    if let Some(mut tracking) = world.get_resource_mut::<
        toolbar::resouces_and_components::ToolbarFocusTracking>() {
        tracking.clear_session();
    }
    if let Some(mut slider) = world.get_resource_mut::<
        toolbar::resouces_and_components::ToolbarSliderSession>() {
        slider.clear_session();
    }
    if let Some(mut state) = world.get_resource_mut::<crate::gui::gui_tod::TodButtonState>() {
        state.clear_session();
    }
    if let Some(mut anchors) = world.get_resource_mut::<
        toolbar::demo_stuff::demo_show_full_glade_size::TextAnchors>() {
        // 同App重载必须释放原Local的会话位置，不携带上个谷地的插值起点。
        anchors.0 = None;
    }
    if let Some(mut screen) = world.get_resource_mut::<
        country_core::resources::gui::TimeOfDayScreen>() {
        // 清除旧会话的目标、拖动和打开边沿，不能带入下一次 Game。
        *screen = Default::default();
    }
    if let Some(mut message) = world.get_resource_mut::<
        country_core::startup::init_photomode_state::SteamdeckScreenshotMsgScreen>() {
        message.0 = false;
    }
    if let Some(mut screen) = world.get_resource_mut::<
        country_core::resources::gui::PhotomodeScreen>() {
        screen.enabled = false;
        screen.was_enabled = false;
    }
    if let Some(mut stream) = world.get_resource_mut::<
        country_core::systems::ui::signifiers::UiSignifierStream>() {
        *stream = Default::default();
    }
    if let Some(mut mei) = world.get_resource_mut::<toolbar::MeiContext>() {
        let font_table = std::sync::Arc::clone(&mei.font_table);
        let configs = std::mem::take(&mut mei.configs);
        *mei = toolbar::MeiContext { font_table, configs, ..Default::default() };
    }
    macro_rules! clear_events {
        ($($kind:ty),* $(,)?) => { $(
            if let Some(mut events) = world.get_resource_mut::<Events<$kind>>() { events.clear(); }
        )* };
    }
    clear_events!(country_core::resources::glade_border::MaxDemoElevationReached,
        toolbar::ui_interaction::UiClickEvent,
        toolbar::ui_interaction::UiDragEvent,
        toolbar::ui_interaction::UiJustBecameInFocusEvent,
        toolbar::toolbar_button::UiToolbarButtonResetAllCooldowns,
        country_core::resources::toolbar::ToolbarUiIsInFocus,
        // Produced after input translation and consumed on the next frame.
        // A direct Reload must not deliver the old session's undo/redo press.
        country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed);
    if let Some(mut cursor) = world.get_resource_mut::<Events<OsCursorVisibilityCmd>>() {
        cursor.send(OsCursorVisibilityCmd::Unsuppress(OsCursorVisibilityCmdOrigin(1)));
    }
    if let Some(mut brush) = world.get_resource_mut::<Events<OverwriteBrushPreview>>() {
        brush.send(OverwriteBrushPreview::default());
    }
}

#[cfg(test)]
mod registration_tests {
    use super::*;
    use bevy_state::app::AppExtStates;

    #[test]
    fn reload_releases_demo_anchor_and_unconsumed_old_shape() {
        use toolbar::demo_stuff::demo_show_full_glade_size::TextAnchors;
        use country_core::{resources::glade_border::MaxDemoElevationReached,
            systems::wall::valid_enclosure::{Shape,ShapeParameters}};
        let mut world=bevy_ecs::world::World::new();
        world.insert_resource(TextAnchors(Some(glam::Vec3::new(1.,2.,3.))));
        world.init_resource::<bevy_ecs::event::Events<MaxDemoElevationReached>>();
        world.send_event(MaxDemoElevationReached(ShapeParameters::from_shape(Shape::Circle(
            utils::geometry::Circle2d::new(glam::Vec2::ZERO,2.)))));
        clear_toolbar_session(&mut world);
        assert!(world.resource::<TextAnchors>().0.is_none());
        assert!(world.resource::<bevy_ecs::event::Events<MaxDemoElevationReached>>().is_empty());
    }

    #[test]
    fn production_registration_preserves_picker_signifiers_mei_and_toolbar_order() {
        let mut app = App::new();
        app.add_plugins(bevy_state::app::StatesPlugin);
        app.init_state::<country_core::app_loading_state::AppLoadingState>();
        for stage in 0..44 {
            app.configure_sets(Update, GameSystemSet(stage).before(GameSystemSet(stage + 1)));
        }
        crate::mei_host::install_mei_frame_systems(&mut app);
        // 对照 run_game 的摄影 tuple，验证与本插件约束合并后没有循环。
        install_photomode_host_systems(&mut app);
        build(&mut app);
        app.world_mut().schedule_scope(Update, |world, schedule| {
            schedule.initialize(world).expect("real toolbar registration must initialize");
            let names: Vec<_> = schedule.systems().unwrap()
                .map(|(_, system)| system.name().to_string()).collect();
            let position = |suffix: &str| {
                let matches: Vec<_> = names.iter().enumerate()
                    .filter(|(_, name)| name.ends_with(suffix)).map(|(index, _)| index).collect();
                assert_eq!(matches.len(), 1, "exactly one production {suffix}: {names:?}");
                matches[0]
            };
            for sequence in [
                &[
                    "::hover_intersection", "::ui_update", "::signifier_create",
                    "::signfier_mark_for_delete", "::signifier_animation_t",
                    "::signifier_animation", "::despawn_marked", "::mei_pre_update",
                    "::hide_ui", "::render_taffy_ui_to_widgets", "::ui_nav_update",
                ][..],
                &["::toolbar_size_mult_update", "::toolbar_steal_mouse_input",
                    "::switch_toolbar_category_button"][..],
                &["::update_photomode_visibility", "::process_photomode_transitions",
                    "::restore_after_timelapse", "::photomode_focus",
                    "::photomode_buttons", "::finish_photomode_frame",
                    "::toolbar_auto_hide"][..],
            ] {
                for pair in sequence.windows(2) {
                    assert!(position(pair[0]) < position(pair[1]), "{pair:?}: {names:?}");
                }
            }
        });
    }
}
