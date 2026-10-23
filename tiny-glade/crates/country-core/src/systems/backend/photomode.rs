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

/// Audio request sampled before this frame's camera commands, matching
/// @14234b386..3ec. The actual AudioCore consumer runs in the audio phase.
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, PartialEq)]
pub struct PhotoModeAudioParameter(pub f32);
pub const PHOTOMODE_AUDIO_PARAMETER: &str = "PhotoModeMinusFPV";

/// Camera/lifecycle prefix of toolbar::photomode::do_photomode
/// @14234b1f7..653, split from the original UI construction for crate ownership.
/// The complete focus and photo widgets are separate consumers, not omitted
/// branches hidden behind a substitute do_photomode implementation.
#[cfg(feature = "native-physics")]
#[allow(clippy::too_many_arguments)]
pub fn process_photomode_transitions(
    mut screen: ResMut<PhotomodeScreen>,
    mut photo: ResMut<PhotomodeState>,
    mut camera: ResMut<crate::systems::camera_rig::MainCamera>,
    mut debug: ResMut<crate::resources::gui::DebugGui>,
    mut formation: ResMut<crate::resources::image_formation::ImageFormationConfigs>,
    mut vermeer: ResMut<crate::systems::render_loop::vermeer::VermeerState>,
    mut modes: bevy_ecs::event::EventReader<crate::systems::mode_manager::AppModeChanged>,
    mut commands: bevy_ecs::event::EventReader<crate::resources::photo_mode::PhotoModeCmd>,
    mut camera_sounds: bevy_ecs::event::EventWriter<crate::systems::ui::resources::widget_params::PlayPhotomodeSound>,
    mut audio: bevy_ecs::event::EventWriter<PhotoModeAudioParameter>,
    config: Res<crate::game_config::GameConfig>,
    collision: Res<crate::systems::collision::world::RaycastWorld>,
    heights: Res<crate::resources::terrain_height::TerrainHeightsData>,
) {
    use crate::systems::camera_rig::main_camera::CameraMode;
    if let Some(event) = modes.read().last() {
        if event.current == AppMode::Photomode && event.previous != AppMode::Replay {
            screen.enabled = true;
            photo.on_enable_photomode(config.dev.demo);
            debug.camera_mode = photo.camera_mode.into();
            formation.switch_kind(2);
            if !photo.free_camera_initialized {
                camera.match_free_camera_to_normal();
                photo.free_camera_initialized = true;
            }
        }
        if event.previous == AppMode::Photomode && event.current != AppMode::Replay {
            debug.camera_mode = CameraMode::NORMAL;
            formation.switch_kind(1);
            vermeer.deactivate();
            screen.enabled = false;
        }
    }
    audio.write(PhotoModeAudioParameter(if screen.enabled && debug.camera_mode != CameraMode::WALK { 1.0 } else { 0.0 }));
    for command in commands.read() {
        let Some(mode) = command.camera() else { continue; };
        if debug.camera_mode == CameraMode::WALK {
            // B14234b4cf writes byte1 when leaving the walk camera. This
            // belongs to PlayPhotomodeSound, not the OS focus event stream.
            camera_sounds.write(crate::systems::ui::resources::widget_params::PlayPhotomodeSound(1));
        } else if mode == crate::resources::photo_mode::PhotoModeCameraMode::Walk {
            // B14234b5f7 writes byte0 on entry; preserve the native event
            // convention even though the cached sound names suggest otherwise.
            camera_sounds.write(crate::systems::ui::resources::widget_params::PlayPhotomodeSound(0));
            if !photo.walk_camera_initialized {
                camera.init_walk_camera(&collision, &heights);
                photo.walk_camera_initialized = true;
            }
        }
        debug.camera_mode = mode.into();
        photo.camera_mode = mode;
    }
}

pub fn finish_photomode_frame(mut screen: ResMut<PhotomodeScreen>) {
    screen.finish_frame();
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
        actions.digital[32] = DigitalActionState {
            phase: crate::input::action_set::DigitalActionPhase::JustPressed,
            timer_seconds: 0.0,
        };
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
        actions.digital[32] = DigitalActionState {
            phase: crate::input::action_set::DigitalActionPhase::Repeating,
            timer_seconds: 0.5,
        };
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
