use super::*;
use crate::asset_libraries::ssbo_library::AssetSsboLibrary;
use crate::resources::history::transactions::ObservedHistoryLedger;
use crate::resources::active_raster_stroke::ActiveRasterStrokesStorage;
use crate::resources::brush_sizes::BrushSizes;
use crate::resources::cursor_terrain_raycast::CursorTerrainRaycast;
use crate::resources::glade_border::BorderShape;
use crate::resources::glade_border::DisplayOutOfBorder;
use crate::resources::glade_border::FeedbackInputOutOfBorder;
use crate::resources::glade_border::GladeBorder;
use crate::resources::history::EditType;
use crate::resources::history::History;
use crate::resources::walls::decorator_archivist::DecoratorArchivist;
use crate::resources::walls::decorator_archivist::DecoratorCullPolicy;
use crate::resources::walls::EraserSizeMultiplier;
use crate::resources::world_raster::DirtyTilePublisher;
use crate::resources::world_raster_defs::GardenRaster;
use crate::resources::world_raster_defs::GardenRasterStagingSsbo;
use crate::systems::mode_manager::AppMode;
use crate::systems::ui::resources::ui_state::UiInteraction;
use crate::systems::ui::resources::ui_state::UiState;
use crate::systems::ui::resources::ui_state::WidgetId;
use glam::Vec2;
use std::collections::{HashMap, HashSet};

#[test]
fn garden_replay_feedback_only_routes_garden_to_draw() {
    use crate::systems::history::replay_raster::{RasterDomain, RasterReplayFeedback};
    let mut world = World::new();
    register_garden_events(&mut world);
    let position = Vec3::new(1.0, 2.0, 3.0);
    for domain in [RasterDomain::Garden, RasterDomain::Water] {
        world.send_event(RasterReplayFeedback { domain, position, radius: 2.5, replay: true });
    }
    let mut schedule = Schedule::default();
    schedule.add_systems(route_garden_replay_feedback);
    schedule.run(&mut world);
    let draws = world.resource::<Events<GardenDrawEvent>>();
    let mut reader = draws.get_cursor();
    let events: Vec<_> = reader.read(draws).collect();
    assert_eq!(events.len(), 1);
    assert_eq!((events[0].position, events[0].radius, events[0].observed_flag), (position, 2.5, true));
    assert!(world.resource::<Events<GardenEraseEvent>>().is_empty());
}

#[test]
#[should_panic]
fn garden_record_without_active_history_fails() {
    let mut world = World::new();
    world.init_resource::<Events<record_garden_history::RecordGardenEditToHistoryCmd>>();
    world.insert_resource(ActiveRasterStrokesStorage::default());
    world.insert_resource(History::<()>::from_observed(ObservedHistoryLedger {
        entries: vec![],
        cursor: -1,
        undo_anchor: -1,
        resolved_snapshots: vec![],
        observed_count_58: 0,
        mutation_nonce: 0,
    }));
    world.send_event(record_garden_history::RecordGardenEditToHistoryCmd);
    let mut schedule = Schedule::default();
    schedule.add_systems(record_garden_history::record_garden_history::<()>);
    schedule.run(&mut world);
}

