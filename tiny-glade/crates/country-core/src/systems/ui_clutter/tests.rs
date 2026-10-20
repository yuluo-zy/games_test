use super::{
    DeletedAutoClutter, defered_autoclutter_history::RecordAutoClutterHistory,
    edit_autoclutter::DeleteAutoClutterCmd,
};
use crate::resources::history::building_transactions::RoofHistoryEdit;
use crate::resources::history::building_transactions::RoofReplayCommand;
use crate::resources::history::transactions::ObservedHistoryLedger;
use crate::resources::history::EditType;
use crate::resources::history::History;
use crate::resources::history::ReplayBucket;
use crate::resources::history::ReplayBuffer;
use crate::resources::history::ReplayCommand;
use crate::resources::history::auto_clutter_edit::AutoClutterEdit;
use crate::resources::history::auto_clutter_edit::AutoClutterReplayCmd;
use crate::resources::time::GameTime;
use crate::systems::history::*;
use crate::systems::mode_manager::HistoryCommand;
use bevy_ecs::prelude::*;
use std::sync::Arc;

fn empty_history() -> History<()> {
    History::<()>::from_observed(ObservedHistoryLedger {
        entries: Vec::new(),
        cursor: -1,
        undo_anchor: -1,
        resolved_snapshots: Vec::new(),
        observed_count_58: 0,
        mutation_nonce: 0,
    })
}

#[test]
fn autoclutter_delete_commit_undo_redo_use_one_live_owner() {
    let mut world = World::new();
    register_history_events(&mut world);
    world.insert_resource(empty_history());
    world.insert_resource(ReplayBuffer::new(2.0));
    world.insert_resource(GameTime {
        delta: 0.05,
        ..Default::default()
    });
    let mut schedule = Schedule::default();
    schedule.add_systems(
        (
            enqueue_history_commands,
            process_history_cursor_commands::<()>,
            replay_auto_clutter::replay_auto_clutter,
            super::edit_autoclutter::delete_auto_clutter::<()>,
            super::defered_autoclutter_history::defered_autoclutter_history::<()>,
            pop_empty_replay_bucket,
        )
            .chain(),
    );
    world
        .resource_mut::<History<()>>()
        .start_input_recording(0)
        .unwrap();
    for id in [5, 8] {
        world
            .resource_mut::<Events<DeleteAutoClutterCmd>>()
            .send(DeleteAutoClutterCmd(id));
    }
    schedule.run(&mut world);
    assert!(world.resource::<DeletedAutoClutter>().is_deleted(5));
    assert!(world.resource::<DeletedAutoClutter>().is_deleted(8));
    world
        .resource_mut::<History<()>>()
        .finish_input_recording()
        .unwrap();
    let history = world.resource::<History<()>>();
    assert_eq!(history.ledger().entries.len(), 1);
    let edits = &history.ledger().entries[0].edits;
    assert_eq!(edits.len(), 2);
    for (edit, id) in edits.iter().zip([5, 8]) {
        let EditType::AutoClutter(edit) = edit else {
            panic!("wrong history owner");
        };
        assert_eq!(**edit, AutoClutterEdit::Delete { id });
    }
    for (command, is_deleted, cursor) in [
        (HistoryCommand::UNDO, false, -1),
        (HistoryCommand::REDO, true, 0),
    ] {
        world.resource_mut::<Events<HistoryCommand>>().send(command);
        schedule.run(&mut world);
        // Application is at elapsed==0; completion waits for the strict bound.
        assert_eq!(
            world.resource::<DeletedAutoClutter>().is_deleted(5),
            is_deleted
        );
        assert_eq!(
            world.resource::<DeletedAutoClutter>().is_deleted(8),
            is_deleted
        );
        assert_eq!(world.resource::<History<()>>().ledger().cursor, cursor);
        assert!(!world.resource::<ReplayBuffer>().is_empty());
        schedule.run(&mut world); // old elapsed == 0.05: retain
        assert!(!world.resource::<ReplayBuffer>().is_empty());
        schedule.run(&mut world); // old elapsed == 0.1 > 0.05: remove
        assert!(world.resource::<ReplayBuffer>().is_empty());
    }
    assert_eq!(world.resource::<History<()>>().ledger().entries.len(), 1);
    assert_eq!(
        world.resource::<Events<RecordAutoClutterHistory>>().len(),
        2
    );
    assert!(
        world
            .resource::<Events<recording::HistoryRecordingFailed>>()
            .is_empty()
    );
    assert!(world.resource::<Events<HistoryReplayFailed>>().is_empty());
}

