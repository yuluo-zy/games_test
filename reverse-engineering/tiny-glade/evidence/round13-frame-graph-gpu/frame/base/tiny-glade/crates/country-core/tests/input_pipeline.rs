//! 输入完整业务链回归：录制、文件、恢复、真实消费者和最后一帧。
use bevy_ecs::prelude::*;
use country_core::{
    input::{
        InputRecordReplayMode,
        action_set::{ActionSetStates, ActionSetValues, MainActionSet},
        input_stream::InputSnapshot,
    },
    resources::{
        gui::EscMenuState, mouse_capture::CursorCapturedPosition,
        mouse_terrain_raycast::CursorPositionSS, time::GameTime, toolbar::Toolbar,
        window::WindowSize,
    },
    systems::{
        backend::input::{
            self, InputSystemSet,
            button_input::MouseButtonInput,
            input_record_replay::{InputRecordReplayState, InputReplayCommand, save_recording},
            translate_input_to_cursor_moved::InputCursorMoved,
        },
        camera_rig::main_camera::{MainCamera, NORMAL_POSITION_OFFSET},
        mode_manager::{AppMode, AppModeChanged},
    },
};
use glam::Vec2;

#[derive(Resource, Default)]
struct ObservedActions(Vec<MainActionSet>);
fn observe(actions: Res<ActionSetStates>, mut observed: ResMut<ObservedActions>) {
    observed.0.push(*actions);
}
fn fixture() -> (World, Schedule) {
    let mut world = World::new();
    world.init_resource::<country_core::resources::read_mouse_depth_value::MouseDepthValue>();
    input::register_input_events(&mut world);
    world.init_resource::<country_core::game_profile::GameProfile>();
    let mut toolbar = Toolbar::default();
    toolbar.switch_mode(&AppMode::Path);
    world.insert_resource(toolbar);
    world.insert_resource(AppMode::Path);
    world
        .init_resource::<country_core::resources::shader_refactoring_state::ShaderRefactoringState>(
        );
    world.insert_resource(MainCamera::new(1.0));
    world.insert_resource(EscMenuState::default());
    world.insert_resource(CursorPositionSS(Vec2::ZERO));
    world.insert_resource(CursorCapturedPosition::default());
    world.insert_resource(WindowSize::new(100, 100));
    world.insert_resource(ObservedActions::default());
    world.resource_mut::<GameTime>().delta = 0.016;
    world
        .resource_mut::<bevy_time::Time>()
        .advance_by(std::time::Duration::from_millis(16));
    let mut schedule = Schedule::default();
    input::add_input_cpu_consumers(&mut schedule);
    schedule.add_systems(observe.in_set(InputSystemSet::Consume));
    // 模拟真实宿主的 PostUpdate，验证晚到的捕获请求不会覆盖回放策略。
    schedule.add_systems(
        country_core::resources::mouse_capture::mouse_capture.after(InputSystemSet::Finish),
    );
    (world, schedule)
}
fn record_three_frames(world: &mut World, schedule: &mut Schedule) {
    world.send_event(InputReplayCommand::StartRecording);
    for pressed in [true, true, false] {
        let mut mouse = world.resource_mut::<MouseButtonInput>();
        mouse.clear();
        if pressed {
            mouse.press(0);
        } else {
            mouse.release(0);
        }
        world.send_event(InputCursorMoved::Relative(Vec2::new(1.0, 2.0)));
        schedule.run(world);
    }
}

#[test]
fn input_pipeline_records_loads_and_replays_every_edge_to_actual_consumers() {
    let (mut world, mut schedule) = fixture();
    record_three_frames(&mut world, &mut schedule);
    let recorded = world.resource::<ObservedActions>().0.clone();
    assert!(recorded[0].digital[0].just_pressed());
    assert!(!recorded[1].digital[0].just_pressed());
    assert!(recorded[2].digital[0].just_released());
    let file = tempfile::NamedTempFile::new().unwrap();
    save_recording(
        file.path(),
        &world.resource::<InputRecordReplayState>().stream,
    )
    .unwrap();
    world.resource_mut::<ObservedActions>().0.clear();
    // 相反的实时输入不应改变回放动作。
    world.resource_mut::<MouseButtonInput>().press(0);
    world.resource_mut::<Toolbar>().switch_mode(&AppMode::Water);
    *world.resource_mut::<AppMode>() = AppMode::Water;
    world
        .resource_mut::<MainCamera>()
        .write_f32(NORMAL_POSITION_OFFSET, 99.0);
    world.send_event(InputReplayCommand::Load {
        path: file.path().to_owned(),
        rewind: 0,
    });
    for _ in 0..3 {
        schedule.run(&mut world);
    }
    assert_eq!(world.resource::<ObservedActions>().0, recorded);
    assert_eq!(*world.resource::<AppMode>(), AppMode::Path);
    assert_eq!(
        world.resource::<Toolbar>().current,
        country_core::resources::toolbar::ToolbarCategory::Path
    );
    assert_eq!(
        world.resource::<InputRecordReplayState>().mode,
        InputRecordReplayMode::None
    );
    assert_eq!(
        world
            .resource::<InputRecordReplayState>()
            .stream
            .frame_count(),
        3
    );
    assert!(
        world
            .resource::<InputRecordReplayState>()
            .last_error
            .is_none()
    );
}

