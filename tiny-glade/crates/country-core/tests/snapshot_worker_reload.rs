//! Actual snapshot encoder, asynchronous command dispatcher, filesystem decoder
//! and canonical World restoration. No installed assets or GPU substitutes.
use bevy_ecs::{event::Events, world::World};
use country_core::{
    resources::{
        color_id::{GateColorIds, HalftimberColorIds, WallColorIds},
        history::MaybePendingSnapshotId,
        session::SessionId,
        stairs::{stairs_removed_supports::StairsRemovedSupports, stairs_state::StairsState},
        terrain_editing::TerrainEditingData,
        terrain_height::{TerrainHeightSsbo, TerrainHeightsData},
        walls::{InnerWalls, WallId, WallState, decorator::DecoratorIdGen,
            decorator_storage::DecoratorStorage},
        world_raster::{DirtyTilePublisher, UploadWorldRasterBuffersToStagingCmd},
        world_raster_defs::{Cobblestone1Raster, Cobblestone2Raster, Cobblestone3Raster,
            GardenRaster, PathRaster, WaterRaster},
    },
    systems::{
        save::{
            save_game::{AsyncSaveCmd, AutoSaveState, GameSaveSections, SerializedJson},
            save_thread::{CurrentSaveFile, UnsavedFileList, execute_save_cmd},
            snapshot::{SnapshotDomainData, SnapshotLoadResources, capture_game_snapshot_save_data,
                init_game_state_from_snapshot, load_snapshot_file, take_stairs_snapshot},
        },
        terrain::draw_terrain_stroke::EvTerrainStrokeChanged,
        ui_clutter::DeletedAutoClutter,
    },
};

fn target_world() -> World {
    let mut world = World::new();
    world.insert_resource(InnerWalls::new());
    world.insert_resource(DecoratorStorage::default());
    world.insert_resource(DecoratorIdGen::default());
    world.insert_resource(TerrainEditingData(tg_terrain::stroke::TerrainEditingData::default()));
    world.insert_resource(TerrainHeightsData::<TerrainHeightSsbo>::new());
    world.insert_resource(WallColorIds::new());
    world.insert_resource(GateColorIds::new());
    world.insert_resource(HalftimberColorIds::new());
    world.insert_resource(StairsState::new());
    world.insert_resource(StairsRemovedSupports::default());
    world.insert_resource(DeletedAutoClutter([999].into_iter().collect()));
    world.insert_resource(PathRaster::new(DirtyTilePublisher::unique()));
    world.insert_resource(Cobblestone1Raster::new(DirtyTilePublisher::unique()));
    world.insert_resource(Cobblestone2Raster::new(DirtyTilePublisher::unique()));
    world.insert_resource(Cobblestone3Raster::new(DirtyTilePublisher::unique()));
    world.insert_resource(WaterRaster::new(DirtyTilePublisher::unique()));
    world.insert_resource(GardenRaster::new(DirtyTilePublisher::unique()));
    world.init_resource::<Events<EvTerrainStrokeChanged>>();
    world.init_resource::<Events<UploadWorldRasterBuffersToStagingCmd>>();
    world
}

#[test]
fn persisted_snapshot_restores_deleted_clutter_and_publishes_real_terrain_work() {
    let source: serde_json::Value = serde_json::from_str(include_str!(
        "fixtures/original-intro-wall-states-b.json")).unwrap();
    let wall: WallState = serde_json::from_value(source[0]["state"].clone()).unwrap();
    let mut terrain = TerrainEditingData(tg_terrain::stroke::TerrainEditingData::default());
    terrain.new_stroke([2., 3., 4.]).unwrap().unwrap();
    let expected_strokes: Vec<_> = terrain.strokes.keys().copied().collect();
    let mut stairs = StairsState::new();
    stairs.make_backup();
    let mut heights = TerrainHeightsData::<TerrainHeightSsbo>::new();
    heights.heights_mut()[17] = 0xabcd;
    heights.heights_mut()[512] = 0x1234;
    let height_bytes = bytemuck::cast_slice::<u16, u8>(heights.heights()).to_vec();
    let mut path = PathRaster::new(DirtyTilePublisher::unique());
    path.composite_mut()[17] = 231;
    let expected_path = path.current_buffer().to_vec();
    let domain = SnapshotDomainData::collect(
        [], [(WallId(1), wall)], [73, u64::MAX], Some(take_stairs_snapshot(&stairs)),
        [], ([], [], []), terrain, [("PathRaster", expected_path.clone())], [],
    ).with_terrain_and_supports(height_bytes, -2.7..12.5, StairsRemovedSupports::default());
    let snapshot = capture_game_snapshot_save_data(SessionId::new(), 0, -1, domain);
    let pending = MaybePendingSnapshotId::new();
    let root = tempfile::tempdir().unwrap();
    let current = CurrentSaveFile::create(root.path(), GameSaveSections {
        values: std::array::from_fn(|_| SerializedJson::new(&serde_json::Value::Null).unwrap()),
    }, &[]);
    let (send, receive) = crossbeam_channel::unbounded();
    let worker = std::thread::spawn(move || {
        let mut current = Some(current);
        let unsaved = UnsavedFileList::default();
        while let Ok(command) = receive.recv() {
            execute_save_cmd(&mut current, command, &unsaved);
        }
        assert_eq!(unsaved.len(), 0);
    });
    send.send(AsyncSaveCmd::CaptureSnapshot { snapshot, pending: pending.clone() }).unwrap();
    let mut autosave = AutoSaveState::default();
    autosave.attach_command_sender(send);
    autosave.flush_synchronously().unwrap();
    let id = pending.snapshot_id().expect("worker must publish the real persisted snapshot ID");
    let snapshot_path = root.path().join("snapshots").join(format!("{}.snapshot", id.0));
    let loaded = load_snapshot_file(&snapshot_path).unwrap();
    let metadata = loaded.sections.iter().find(|(name, _)| *name == "AutoClutter").unwrap();
    assert_eq!(metadata.1, serde_json::json!([73, u64::MAX]));
    let mut world = target_world();
    let report = init_game_state_from_snapshot(&mut world,
        &SnapshotLoadResources { loaded: Some(loaded) }).unwrap();
    assert!(report.gaps.is_empty(), "snapshot restore gaps: {:?}", report.gaps);
    assert_eq!(report.walls, 1);
    assert_eq!(report.terrain_strokes, 1);
    assert!(report.terrain_heights);
    assert_eq!(report.auto_clutter, 2);
    assert_eq!(world.resource::<DeletedAutoClutter>().0, [73, u64::MAX].into_iter().collect::<std::collections::HashSet<_>>());
    assert!(!world.resource::<DeletedAutoClutter>().is_deleted(999));
    assert_eq!(world.resource::<PathRaster>().current_buffer(), expected_path);
    assert_eq!(world.resource::<TerrainHeightsData<TerrainHeightSsbo>>().heights()[17], 0xabcd);
    assert_eq!(world.resource::<TerrainHeightsData<TerrainHeightSsbo>>().heights()[512], 0x1234);
    let changed: Vec<_> = world.resource_mut::<Events<EvTerrainStrokeChanged>>()
        .drain().map(|event| event.0).collect();
    assert_eq!(changed, expected_strokes);
    assert_eq!(world.resource::<Events<UploadWorldRasterBuffersToStagingCmd>>().len(), 1);
    drop(autosave);
    worker.join().unwrap();
}