#[test]
fn autoclutter_replay_preserves_native_timing_and_front_bucket() {
    use replay_auto_clutter::advance_auto_clutter;
    let edit = Arc::new(AutoClutterEdit::Delete { id: 42 });
    let mut deleted = DeletedAutoClutter::default();
    let mut command = AutoClutterReplayCmd::expanded_shared(edit.clone(), false, false);
    assert!(!advance_auto_clutter(&mut command, &mut deleted, 1.0, 0.0));
    deleted.0.remove(&42);
    assert!(!advance_auto_clutter(&mut command, &mut deleted, 1.0, 0.0));
    assert!(deleted.is_deleted(42)); // no synthetic started flag
    command.duration = f32::NAN;
    command.elapsed = 0.06;
    assert!(advance_auto_clutter(&mut command, &mut deleted, 1.0, 0.01));
    let mut instant = AutoClutterReplayCmd::expanded_shared(edit.clone(), true, true);
    assert!(advance_auto_clutter(&mut instant, &mut deleted, 1.0, 1.0));
    assert_eq!(instant.elapsed, 0.0);
    assert!(!deleted.is_deleted(42));

    let mut world = World::new();
    world.insert_resource(deleted);
    world.insert_resource(GameTime::default());
    let mut buffer = ReplayBuffer::new(1.0);
    buffer.push(ReplayBucket {
        entry_index: 0,
        undo: false,
        commands: vec![
            ReplayCommand::AutoClutter(AutoClutterReplayCmd::expanded_shared(
                edit.clone(),
                false,
                true,
            )),
            ReplayCommand::Roof(RoofReplayCommand::expanded_shared(
                Arc::new(RoofHistoryEdit::direction(1, 0, 1)),
                false,
                true,
            )),
        ],
        timing: vec![Default::default(); 2],
    });
    buffer.push(ReplayBucket {
        entry_index: 1,
        undo: true,
        commands: vec![ReplayCommand::AutoClutter(
            AutoClutterReplayCmd::expanded_shared(edit, true, true),
        )],
        timing: vec![Default::default(); 1],
    });
    world.insert_resource(buffer);
    let mut schedule = Schedule::default();
    schedule.add_systems(replay_auto_clutter::replay_auto_clutter);
    schedule.run(&mut world);
    let buffer = world.resource::<ReplayBuffer>();
    assert_eq!(buffer.buckets[0].commands.len(), 1);
    assert!(matches!(
        buffer.buckets[0].commands[0],
        ReplayCommand::Roof(_)
    ));
    assert_eq!(buffer.buckets[1].commands.len(), 1);
    assert!(world.resource::<DeletedAutoClutter>().is_deleted(42));
}

#[test]
fn rmb_direct_entry_adds_history_unconditionally_and_rotates() {
    use super::rmb_autoclutter::{
        EvRightClickedAutoClutter, RmbAutoClutterDirect, RmbAutoClutterHit,
        get_clutter_rot_in_roof_space, handle_rmb_autoclutter,
    };
    use super::{AutoClutterRecord, registration::register_ui_clutter_resources};
    use crate::systems::mode_manager::{AppMode, ClutterMeshTy};

    let mut world = World::new();
    register_ui_clutter_resources(&mut world);
    world.insert_resource(empty_history());
    world.insert_resource(AppMode::default());
    // 入口 +0x18 门直达路径（跳过 probe）。
    world.insert_resource(RmbAutoClutterDirect(Some(AutoClutterRecord {
        clutter_id: 42,
        destination: [0x75dd10c75433c8db, 0xe038d51dca8bf380, 0xecd9fb9341459e0e],
        position: glam::Vec3::new(1.0, 2.0, 3.0),
        w: 1.0,
        x: 0.0,
        y: 0.0,
        z: 0.0,
        entry_angle: 0.0,
        roof_space: false,
        local_x: 1.0,
        local_y: 0.0,
        ty: ClutterMeshTy::Bench,
    })));
    world
        .resource_mut::<Events<RmbAutoClutterHit>>()
        .send(RmbAutoClutterHit(42));
    let mut schedule = Schedule::default();
    schedule.add_systems(handle_rmb_autoclutter::<()>);
    schedule.run(&mut world);
    assert_eq!(
        world.resource::<Events<super::ui_autoclutter::InstaCreateDeco>>().len(),
        1
    );
    // 原版无条件：start_input_recording → History::add(tag 0xd)；提交后入 ledger。
    world
        .resource_mut::<History<()>>()
        .finish_input_recording()
        .unwrap();
    let history = world.resource::<History<()>>();
    assert_eq!(history.ledger().entries.len(), 1);
    match &history.ledger().entries[0].edits[0] {
        EditType::AutoClutter(edit) => assert_eq!(
            **edit,
            crate::resources::history::auto_clutter_edit::AutoClutterEdit::Delete { id: 42 }
        ),
        other => panic!("unexpected edit {other:?}"),
    }
    // 旋转公式（identity 四元数、entry=0、非 roof → π/2）。
    let rotation = get_clutter_rot_in_roof_space(1.0, 0.0, 0.0, 0.0, 0.0, false, 1.0, 0.0);
    // identity 四元数 → rot1=π（或 −π，取决于 ±0）；base=−π/2；diff=3π/2 → 双角 = −π/2。
    assert!((rotation + core::f32::consts::FRAC_PI_2).abs() < 1e-5, "{rotation}");
}