#[test]
fn input_pipeline_failed_load_and_rewind_preserve_existing_state() {
    let (mut world, mut schedule) = fixture();
    record_three_frames(&mut world, &mut schedule);
    let stream = world.resource::<InputRecordReplayState>().stream.clone();
    world.send_event(InputReplayCommand::Stop);
    schedule.run(&mut world);
    world.send_event(InputReplayCommand::Load {
        path: "/missing/recording.json".into(),
        rewind: 0,
    });
    schedule.run(&mut world);
    assert_eq!(world.resource::<InputRecordReplayState>().stream, stream);
    assert!(
        world
            .resource::<InputRecordReplayState>()
            .last_error
            .is_some()
    );
    world.send_event(InputReplayCommand::Replay { rewind: u64::MAX });
    schedule.run(&mut world);
    assert_eq!(world.resource::<InputRecordReplayState>().stream, stream);
    assert_eq!(
        world.resource::<InputRecordReplayState>().mode,
        InputRecordReplayMode::None
    );
}

#[test]
fn input_pipeline_disabled_and_empty_replay_never_leak_live_buttons() {
    let (mut world, mut schedule) = fixture();
    world.resource_mut::<MouseButtonInput>().press(0);
    world.resource_mut::<InputRecordReplayState>().mode = InputRecordReplayMode::DisableInput;
    schedule.run(&mut world);
    assert_eq!(
        *world.resource::<ActionSetStates>(),
        MainActionSet::default()
    );
    assert!(!world.resource::<ActionSetValues>().current.digital[0]);
    world.resource_mut::<InputRecordReplayState>().mode = InputRecordReplayMode::Replay;
    schedule.run(&mut world);
    assert_eq!(
        *world.resource::<ActionSetStates>(),
        MainActionSet::default()
    );
    assert_eq!(
        world.resource::<InputRecordReplayState>().mode,
        InputRecordReplayMode::None
    );
}

#[test]
fn input_snapshot_owns_toolbar_and_restores_camera_immediately() {
    let mut toolbar = Toolbar::default();
    toolbar.switch_mode(&AppMode::Path);
    let expected_toolbar = toolbar.clone();
    let mut app_mode = AppMode::Path;
    let mut camera = MainCamera::new(1.0);
    let expected_camera = camera.store_parameters();
    let snapshot = InputSnapshot::capture(&toolbar, app_mode, &camera);
    toolbar.categories.clear();
    app_mode = AppMode::Water;
    camera.write_f32(NORMAL_POSITION_OFFSET, 99.0);
    snapshot
        .restore(&mut toolbar, &mut app_mode, &mut camera)
        .unwrap();
    assert_eq!(toolbar, expected_toolbar);
    assert_eq!(app_mode, AppMode::Path);
    assert_eq!(camera.store_parameters(), expected_camera);
    snapshot.validate().unwrap();
}

#[derive(Resource, Default)]
struct ObservedFrame {
    clock: GameTime,
    cursor: Vec2,
    modifiers: u32,
    captured_cursor: Option<Vec2>,
    mode_changes: Vec<AppModeChanged>,
}

fn observe_derived_input(
    clock: Res<GameTime>,
    cursor: Res<CursorPositionSS>,
    modifiers: Res<input::mod_key::Modifiers>,
    captured: Res<CursorCapturedPosition>,
    mut changes: EventReader<AppModeChanged>,
    mut observed: ResMut<ObservedFrame>,
) {
    observed.clock = *clock;
    observed.cursor = cursor.0;
    observed.modifiers = modifiers.flags;
    observed.captured_cursor = captured.0;
    observed.mode_changes.extend(changes.read().copied());
}

