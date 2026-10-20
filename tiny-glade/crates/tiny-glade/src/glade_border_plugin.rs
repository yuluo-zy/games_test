//! B `system_glade_border::GladeBorderPlugin::build`（0x140650ab0）。
//!
//! 机器码顺序：`chain_ignore_deferred((display_glade_border, feedback_input_out_of_border))` →
//! `in_set(GameSystemSet(0x24))` → `add_systems(Update)`；随后两个事件按 TypeId 存在性检查注册
//! （`DisplayOutOfBorder`、`FeedbackInputOutOfBorder`）。`ShowDemoBorderOverlay`/`PlayUiSound`
//! 由其它插件注册，这里不重复。
use bevy_app::{App, Update};
use bevy_ecs::prelude::*;
use country_core::resources::glade_border::{DisplayOutOfBorder, FeedbackInputOutOfBorder};
use system_glade_border::{
    display_glade_border::display_glade_border,
    feedback_input_out_of_border::feedback_input_out_of_border,
};

use crate::run_game::GameSystemSet;

pub fn build(app: &mut App) {
    app.add_systems(
        Update,
        (display_glade_border, feedback_input_out_of_border)
            .chain_ignore_deferred()
            .in_set(GameSystemSet(0x24)),
    );
    app.add_event::<DisplayOutOfBorder>();
    app.add_event::<FeedbackInputOutOfBorder>();
}
