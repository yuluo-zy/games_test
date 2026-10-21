//! Menu/photomode visibility and toolbar focus invalidation.
//! Original B 0x14233eb50, 894 bytes; static evidence in
//! reports/business-module-recovery-2026-10-21/toolbar-input-evidence.
use crate::resouces_and_components::{HideIfMenuIsOpen, ShowOnlyInPhotomode};
use bevy_ecs::prelude::*;
use country_core::{
    components::UiInteractionState,
    resources::{
        gui::{EscMenuState, PhotomodeScreen},
        onboarding::WelcomeScreenState,
    },
    startup::init_photomode_state::{PhotomodeState, SteamdeckScreenshotMsgScreen},
    systems::ui::resources::widget_params::{UiFocus, UiNavState},
};

// Native stores only the discriminant qword; the inactive payload survives.
fn clear_toolbar_focus(focus: &mut UiFocus) {
    focus.tag = 0;
    focus._pad = 0;
}

/// Runs before render_taffy_ui_to_widgets in the original set7 chain. Only
/// HideIfMenuIsOpen entities participate; animation keep/selected bytes are
/// deliberately left to the later interaction/animation systems.
pub fn hide_ui(
    menu: Res<EscMenuState>,
    photo_screen: Res<PhotomodeScreen>,
    photo: Res<PhotomodeState>,
    welcome: Res<WelcomeScreenState>,
    mut navigation: ResMut<UiNavState>,
    screenshot_message: Res<SteamdeckScreenshotMsgScreen>,
    mut items: Query<(
        Entity,
        &mut UiInteractionState,
        &HideIfMenuIsOpen,
        Option<&ShowOnlyInPhotomode>,
    )>,
) {
    // 1409a9140 is folded with UiFocus::is_mei, but here RCX comes from the
    // fourth SystemParam (WelcomeScreenState), not from navigation.focus.
    let menus_closed = menu.is_closed() && welcome.is_closed() && !screenshot_message.0;
    if !menus_closed && navigation.focus.is_toolbar() {
        clear_toolbar_focus(&mut navigation.focus);
    }
    for (entity, mut interaction, _, photo_only) in &mut items {
        let visible = menus_closed
            && if photo_only.is_some() {
                photo_screen.enabled && !photo.hide_ui
            } else {
                !photo_screen.enabled
            };
        interaction.state = u8::from(visible);
        if !visible {
            if navigation.focus.is_toolbar() && navigation.focus.payload == entity.to_bits() {
                clear_toolbar_focus(&mut navigation.focus);
            }
            if navigation.staged_focus.is_toolbar()
                && navigation.staged_focus.payload == entity.to_bits()
            {
                clear_toolbar_focus(&mut navigation.staged_focus);
            }
        }
    }
}

#[cfg(test)]
#[path = "hide_tests.rs"]
mod tests;