#[test]
fn ui_convert_maintains_parent_hashmap_for_type_below_three() {
    use super::registration::register_ui_clutter_resources;
    use super::rmb_autoclutter::EvRightClickedAutoClutter;
    use super::ui_autoclutter::{
        AutoClutterChild, AutoClutterChildTable, DisplayChildAutoClutterInfo,
        ui_convert_autoclutter_to_decoclutter,
    };
    use crate::systems::mode_manager::{AppMode, ClutterMeshTy, EraserLayer};

    let mut world = World::new();
    register_ui_clutter_resources(&mut world);
    world.insert_resource(AppMode::Eraser(EraserLayer::Bulldoze));
    let info = DisplayChildAutoClutterInfo {
        mesh: [1, 2, 3, 4],
        transform: [5; 6],
        field: 9,
    };
    let mut table = AutoClutterChildTable::default();
    table.0.insert(
        10,
        vec![
            AutoClutterChild { parent_tag: 2, parent_key: 9, info },
            AutoClutterChild { parent_tag: 3, parent_key: 11, info },
        ],
    );
    world.insert_resource(table);
    world
        .resource_mut::<Events<EvRightClickedAutoClutter>>()
        .send(EvRightClickedAutoClutter { entity: 10 });
    let mut schedule = Schedule::default();
    schedule.add_systems(ui_convert_autoclutter_to_decoclutter);
    schedule.run(&mut world);
    let parents = world.resource::<crate::resources::clutter_meshes::DecoratorAutoClutterChildren>();
    assert_eq!(parents.parent_of(9).unwrap().len(), 1);
    assert_eq!(parents.parent_of(9).unwrap()[0].mesh, [1, 2, 3, 4]);
    assert!(parents.parent_of(11).is_none()); // parent_tag >= 3 跳过
}

#[test]
fn debug_spawn_single_uses_payload_rotation_and_batch_uses_zero() {
    use super::debug_spawn_clutter::{
        DebugSpawnAllClutterCmd, DebugSpawnClutterCmd, SpawnAutoClutter, debug_spawn_all_clutter,
    };
    use super::registration::register_ui_clutter_resources;
    use crate::resources::terrain_height::{TerrainHeightSsbo, TerrainHeightsData};
    use crate::systems::mode_manager::ClutterMeshTy;

    let mut world = World::new();
    register_ui_clutter_resources(&mut world);
    world.insert_resource(empty_history());
    world.insert_resource(crate::systems::mode_manager::AppMode::default());
    world.insert_resource(TerrainHeightsData::<TerrainHeightSsbo>::new());
    world
        .resource_mut::<Events<DebugSpawnClutterCmd>>()
        .send(DebugSpawnClutterCmd {
            ty: ClutterMeshTy::PlantPotV1,
            position: glam::Vec3::new(1.0, 2.0, 3.0),
            rotation: 1.5,
        });
    let mut schedule = Schedule::default();
    schedule.add_systems(debug_spawn_all_clutter::<()>);
    schedule.run(&mut world);
    let mut events = world.resource_mut::<Events<SpawnAutoClutter>>();
    let spawned: Vec<_> = events.drain().collect();
    assert_eq!(spawned.len(), 1);
    assert_eq!(spawned[0].rotation, 1.5);
    assert_eq!(spawned[0].position, glam::Vec3::new(1.0, 2.0, 3.0));
    // 批量：42 条，rotation=0。
    world
        .resource_mut::<Events<DebugSpawnAllClutterCmd>>()
        .send(DebugSpawnAllClutterCmd);
    schedule.run(&mut world);
    let mut events = world.resource_mut::<Events<SpawnAutoClutter>>();
    let spawned: Vec<_> = events.drain().collect();
    assert_eq!(spawned.len(), 42);
    assert!(spawned.iter().all(|event| event.rotation == 0.0));
}

