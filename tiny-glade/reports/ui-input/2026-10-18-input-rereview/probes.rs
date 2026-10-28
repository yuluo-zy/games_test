use bevy_ecs::prelude::*;
use country_core::{
    input::{input_stream::{InputSnapshot, RecordingRetention}, recording::InputFrame},
    resources::{
        gui::{EscMenuState, EguiContextResource}, mouse_capture::CursorCapturedPosition,
        mouse_terrain_raycast::CursorPositionSS, time::GameTime, toolbar::ToolbarCategoryModes,
        window::WindowSize, shader_refactoring_state::ShaderRefactoringState,
    },
    systems::{backend::input::{self, InputSystemSet,
        input_record_replay::{InputRecordReplayState, InputReplayCommand},
        translate_input_to_cursor_moved::InputCursorMoved,
    }, camera_rig::main_camera::MainCamera, time_update::time_update},
};
use glam::Vec2;
use std::time::Duration;

fn fixture() -> (World, Schedule) {
    let mut world = World::new();
    input::register_input_events(&mut world);
    world.insert_resource(ToolbarCategoryModes::default());
    world.insert_resource(MainCamera::new(1.0));
    world.insert_resource(EscMenuState::default());
    world.insert_resource(CursorPositionSS(Vec2::ZERO));
    world.insert_resource(CursorCapturedPosition::default());
    world.insert_resource(WindowSize::new(100, 100));
    world.resource_mut::<GameTime>().delta = 0.016;
    world.resource_mut::<bevy_time::Time>().advance_by(Duration::from_millis(16));
    let mut schedule = Schedule::default();
    input::add_input_cpu_consumers(&mut schedule);
    (world, schedule)
}

// 本报告探针断言当前缺陷表现，成功不表示修复通过。
#[test]
fn reproduce_cursor_exceeding_window_boundary() {
    let (mut world, mut schedule) = fixture();
    world.resource_mut::<CursorPositionSS>().0 = Vec2::new(99.0, 50.0);
    world.send_event(InputCursorMoved::Relative(Vec2::new(5.0, 0.0)));
    schedule.run(&mut world);
    let actual = world.resource::<CursorPositionSS>().0;
    println!("cursor: expected x=100, actual={actual:?}");
    assert_eq!(actual.x, 104.0);
}

#[derive(Resource, Default)]
struct ObservedCursor(Vec2);
fn observe_cursor(cursor: Res<CursorPositionSS>, mut output: ResMut<ObservedCursor>) {
    output.0 = cursor.0;
}

#[test]
fn reproduce_unordered_derived_cursor_and_consumer() {
    use bevy_ecs::schedule::{ScheduleBuildSettings, LogLevel};
    let (mut world, mut schedule) = fixture();
    world.init_resource::<ObservedCursor>();
    schedule.add_systems(observe_cursor.in_set(InputSystemSet::Consume));
    schedule.set_build_settings(ScheduleBuildSettings {
        ambiguity_detection: LogLevel::Error,
        ..Default::default()
    });
    let error = schedule.initialize(&mut world).unwrap_err();
    println!("schedule ambiguity: {error}");
    assert!(format!("{error}").contains("ambigu"));
}

#[test]
fn reproduce_replay_command_clock_mismatch() {
    let (mut world, mut schedule) = fixture();
    world.insert_resource(ShaderRefactoringState::default());
    world.resource_mut::<GameTime>().frame_count = 10;
    schedule.add_systems(time_update.before(InputSystemSet::Prepare));
    let snapshot = InputSnapshot::capture(
        world.resource::<ToolbarCategoryModes>(), world.resource::<MainCamera>());
    let mut replay = world.resource_mut::<InputRecordReplayState>();
    replay.start_recording(snapshot.clone());
    for delta in [0.1, 0.2] {
        replay.stream.record_frame(InputFrame { time_delta_seconds: delta,
            ..Default::default() }, || snapshot.clone(), RecordingRetention::All);
    }
    drop(replay);
    world.send_event(InputReplayCommand::Replay { rewind: 0 });
    schedule.run(&mut world);
    let game = *world.resource::<GameTime>();
    println!("replay first frame: delta={}, elapsed={:?}, total_secs={}, ticks_60={}",
        game.delta, game.elapsed, game.total_secs, game.ticks_60);
    assert_eq!(game.delta, 0.1);
    assert_ne!(game.elapsed, Duration::from_secs_f32(game.delta));
}

#[test]
fn reproduce_ui_context_exists_without_active_frame() {
    let (mut world, mut schedule) = fixture();
    world.insert_resource(EguiContextResource(egui::Context::default()));
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| schedule.run(&mut world)));
    println!("context exists, no egui frame: panicked={}", result.is_err());
    assert!(result.is_err());
}

#[test]
fn reproduce_validated_recording_overflowing_clock() {
    use country_core::input::recording::{load_recording, save_recording};
    use country_core::resources::game_clock::{GameClock, ClockGate, DeltaSource, GameClockError};
    let snapshot = InputSnapshot::capture(&ToolbarCategoryModes::default(), &MainCamera::new(1.0));
    let mut replay = InputRecordReplayState::default();
    replay.start_recording(snapshot.clone());
    for _ in 0..2 {
        replay.stream.record_frame(InputFrame { time_delta_seconds: 1.0e19,
            ..Default::default() }, || snapshot.clone(), RecordingRetention::All);
    }
    let path = tempfile::NamedTempFile::new().unwrap();
    save_recording(path.path(), &replay.stream).unwrap();
    let loaded = load_recording(path.path()).unwrap();
    let mut clock = GameClock::default();
    let gate = ClockGate { state_is_zero: true, mode_tag: 2, mode_flag: 0, paused: false };
    let results = loaded.frames().map(|frame| clock.update(gate,
        DeltaSource::Replay { frame_delta: frame.time_delta_seconds })).collect::<Vec<_>>();
    println!("validated and reloaded two frames: clock results={results:?}");
    assert_eq!(results, vec![Ok(()), Err(GameClockError::DurationOverflow)]);
}

#[test]
fn reproduce_snapshot_not_restoring_real_toolbar_and_app_mode() {
    use country_core::resources::toolbar::Toolbar;
    use country_core::systems::mode_manager::{AppMode, select_app_mode};
    let (mut world, mut schedule) = fixture();
    let mut toolbar = Toolbar::default();
    let mut mode = AppMode::Path;
    select_app_mode(AppMode::Path, &mut toolbar, &mut mode);
    world.insert_resource(toolbar);
    world.insert_resource(mode);
    world.send_event(InputReplayCommand::StartRecording);
    schedule.run(&mut world);
    let mut toolbar = world.remove_resource::<Toolbar>().unwrap();
    let mut mode = world.remove_resource::<AppMode>().unwrap();
    select_app_mode(AppMode::Water, &mut toolbar, &mut mode);
    world.insert_resource(toolbar);
    world.insert_resource(mode);
    world.send_event(InputReplayCommand::Replay { rewind: 0 });
    schedule.run(&mut world);
    println!("recorded Path, actual mode after replay seek={:?}", world.resource::<AppMode>());
    assert_eq!(*world.resource::<AppMode>(), AppMode::Water);
}
