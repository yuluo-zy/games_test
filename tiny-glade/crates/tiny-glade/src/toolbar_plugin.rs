//! Recovered toolbar registration. Original constructors assign interaction to
//! Update/set7, gameplay toolbar logic to set8, demo policy to set36, and text /
//! Taffy finalization to set44. Exact native members and tuple boundaries are in
//! phase2-toolbar-layout/native-schedule-config-names.json.
//!
//! Text/Taffy set44 and renderer-scale set7 are already installed by mei_host.
//! This entry installs recovered members once. Missing ui_nav_update,
//! toolbar_steal_mouse_input and demo policy remain explicit gaps.
use bevy_app::{App, Plugin, Update};
use bevy_ecs::schedule::IntoScheduleConfigs;
use crate::run_game::GameSystemSet;

pub struct ToolbarPlugin;
impl Plugin for ToolbarPlugin {
    fn build(&self, app: &mut App) { build(app); }
}

pub fn build(app: &mut App) {
    use toolbar::{ui_interaction as ui, ui_presentation as presentation,
        toolbar_button as button, toolbar_systems as systems,
        toolbar_update as sizing, photomode as photo, undo_redo_systems as undo};
    toolbar::configure_app(app);
    app.add_event::<ui::UiClickEvent>();
    app.add_event::<ui::UiDragEvent>();
    app.add_event::<ui::UiJustBecameInFocusEvent>();
    app.add_event::<button::UiToolbarButtonResetAllCooldowns>();
    app.add_systems(bevy_state::state::OnEnter(
        country_core::app_loading_state::AppLoadingState::Game),
        toolbar::toolbar_startup::toolbar_setup);

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
    ).chain_ignore_deferred().in_set(GameSystemSet(7))
        .after(crate::mei_host::update_text_renderer_scale));

    let game = || bevy_state::condition::in_state(
        country_core::app_loading_state::AppLoadingState::Game);
    app.add_systems(Update, (
        sizing::toolbar_size_mult_update,
        (button::switch_app_mode_button, button::switch_toolbar_category_button,
            button::tod_v2_button),
    ).chain_ignore_deferred().in_set(GameSystemSet(8)).run_if(game())
        .before(country_core::systems::backend::photomode::update_photomode_visibility));
    // do_photomode's existing host implementation is split into ordered
    // systems. Preserve that implementation and place its recovered button /
    // slider companions before the final previous-state copy.
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