#[test]
fn show_pins_applies_type_gate_geometry_and_dedup() {
    use super::decoclutter_show_pins::{
        PinAnimationTable, PinHitDedup, PinVisibilityToggle, PinnedDecoClutter,
        PinnedDecoClutterBuffers, PinnedDecoGeometry, PinnedDecoGeometryTable,
        ShowPinnedDecoClutter, decoclutter_show_pins,
    };
    use super::registration::register_ui_clutter_resources;
    use utils::geometry::{Circle2d, Rectangle2d, Rectangle3d};

    let mut world = World::new();
    register_ui_clutter_resources(&mut world);
    let footprint = Rectangle3d::new(
        glam::Vec3::ZERO,
        glam::Vec3::new(2.0, 2.0, 2.0),
        glam::Vec2::X,
    );
    let trigger3d = Rectangle3d::new(
        glam::Vec3::new(1.0, 0.0, 1.0),
        glam::Vec3::new(10.0, 4.0, 10.0),
        glam::Vec2::X,
    );
    let trigger2d = Rectangle2d::new(
        glam::Vec2::new(1.0, 1.0),
        glam::Vec2::new(10.0, 10.0),
        glam::Vec2::X,
    );
    let base = PinnedDecoGeometry {
        type_byte: 0x40,
        footprint,
        axis: glam::Vec2::X,
        position: glam::Vec3::ZERO,
        trigger_kind: 1,
        trigger3d,
        trigger2d,
        circle: Circle2d::new(glam::Vec2::ZERO, 1.0),
        ..Default::default()
    };
    let mut geo = PinnedDecoGeometryTable::default();
    geo.0.insert(10, base); // 命中
    geo.0.insert(11, PinnedDecoGeometry { type_byte: 0x38, ..base }); // 类型门跳过
    world.insert_resource(geo);
    world.insert_resource(PinnedDecoClutter([10, 11].into_iter().collect()));
    world.send_event(ShowPinnedDecoClutter {
        show: true,
        ..Default::default()
    });
    world.init_resource::<PinAnimationTable>();
    world.init_resource::<PinnedDecoClutterBuffers>();
    world.init_resource::<PinHitDedup>();
    let mut schedule = Schedule::default();
    schedule.add_systems(decoclutter_show_pins);
    schedule.run(&mut world);
    let mut events = world.resource_mut::<Events<PinVisibilityToggle>>();
    let toggles: Vec<_> = events.drain().collect();
    assert_eq!(toggles.len(), 1);
    assert_eq!(toggles[0].entity, 10);
    assert!(toggles[0].visible);
}

#[test]
fn focus_decoclutter_gates_disc_three_and_decorator_type() {
    use super::registration::register_ui_clutter_resources;
    use super::ui_focus_decoclutter::{
        DecoClutterFocusState, DecoClutterFocusUi, RequestMoveClutter, RequestRotateDecorator,
        ui_focus_decoclutter,
    };
    use crate::resources::walls::decorator::{DecoratorId, DecoratorInfo};
    use crate::resources::walls::decorator_storage::{DecoratorStorage, DecoratorStorageAddr};
    use crate::systems::audio::PlayDecoratorSoundCmd;
    use crate::systems::mode_manager::RequestAppModeChange;
    use crate::systems::ui::signifiers::signifier_create::UiSignifierStream;

    let mut world = World::new();
    register_ui_clutter_resources(&mut world);
    world.init_resource::<Events<PlayDecoratorSoundCmd>>();
    world.init_resource::<Events<RequestAppModeChange>>();
    world.init_resource::<UiSignifierStream>();
    let mut storage = DecoratorStorage::default();
    let addr = DecoratorStorageAddr::Terrain;
    storage
        .get_or_insert_default(addr)
        .insert(DecoratorId(7), DecoratorInfo { ty: 0x40, ..Default::default() });
    storage.decorator_to_storage.insert(DecoratorId(7), addr);
    world.insert_resource(storage);
    world.insert_resource(DecoClutterFocusState { tag: 3, decorator_id: 7 });
    world.insert_resource(DecoClutterFocusUi {
        click_phase: 3,
        rotate_active: true,
        move_active: false,
        position: glam::Vec3::new(1.0, 2.0, 3.0),
    });
    let mut schedule = Schedule::default();
    schedule.add_systems(ui_focus_decoclutter);
    schedule.run(&mut world);
    assert_eq!(world.resource::<Events<PlayDecoratorSoundCmd>>().len(), 1);
    assert_eq!(world.resource::<Events<RequestAppModeChange>>().len(), 1);
    assert_eq!(world.resource::<Events<RequestRotateDecorator>>().len(), 1);
    assert!(world.resource::<Events<RequestMoveClutter>>().is_empty());
    // 类型门：ty=0x38 → d=0 (<8, !=6) → 全部早退。先清空上一帧事件。
    world.resource_mut::<Events<PlayDecoratorSoundCmd>>().clear();
    world.resource_mut::<Events<RequestAppModeChange>>().clear();
    world.resource_mut::<Events<RequestRotateDecorator>>().clear();
    world.resource_mut::<DecoratorStorage>().decorator_info_mut(DecoratorId(7)).unwrap().ty = 0x38;
    schedule.run(&mut world);
    assert_eq!(world.resource::<Events<PlayDecoratorSoundCmd>>().len(), 0);
}