fn observe_frame(world: &mut World, schedule: &mut Schedule) {
    world.init_resource::<ObservedFrame>();
    schedule.add_systems(observe_derived_input.in_set(InputSystemSet::Consume));
}

fn two_frame_recording(world: &mut World) {
    use country_core::input::{input_stream::RecordingRetention, recording::InputFrame};
    let snapshot = InputSnapshot::capture(
        world.resource::<Toolbar>(),
        *world.resource::<AppMode>(),
        world.resource::<MainCamera>(),
    );
    let mut replay = world.resource_mut::<InputRecordReplayState>();
    replay.start_recording(snapshot.clone());
    for delta in [0.1, 0.2] {
        replay.stream.record_frame(
            InputFrame {
                time_delta_seconds: delta,
                ..Default::default()
            },
            || snapshot.clone(),
            RecordingRetention::All,
        );
    }
}

#[test]
fn replay_start_last_frame_and_return_to_live_update_the_entire_clock_once() {
    use country_core::resources::game_clock::{ClockGate, DeltaSource};
    let (mut world, mut schedule) = fixture();
    observe_frame(&mut world, &mut schedule);
    two_frame_recording(&mut world);
    let mut expected = *world.resource::<GameTime>();
    let gate = ClockGate {
        state_is_zero: true,
        mode_tag: 2,
        mode_flag: 0,
        paused: false,
    };
    world.send_event(InputReplayCommand::Replay { rewind: 0 });
    for delta in [0.1, 0.2] {
        expected
            .update(gate, DeltaSource::Replay { frame_delta: delta })
            .unwrap();
        schedule.run(&mut world);
        assert_eq!(world.resource::<ObservedFrame>().clock, expected);
    }
    assert_eq!(
        world.resource::<InputRecordReplayState>().mode,
        InputRecordReplayMode::None
    );
    expected
        .update(
            gate,
            DeltaSource::Live {
                secs: 0,
                nanos: 16_000_000,
            },
        )
        .unwrap();
    schedule.run(&mut world);
    assert_eq!(world.resource::<ObservedFrame>().clock, expected);
}

#[test]
fn stop_command_selects_live_time_before_clock_update() {
    use country_core::resources::game_clock::{ClockGate, DeltaSource};
    let (mut world, mut schedule) = fixture();
    two_frame_recording(&mut world);
    world.send_event(InputReplayCommand::Replay { rewind: 0 });
    schedule.run(&mut world);
    let mut expected = *world.resource::<GameTime>();
    let gate = ClockGate {
        state_is_zero: true,
        mode_tag: 2,
        mode_flag: 0,
        paused: false,
    };
    expected
        .update(
            gate,
            DeltaSource::Live {
                secs: 0,
                nanos: 16_000_000,
            },
        )
        .unwrap();
    world.send_event(InputReplayCommand::Stop);
    schedule.run(&mut world);
    assert_eq!(*world.resource::<GameTime>(), expected);
}

#[test]
fn seek_restores_actual_tool_state_and_publishes_mode_change_once() {
    let (mut world, mut schedule) = fixture();
    observe_frame(&mut world, &mut schedule);
    two_frame_recording(&mut world);
    world.resource_mut::<Toolbar>().switch_mode(&AppMode::Water);
    *world.resource_mut::<AppMode>() = AppMode::Water;
    world
        .resource_mut::<country_core::systems::mode_manager::PreviousAppMode>()
        .0 = AppMode::Water;
    world.send_event(InputReplayCommand::Replay { rewind: 0 });
    schedule.run(&mut world);
    assert_eq!(*world.resource::<AppMode>(), AppMode::Path);
    assert_eq!(
        world.resource::<Toolbar>().current,
        country_core::resources::toolbar::ToolbarCategory::Path
    );
    assert_eq!(
        world.resource::<ObservedFrame>().mode_changes,
        [AppModeChanged {
            previous: AppMode::Water,
            current: AppMode::Path,
        }]
    );
    schedule.run(&mut world);
    assert_eq!(world.resource::<ObservedFrame>().mode_changes.len(), 1);
}

