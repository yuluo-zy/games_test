use bevy_ecs::prelude::*;
use country_core::{
    input::{InputRecordReplayMode, action_set::ActionSetStates},
    resources::{
        gui::EscMenuState,
        mouse_capture::{CursorCapturedPosition, OsCursorCaptureCmd},
        mouse_terrain_raycast::CursorPositionSS,
        shader_refactoring_state::ShaderRefactoringState,
        toolbar::Toolbar,
        window::WindowSize,
    },
    systems::{
        backend::input::{self,
            button_input::MouseButtonInput,
            input_record_replay::{InputRecordReplayState, InputReplayCommand, save_recording, load_recording},
            translate_input_to_cursor_moved::InputCursorMoved,
        },
        camera_rig::main_camera::MainCamera,
        mode_manager::AppMode,
    },
};
use glam::Vec2;

fn fixture() -> (World, Schedule) {
    let mut world = World::new();
    world.insert_resource(AppMode::Path);
    let mut toolbar = Toolbar::default();
    toolbar.switch_mode(&AppMode::Path);
    world.insert_resource(toolbar);
    input::register_input_events(&mut world);
    world.insert_resource(MainCamera::new(1.0));
    world.insert_resource(EscMenuState::Closed);
    world.init_resource::<ShaderRefactoringState>();
    world.insert_resource(CursorPositionSS(Vec2::ZERO));
    world.init_resource::<CursorCapturedPosition>();
    world.insert_resource(WindowSize::new(100, 100));
    world.init_resource::<Events<OsCursorCaptureCmd>>();
    world.resource_mut::<bevy_time::Time>().advance_by(std::time::Duration::from_millis(16));
    let mut schedule = Schedule::default();
    input::add_input_cpu_consumers(&mut schedule);
    (world, schedule)
}

// 诊断探针断言缺陷表现；通过表示反例已复现，不表示生产代码正确。
#[test]
fn reproduce_right_drag_release_being_misclassified_as_click() {
    let (mut world, mut schedule) = fixture();
    {
        let mut mouse = world.resource_mut::<MouseButtonInput>();
        mouse.pressed.insert(1);
        mouse.just_pressed.insert(1);
    }
    schedule.run(&mut world);
    world.resource_mut::<MouseButtonInput>().just_pressed.clear();
    world.send_event(InputCursorMoved::Relative(Vec2::new(20.0, 0.0)));
    schedule.run(&mut world);
    // 与 event_loop.rs 的实际 Released 分支一致。
    {
        let mut mouse = world.resource_mut::<MouseButtonInput>();
        mouse.pressed.remove(&1);
        mouse.just_pressed.insert(1);
    }
    schedule.run(&mut world);
    let right_click = world.resource::<ActionSetStates>().digital[1];
    println!("20px drag, release: right_click={right_click:?}");
    assert!(right_click.just_pressed());
}

#[test]
fn reproduce_end_sync_point_making_nonempty_recording_unplayable_by_default() {
    let (mut world, mut schedule) = fixture();
    world.send_event(InputReplayCommand::StartRecording);
    schedule.run(&mut world);
    // 同一帧控制命令在录制下一帧之前打点并停止。
    world.send_event(InputReplayCommand::Sync { record: true });
    world.send_event(InputReplayCommand::Stop);
    schedule.run(&mut world);
    let file = tempfile::NamedTempFile::new().unwrap();
    save_recording(file.path(), &world.resource::<InputRecordReplayState>().stream).unwrap();
    let loaded = load_recording(file.path()).unwrap();
    assert_eq!(loaded.frame_count(), 1);
    assert_eq!(loaded.sync_points().back().unwrap().frame_index, 1);
    world.send_event(InputReplayCommand::Load { path: file.path().to_owned(), rewind: 0 });
    schedule.run(&mut world);
    let state = world.resource::<InputRecordReplayState>();
    println!("valid nonempty saved file: default Load error={:?}", state.last_error);
    assert_eq!(state.mode, InputRecordReplayMode::None);
    assert!(state.last_error.as_ref().unwrap().contains("没有可回放"));
    world.send_event(InputReplayCommand::Replay { rewind: 1 });
    schedule.run(&mut world);
    assert!(world.resource::<InputRecordReplayState>().last_error.is_none());
}

#[test]
fn reproduce_replay_clearing_capture_without_releasing_the_os_cursor() {
    let (mut world, mut schedule) = fixture();
    world.send_event(InputReplayCommand::StartRecording);
    schedule.run(&mut world); // 录制未捕获的一帧。
    world.resource_mut::<CursorCapturedPosition>().0 = Some(Vec2::new(30.0, 40.0));
    // 此时真实宿主已捕获窗口；事件清空，等待回放产生对应 Release。
    world.send_event(InputReplayCommand::Replay { rewind: 0 });
    schedule.run(&mut world);
    let commands = world.resource_mut::<Events<OsCursorCaptureCmd>>().drain().collect::<Vec<_>>();
    println!("after replay: internal_capture={:?}, OS commands={commands:?}", world.resource::<CursorCapturedPosition>());
    assert_eq!(world.resource::<CursorCapturedPosition>().0, None);
    assert!(commands.is_empty());
    assert_eq!(world.resource::<InputRecordReplayState>().mode, InputRecordReplayMode::None);
}
