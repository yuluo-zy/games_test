//! B reset_input_state @0x140942b00.
//!
//! The original in_set helper @0x14168c0e0 explicitly names ActionSetValues,
//! NewSessionStartedCmd, SessionLoadedCmd and ButtonInput<KeyCode>. Earlier
//! recovery incorrectly substituted translation state and cursor/replay events
//! just because those types had similar byte shapes.
use bevy_ecs::prelude::*;

use super::button_input::KeyButtonInput;
use crate::{
    input::action_set::ActionSetValues,
    resources::session::{NewSessionStartedCmd, SessionLoadedCmd},
};

pub fn reset_input_state(
    mut values: ResMut<ActionSetValues>,
    mut new_sessions: EventReader<NewSessionStartedCmd>,
    mut loaded_sessions: EventReader<SessionLoadedCmd>,
    mut keyboard: ResMut<KeyButtonInput>,
) {
    // Native 11-qword move: previous <- current; current <- zero, every
    // frame. It occurs even when neither session event has arrived.
    values.previous = values.current;
    values.current = Default::default();

    // Preserve short-circuit reader consumption: a new-session event leaves
    // the loaded-session reader untouched until the next call.
    if new_sessions.read().last().is_none() && loaded_sessions.read().last().is_none() {
        return;
    }
    // 会话切换时同时清除按住、按下和释放状态，复用 Bevy 的完整复位语义。
    keyboard.reset_all();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_frame_rotates_values_but_only_session_events_reset_held_keys() {
        let mut world = World::new();
        let mut values = ActionSetValues::default();
        values.current.digital[3] = true;
        world.insert_resource(values);
        let mut keyboard = KeyButtonInput::default();
        keyboard.press(0x14);
        world.insert_resource(keyboard);
        world.init_resource::<Events<NewSessionStartedCmd>>();
        world.init_resource::<Events<SessionLoadedCmd>>();
        let mut schedule = Schedule::default();
        schedule.add_systems(reset_input_state);
        schedule.run(&mut world);
        assert!(world.resource::<ActionSetValues>().previous.digital[3]);
        assert!(!world.resource::<ActionSetValues>().current.digital[3]);
        assert!(world.resource::<KeyButtonInput>().pressed(0x14));
        world.send_event(SessionLoadedCmd);
        schedule.run(&mut world);
        assert!(
            world
                .resource::<KeyButtonInput>()
                .get_pressed()
                .next()
                .is_none()
        );
        assert!(
            world
                .resource::<KeyButtonInput>()
                .get_just_pressed()
                .next()
                .is_none()
        );
    }
}
