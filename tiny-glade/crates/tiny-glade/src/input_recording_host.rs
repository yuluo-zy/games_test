//! Desktop lifetime of the current Rust input-stream format. The gameplay
//! input owner handles commands; the OS loop only starts it and saves on exit.
use anyhow::{Context, Result};
use bevy_ecs::world::World;
use country_core::systems::backend::input::input_record_replay::{
    InputRecordReplayState, InputReplayCommand, save_recording,
};
use crate::app_config::InputRecordingOpts;

pub(crate) struct InputRecordingHost {
    option: Option<InputRecordingOpts>,
    awaiting_start: bool,
    started: bool,
}
impl InputRecordingHost {
    pub(crate) fn new(option: Option<InputRecordingOpts>) -> Self {
        Self { option, awaiting_start: false, started: false }
    }
    /// Called exactly when the gameplay App becomes the active input owner.
    /// Loading/warm-up never consumes the CLI request or records its inputs.
    pub(crate) fn activate(&mut self, world: &mut World) {
        let command = match &self.option {
            Some(InputRecordingOpts::Record(_)) => InputReplayCommand::StartRecording,
            Some(InputRecordingOpts::Replay(path)) => InputReplayCommand::Load { path: path.clone(), rewind: 0 },
            None => return,
        };
        world.send_event(command);
        self.awaiting_start = true;
    }
    /// Check the command's result once, after the first actual input update.
    /// Subsequent frames do no stream validation or filesystem work.
    pub(crate) fn input_consumed(&mut self, world: &World) -> Result<()> {
        if !std::mem::take(&mut self.awaiting_start) { return Ok(()); }
        let state = world.get_resource::<InputRecordReplayState>()
            .context("gameplay input recording owner is missing")?;
        if let Some(error) = &state.last_error { anyhow::bail!("input recording activation: {error}"); }
        self.started = true;
        Ok(())
    }
    pub(crate) fn finish(&mut self, world: &World) -> Result<()> {
        if !std::mem::take(&mut self.started) { return Ok(()); }
        let Some(InputRecordingOpts::Record(path)) = &self.option else { return Ok(()); };
        let state = world.get_resource::<InputRecordReplayState>()
            .context("gameplay input recording owner disappeared before save")?;
        save_recording(path, &state.stream)
            .with_context(|| format!("save full input recording {}", path.display()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_ecs::{schedule::{Schedule, IntoScheduleConfigs}, world::World};
    use country_core::{input::action_set::{ActionSetStates, DigitalActionPhase},
        resources::{mouse_capture::CursorCapturedPosition, mouse_terrain_raycast::CursorPositionSS,
            toolbar::Toolbar, time::GameTime},
        systems::{camera_rig::main_camera::MainCamera, mode_manager::AppMode,
            backend::input::{register_input_events, input_record_replay::{
                control_input_replay, input_record_replay, advance_input_replay_frame,
            }}}};

    fn gameplay() -> (World, Schedule) {
        let mut world = World::new();
        register_input_events(&mut world);
        world.insert_resource(Toolbar::default());
        world.insert_resource(AppMode::default());
        world.insert_resource(MainCamera::new(1.0));
        world.init_resource::<CursorCapturedPosition>();
        world.insert_resource(CursorPositionSS(glam::Vec2::ZERO));
        world.resource_mut::<GameTime>().delta = 0.016;
        let mut schedule = Schedule::default();
        schedule.add_systems((control_input_replay, input_record_replay, advance_input_replay_frame).chain());
        (world, schedule)
    }

    #[test]
    fn desktop_owner_saves_the_full_stream_once_and_replays_every_action() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("inputs.json");
        let (mut world, mut schedule) = gameplay();
        let mut recorder = InputRecordingHost::new(Some(InputRecordingOpts::Record(path.clone())));
        recorder.activate(&mut world);
        for phase in [DigitalActionPhase::JustPressed, DigitalActionPhase::Held, DigitalActionPhase::JustReleased] {
            world.resource_mut::<ActionSetStates>().digital[0].phase = phase;
            schedule.run(&mut world);
            recorder.input_consumed(&world).unwrap();
        }
        let expected = world.resource::<InputRecordReplayState>().stream.clone();
        assert_eq!(expected.frame_count(), 3);
        recorder.finish(&world).unwrap();
        recorder.finish(&World::new()).unwrap();
        assert_eq!(country_core::input::recording::load_recording(&path).unwrap(), expected);
        let mut replay = InputRecordingHost::new(Some(InputRecordingOpts::Replay(path)));
        replay.activate(&mut world);
        for frame in expected.frames() {
            *world.resource_mut::<ActionSetStates>() = Default::default();
            schedule.run(&mut world);
            replay.input_consumed(&world).unwrap();
            assert_eq!(*world.resource::<ActionSetStates>(), frame.actions);
        }
        assert_eq!(world.resource::<InputRecordReplayState>().mode, country_core::input::InputRecordReplayMode::None);
    }

    #[test]
    fn a_failed_replay_load_is_returned_at_the_single_activation_boundary() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("invalid.json");
        std::fs::write(&path, "{}").unwrap();
        let (mut world, mut schedule) = gameplay();
        let mut replay = InputRecordingHost::new(Some(InputRecordingOpts::Replay(path)));
        replay.activate(&mut world);
        schedule.run(&mut world);
        assert!(replay.input_consumed(&world).unwrap_err().to_string().contains("input recording activation"));
        replay.finish(&World::new()).unwrap();
    }

    #[test]
    fn closing_during_loading_does_not_create_a_gameplay_recording() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("inputs.json");
        let mut recording = InputRecordingHost::new(Some(InputRecordingOpts::Record(path.clone())));
        recording.finish(&World::new()).unwrap();
        assert!(!path.exists());
    }
}
