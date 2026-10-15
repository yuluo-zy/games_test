//! Independently recovered photomode consumers. Camera, audio, focus readback
//! and widget dependencies of the large original do_photomode remain separate.
use bevy_ecs::{
    prelude::{Res, ResMut, World},
    schedule::Schedule,
};

use crate::{
    input::action_set::MainActionSet,
    resources::gui::{EscMenuState, PhotomodeScreen},
    startup::init_photomode_state::PhotomodeState,
    systems::mode_manager::AppMode,
};

/// B 0x14234b190..0x14234b1f7. Menu reset happens first, then action 32 may
/// toggle again in the same frame. Replay preserves flags only with a closed
/// menu. This prefix precedes AppModeChanged processing.
pub fn update_photomode_visibility(
    mut photo: ResMut<PhotomodeState>,
    menu: Res<EscMenuState>,
    mode: Res<AppMode>,
    actions: Res<MainActionSet>,
) {
    if menu.is_open() {
        photo.hide_ui = false;
        photo.show_focus_marker = true;
    }
    match *mode {
        AppMode::Replay => {}
        AppMode::Photomode => {
            if actions.digital[32].just_pressed() {
                let was_hidden = photo.hide_ui;
                photo.hide_ui = !was_hidden;
                photo.show_focus_marker = was_hidden;
            }
        }
        _ => {
            photo.hide_ui = false;
            photo.show_focus_marker = true;
        }
    }
}

/// Register after mode input and before photomode transitions/UI. The full
/// original schedule is not implied. Existing resources retain their state.
pub fn configure_photomode_visibility(world: &mut World, schedule: &mut Schedule, demo: bool) {
    if !world.contains_resource::<PhotomodeState>() {
        world.insert_resource(PhotomodeState::new(demo));
    }
    world.init_resource::<PhotomodeScreen>();
    schedule.add_systems(update_photomode_visibility);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::action_set::DigitalActionState;

    #[test]
    fn menu_reset_precedes_input_and_replay_preserves_closed_menu_state() {
        let mut world = World::new();
        let mut schedule = Schedule::default();
        world.insert_resource(EscMenuState::Main);
        world.insert_resource(AppMode::Photomode);
        let mut actions = MainActionSet::default();
        actions.digital[32] = DigitalActionState { disc: 0, val: 0.0 };
        world.insert_resource(actions);
        configure_photomode_visibility(&mut world, &mut schedule, true);
        assert_eq!(world.resource::<PhotomodeState>().scalar, 1.0);
        schedule.run(&mut world);
        assert!(world.resource::<PhotomodeState>().hide_ui);
        assert!(!world.resource::<PhotomodeState>().show_focus_marker);

        *world.resource_mut::<AppMode>() = AppMode::Replay;
        *world.resource_mut::<EscMenuState>() = EscMenuState::Closed;
        schedule.run(&mut world);
        assert!(world.resource::<PhotomodeState>().hide_ui);
        assert!(!world.resource::<PhotomodeState>().show_focus_marker);

        *world.resource_mut::<EscMenuState>() = EscMenuState::Main;
        schedule.run(&mut world);
        assert!(!world.resource::<PhotomodeState>().hide_ui);
        assert!(world.resource::<PhotomodeState>().show_focus_marker);
    }

    #[test]
    fn outside_photomode_resets_visibility_and_held_input_does_not_toggle() {
        let mut world = World::new();
        let mut schedule = Schedule::default();
        world.insert_resource(EscMenuState::Closed);
        world.insert_resource(AppMode::Photomode);
        let mut actions = MainActionSet::default();
        actions.digital[32] = DigitalActionState { disc: 2, val: 0.5 };
        world.insert_resource(actions);
        configure_photomode_visibility(&mut world, &mut schedule, false);
        world.resource_mut::<PhotomodeState>().hide_ui = true;
        world.resource_mut::<PhotomodeState>().show_focus_marker = false;
        schedule.run(&mut world);
        assert!(world.resource::<PhotomodeState>().hide_ui);
        *world.resource_mut::<AppMode>() = AppMode::Path;
        schedule.run(&mut world);
        assert!(!world.resource::<PhotomodeState>().hide_ui);
        assert!(world.resource::<PhotomodeState>().show_focus_marker);
    }
}
