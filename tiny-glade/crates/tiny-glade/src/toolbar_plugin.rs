//! B `ToolbarPlugin` host entry (`impl$1::build` region 0x14068d610).
//!
//! The toolbar crate owns the real resources/events/components and the system
//! functions; this host module supplies the `GameSystemSet` ordering that lives
//! in `tiny-glade` (original set numbers 8..43 are U, see TB contract §1.1).
//!
//! Central integration is one line in `run_game.rs` (not modified here):
//! `app.add_plugins(tiny_glade::toolbar_plugin::ToolbarPlugin);`
//!
//! Ordering: `render_taffy_ui_to_widgets` is the UiState producer consumed by
//! the *next* frame's `mei_pre_update` (set 6). It runs at set 43, after the
//! previous frame's taffy finalize/layout (set 44). The animation/button
//! systems directly follow it; the native `render_taffy -> animation/button`
//! dependency is therefore explicit.

use bevy_app::{App, Plugin, Update};
use bevy_ecs::schedule::IntoScheduleConfigs;

use crate::run_game::GameSystemSet;

/// B `toolbar::impl$1::build` plugin wrapper.
pub struct ToolbarPlugin;

impl Plugin for ToolbarPlugin {
    fn build(&self, app: &mut App) {
        build(app);
    }
}

/// Install the toolbar resources/components and the recovered interaction
/// system group.
pub fn build(app: &mut App) {
    toolbar::configure_app(app);

    // The producer and its consumers share a set and an explicit order:
    // render_taffy publishes UiState/widget bytes used next frame; the
    // animation and button systems consume the events it writes this frame.
    app.add_systems(
        Update,
        (
            toolbar::ui_interaction::render_taffy_ui_to_widgets,
            toolbar::ui_interaction::ui_animation_state_update,
            toolbar::ui_interaction::ui_select_animation,
            toolbar::ui_interaction::ui_locked_animation,
            toolbar::toolbar_button::toolbar_button_keep_focus,
            toolbar::toolbar_button::switch_toolbar_category_button,
            toolbar::toolbar_button::switch_app_mode_button,
            toolbar::toolbar_button::tod_v2_button,
        )
            .chain()
            .in_set(GameSystemSet(43)),
    );
}
