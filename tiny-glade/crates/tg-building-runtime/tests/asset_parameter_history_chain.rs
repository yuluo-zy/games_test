use bevy_ecs::prelude::*;
use country_core::resources::walls::decorator::placement::WallDecoratorRequest;
use country_core::resources::history::building_transactions::RoofHistoryEdit;
use country_core::resources::history::building_transactions::RoofVisualHistoryRecord;
use country_core::resources::walls::wall_holes::opening_rules::OpeningFamily;
use country_core::resources::roofs::creation::RoofCreateRequestObserved;
use country_core::resources::roofs::direction::RoofDirectionRequest;
use country_core::resources::walls::publication_source::RawWallPublicationSource;
use country_core::resources::walls::publication_source::TerrainHeightGrid512;
use country_core::resources::walls::publication_fields::WallId;
use country_core::resources::walls::publication_fields::WallShapeSnapshot;
use country_core::resources::roofs::geometry::edit_rules::RoofEditEvent;
use country_core::resources::roofs::geometry::surface::Curve2;
use std::{path::PathBuf, sync::Arc};
use tg_building_runtime::{
    decorator_assets::*, decorator_placement::*, history::*, openings::*, roof_edits::*, *,
};

fn emit<T: Event>(app: &mut bevy_app::App, event: T) {
    app.world_mut().resource_mut::<Events<T>>().send(event);
}
fn configure_clear_path_resources(app: &mut bevy_app::App) {
    use country_core::systems::wall::wall_state::path_preparation::PathRasters512;
    emit(
        app,
        tg_building_runtime::wall_paths::SetWallPathRasters(
            PathRasters512::from_encoded(
                std::array::from_fn(|_| Arc::from(vec![0; 512 * 512])),
                true,
            )
            .unwrap(),
        ),
    );
}
fn step(app: &mut bevy_app::App, delta: f32) {
    app.world_mut().resource_mut::<BuildingFrame>().delta_time = delta;
    app.update();
    assert!(
        app.world().resource::<BuildingReport>().faults.is_empty(),
        "{:?}",
        app.world().resource::<BuildingReport>().faults
    );
}
fn source() -> RawWallPublicationSource {
    let mut shape = [0; 28];
    shape[..4].copy_from_slice(&1u32.to_le_bytes());
    for (index, value) in [1f32, 0., 0., 0., 6., 8.].iter().enumerate() {
        shape[4 + index * 4..8 + index * 4].copy_from_slice(&value.to_le_bytes());
    }
    RawWallPublicationSource {
        curve: Curve2::try_new(
            vec![[-3., -4.], [-3., 4.], [3., 4.], [3., -4.], [-3., -4.]],
            false,
        )
        .unwrap(),
        curve_kind_tag: 1,
        terrain_floor: 2.,
        shape: WallShapeSnapshot::from_observed(shape),
        style: 0,
        extra: 3.,
        special: false,
        observed_path_scalar: 0.,
        clutter_version: 0,
        observed_source_flag_64: 0,
    }
}
fn scene() -> (bevy_app::App, WallId, Entity) {
    let mut app = building_app();
    configure_clear_path_resources(&mut app);
    let wall = WallId(40);
    emit(
        &mut app,
        SetTerrainGrid {
            grid: TerrainHeightGrid512 {
                encoded: Arc::from(vec![0; 512 * 512]),
                minimum_y: 0.,
                maximum_y: 0.,
                sampling_fix: true,
            },
            affected_walls: vec![],
        },
    );
    emit(
        &mut app,
        UpsertRawWall {
            wall,
            source: source(),
        },
    );
    let mut record = [0; 64];
    record[8..16].copy_from_slice(&wall.0.to_le_bytes());
    for (offset, value) in [(40, 0.5f32), (44, 0.5), (48, 1.), (52, 0.5)] {
        record[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
    }
    emit(
        &mut app,
        CreateRoof(RoofCreateRequestObserved::from_observed(record)),
    );
    step(&mut app, 0.5);
    let entity = app.world().resource::<WallRoofMap>().entity(wall).unwrap();
    (app, wall, entity)
}
fn set_height(wall: WallId, value: f32) -> EditRoof {
    let mut bytes = [0; 64];
    bytes[8..12].copy_from_slice(&1u32.to_le_bytes());
    bytes[12..16].copy_from_slice(&value.to_le_bytes());
    bytes[32..40].copy_from_slice(&wall.0.to_le_bytes());
    EditRoof(RoofEditEvent::from_observed(&bytes))
}
fn height(app: &bevy_app::App, entity: Entity) -> f32 {
    f32::from_le_bytes(
        app.world().get::<Roof>(entity).unwrap().0[0x30..0x34]
            .try_into()
            .unwrap(),
    )
}

#[test]
fn shipped_assets_world_projection_rebuild_and_removal_reach_opening_storage() {
    let (mut app, wall, _) = scene();
    let assets = std::env::var_os("TG_B_ASSETS")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/assets"));
    app.world_mut()
        .resource_mut::<DecoratorAssetLibrary>()
        .load_b_plain_wall_assets(&assets)
        .unwrap();
    assert_eq!(
        app.world()
            .resource::<DecoratorAssetLibrary>()
            .collisions
            .len(),
        12
    );
    let id = OpeningId(5);
    emit(
        &mut app,
        PlaceWallOpening {
            id,
            wall,
            request: WallDecoratorRequest {
                family: OpeningFamily::CottageWindow,
                rank: 1,
                world_position: [-3., 2., 0.],
            },
        },
    );
    step(&mut app, 0.);
    let entity = app.world().resource::<OpeningEntities>().0[&id];
    let authority = app.world().get::<OpeningAuthority>(entity).unwrap();
    assert_eq!(authority.input.center, [4., 2.]);
    let initial_holes = app.world().resource::<WallOpenings>().storage[&wall]
        .finalized_holes()
        .unwrap()
        .to_vec();
    assert!(!initial_holes.is_empty());
    let generation = app.world().get::<OpeningState>(entity).unwrap().generation;
    step(&mut app, 0.);
    assert_eq!(
        app.world().get::<OpeningState>(entity).unwrap().generation,
        generation
    );
    assert!(
        app.world()
            .resource::<WallOpenings>()
            .geometry_rebuilt
            .is_empty()
    );
    emit(
        &mut app,
        PlaceWallOpening {
            id,
            wall,
            request: WallDecoratorRequest {
                family: OpeningFamily::GothicBalconyDoor,
                rank: 2,
                world_position: [-3., 2., 2.],
            },
        },
    );
    step(&mut app, 0.);
    assert_eq!(app.world().resource::<OpeningEntities>().0[&id], entity);
    assert_ne!(
        app.world().resource::<WallOpenings>().storage[&wall]
            .finalized_holes()
            .unwrap(),
        initial_holes
    );
    emit(&mut app, RemoveWall(wall));
    step(&mut app, 0.5);
    assert!(
        app.world()
            .get::<OpeningState>(entity)
            .unwrap()
            .raw_holes
            .is_empty()
    );
    assert!(
        !app.world()
            .resource::<WallOpenings>()
            .storage
            .contains_key(&wall)
    );
    emit(
        &mut app,
        UpsertRawWall {
            wall,
            source: source(),
        },
    );
    step(&mut app, 0.);
    assert_eq!(
        app.world()
            .get::<OpeningState>(entity)
            .unwrap()
            .resolved_host,
        Some(wall)
    );
    emit(&mut app, RemoveOpening(id));
    step(&mut app, 0.);
    assert!(app.world().get_entity(entity).is_err());
    assert!(app.world().resource::<WallOpeningIntents>().0.is_empty());
    assert!(
        app.world().resource::<WallOpenings>().storage[&wall]
            .finalized_holes()
            .unwrap()
            .is_empty()
    );
}

#[test]
fn parameter_type_crossing_replaces_tiles_and_recomputes_gable_then_restores() {
    let (mut app, wall, entity) = scene();
    let before = app.world().resource::<PublicWalls>().walls[&wall]
        .arch_segment_heights
        .clone();
    assert!(app.world().get::<RoofCpuTiles>(entity).is_some());
    emit(&mut app, set_height(wall, 0.));
    step(&mut app, 0.);
    assert!(app.world().get::<RoofCpuTiles>(entity).is_none());
    let flat = &app.world().resource::<PublicWalls>().walls[&wall];
    assert_eq!(flat.arch_segment_heights, flat.base_arch_segment_heights);
    assert_eq!(
        app.world().resource::<RoofTypeTransitions>().applied.len(),
        1
    );
    assert_eq!(app.world().resource::<BuildingReport>().sounds[0].kind, 6);
    emit(&mut app, set_height(wall, 0.5));
    step(&mut app, 0.);
    assert!(app.world().get::<RoofCpuTiles>(entity).is_some());
    assert_eq!(
        app.world().resource::<PublicWalls>().walls[&wall].arch_segment_heights,
        before
    );
    assert_eq!(app.world().resource::<BuildingReport>().sounds[0].kind, 0);
    assert!(
        app.world()
            .resource::<BuildingHistoryRequests>()
            .0
            .is_empty()
    );
}

#[test]
fn every_visual_change_reaches_replay_and_undo_restores_the_batch_start_reach_actual_roof_without_recording_echo()
 {
    let (mut app, wall, entity) = scene();
    emit(&mut app, HistoryAction::ResetAfterFutureClear);
    emit(&mut app, HistoryAction::Start);
    let initial = app.world().get::<Roof>(entity).unwrap().0;
    let mut middle = initial;
    middle[0x30..0x34].copy_from_slice(&0.7f32.to_le_bytes());
    let mut final_state = initial;
    final_state[0x30..0x34].copy_from_slice(&0.9f32.to_le_bytes());
    emit(
        &mut app,
        RecordRoofVisualChange(RoofVisualHistoryRecord::from_before_after(
            wall.0, &initial, &middle,
        )),
    );
    emit(
        &mut app,
        RecordRoofVisualChange(RoofVisualHistoryRecord::from_before_after(
            wall.0,
            &middle,
            &final_state,
        )),
    );
    emit(&mut app, set_height(wall, 0.9));
    emit(&mut app, HistoryAction::Finish);
    step(&mut app, 0.);
    let transaction = app
        .world()
        .resource::<BuildingHistory>()
        .transactions
        .as_ref()
        .unwrap();
    assert_eq!(transaction.ledger().entries.len(), 1);
    assert_eq!(transaction.ledger().entries[0].edits.len(), 2);
    emit(&mut app, HistoryAction::Undo { instant: false });
    step(&mut app, 0.05);
    assert!(height(&app, entity) < 0.9 && height(&app, entity) > 0.7);
    assert_eq!(app.world().resource::<RoofHistoryReplay>().buckets.len(), 2);
    for action in [HistoryAction::Start, HistoryAction::TryStart] {
        emit(&mut app, action);
        app.world_mut().resource_mut::<BuildingFrame>().delta_time = 0.0;
        app.update();
        assert!(
            app.world()
                .resource::<BuildingReport>()
                .faults
                .iter()
                .any(|fault| fault
                    .contains("History recording requested during an active replay bucket"))
        );
        assert!(
            !app.world()
                .resource::<BuildingHistory>()
                .transactions
                .as_ref()
                .unwrap()
                .is_recording()
        );
        assert_eq!(app.world().resource::<RoofHistoryReplay>().buckets.len(), 2);
    }
    for _ in 0..3 {
        step(&mut app, 0.05);
    }
    assert_eq!(height(&app, entity), 0.7);
    for _ in 0..4 {
        step(&mut app, 0.05);
    }
    assert_eq!(
        height(&app, entity),
        f32::from_le_bytes(initial[0x30..0x34].try_into().unwrap())
    );
    assert!(
        app.world()
            .resource::<RoofHistoryReplay>()
            .buckets
            .is_empty()
    );
    assert_eq!(
        app.world()
            .resource::<RoofHistoryReplay>()
            .completed_this_frame,
        vec![0]
    );
    assert_eq!(
        app.world()
            .resource::<BuildingHistory>()
            .transactions
            .as_ref()
            .unwrap()
            .ledger()
            .entries
            .len(),
        1
    );
    emit(&mut app, HistoryAction::Redo { instant: true });
    step(&mut app, 0.);
    step(&mut app, 0.);
    assert_eq!(height(&app, entity), 0.9);
    assert!(
        app.world()
            .resource::<BuildingHistoryRequests>()
            .0
            .is_empty()
    );
}

#[test]
fn direction_source_records_all_events_and_replay_retains_command_order() {
    let (mut app, wall, entity) = scene();
    emit(&mut app, HistoryAction::ResetAfterFutureClear);
    emit(&mut app, HistoryAction::Start);
    for (from, to) in [(0, 1), (1, 1), (1, 0)] {
        emit(
            &mut app,
            RecordRoofDirectionChange {
                wall: wall.0,
                from,
                to,
            },
        );
        emit(
            &mut app,
            SwitchRoofDirection(RoofDirectionRequest {
                wall,
                direction: to,
            }),
        );
    }
    emit(&mut app, HistoryAction::Finish);
    step(&mut app, 0.);
    assert_eq!(
        app.world()
            .resource::<BuildingHistory>()
            .transactions
            .as_ref()
            .unwrap()
            .ledger()
            .entries[0]
            .edits
            .len(),
        3
    );
    emit(&mut app, HistoryAction::Undo { instant: true });
    for (direction, pending) in [(1, 2), (1, 1), (0, 0)] {
        step(&mut app, 0.);
        assert_eq!(app.world().get::<Roof>(entity).unwrap().0[0x3c], direction);
        assert_eq!(
            app.world()
                .resource::<BuildingHistory>()
                .replayed_this_frame,
            1
        );
        assert_eq!(
            app.world()
                .resource::<BuildingReport>()
                .geometry_changes
                .len(),
            1
        );
        assert_eq!(
            app.world().resource::<RoofHistoryReplay>().buckets.len(),
            pending
        );
    }
}

#[test]
fn authoritative_height_updates_republish_on_equal_and_flat_switch_reaches_the_owner() {
    use tg_building_runtime::wall_height::*;
    let (mut app, wall, entity) = scene();
    let original_base_height = app.world().get::<Roof>(entity).unwrap().0[0x4c..0x50].to_vec();
    emit(&mut app, SetWallSourceHeight { wall, height: 4. });
    step(&mut app, 0.);
    assert_eq!(
        app.world().resource::<WallCollectionState>().raw_sources[&wall].extra,
        4.
    );
    assert_eq!(
        app.world().resource::<PublicWalls>().walls[&wall].publication_revision,
        2
    );
    assert_ne!(
        app.world().get::<Roof>(entity).unwrap().0[0x4c..0x50],
        original_base_height
    );
    emit(&mut app, SetWallSourceHeight { wall, height: 4. });
    step(&mut app, 0.);
    assert_eq!(
        app.world().resource::<PublicWalls>().walls[&wall].publication_revision,
        3
    );
    emit(
        &mut app,
        SetWallSourceHeight {
            wall: WallId(999),
            height: 7.,
        },
    );
    step(&mut app, 0.);
    assert!(app.world().resource::<WallHeightUpdates>().0.is_empty());
    assert_eq!(
        app.world().resource::<PublicWalls>().walls[&wall].publication_revision,
        3
    );
    let mut flat = set_height(wall, 0.);
    flat.0.source_flag = 1;
    emit(&mut app, flat);
    step(&mut app, 0.);
    assert!(app.world().get::<RoofCpuTiles>(entity).is_none());
    step(&mut app, 0.);
    assert_eq!(
        app.world().resource::<WallCollectionState>().raw_sources[&wall].extra,
        1.6
    );
    assert_eq!(
        app.world().resource::<PublicWalls>().walls[&wall].publication_revision,
        4
    );
    assert!(
        app.world()
            .resource::<RoofTypeTransitions>()
            .pending_wall_height_owner
            .is_empty()
    );
}

#[test]
fn observed_presence_history_recreates_all_components_and_cleans_deleted_entities() {
    let (mut app, wall, original) = scene();
    let deletion = RoofHistoryEdit::delete_from_roof(&app.world().get::<Roof>(original).unwrap().0);
    emit(&mut app, HistoryAction::ResetAfterFutureClear);
    emit(&mut app, HistoryAction::Start);
    emit(&mut app, RecordRoofOwner(deletion.clone()));
    emit(&mut app, DeleteRoof(wall));
    emit(&mut app, HistoryAction::Finish);
    step(&mut app, 0.5);
    assert!(app.world().get_entity(original).is_err());
    emit(&mut app, HistoryAction::Undo { instant: true });
    step(&mut app, 0.5);
    let restored = app.world().resource::<WallRoofMap>().entity(wall).unwrap();
    assert_ne!(original, restored);
    assert!(app.world().get::<TriggerVolume>(restored).is_some());
    assert!(app.world().get::<RoofSupportBeams>(restored).is_some());
    assert!(app.world().get::<RoofSpire>(restored).is_some());
    assert!(app.world().get::<RoofCpuTiles>(restored).is_some());
    assert!(
        app.world()
            .resource::<nani::NaniCpuStagingState>()
            .contains(restored)
    );
    emit(&mut app, HistoryAction::Redo { instant: true });
    step(&mut app, 0.5);
    assert!(app.world().get_entity(restored).is_err());
    let creation = RoofHistoryEdit::create(wall.0, deletion.payload()[4..48].try_into().unwrap());
    emit(&mut app, HistoryAction::Start);
    emit(&mut app, RecordRoofOwner(creation.clone()));
    let mut replay =
        country_core::resources::roofs::geometry::replay::VisualReplayState::from_observed([0; 24]);
    emit(
        &mut app,
        ReplayRoofEvent(
            replay
                .advance_discrete_observed(creation.payload(), 0.)
                .unwrap()
                .unwrap(),
        ),
    );
    emit(&mut app, HistoryAction::Finish);
    step(&mut app, 0.5);
    let created = app.world().resource::<WallRoofMap>().entity(wall).unwrap();
    emit(&mut app, HistoryAction::Undo { instant: true });
    step(&mut app, 0.5);
    assert!(app.world().get_entity(created).is_err());
    emit(&mut app, HistoryAction::Redo { instant: true });
    step(&mut app, 0.5);
    assert!(app.world().resource::<WallRoofMap>().entity(wall).is_some());
    assert_eq!(
        app.world()
            .resource::<BuildingHistory>()
            .transactions
            .as_ref()
            .unwrap()
            .ledger()
            .entries
            .len(),
        2
    );
    assert!(
        app.world()
            .resource::<BuildingHistoryRequests>()
            .0
            .is_empty()
    );
}

#[test]
fn actual_half_window_assets_follow_roof_direction_and_flat_type_changes() {
    let (mut app, wall, _) = scene();
    let assets = std::env::var_os("TG_B_ASSETS")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/assets"));
    app.world_mut()
        .resource_mut::<DecoratorAssetLibrary>()
        .load_b_wall_opening_assets(&assets)
        .unwrap();
    assert_eq!(
        app.world()
            .resource::<DecoratorAssetLibrary>()
            .collisions
            .len(),
        18
    );
    let id = OpeningId(12);
    emit(
        &mut app,
        PlaceWallOpening {
            id,
            wall,
            request: WallDecoratorRequest {
                family: OpeningFamily::CottageWindow,
                rank: 1,
                world_position: [0., 5., 4.],
            },
        },
    );
    step(&mut app, 0.);
    let entity = app.world().resource::<OpeningEntities>().0[&id];
    let half = app
        .world()
        .get::<OpeningAuthority>(entity)
        .unwrap()
        .input
        .collision;
    let half_key =
        country_core::resources::walls::decorator::placement::DecoratorLibKey {
            subtype: 1,
            rank: 1,
        };
    assert_eq!(
        half,
        app.world().resource::<DecoratorAssetLibrary>().collisions[&half_key]
            .bounding_box
            .metrics()
    );
    let original_holes = app.world().resource::<WallOpenings>().storage[&wall]
        .finalized_holes()
        .unwrap()
        .to_vec();
    emit(
        &mut app,
        SwitchRoofDirection(RoofDirectionRequest { wall, direction: 1 }),
    );
    step(&mut app, 0.);
    let plain = app
        .world()
        .get::<OpeningAuthority>(entity)
        .unwrap()
        .input
        .collision;
    assert_ne!(plain, half);
    assert_ne!(
        app.world().resource::<WallOpenings>().storage[&wall]
            .finalized_holes()
            .unwrap(),
        original_holes
    );
    emit(
        &mut app,
        SwitchRoofDirection(RoofDirectionRequest { wall, direction: 0 }),
    );
    step(&mut app, 0.);
    assert_eq!(
        app.world()
            .get::<OpeningAuthority>(entity)
            .unwrap()
            .input
            .collision,
        half
    );
    emit(&mut app, set_height(wall, 0.));
    step(&mut app, 0.);
    assert_eq!(
        app.world()
            .get::<OpeningAuthority>(entity)
            .unwrap()
            .input
            .collision,
        plain
    );
    emit(&mut app, set_height(wall, 0.5));
    step(&mut app, 0.);
    assert_eq!(
        app.world()
            .get::<OpeningAuthority>(entity)
            .unwrap()
            .input
            .collision,
        half
    );
    emit(&mut app, RemoveOpening(id));
    step(&mut app, 0.);
    assert!(
        app.world().resource::<WallOpenings>().storage[&wall]
            .finalized_holes()
            .unwrap()
            .is_empty()
    );
}

#[test]
fn raw_wall_production_rebuilds_for_holes_and_height_and_cleans_owned_entities() {
    use tg_building_runtime::wall_bricks::*;
    let mut app = building_app();
    configure_clear_path_resources(&mut app);
    let wall = WallId(71);
    let mut raw = source();
    raw.terrain_floor = 0.;
    raw.extra = 2.46;
    emit(
        &mut app,
        SetTerrainGrid {
            grid: TerrainHeightGrid512 {
                encoded: Arc::from(vec![0; 512 * 512]),
                minimum_y: 0.,
                maximum_y: 0.,
                sampling_fix: true,
            },
            affected_walls: vec![],
        },
    );
    emit(&mut app, UpsertRawWall { wall, source: raw });
    step(&mut app, 0.);
    let entity = app.world().resource::<WallBrickEntities>().0[&wall];
    let records = |app: &bevy_app::App| {
        app.world()
            .get::<WallCpuBricks>(entity)
            .unwrap()
            .instances
            .iter()
            .map(|value| value.observed_bytes())
            .collect::<Vec<_>>()
    };
    let initial = records(&app);
    assert!(!initial.is_empty());
    assert_eq!(
        app.world().get::<WallCpuBricks>(entity).unwrap().generation,
        1
    );
    step(&mut app, 0.);
    assert_eq!(records(&app), initial);
    assert_eq!(
        app.world().get::<WallCpuBricks>(entity).unwrap().generation,
        1
    );
    emit(
        &mut app,
        UpsertOpening(OpeningAuthority {
            id: OpeningId(1),
            wall,
            input: country_core::resources::walls::wall_holes::opening_rules::DecoratorOpeningInput {
                family: OpeningFamily::CottageWindow,
                size_variant: 1,
                center: [4., 1.],
                collision:
                    country_core::resources::walls::wall_holes::opening_rules::DecoratorCollisionMetrics {
                        extent_xz: [1., 0.2],
                        vertical_bounds: [-0.5, 0.5],
                    },
            },
        }),
    );
    step(&mut app, 0.);
    assert_ne!(records(&app), initial);
    emit(&mut app, RemoveOpening(OpeningId(1)));
    step(&mut app, 0.);
    assert_eq!(records(&app), initial);
    emit(
        &mut app,
        wall_height::SetWallSourceHeight { wall, height: 3. },
    );
    step(&mut app, 0.);
    assert_ne!(records(&app), initial);
    assert!(
        app.world()
            .resource::<WallBrickBuildState>()
            .pending
            .is_empty()
    );
    emit(&mut app, RemoveWall(wall));
    step(&mut app, 0.);
    assert!(app.world().get_entity(entity).is_err());
    assert!(app.world().resource::<WallBrickEntities>().0.is_empty());
    assert!(
        app.world()
            .resource::<WallBrickBuildState>()
            .pending
            .is_empty()
    );
    assert!(
        app.world()
            .resource::<wall_paths::WallPreparedPaths>()
            .records
            .is_empty()
    );
}

#[test]
fn actual_height_command_and_recovered_history_request_drive_the_owned_source() {
    use country_core::systems::wall::height_command::AdjustWallHeightRequest;
    use country_core::systems::wall::height_command::WallHeightFeedback;
    use tg_building_runtime::wall_height::*;
    let (mut app, wall, entity) = scene();
    emit(
        &mut app,
        AdjustWallHeight {
            request: AdjustWallHeightRequest {
                anchor: None,
                mode: 1,
                value: 0.,
                wall,
                observed_audio_flag: 0,
            },
        },
    );
    step(&mut app, 0.5);
    assert_eq!(
        app.world().resource::<WallCollectionState>().raw_sources[&wall].extra,
        0.7
    );
    assert_eq!(
        app.world().resource::<PublicWalls>().walls[&wall].publication_revision,
        2
    );
    let outcome = &app.world().resource::<WallHeightCommandUpdates>().0[0];
    assert_eq!(outcome.authority_updates.len(), 2);
    assert_eq!(outcome.anchor_world, [3., 0., 4.]);
    assert!(matches!(
        outcome.feedback,
        WallHeightFeedback::Building {
            observed_kind: 3,
            ..
        }
    ));
    emit(
        &mut app,
        AdjustWallHeight {
            request: AdjustWallHeightRequest {
                anchor: None,
                mode: 1,
                value: 0.,
                wall,
                observed_audio_flag: 1,
            },
        },
    );
    step(&mut app, 0.);
    assert!(
        app.world()
            .resource::<WallHeightCommandUpdates>()
            .0
            .is_empty()
    );
    assert_eq!(
        app.world().resource::<PublicWalls>().walls[&wall].publication_revision,
        2
    );
    let edit =
        country_core::resources::history::building_transactions::WallHeightHistoryEdit {
            wall_id: wall.0,
            from: 3.,
            to: 0.7,
        };
    let replay = edit.instant_request(true);
    let mut observed = [0; 40];
    replay.write_observed_fields(&mut observed);
    emit(
        &mut app,
        AdjustWallHeight {
            request: AdjustWallHeightRequest::from_observed(&observed),
        },
    );
    step(&mut app, 0.);
    assert_eq!(
        app.world().resource::<WallCollectionState>().raw_sources[&wall].extra,
        3.
    );
    assert!(matches!(
        app.world().resource::<WallHeightCommandUpdates>().0[0].feedback,
        WallHeightFeedback::WallMaterial {
            observed_kind: 0,
            observed_flag: 1,
            ..
        }
    ));
    assert_eq!(
        app.world().resource::<WallHeightCommandUpdates>().0[0]
            .authority_updates
            .len(),
        1
    );
    assert!(app.world().get::<RoofCpuTiles>(entity).is_some());
    assert!(
        app.world()
            .resource::<BuildingHistoryRequests>()
            .0
            .is_empty()
    );
    emit(&mut app, set_height(wall, 0.));
    step(&mut app, 0.);
    emit(
        &mut app,
        AdjustWallHeight {
            request: AdjustWallHeightRequest {
                anchor: Some([0., 2., 0.]),
                mode: 1,
                value: 0.,
                wall,
                observed_audio_flag: 1,
            },
        },
    );
    step(&mut app, 0.);
    assert_eq!(
        app.world().resource::<WallCollectionState>().raw_sources[&wall].extra,
        1.6
    );
    emit(
        &mut app,
        AdjustWallHeight {
            request: AdjustWallHeightRequest {
                anchor: None,
                mode: 2,
                value: 7.,
                wall,
                observed_audio_flag: 1,
            },
        },
    );
    app.update();
    assert_eq!(app.world().resource::<BuildingReport>().faults.len(), 1);
    assert_eq!(
        app.world().resource::<WallCollectionState>().raw_sources[&wall].extra,
        1.6
    );
}

#[test]
fn height_command_reads_published_special_flag_from_the_real_source() {
    use country_core::systems::wall::height_command::AdjustWallHeightRequest;
    use tg_building_runtime::wall_height::AdjustWallHeight;
    let mut app = building_app();
    configure_clear_path_resources(&mut app);
    let wall = WallId(73);
    emit(
        &mut app,
        SetTerrainGrid {
            grid: TerrainHeightGrid512 {
                encoded: Arc::from(vec![0; 512 * 512]),
                minimum_y: 0.,
                maximum_y: 0.,
                sampling_fix: true,
            },
            affected_walls: vec![],
        },
    );
    for (special, expected) in [(true, 2.5), (false, 14.)] {
        let mut raw = source();
        raw.special = special;
        raw.extra = 1.;
        raw.clutter_version = 7;
        raw.observed_path_scalar = 0.75;
        emit(&mut app, UpsertRawWall { wall, source: raw });
        step(&mut app, 0.);
        emit(
            &mut app,
            AdjustWallHeight {
                request: AdjustWallHeightRequest {
                    anchor: None,
                    mode: 1,
                    value: 20.,
                    wall,
                    observed_audio_flag: 0,
                },
            },
        );
        step(&mut app, 0.);
        assert_eq!(
            app.world().resource::<WallCollectionState>().raw_sources[&wall].extra,
            expected
        );
        assert_eq!(
            app.world().resource::<PublicWalls>().walls[&wall]
                .source
                .special,
            special
        );
    }
}

#[test]
fn failed_height_commands_do_not_block_wall_removal_or_republish_deleted_sources() {
    use country_core::systems::wall::height_command::AdjustWallHeightRequest;
    use tg_building_runtime::wall_height::*;
    let (mut app, wall, _) = scene();
    emit(&mut app, SetWallSourceHeight { wall, height: 4. });
    emit(&mut app, RemoveWall(wall));
    for _ in 0..3 {
        emit(
            &mut app,
            AdjustWallHeight {
                request: AdjustWallHeightRequest {
                    anchor: None,
                    mode: 1,
                    value: 7.,
                    wall: WallId(999),
                    observed_audio_flag: 1,
                },
            },
        );
        app.update();
        assert_eq!(app.world().resource::<BuildingReport>().faults.len(), 1);
        assert!(
            !app.world()
                .resource::<PublicWalls>()
                .walls
                .contains_key(&wall)
        );
        let collection = app.world().resource::<WallCollectionState>();
        assert!(!collection.raw_sources.contains_key(&wall));
        assert!(
            !collection
                .spatial_index
                .candidates(&[u64::MAX; 64])
                .contains(&wall)
        );
    }
    for _ in 0..3 {
        step(&mut app, 0.5);
        assert!(
            !app.world()
                .resource::<PublicWalls>()
                .walls
                .contains_key(&wall)
        );
        assert!(app.world().resource::<WallRoofMap>().entity(wall).is_none());
    }
}

#[test]
fn each_door_family_places_with_only_its_own_real_collision_asset() {
    for family in [
        OpeningFamily::CottageBalconyDoor,
        OpeningFamily::GothicBalconyDoor,
    ] {
        let (mut app, wall, _) = scene();
        let assets = std::env::var_os("TG_B_ASSETS")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/assets"));
        let binding = b_plain_wall_bindings()
            .into_iter()
            .find(|binding| binding.family == family && binding.rank == 1)
            .unwrap();
        app.world_mut()
            .resource_mut::<DecoratorAssetLibrary>()
            .load_bindings(&assets, &[binding])
            .unwrap();
        assert_eq!(
            app.world()
                .resource::<DecoratorAssetLibrary>()
                .collisions
                .len(),
            1
        );
        let id = OpeningId(99);
        emit(
            &mut app,
            PlaceWallOpening {
                id,
                wall,
                request: WallDecoratorRequest {
                    family,
                    rank: 1,
                    world_position: [-3., 2., 0.],
                },
            },
        );
        step(&mut app, 0.);
        let entity = app.world().resource::<OpeningEntities>().0[&id];
        assert_eq!(
            app.world()
                .get::<OpeningAuthority>(entity)
                .unwrap()
                .input
                .family,
            family
        );
        assert!(
            !app.world()
                .get::<OpeningState>(entity)
                .unwrap()
                .raw_holes
                .is_empty()
        );
        emit(&mut app, RemoveOpening(id));
        step(&mut app, 0.);
        assert!(app.world().get_entity(entity).is_err());
    }
}

#[test]
fn roof_removal_finishes_opening_and_brick_updates_in_the_same_adapter_frame() {
    use tg_building_runtime::wall_bricks::*;
    let (mut app, wall, roof) = scene();
    let assets = std::env::var_os("TG_B_ASSETS")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/assets"));
    app.world_mut()
        .resource_mut::<DecoratorAssetLibrary>()
        .load_b_wall_opening_assets(&assets)
        .unwrap();
    emit(
        &mut app,
        PlaceWallOpening {
            id: OpeningId(90),
            wall,
            request: WallDecoratorRequest {
                family: OpeningFamily::CottageWindow,
                rank: 1,
                world_position: [0., 5., 4.],
            },
        },
    );
    step(&mut app, 0.);
    emit(&mut app, DeleteRoof(wall));
    for _ in 0..4 {
        step(&mut app, 0.5);
        if app.world().get::<Roof>(roof).is_none() {
            break;
        }
    }
    assert!(app.world().get::<Roof>(roof).is_none());
    let bricks = app.world().resource::<WallBrickEntities>().0[&wall];
    let before = app
        .world()
        .get::<WallCpuBricks>(bricks)
        .unwrap()
        .instances
        .iter()
        .map(|record| record.observed_bytes())
        .collect::<Vec<_>>();
    let generation = app.world().get::<WallCpuBricks>(bricks).unwrap().generation;
    let opening = app.world().resource::<OpeningEntities>().0[&OpeningId(90)];
    let collision = app
        .world()
        .get::<OpeningAuthority>(opening)
        .unwrap()
        .input
        .collision;
    let holes = app.world().resource::<WallOpenings>().storage[&wall]
        .finalized_holes()
        .unwrap()
        .to_vec();
    step(&mut app, 0.);
    assert_eq!(
        app.world().get::<WallCpuBricks>(bricks).unwrap().generation,
        generation
    );
    assert_eq!(
        app.world()
            .get::<WallCpuBricks>(bricks)
            .unwrap()
            .instances
            .iter()
            .map(|record| record.observed_bytes())
            .collect::<Vec<_>>(),
        before
    );
    assert_eq!(
        app.world()
            .get::<OpeningAuthority>(opening)
            .unwrap()
            .input
            .collision,
        collision
    );
    assert_eq!(
        app.world().resource::<WallOpenings>().storage[&wall]
            .finalized_holes()
            .unwrap(),
        holes
    );
}

#[test]
fn a_bad_roof_edit_does_not_block_later_edits_and_final_crossing_wins() {
    let (mut app, wall, entity) = scene();
    emit(&mut app, set_height(WallId(999), 0.));
    emit(&mut app, set_height(wall, 0.));
    emit(&mut app, set_height(wall, 0.5));
    app.update();
    assert_eq!(app.world().resource::<BuildingReport>().faults.len(), 1);
    assert_eq!(height(&app, entity), 0.5);
    let applied = &app.world().resource::<RoofTypeTransitions>().applied;
    assert_eq!(applied.len(), 1);
    assert_eq!(
        applied[0].crossing,
        country_core::resources::roofs::geometry::edit_rules::HeightCrossing::Up
    );
    assert!(app.world().get::<RoofCpuTiles>(entity).is_some());
    assert_eq!(app.world().resource::<BuildingReport>().sounds[0].kind, 0);
    step(&mut app, 0.);
}

#[test]
fn visual_history_keeps_changes_to_distinct_roofs_in_the_same_update() {
    let (mut app, wall, entity) = scene();
    let initial = app.world().get::<Roof>(entity).unwrap().0;
    let mut changed = initial;
    changed[0x30..0x34].copy_from_slice(&0.9f32.to_le_bytes());
    emit(&mut app, HistoryAction::ResetAfterFutureClear);
    emit(&mut app, HistoryAction::Start);
    for wall_id in [wall.0, wall.0 + 1] {
        emit(
            &mut app,
            RecordRoofVisualChange(RoofVisualHistoryRecord::from_before_after(
                wall_id, &initial, &changed,
            )),
        );
    }
    emit(&mut app, HistoryAction::Finish);
    step(&mut app, 0.);
    let transaction = app
        .world()
        .resource::<BuildingHistory>()
        .transactions
        .as_ref()
        .unwrap();
    let walls: Vec<_> = transaction.ledger().entries[0]
        .edits
        .iter()
        .map(|edit| {
            let country_core::resources::history::building_transactions::BuildingEdit::Roof(
                edit,
            ) = edit
            else {
                panic!("expected roof edit")
            };
            edit.wall_id()
        })
        .collect();
    assert_eq!(walls, vec![wall.0, wall.0 + 1]);
}

#[test]
fn unsupported_wall_history_is_rejected_before_ledger_mutation() {
    use country_core::resources::history::building_transactions::BuildingEdit;
    use country_core::resources::history::wall_edit::UnresolvedWallEditPayload;
    use country_core::resources::history::wall_edit::WallCompoundHistoryEdit;
    use country_core::resources::history::wall_edit::WallHistoryEdit;
    let mut app = building_app();
    emit(&mut app, HistoryAction::ResetAfterFutureClear);
    emit(&mut app, HistoryAction::Start);
    step(&mut app, 0.0);
    app.world_mut()
        .resource_mut::<BuildingHistoryRequests>()
        .0
        .push(BuildingEdit::Wall(Arc::new(WallCompoundHistoryEdit {
            groups: vec![vec![WallHistoryEdit::Unresolved15(
                UnresolvedWallEditPayload,
            )]],
        })));
    emit(&mut app, HistoryAction::Finish);
    app.update();
    assert!(
        app.world()
            .resource::<BuildingReport>()
            .faults
            .iter()
            .any(|fault| fault.contains("unsupported wall operation"))
    );
    assert_eq!(app.world().resource::<BuildingHistoryRequests>().0.len(), 1);
    let history = app.world().resource::<BuildingHistory>();
    let ledger = history.transactions.as_ref().unwrap();
    assert!(ledger.ledger().entries.is_empty());
    assert!(ledger.is_recording());
}

#[test]
fn rejected_cursor_and_reset_actions_do_not_stall_an_active_replay() {
    let (mut app, wall, entity) = scene();
    let initial = app.world().get::<Roof>(entity).unwrap().0;
    let mut changed = initial;
    changed[0x30..0x34].copy_from_slice(&0.9f32.to_le_bytes());
    emit(&mut app, HistoryAction::ResetAfterFutureClear);
    emit(&mut app, HistoryAction::Start);
    emit(
        &mut app,
        RecordRoofVisualChange(RoofVisualHistoryRecord::from_before_after(
            wall.0, &initial, &changed,
        )),
    );
    emit(&mut app, set_height(wall, 0.9));
    emit(&mut app, HistoryAction::Finish);
    step(&mut app, 0.);
    emit(&mut app, HistoryAction::Undo { instant: false });
    step(&mut app, 0.02);
    for action in [
        HistoryAction::Undo { instant: false },
        HistoryAction::Redo { instant: false },
        HistoryAction::ResetAfterFutureClear,
    ] {
        let previous = height(&app, entity);
        emit(&mut app, action);
        app.world_mut().resource_mut::<BuildingFrame>().delta_time = 0.02;
        app.update();
        assert!(!app.world().resource::<BuildingReport>().faults.is_empty());
        assert!(
            height(&app, entity) < previous,
            "rejected action must not prevent replay progress"
        );
        assert!(
            app.world()
                .resource::<BuildingHistory>()
                .replayed_this_frame
                > 0
        );
    }
}

#[test]
fn withdrawn_world_opening_recovers_when_prior_valid_roof_state_returns() {
    let (mut app, wall, roof_entity) = scene();
    let asset_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/assets");
    app.world_mut()
        .resource_mut::<DecoratorAssetLibrary>()
        .load_b_wall_opening_assets(&asset_root)
        .unwrap();
    let id = OpeningId(7001);
    emit(
        &mut app,
        PlaceWallOpening {
            id,
            wall,
            request: WallDecoratorRequest {
                family: OpeningFamily::CottageWindow,
                rank: 1,
                world_position: [-3., 2., 0.],
            },
        },
    );
    step(&mut app, 0.);
    assert!(
        app.world()
            .resource::<OpeningEntities>()
            .0
            .contains_key(&id)
    );
    let valid_roof = app.world().get::<Roof>(roof_entity).unwrap().0;
    app.world_mut().component_mut::<Roof>(roof_entity).unwrap().0[0x3c] = 2;
    app.update();
    assert!(
        !app.world()
            .resource::<OpeningEntities>()
            .0
            .contains_key(&id)
    );
    app.world_mut().component_mut::<Roof>(roof_entity).unwrap().0 = valid_roof;
    step(&mut app, 0.);
    assert!(
        app.world()
            .resource::<OpeningEntities>()
            .0
            .contains_key(&id)
    );
}

use country_core::render::extract::WorldComponentWrite as _;