#[test]
fn garden_drag_border_resume_commit_records_the_live_raster_once() {
    let mut world = World::new();
    register_garden_events(&mut world);
    world.insert_resource(AppMode::Garden);
    let widget = WidgetId::clearing_canvas("garden");
    world.insert_resource(UiState::new(UiInteraction {
        native_tag: 2,
        native_phase: 3,
        widget,
    }));
    world.insert_resource(BrushSizes {
        path: 0.8,
        water: 2.0,
        garden: 3.0,
    });
    world.insert_resource(EraserSizeMultiplier(1.0));
    world.insert_resource(GladeBorder(BorderShape::Circle(Circle2d::new(
        Vec2::ZERO,
        10.0,
    ))));
    let mut cursor = CursorTerrainRaycast::new();
    cursor.update(Some(Vec3::new(-2.0, 0.0, 0.0)));
    world.insert_resource(cursor);
    world.insert_resource(ActiveRasterStrokesStorage::default());
    world.insert_resource(DecoratorArchivist::<()> {
        walls: HashMap::new(),
        special1: None,
        special2: None,
        collision_a: HashMap::new(),
        collision_b: HashMap::new(),
        culling: DecoratorCullPolicy {
            allowed_ids: HashSet::new(),
            allow_any: false,
        },
    });
    world.insert_resource(GardenRaster::new(DirtyTilePublisher::unique()));
    // Existing cfg(test) adapter allocates mapped host bytes. This verifies the
    // real staging writes, not a graphics driver or GPU execution.
    let library = AssetSsboLibrary::new();
    library.add_new::<GardenRasterStagingSsbo>(&crate::render::test_device::TestDevice(0));
    world.insert_resource(library);
    let mut history: History<()> = History::<()>::from_observed(ObservedHistoryLedger {
        entries: vec![],
        cursor: -1,
        undo_anchor: -1,
        resolved_snapshots: vec![],
        observed_count_58: 0,
        mutation_nonce: 0,
    });
    history.start_input_recording(17).unwrap();
    world.insert_resource(history);
    let mut schedule = Schedule::default();
    add_garden_cpu_systems::<(), ()>(&mut schedule);
    schedule.run(&mut world); // Begin.
    let canvas = world
        .resource::<UiState>()
        .clearing_canvas_params(widget)
        .unwrap();
    assert_eq!((canvas.argument_79, canvas.layer), (5, 3));
    assert_eq!(canvas.extent, Vec2::splat(130.0));
    assert!(world.resource::<DecoratorArchivist<()>>().culling.allow_any);

    world
        .resource_mut::<UiState>()
        .replace_interaction(UiInteraction {
            native_tag: 2,
            native_phase: 4,
            widget,
        });
    world
        .resource_mut::<CursorTerrainRaycast>()
        .update(Some(Vec3::new(20.0, 0.0, 0.0)));
    schedule.run(&mut world); // Outside: stays in extension state without drawing.
    assert_eq!(world.resource::<Events<DisplayOutOfBorder>>().len(), 1);
    assert_eq!(
        world.resource::<Events<FeedbackInputOutOfBorder>>().len(),
        1
    );
    assert!(world.resource::<Events<GardenDrawEvent>>().is_empty());
    world
        .resource_mut::<CursorTerrainRaycast>()
        .update(Some(Vec3::new(2.0, 0.0, 0.0)));
    schedule.run(&mut world); // Reenter: resumes the existing stroke.
    assert_eq!(world.resource::<Events<GardenDrawEvent>>().len(), 1);
    assert_eq!(
        world.resource::<Events<GardenPinnedClutterCircle>>().len(),
        1
    );
    let pins = world.resource::<Events<crate::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter>>();
    let mut reader = pins.get_cursor();
    assert!(matches!(reader.read(pins).last(), Some(
        GardenPinnedClutterCircle::Circle { vertical: [0.0, 1.5], containment: true, exclude_wall: None, .. }
    )));
    assert!(world
        .resource::<GardenRaster>()
        .scratch()
        .iter()
        .any(|&value| value != 0));
    assert!(world
        .resource::<AssetSsboLibrary>()
        .get::<GardenRasterStagingSsbo>()
        .unwrap()
        .mapped
        .iter()
        .any(|value| value.0 != 0));
    world
        .resource_mut::<UiState>()
        .replace_interaction(UiInteraction {
            native_tag: 2,
            native_phase: 5,
            widget,
        });
    schedule.run(&mut world); // Commit; the same frame's record consumer takes slot 5.
    assert_eq!(
        world.resource::<History<()>>().pending().unwrap().edits.len(),
        1
    );
    let EditType::Garden(stroke) = &world.resource::<History<()>>().pending().unwrap().edits[0] else {
        panic!("wrong edit owner")
    };
    assert_eq!(
        (stroke.flags, stroke.strength, stroke.radius, stroke.draw),
        (1, 0.5, 3.0, true)
    );
    assert!(stroke.points.len() >= 2);
    assert!(world
        .resource::<GardenRaster>()
        .composite()
        .iter()
        .any(|&value| value != 0));
    assert!(world.resource::<ActiveRasterStrokesStorage>()[5]
        .resampled()
        .is_empty());
    schedule.run(&mut world); // EventReader must not rerecord the old notification.
    assert_eq!(
        world.resource::<History<()>>().pending().unwrap().edits.len(),
        1
    );
    assert!(world
        .resource::<Events<crate::systems::history::recording::HistoryRecordingFailed>>()
        .is_empty());

    // The save owner serializes committed MultiEdits, not the pending edit.
    // Keep the identity generated by the real transaction, and use the saved
    // ledger's actual cursor, undo boundary and snapshot references.
    use crate::resources::{
        history::{serialized::SavedMultiEdit, ReplayBuffer, ReplayCommand},
        time::GameTime,
        world_raster_defs::{PathRaster, Cobblestone1Raster, Cobblestone2Raster,
            Cobblestone3Raster, WaterRaster},
    };
    use crate::systems::{
        history::{self, replay_raster::{self, RasterDomain, RasterReplayFeedback}},
        mode_manager::HistoryCommand,
        save::load_game::{emit_load_history_commands, HistorySectionJson, LoadGameManager, LoadGameState},
    };
    let expected_pixels = world.resource::<GardenRaster>().composite().to_vec();
    let library = world.resource::<AssetSsboLibrary>();
    let expected_staging = bytemuck::cast_slice::<_, u8>(
        library.get::<GardenRasterStagingSsbo>().unwrap().mapped,
    ).to_vec();
    let commit = world.resource_mut::<History<()>>().finish_input_recording().unwrap().unwrap();
    assert_eq!(commit.entry_index, 0);
    let history = world.resource::<History<()>>();
    let ledger = history.ledger();
    assert_eq!((ledger.cursor, ledger.undo_anchor, ledger.entries.len()), (0, -1, 1));
    assert!(ledger.resolved_snapshots.is_empty());
    let original_stroke = match &ledger.entries[0].edits[0] {
        EditType::Garden(stroke) => stroke.clone(),
        _ => panic!("committed edit lost its garden domain"),
    };
    let section = serde_json::json!({
        "chain_index": ledger.cursor,
        "history_start": ledger.undo_anchor,
        "snapshots": [],
        "edit_chain": ledger.entries.iter().cloned().map(SavedMultiEdit).collect::<Vec<_>>(),
    });
    let garden = &section["edit_chain"][0]["edits"][0]["Garden"];
    assert_eq!(garden["brush_size"], 3.0);
    assert_eq!(garden["is_additive"], true);
    assert_eq!(garden["blend_mode"]["Add"]["opacity"], 0.5);
    assert_eq!(garden["points"].as_array().unwrap().len(), original_stroke.points.len());
    let encoded = serde_json::to_vec(&section).unwrap();
    let loaded: HistorySectionJson = serde_json::from_slice(&encoded).unwrap();
    assert_eq!((loaded.chain_index, loaded.history_start, loaded.edit_chain.len()), (0, -1, 1));
    assert!(loaded.snapshots.is_empty());
    let entries: Vec<_> = loaded.edit_chain.into_iter().map(|entry| entry.0).collect();
    let EditType::Garden(restored_stroke) = &entries[0].edits[0] else {
        panic!("decoded MultiEdit lost its garden domain");
    };
    assert_eq!(**restored_stroke, *original_stroke);
    assert!(!std::sync::Arc::ptr_eq(restored_stroke, &original_stroke));

    // load_game's no-snapshot branch installs cursor -1 and queues quiet,
    // instant redo to the saved target. Run the same ECS consumers on the new
    // History and its GardenRaster/staging owner, not a standalone stroke.
    let mut restored_world = World::new();
    history::register_history_events(&mut restored_world);
    restored_world.insert_resource(History::<()>::from_observed(ObservedHistoryLedger {
        observed_count_58: entries.len(),
        entries,
        cursor: -1,
        undo_anchor: loaded.history_start,
        resolved_snapshots: Vec::new(),
        mutation_nonce: ledger.mutation_nonce,
    }));
    restored_world.insert_resource(ReplayBuffer::new(1.0));
    restored_world.init_resource::<GameTime>();
    restored_world.insert_resource(PathRaster::new(DirtyTilePublisher::unique()));
    restored_world.insert_resource(Cobblestone1Raster::new(DirtyTilePublisher::unique()));
    restored_world.insert_resource(Cobblestone2Raster::new(DirtyTilePublisher::unique()));
    restored_world.insert_resource(Cobblestone3Raster::new(DirtyTilePublisher::unique()));
    restored_world.insert_resource(WaterRaster::new(DirtyTilePublisher::unique()));
    restored_world.insert_resource(GardenRaster::new(DirtyTilePublisher::unique()));
    let restored_library = AssetSsboLibrary::new();
    restored_library.add_new::<GardenRasterStagingSsbo>(&crate::render::test_device::TestDevice(0));
    restored_world.insert_resource(restored_library);
    let mut manager = LoadGameManager::new();
    manager.state = LoadGameState::ApplyCounter;
    manager.progress_counter = loaded.chain_index.wrapping_sub(restored_world.resource::<History<()>>().ledger().cursor);
    emit_load_history_commands(&mut manager, &mut restored_world.resource_mut::<Events<HistoryCommand>>());
    let mut history_schedule = Schedule::default();
    history_schedule.add_systems(history::history_process_system::<()>);
    history_schedule.run(&mut restored_world);
    assert_eq!(restored_world.resource::<History<()>>().ledger().cursor, loaded.chain_index);
    assert!(restored_world.resource::<Events<history::HistoryReplayFailed>>().is_empty());
    let replay = restored_world.resource::<ReplayBuffer>();
    assert_eq!(replay.buckets.len(), 1);
    assert!(matches!(&replay.buckets[0].commands[..], [ReplayCommand::Raster(command)]
        if command.domain == RasterDomain::Garden && !command.undo && command.duration == 0.0));
    let mut raster_schedule = Schedule::default();
    raster_schedule.add_systems(replay_raster::replay_rasters::<()>);
    raster_schedule.run(&mut restored_world);
    assert!(restored_world.resource::<ReplayBuffer>().buckets[0].commands.is_empty());
    let raster = restored_world.resource::<GardenRaster>();
    assert_eq!(raster.composite(), expected_pixels);
    assert!(raster.published());
    let restored_library = restored_world.resource::<AssetSsboLibrary>();
    let restored_staging = bytemuck::cast_slice::<_, u8>(
        restored_library.get::<GardenRasterStagingSsbo>().unwrap().mapped,
    );
    assert_eq!(restored_staging, expected_staging);
    let feedback = restored_world.resource::<Events<RasterReplayFeedback>>();
    let mut reader = feedback.get_cursor();
    let feedback: Vec<_> = reader.read(feedback).collect();
    assert_eq!(feedback.len(), 1);
    assert_eq!(feedback[0].domain, RasterDomain::Garden);
    assert!(feedback[0].replay);
}