#[test]
fn consume_is_ordered_after_cursor_and_modifier_derivation() {
    use bevy_ecs::schedule::{LogLevel, ScheduleBuildSettings};
    let (mut world, mut schedule) = fixture();
    observe_frame(&mut world, &mut schedule);
    schedule.set_build_settings(ScheduleBuildSettings {
        ambiguity_detection: LogLevel::Error,
        ..Default::default()
    });
    schedule.initialize(&mut world).unwrap();
    // Derive the actual modifier from the loaded profile binding, rather than
    // relying on the removed always-enabled ModKeyConfig fixture.
    let disable_key = world.resource::<country_core::game_profile::GameProfile>()
        .key_bindings.bindings[24].unwrap();
    world.resource_mut::<country_core::systems::backend::input::button_input::KeyButtonInput>()
        .press(country_core::game_profile::desktop_keycode_index(disable_key).unwrap());
    world.resource_mut::<CursorPositionSS>().0 = Vec2::new(99.0, 50.0);
    world.send_event(InputCursorMoved::Relative(Vec2::new(5.0, 0.0)));
    schedule.run(&mut world);
    assert_eq!(
        world.resource::<ObservedFrame>().cursor,
        Vec2::new(100.0, 50.0)
    );
    assert_eq!(world.resource::<ObservedFrame>().modifiers, 0x101);
    let actions = world.resource::<ActionSetStates>();
    assert_eq!(actions.analog[0], Vec2::new(5.0, 0.0));
    assert_eq!(actions.analog[1], Vec2::new(1.0, 0.0));
}

#[test]
fn absolute_cursor_is_clamped_and_capture_preserves_anchor() {
    let (mut world, mut schedule) = fixture();
    world.send_event(InputCursorMoved::Absolute(Vec2::new(-10.0, 200.0)));
    schedule.run(&mut world);
    assert_eq!(
        world.resource::<CursorPositionSS>().0,
        Vec2::new(0.0, 100.0)
    );
    world.resource_mut::<CursorCapturedPosition>().0 = Some(Vec2::new(30.0, 40.0));
    world.send_event(InputCursorMoved::RelativeWhileCaptured(Vec2::new(
        20.0, 15.0,
    )));
    schedule.run(&mut world);
    assert_eq!(
        world.resource::<CursorPositionSS>().0,
        Vec2::new(30.0, 40.0)
    );
    assert_eq!(
        world.resource::<ActionSetStates>().analog[0],
        Vec2::new(20.0, 15.0)
    );
}

#[test]
fn replay_restores_recorded_capture_instead_of_live_capture() {
    let (mut world, mut schedule) = fixture();
    observe_frame(&mut world, &mut schedule);
    world.resource_mut::<CursorCapturedPosition>().0 = Some(Vec2::new(30.0, 40.0));
    world.resource_mut::<CursorPositionSS>().0 = Vec2::new(30.0, 40.0);
    world.send_event(InputReplayCommand::StartRecording);
    world.send_event(InputCursorMoved::RelativeWhileCaptured(Vec2::new(8.0, 7.0)));
    schedule.run(&mut world);
    world.resource_mut::<CursorCapturedPosition>().0 = Some(Vec2::new(90.0, 90.0));
    world.send_event(InputReplayCommand::Replay { rewind: 0 });
    schedule.run(&mut world);
    assert_eq!(
        world.resource::<ObservedFrame>().captured_cursor,
        Some(Vec2::new(30.0, 40.0))
    );
    assert_eq!(world.resource::<CursorCapturedPosition>().0, None);
    assert_eq!(
        world.resource::<CursorPositionSS>().0,
        Vec2::new(30.0, 40.0)
    );
}

#[test]
fn clock_overflow_stops_replay_without_partial_clock_or_live_action_leak() {
    let (mut world, mut schedule) = fixture();
    observe_frame(&mut world, &mut schedule);
    two_frame_recording(&mut world);
    world.resource_mut::<GameTime>().elapsed = std::time::Duration::MAX;
    let mut expected = *world.resource::<GameTime>();
    expected.delta = 0.0;
    expected.ticks_60 = 0;
    expected.ran = false;
    world.resource_mut::<MouseButtonInput>().press(0);
    world.send_event(InputReplayCommand::Replay { rewind: 0 });
    schedule.run(&mut world);
    assert_eq!(world.resource::<ObservedFrame>().clock, expected);
    assert_eq!(
        *world.resource::<ActionSetStates>(),
        MainActionSet::default()
    );
    let replay = world.resource::<InputRecordReplayState>();
    assert_eq!(replay.mode, InputRecordReplayMode::None);
    assert_eq!(replay.stream.frame_count(), 2);
    assert!(replay.last_error.as_ref().unwrap().contains("overflow"));
}

