//! Audit probes: passing means the documented gap is reproducible, not fixed.
use bevy_ecs::prelude::*;
use country_core::{
    input::{InputRecordReplayMode, action_set::ActionSetAction},
    resources::toolbar::ToolbarCategoryModes,
    systems::{
        backend::input::{add_input_cpu_consumers, input_record_replay::*},
        camera_rig::main_camera::MainCamera,
        mode_manager::AppMode,
    },
};

fn recording() -> RecordingFile {
    RecordingFile {
        replay_initial_state: InputReplayInitialState::default(),
        first_frame: InputFrameSerde {
            translation_state: [0; 11],
            prev_normalized_cursor_pos: [0.0; 2],
            time_delta_seconds: 0.016,
            main_action_set_value: Vec::new(),
        },
    }
}

#[test]
fn gap_snapshot_does_not_restore_toolbar_modes() {
    let mut toolbar = ToolbarCategoryModes {
        modes: vec![AppMode::Path, AppMode::Water],
        current: 1,
    };
    let saved_modes = toolbar.modes.clone();
    let mut translation = InputTranslationStateResource::default();
    let mut camera = MainCamera::default();
    let snapshot = sync_state_from_resources(&translation, &toolbar, &camera);
    assert_eq!(
        snapshot.toolbar_modes_header().1,
        toolbar.modes.as_ptr() as u64
    );
    let mut replay = InputRecordReplayState::default();
    replay.stream.on_history_sync(snapshot, false);
    replay.mode = InputRecordReplayMode::Replay;
    toolbar.modes = vec![AppMode::Path];
    toolbar.current = 0;
    seek_to_sync_point(&mut replay, &mut toolbar, &mut translation, &mut camera, 0);
    assert_ne!(toolbar.modes, saved_modes);
    assert_eq!(toolbar.current, 1);
    assert!(std::panic::catch_unwind(|| toolbar.last_selected_mode()).is_err());
    println!("F1: restored current=1 into modes.len=1; last_selected_mode panicked");
}

#[test]
fn gap_load_on_a_fresh_stream_cannot_seek() {
    let file = recording();
    let path = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(path.path(), serde_json::to_vec(&file).unwrap()).unwrap();
    let mut replay = InputRecordReplayState::default();
    try_load(path.path(), &mut replay.stream).unwrap();
    assert_eq!(replay.stream.frame_count(), 1);
    assert_eq!(replay.stream.sync_point_count(), 0);
    replay.mode = InputRecordReplayMode::Replay;
    let mut toolbar = ToolbarCategoryModes::default();
    let mut translation = InputTranslationStateResource::default();
    let mut camera = MainCamera::default();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        seek_to_sync_point(&mut replay, &mut toolbar, &mut translation, &mut camera, 0);
    }));
    let panic = result.unwrap_err();
    assert_eq!(
        panic.downcast_ref::<&str>().copied(),
        Some("No sync points found in replay state")
    );
    // Loading again appends, rather than replacing the previous recording.
    try_load(path.path(), &mut replay.stream).unwrap();
    assert_eq!(replay.stream.frame_count(), 2);
    println!("F2: load succeeded with zero sync points; seek panicked; second load appended");
}

#[test]
fn gap_frame_conversion_discards_packed_actions_and_cursor() {
    let mut file = recording();
    file.first_frame.main_action_set_value.push(
        ActionSetAction::Analog {
            slot: 2,
            value: glam::Vec2::new(3.0, 4.0),
        }
        .to_bytes(),
    );
    let runtime: InputFrame = file.first_frame.into();
    assert!(
        InputFrameSerde::from(&runtime)
            .main_action_set_value
            .is_empty()
    );
    let mut live = InputFrame::default();
    live.output_qword = 123;
    live.cursor_pos = glam::Vec2::new(100.0, 200.0);
    live.divisor = 200.0;
    let restored: InputFrame = InputFrameSerde::from(&live).into();
    assert_eq!(restored.output_qword, 0);
    assert_eq!(restored.cursor_pos, glam::Vec2::ZERO);
    assert_eq!(restored.divisor, 0.0);
    println!("F3: nonempty packed actions became empty; runtime cursor/output were zeroed");
}

#[test]
fn gap_cpu_schedule_has_no_ordering_edges() {
    let mut schedule = Schedule::default();
    add_input_cpu_consumers(&mut schedule);
    schedule.initialize(&mut World::new()).unwrap();
    // Build transfers systems from ScheduleGraph into the executable schedule.
    assert!(schedule.systems().unwrap().count() >= 11);
    let graph = schedule.graph();
    assert_eq!(graph.dependency().graph().all_edges().count(), 0);
    println!("F4: input CPU helper registered systems with zero ordering edges");
}
