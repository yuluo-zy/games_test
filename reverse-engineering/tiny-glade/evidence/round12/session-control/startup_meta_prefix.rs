//! Native run_game_thread1405a2e10..309c /3976..3aae /6441..651b.
//! Candidate actual prefix dispatcher AFTER original MC RON decode succeeds.
//! Not wired: complete serde/owned MetaCommand path restoration remains needed.
use bevy_app::App;
use country_core::{
    input::InputRecordReplayMode,
    meta_commander::{MetaCommand, MetaCommander},
    resources::session::{NextSession, OnLoadEntry},
    systems::backend::input::input_record_replay::InputRecordReplayState,
};
use std::{collections::VecDeque, time::Duration};
/// Process the complete original prefix, then transfer the untouched suffix
/// either into the actual on-load batch or the existing canonical commander.
/// Never create a second default session: SessionFlowPlugin already owns it.
pub fn apply_startup_meta_prefix(app: &mut App, mut script: VecDeque<MetaCommand>) {
    while let Some(command) = script.pop_front() {
        match command.decoded() {
            6 => {
                // Original uses std Duration::from_secs_f32 with its panic edges.
                let seconds = f32::from_bits(command.payloads[0] as u32);
                app.insert_resource(bevy_time::TimeUpdateStrategy::ManualDuration(
                    Duration::from_secs_f32(seconds),
                ));
            }
            7 => {
                let mut input = InputRecordReplayState::default();
                input.mode = InputRecordReplayMode::DisableInput;
                app.insert_resource(input);
            }
            0 => {
                // Native1405a39a2→658d panics on an incoming owned fence.
                assert_eq!(
                    command.payloads[2], 0,
                    "startup LoadSession fence must be absent"
                );
                let mut next = NextSession::load(command.session_id())
                    .ignore_snapshot(command.payloads[3] as u8 != 0);
                next.on_load.push(OnLoadEntry::Batch {
                    commands: script.into_iter().collect(),
                });
                app.insert_resource(next);
                return;
            }
            _ => {
                // Original restores the front slot before moving the whole
                // remainder into MetaCommander::new; no command is dropped.
                script.push_front(command);
                let mut commander = MetaCommander::new();
                commander.extend_script(script);
                app.insert_resource(commander);
                return;
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use country_core::resources::session::{NextSessionKind, SessionFlowPlugin, SessionId};
    #[test]
    fn no_cli_script_preserves_the_real_session_flow_default_request() {
        let mut app = App::new();
        SessionFlowPlugin.build(app.world_mut());
        apply_startup_meta_prefix(&mut app, VecDeque::new());
        assert_eq!(
            app.world().resource::<NextSession>().kind,
            NextSessionKind::LoadSavedGame
        );
    }
    #[test]
    fn load_prefix_moves_every_later_command_to_owning_on_load_batch() {
        let mut app = App::new();
        SessionFlowPlugin.build(app.world_mut());
        let id = SessionId::new();
        let suffix = [
            MetaCommand::scalar(4, 0),
            MetaCommand::scalar(5, 37),
            MetaCommand::scalar(17, 0),
        ];
        let mut script = VecDeque::from([
            MetaCommand::scalar(6, 0.02f32.to_bits() as u64),
            MetaCommand::scalar(7, 0),
            MetaCommand::load_session(id, true),
        ]);
        script.extend(suffix);
        apply_startup_meta_prefix(&mut app, script);
        let next = app.world().resource::<NextSession>();
        assert_eq!(next.id, id);
        assert!(next.ignore_snapshot);
        assert_eq!(next.kind, NextSessionKind::LoadOnLoad);
        assert!(matches!(&next.on_load[..],[OnLoadEntry::Batch{commands}] if commands==&suffix));
        assert_eq!(
            app.world().resource::<InputRecordReplayState>().mode,
            InputRecordReplayMode::DisableInput
        );
        assert!(
            matches!(app.world().resource::<bevy_time::TimeUpdateStrategy>(),bevy_time::TimeUpdateStrategy::ManualDuration(d) if *d==Duration::from_secs_f32(0.02))
        );
    }
    #[test]
    fn first_nonprefix_command_stops_preprocessing_and_preserves_order() {
        let mut app = App::new();
        SessionFlowPlugin.build(app.world_mut());
        let expected = [
            MetaCommand::scalar(4, 0),
            MetaCommand::scalar(6, 0.1f32.to_bits() as u64),
            MetaCommand::scalar(18, 0),
        ];
        apply_startup_meta_prefix(&mut app, VecDeque::from(expected));
        assert!(
            !app.world()
                .contains_resource::<bevy_time::TimeUpdateStrategy>()
        );
        let mut commander = app.world_mut().resource_mut::<MetaCommander>();
        for command in expected {
            assert_eq!(commander.consume(), Some(command));
        }
        assert!(commander.is_empty());
        assert_eq!(
            app.world().resource::<NextSession>().kind,
            NextSessionKind::LoadSavedGame
        );
    }
}