#[test]
fn malformed_in_memory_replay_frame_is_stopped_before_consumption() {
    let (mut world, mut schedule) = fixture();
    two_frame_recording(&mut world);
    world
        .resource_mut::<InputRecordReplayState>()
        .stream
        .last_frame_mut()
        .unwrap()
        .time_delta_seconds = f32::MAX;
    world.send_event(InputReplayCommand::Replay { rewind: 0 });
    schedule.run(&mut world);
    let before = *world.resource::<GameTime>();
    schedule.run(&mut world);
    assert_eq!(world.resource::<GameTime>().elapsed, before.elapsed);
    assert_eq!(world.resource::<GameTime>().delta, 0.0);
    assert_eq!(
        *world.resource::<ActionSetStates>(),
        MainActionSet::default()
    );
    assert!(
        world
            .resource::<InputRecordReplayState>()
            .last_error
            .is_some()
    );
}

#[test]
fn right_click_uses_only_motion_accumulated_on_held_frames() {
    for (press_motion, held_motion, release_motion, expect_click) in [
        (0.0, 20.0, 0.0, false),
        // B1417cd959 resets at just-press, accumulates only while held, and
        // checks the stored length on release. Press/release-frame deltas are
        // deliberately excluded; do not alter the original gesture algorithm.
        (0.0, 0.0, 20.0, true),
        (20.0, 0.0, 0.0, true),
        (0.0, 1.0, 1.0, true),
    ] {
        let (mut world, mut schedule) = fixture();
        world.resource_mut::<MouseButtonInput>().press(1);
        world.send_event(InputCursorMoved::Relative(Vec2::new(press_motion, 0.0)));
        schedule.run(&mut world);
        world.resource_mut::<MouseButtonInput>().clear();
        world.send_event(InputCursorMoved::Relative(Vec2::new(held_motion, 0.0)));
        schedule.run(&mut world);
        {
            let mut mouse = world.resource_mut::<MouseButtonInput>();
            mouse.clear();
            mouse.release(1);
            assert!(!mouse.just_pressed(1));
            assert!(mouse.just_released(1));
        }
        world.send_event(InputCursorMoved::Relative(Vec2::new(release_motion, 0.0)));
        schedule.run(&mut world);
        assert_eq!(
            world.resource::<ActionSetStates>().digital[1].just_pressed(),
            expect_click
        );
    }
}

#[test]
fn terminal_sync_point_does_not_hide_the_last_playable_checkpoint() {
    let (mut world, mut schedule) = fixture();
    world.send_event(InputReplayCommand::StartRecording);
    schedule.run(&mut world);
    world.send_event(InputReplayCommand::Sync { record: true });
    world.send_event(InputReplayCommand::Stop);
    schedule.run(&mut world);
    let file = tempfile::NamedTempFile::new().unwrap();
    save_recording(
        file.path(),
        &world.resource::<InputRecordReplayState>().stream,
    )
    .unwrap();
    world.send_event(InputReplayCommand::Load {
        path: file.path().to_owned(),
        rewind: 0,
    });
    schedule.run(&mut world);
    let replay = world.resource::<InputRecordReplayState>();
    assert!(replay.last_error.is_none());
    assert_eq!(replay.next_frame_index(), 1);
    assert_eq!(replay.stream.sync_point_count(), 2);
    world.send_event(InputReplayCommand::Replay { rewind: 1 });
    schedule.run(&mut world);
    assert!(
        world
            .resource::<InputRecordReplayState>()
            .last_error
            .is_some()
    );
}

fn capture_commands(
    world: &mut World,
) -> (
    Vec<country_core::resources::mouse_capture::OsCursorCaptureCmd>,
    Vec<country_core::resources::mouse_capture::OsCursorVisibilityCmd>,
) {
    use country_core::resources::mouse_capture::{OsCursorCaptureCmd, OsCursorVisibilityCmd};
    (
        world
            .resource_mut::<Events<OsCursorCaptureCmd>>()
            .drain()
            .collect(),
        world
            .resource_mut::<Events<OsCursorVisibilityCmd>>()
            .drain()
            .collect(),
    )
}

