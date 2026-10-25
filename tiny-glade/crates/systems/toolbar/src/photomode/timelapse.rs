//! Timelapse completion branch of original B do_photomode@14234b653..7b0.
//! The reader is independent of cleanup_timelapse's PostUpdate cursor.
use bevy_ecs::prelude::*;
use country_core::{
    resources::gui::PhotomodeScreen,
    startup::init_photomode_state::PhotomodeState,
    systems::{
        mode_manager::{AppMode, RequestAppModeChange},
        timelapse_replay::{TimelapseReplayFinished, TimelapseState},
    },
};

/// Original reads the last finished payload, including when the screen is
/// disabled. Only tag1 unhides and reopens Photomode; tag0 leaves it unchanged.
/// Run after camera transitions and before finish_photomode_frame in set8.
pub fn restore_after_timelapse(
    mut finished: EventReader<TimelapseReplayFinished>,
    mut photo: ResMut<PhotomodeState>,
    mut screen: ResMut<PhotomodeScreen>,
    mut modes: EventWriter<RequestAppModeChange>,
) {
    if finished
        .read()
        .last()
        .is_some_and(|event| event.0 == TimelapseState::Restoring)
    {
        photo.hide_ui = false;
        screen.enabled = true;
        modes.write(RequestAppModeChange(AppMode::Photomode));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn last_finished_payload_controls_screen_restore_and_is_consumed_once() {
        let mut world = World::new();
        world.init_resource::<PhotomodeScreen>();
        let mut photo = PhotomodeState::default();
        photo.hide_ui = true;
        world.insert_resource(photo);
        world.init_resource::<Events<TimelapseReplayFinished>>();
        world.init_resource::<Events<RequestAppModeChange>>();
        let mut schedule = Schedule::default();
        schedule.add_systems(restore_after_timelapse);
        world
            .resource_mut::<Events<TimelapseReplayFinished>>()
            .send_batch([
                TimelapseReplayFinished(TimelapseState::Restoring),
                TimelapseReplayFinished(TimelapseState::Running),
            ]);
        schedule.run(&mut world);
        assert!(world.resource::<PhotomodeState>().hide_ui);
        assert!(!world.resource::<PhotomodeScreen>().enabled);
        assert!(world.resource::<Events<RequestAppModeChange>>().is_empty());
        world
            .resource_mut::<Events<TimelapseReplayFinished>>()
            .send(TimelapseReplayFinished(TimelapseState::Restoring));
        schedule.run(&mut world);
        assert!(!world.resource::<PhotomodeState>().hide_ui);
        assert!(world.resource::<PhotomodeScreen>().enabled);
        assert_eq!(world.resource::<Events<RequestAppModeChange>>().len(), 1);
        schedule.run(&mut world);
        assert_eq!(world.resource::<Events<RequestAppModeChange>>().len(), 1);
    }
}