fn captured_recording(world: &mut World, schedule: &mut Schedule) {
    let anchor = Vec2::new(30.0, 40.0);
    world.resource_mut::<CursorCapturedPosition>().0 = Some(anchor);
    world.resource_mut::<CursorPositionSS>().0 = anchor;
    world.send_event(InputReplayCommand::StartRecording);
    for _ in 0..3 {
        schedule.run(world);
    }
    // 模拟开始回放时宿主已经释放鼠标。
    world.resource_mut::<CursorCapturedPosition>().0 = None;
    capture_commands(world);
    world.send_event(InputReplayCommand::Replay { rewind: 0 });
}

fn assert_capture_released(world: &mut World) {
    use country_core::resources::mouse_capture::{
        OsCursorCaptureCmd, OsCursorVisibilityCmd, OsCursorVisibilityCmdOrigin,
    };
    assert_eq!(world.resource::<CursorCapturedPosition>().0, None);
    assert_eq!(
        capture_commands(world),
        (
            vec![OsCursorCaptureCmd::Release],
            vec![OsCursorVisibilityCmd::Unsuppress(
                OsCursorVisibilityCmdOrigin::MOUSE_CAPTURE
            )]
        )
    );
}

#[test]
fn replay_capture_updates_os_and_visibility_then_releases_at_completion() {
    use country_core::resources::mouse_capture::{
        CursorCaptureCmd, OsCursorCaptureCmd, OsCursorVisibilityCmd, OsCursorVisibilityCmdOrigin,
    };
    let (mut world, mut schedule) = fixture();
    captured_recording(&mut world, &mut schedule);
    schedule.run(&mut world);
    assert_eq!(
        capture_commands(&mut world),
        (
            vec![OsCursorCaptureCmd::Capture(Vec2::new(30.0, 40.0))],
            vec![OsCursorVisibilityCmd::Suppress(
                OsCursorVisibilityCmdOrigin::MOUSE_CAPTURE
            )]
        )
    );
    // 回放中间帧忽略实时释放，且相同锚点不会重复发出 OS 命令。
    world.send_event(CursorCaptureCmd(true));
    schedule.run(&mut world);
    assert_eq!(
        world.resource::<CursorCapturedPosition>().0,
        Some(Vec2::new(30.0, 40.0))
    );
    assert_eq!(capture_commands(&mut world), (vec![], vec![]));
    // 最后一帧晚到的实时捕获不能把刚释放的鼠标重新抓住。
    world.send_event(CursorCaptureCmd(false));
    schedule.run(&mut world);
    assert_capture_released(&mut world);
    schedule.run(&mut world);
    assert_eq!(world.resource::<CursorCapturedPosition>().0, None);
    assert_eq!(capture_commands(&mut world), (vec![], vec![]));
}

#[test]
fn uncaptured_replay_frame_releases_a_previously_captured_os_cursor() {
    let (mut world, mut schedule) = fixture();
    world.send_event(InputReplayCommand::StartRecording);
    schedule.run(&mut world);
    world.resource_mut::<CursorCapturedPosition>().0 = Some(Vec2::new(30.0, 40.0));
    world.send_event(InputReplayCommand::Replay { rewind: 0 });
    schedule.run(&mut world);
    assert_capture_released(&mut world);
}

#[test]
fn stop_and_invalid_frame_release_capture_and_allow_future_live_requests() {
    use country_core::resources::mouse_capture::CursorCaptureCmd;
    for invalid_frame in [false, true] {
        let (mut world, mut schedule) = fixture();
        captured_recording(&mut world, &mut schedule);
        schedule.run(&mut world);
        capture_commands(&mut world);
        if invalid_frame {
            world
                .resource_mut::<InputRecordReplayState>()
                .stream
                .last_frame_mut()
                .unwrap()
                .time_delta_seconds = f32::NAN;
            schedule.run(&mut world);
        } else {
            world.send_event(InputReplayCommand::Stop);
        }
        world.send_event(CursorCaptureCmd(false));
        schedule.run(&mut world);
        assert_capture_released(&mut world);
        assert_eq!(
            world.resource::<InputRecordReplayState>().mode,
            InputRecordReplayMode::None
        );
        // 上一帧请求已经被消费；后续新的实时捕获恢复正常工作。
        schedule.run(&mut world);
        assert_eq!(world.resource::<CursorCapturedPosition>().0, None);
        world.send_event(CursorCaptureCmd(false));
        schedule.run(&mut world);
        assert!(world.resource::<CursorCapturedPosition>().0.is_some());
    }
}
