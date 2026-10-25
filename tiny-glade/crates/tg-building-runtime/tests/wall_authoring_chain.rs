use bevy_ecs::prelude::*;
use country_core::resources::walls::freeform_height_cache::*;
use country_core::resources::walls::publication_fields::WallId;
use country_core::resources::walls::publication_source::TerrainHeightGrid512;
use country_core::systems::wall::shapes::authoring::*;
use country_core::systems::wall::wall_state::path_preparation::PathRasters512;
use std::sync::Arc;
use tg_building_runtime::{wall_authoring::*, wall_bricks::*, wall_paths::*, *};
fn emit<T: Event>(app: &mut bevy_app::App, event: T) {
    app.world_mut().resource_mut::<Events<T>>().send(event);
}
fn configure(app: &mut bevy_app::App) {
    emit(
        app,
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
        app,
        SetWallPathRasters(
            PathRasters512::from_encoded(
                std::array::from_fn(|_| Arc::from(vec![0; 512 * 512])),
                true,
            )
            .unwrap(),
        ),
    );
    let mut owner = app.world_mut().resource_mut::<WallAuthoringState>();
    owner.id_counter = Some(99);
    owner.settings = Some(CreationSettingsObserved {
        wall_color_id: 7,
        roof_color_id: 8,
        timber_color_id: 9,
        wall_style: 0,
        rim_style: 0,
    });
}
fn placement(height: f32, elevation: f32, previous: Option<WallId>) -> [u8; 80] {
    let mut bytes = [0; 80];
    bytes[0x40..0x44].copy_from_slice(&height.to_le_bytes());
    bytes[0x3c..0x40].copy_from_slice(&elevation.to_le_bytes());
    bytes[0x2c] = if previous.is_some() { 0 } else { 2 };
    if let Some(id) = previous {
        bytes[0x20..0x28].copy_from_slice(&id.0.to_le_bytes());
    }
    bytes
}
#[test]
fn authored_creation_reaches_source_public_roof_masks_and_bricks_and_inherits_real_host() {
    let mut app = building_app();
    configure(&mut app);
    emit(
        &mut app,
        CreateBuildingFromPlacement {
            shape: AuthoringShape::Rectangle(PreviewRectangleObserved {
                axes: [1., 0.],
                center: [-10., 0.],
                dimensions: [4., 6.],
            }),
            placement: placement(3.5, 0.75, None),
        },
    );
    app.update();
    assert!(app.world().resource::<BuildingReport>().faults.is_empty());
    let wall = WallId(100);
    let source = &app.world().resource::<WallCollectionState>().raw_sources[&wall];
    assert_eq!(source.extra, 3.5);
    assert_eq!(source.terrain_floor, 0.75);
    assert_eq!(source.observed_path_scalar.to_bits(), 0x40333333);
    assert!(
        app.world()
            .resource::<PublicWalls>()
            .walls
            .contains_key(&wall)
    );
    assert!(app.world().resource::<WallRoofMap>().entity(wall).is_some());
    assert!(
        app.world()
            .resource::<wall_segments::WallPathMaskRegistry>()
            .records
            .contains_key(&wall)
    );
    let entity = app.world().resource::<WallBrickEntities>().0[&wall];
    assert!(
        !app.world()
            .get::<WallCpuBricks>(entity)
            .unwrap()
            .instances
            .is_empty()
    );
    assert_eq!(
        app.world()
            .resource::<WallAuthoringState>()
            .pending_creation_history
            .len(),
        0
    );
    app.world_mut()
        .resource_mut::<WallAuthoringState>()
        .settings = Some(CreationSettingsObserved {
        wall_color_id: 70,
        roof_color_id: 80,
        timber_color_id: 90,
        wall_style: 2,
        rim_style: 2,
    });
    emit(
        &mut app,
        CreateBuildingFromPlacement {
            shape: AuthoringShape::Circle(PreviewCircleObserved {
                center: [10., 0.],
                radius: 2.,
                angle: 0.,
            }),
            placement: placement(2.5, 0., Some(wall)),
        },
    );
    app.update();
    assert!(app.world().resource::<BuildingReport>().faults.is_empty());
    let next = WallId(101);
    let owner = app.world().resource::<WallAuthoringState>();
    assert_eq!(owner.id_counter, Some(101));
    assert!(owner.new_sources.contains(&next));
    assert!(owner.updated_sources.contains(&next));
    assert_eq!(owner.appearance[&next].wall_color_id, 7);
    assert_eq!(owner.appearance[&next].timber_color_id, 9);
    assert_eq!(owner.appearance[&next].roof_color_id, 8);
    assert_eq!(owner.appearance[&next].wall_style, 0);
    assert!(
        app.world()
            .resource::<WallBrickEntities>()
            .0
            .contains_key(&next)
    );
    let removed_roof = app.world().resource::<WallRoofMap>().entity(next).unwrap();
    app.world_mut().resource_mut::<BuildingFrame>().delta_time = 0.5;
    emit(&mut app, RemoveAuthoredWall(next));
    app.update();
    assert!(app.world().resource::<BuildingReport>().faults.is_empty());
    assert!(
        !app.world()
            .resource::<WallCollectionState>()
            .raw_sources
            .contains_key(&next)
    );
    assert!(
        !app.world()
            .resource::<PublicWalls>()
            .walls
            .contains_key(&next)
    );
    assert!(
        !app.world()
            .resource::<WallBrickEntities>()
            .0
            .contains_key(&next)
    );
    assert!(
        !app.world()
            .resource::<wall_segments::WallPathMaskRegistry>()
            .records
            .contains_key(&next)
    );
    assert!(
        !app.world()
            .resource::<WallAuthoringState>()
            .appearance
            .contains_key(&next)
    );
    assert_eq!(
        app.world().resource::<WallAuthoringState>().id_counter,
        Some(101)
    );
    assert!(app.world().get_entity(removed_roof).is_err());
}
#[test]
fn missing_startup_counter_does_not_create_ids_sources_or_roofs() {
    let mut app = building_app();
    configure(&mut app);
    app.world_mut()
        .resource_mut::<WallAuthoringState>()
        .id_counter = None;
    emit(
        &mut app,
        CreateBuildingFromPlacement {
            shape: AuthoringShape::Circle(PreviewCircleObserved {
                center: [0., 0.],
                radius: 2.,
                angle: 0.,
            }),
            placement: placement(2., 0., None),
        },
    );
    app.update();
    assert_eq!(app.world().resource::<BuildingReport>().faults.len(), 1);
    assert!(
        app.world()
            .resource::<WallCollectionState>()
            .raw_sources
            .is_empty()
    );
    assert!(app.world().resource::<PublicWalls>().walls.is_empty());
    assert!(app.world().resource::<WallBrickEntities>().0.is_empty());
    assert!(
        app.world()
            .resource::<WallAuthoringState>()
            .pending_creation_history
            .is_empty()
    );
}

#[test]
fn same_update_create_and_remove_consume_the_new_id_without_geometry_resurrection() {
    let mut app = building_app();
    configure(&mut app);
    emit(
        &mut app,
        CreateBuildingFromPlacement {
            shape: AuthoringShape::Circle(PreviewCircleObserved {
                center: [0., 0.],
                radius: 2.,
                angle: 0.,
            }),
            placement: placement(2., 0., None),
        },
    );
    emit(&mut app, RemoveAuthoredWall(WallId(100)));
    app.update();
    assert!(app.world().resource::<BuildingReport>().faults.is_empty());
    assert!(
        app.world()
            .resource::<WallCollectionState>()
            .raw_sources
            .is_empty()
    );
    assert!(app.world().resource::<PublicWalls>().walls.is_empty());
    assert!(app.world().resource::<WallBrickEntities>().0.is_empty());
    assert!(
        app.world()
            .resource::<WallAuthoringState>()
            .appearance
            .is_empty()
    );
    assert_eq!(
        app.world().resource::<WallAuthoringState>().id_counter,
        Some(100)
    );
    assert_eq!(
        app.world()
            .resource::<WallAuthoringState>()
            .pending_creation_history
            .len(),
        0
    );
    let world = app.world_mut();
    let mut roofs = world.query::<&Roof>();
    assert_eq!(roofs.iter(world).count(), 0);
    app.update();
    assert!(app.world().resource::<PublicWalls>().walls.is_empty());
}

fn checked_step(app: &mut bevy_app::App, dt: f32) {
    app.world_mut().resource_mut::<BuildingFrame>().delta_time = dt;
    app.update();
    assert!(
        app.world().resource::<BuildingReport>().faults.is_empty(),
        "{:?}",
        app.world().resource::<BuildingReport>().faults
    );
}
#[test]
fn compound_wall_roof_history_restores_authority_and_whole_geometry_in_native_bucket_order() {
    use tg_building_runtime::history::*;
    for instant in [true, false] {
        let mut app = building_app();
        configure(&mut app);
        app.world_mut()
            .resource_mut::<WallAuthoringState>()
            .dev_override_is_demo_mode = Some(false);
        // Establish actual raster/terrain resources before replay admission.
        checked_step(&mut app, 0.5);
        emit(&mut app, HistoryAction::ResetAfterFutureClear);
        emit(&mut app, HistoryAction::Start);
        emit(
            &mut app,
            CreateBuildingFromPlacement {
                shape: AuthoringShape::Rectangle(PreviewRectangleObserved {
                    axes: [1., 0.],
                    center: [0., 0.],
                    dimensions: [4., 6.],
                }),
                placement: placement(3.5, 0.75, None),
            },
        );
        emit(&mut app, HistoryAction::Finish);
        checked_step(&mut app, 0.5);
        let wall = WallId(100);
        let original = app
            .world()
            .get::<WallCpuBricks>(app.world().resource::<WallBrickEntities>().0[&wall])
            .unwrap()
            .instances
            .clone();
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
            2
        );
        assert!(
            app.world()
                .resource::<WallAuthoringState>()
                .pending_creation_history
                .is_empty()
        );
        emit(&mut app, HistoryAction::Undo { instant });
        checked_step(&mut app, 0.5);
        assert!(app.world().resource::<WallRoofMap>().entity(wall).is_none());
        assert!(
            app.world()
                .resource::<WallCollectionState>()
                .raw_sources
                .contains_key(&wall)
        );
        checked_step(&mut app, 0.5);
        assert!(
            app.world()
                .resource::<WallCollectionState>()
                .raw_sources
                .contains_key(&wall)
        );
        for _ in 0..5 {
            if app
                .world()
                .resource::<RoofHistoryReplay>()
                .buckets
                .is_empty()
            {
                break;
            }
            checked_step(&mut app, 0.5);
        }
        assert!(
            app.world()
                .resource::<RoofHistoryReplay>()
                .buckets
                .is_empty()
        );
        assert!(
            !app.world()
                .resource::<WallCollectionState>()
                .raw_sources
                .contains_key(&wall)
        );
        assert!(
            !app.world()
                .resource::<WallBrickEntities>()
                .0
                .contains_key(&wall)
        );
        assert_eq!(
            app.world().resource::<WallAuthoringState>().id_counter,
            Some(100)
        );
        emit(&mut app, HistoryAction::Redo { instant });
        checked_step(&mut app, 0.5);
        assert_eq!(
            app.world().resource::<WallCollectionState>().raw_sources[&wall]
                .shape
                .tag(),
            2
        );
        assert!(app.world().resource::<WallRoofMap>().entity(wall).is_none());
        for _ in 0..6 {
            if app
                .world()
                .resource::<RoofHistoryReplay>()
                .buckets
                .is_empty()
            {
                break;
            }
            checked_step(&mut app, 0.5);
        }
        assert!(
            app.world()
                .resource::<RoofHistoryReplay>()
                .buckets
                .is_empty()
        );
        assert!(app.world().resource::<WallRoofMap>().entity(wall).is_some());
        assert_eq!(
            app.world().resource::<WallCollectionState>().raw_sources[&wall]
                .shape
                .tag(),
            1
        );
        assert_eq!(
            app.world().resource::<WallAuthoringState>().appearance[&wall].wall_color_id,
            7
        );
        assert_eq!(
            app.world()
                .resource::<WallAuthoringState>()
                .history_metadata[&wall]
                .source_69,
            5
        );
        assert_eq!(
            app.world()
                .get::<WallCpuBricks>(app.world().resource::<WallBrickEntities>().0[&wall])
                .unwrap()
                .instances,
            original
        );
        assert!(
            app.world()
                .resource::<roof_invalid::RoofInvalidNotificationState>()
                .history_pending
                .is_empty()
        );
        emit(&mut app, HistoryAction::Start);
        emit(&mut app, RemoveAuthoredWall(wall));
        emit(&mut app, HistoryAction::Finish);
        checked_step(&mut app, 0.5);
        assert_eq!(
            app.world()
                .resource::<BuildingHistory>()
                .transactions
                .as_ref()
                .unwrap()
                .ledger()
                .entries[1]
                .edits
                .len(),
            2
        );
        emit(&mut app, HistoryAction::Undo { instant: true });
        checked_step(&mut app, 0.5);
        // Undo deletion restores Roof request first; source resolution happens in
        // its existing pending owner, then Wall restore admits it next frame.
        checked_step(&mut app, 0.5);
        assert!(
            app.world()
                .resource::<WallCollectionState>()
                .raw_sources
                .contains_key(&wall)
        );
        assert!(app.world().resource::<WallRoofMap>().entity(wall).is_some());
        assert_eq!(
            app.world()
                .get::<WallCpuBricks>(app.world().resource::<WallBrickEntities>().0[&wall])
                .unwrap()
                .instances,
            original
        );
        emit(&mut app, HistoryAction::Redo { instant: true });
        checked_step(&mut app, 0.5);
        checked_step(&mut app, 0.5);
        assert!(
            !app.world()
                .resource::<WallCollectionState>()
                .raw_sources
                .contains_key(&wall)
        );
        assert!(
            !app.world()
                .resource::<WallBrickEntities>()
                .0
                .contains_key(&wall)
        );
        assert!(
            app.world()
                .resource::<roof_invalid::RoofInvalidNotificationState>()
                .history_pending
                .is_empty()
        );
    }
}

#[test]
fn actual_platform_decorator_rows_drive_stair_colliders_holes_bricks_and_cleanup() {
    use country_core::resources::roofs::creation::RoofCreateRequestObserved;
    use country_core::resources::roofs::geometry::surface::Curve2;
    use country_core::resources::walls::publication_fields::WallShapeSnapshot;
    use country_core::resources::walls::publication_source::RawWallPublicationSource;
    use tg_building_runtime::wall_platform_stairs::*;
    let mut app = building_app();
    configure(&mut app);
    let wall = WallId(202);
    let mut shape = [0; 28];
    shape[..4].copy_from_slice(&1u32.to_le_bytes());
    for (i, x) in [1f32, 0., 0., 0., 4., 4.].iter().enumerate() {
        shape[4 + i * 4..8 + i * 4].copy_from_slice(&x.to_le_bytes());
    }
    emit(
        &mut app,
        UpsertRawWall {
            wall,
            source: RawWallPublicationSource {
                curve: Curve2::try_new(
                    vec![[-2., -2.], [-2., 2.], [2., 2.], [2., -2.], [-2., -2.]],
                    false,
                )
                .unwrap(),
                curve_kind_tag: 1,
                terrain_floor: 0.,
                shape: WallShapeSnapshot::from_observed(shape),
                style: 0,
                extra: 2.5,
                special: false,
                observed_path_scalar: 2.8,
                clutter_version: 0,
                observed_source_flag_64: 0,
            },
        },
    );
    let mut request = [0; 64];
    request[8..16].copy_from_slice(&wall.0.to_le_bytes());
    for (offset, x) in [(40, 0.5f32), (48, 1.), (52, 0.5)] {
        request[offset..offset + 4].copy_from_slice(&x.to_le_bytes());
    }
    request[56] = 1;
    emit(
        &mut app,
        CreateRoof(RoofCreateRequestObserved::from_observed(request)),
    );
    let mut raster = vec![0; 512 * 512];
    for z in 246..269 {
        for x in 246..265 {
            raster[z * 512 + x] = 255;
        }
    }
    emit(
        &mut app,
        SetWallPathRasters(
            PathRasters512::from_encoded(
                [
                    Arc::from(raster),
                    Arc::from(vec![0; 512 * 512]),
                    Arc::from(vec![0; 512 * 512]),
                    Arc::from(vec![0; 512 * 512]),
                ],
                true,
            )
            .unwrap(),
        ),
    );
    checked_step(&mut app, 0.);
    assert!(
        !app.world()
            .resource::<wall_segments::WallProcessedMaskRegistry>()
            .records[&wall]
            .output
            .steps
            .is_empty()
    );
    assert!(
        app.world()
            .resource::<WallPlatformStairState>()
            .pending
            .contains(&wall)
    );
    app.world_mut()
        .resource_mut::<PlatformDecoratorRows>()
        .rows
        .insert(wall, vec![]);
    checked_step(&mut app, 0.);
    let entity = app.world().resource::<WallPlatformStairEntities>().0[&wall];
    let stairs = app.world().get::<WallCpuPlatformStairs>(entity).unwrap();
    assert!(!stairs.geometry.cells.is_empty());
    assert!(!stairs.holes.is_empty());
    let generation = stairs.generation;
    let stats = app
        .world()
        .resource::<wall_segments::WallPathMaterialStats>()
        .0[&wall];
    assert_eq!(
        stats.stairs_length.to_bits(),
        stairs.geometry.stairs_length.to_bits()
    );
    let prepared = app.world().resource::<WallPreparedPaths>().records[&wall]
        .path
        .length;
    assert_eq!(
        stats.regular_path_length.to_bits(),
        (prepared - stairs.geometry.stairs_length).to_bits()
    );
    assert!(
        app.world()
            .resource::<openings::WallOpenings>()
            .storage
            .contains_key(&wall)
    );
    let with_holes = app
        .world()
        .get::<WallCpuBricks>(app.world().resource::<WallBrickEntities>().0[&wall])
        .unwrap()
        .instances
        .clone();
    checked_step(&mut app, 0.);
    assert_eq!(
        app.world()
            .get::<WallCpuPlatformStairs>(entity)
            .unwrap()
            .generation,
        generation
    );
    // Release retained terrain input events, then mutate the public Arc in
    // place. Stair fingerprints must still observe the new raster contents.
    checked_step(&mut app, 0.);
    Arc::make_mut(
        &mut app
            .world_mut()
            .resource_mut::<WallCollectionState>()
            .terrain
            .as_mut()
            .unwrap()
            .encoded,
    )[0] = 1;
    checked_step(&mut app, 0.);
    assert_eq!(
        app.world()
            .get::<WallCpuPlatformStairs>(entity)
            .unwrap()
            .generation,
        generation + 1
    );
    let mut row = [0; 80];
    row[0x4e] = 0x3f;
    row[0xc..0x10].copy_from_slice(&1f32.to_le_bytes());
    row[0x3c..0x40].copy_from_slice(&100f32.to_le_bytes());
    app.world_mut()
        .resource_mut::<PlatformDecoratorRows>()
        .rows
        .insert(wall, vec![country_core::systems::wall::construction::platform_stairs::WallPlatformDecoratorObserved(row)]);
    checked_step(&mut app, 0.);
    assert!(
        app.world()
            .get::<WallCpuPlatformStairs>(entity)
            .unwrap()
            .holes
            .is_empty()
    );
    let without_holes = &app
        .world()
        .get::<WallCpuBricks>(app.world().resource::<WallBrickEntities>().0[&wall])
        .unwrap()
        .instances;
    assert_ne!(without_holes, &with_holes);
    emit(&mut app, SetPlatformDecoratorRows { wall, rows: vec![] });
    checked_step(&mut app, 0.);
    assert_eq!(
        app.world()
            .get::<WallCpuBricks>(app.world().resource::<WallBrickEntities>().0[&wall])
            .unwrap()
            .instances,
        with_holes
    );
    emit(&mut app, DeleteRoof(wall));
    checked_step(&mut app, 0.5);
    assert!(
        !app.world()
            .resource::<WallPlatformStairEntities>()
            .0
            .contains_key(&wall)
    );
    assert!(
        !app.world()
            .resource::<WallPlatformStairState>()
            .holes
            .contains_key(&wall)
    );
    assert_eq!(
        app.world()
            .resource::<wall_segments::WallPathMaterialStats>()
            .0[&wall]
            .stairs_length,
        0.
    );
    assert!(app.world().get_entity(entity).is_err());
    emit(&mut app, RemoveWall(wall));
    checked_step(&mut app, 0.);
    assert!(
        !app.world()
            .resource::<PlatformDecoratorRows>()
            .rows
            .contains_key(&wall)
    );
}

#[test]
fn later_recording_does_not_admit_old_unrecorded_authority_or_roof_observations() {
    use tg_building_runtime::history::*;
    let mut app = building_app();
    configure(&mut app);
    app.world_mut()
        .resource_mut::<WallAuthoringState>()
        .dev_override_is_demo_mode = Some(false);
    let shape = AuthoringShape::Circle(PreviewCircleObserved {
        center: [0., 0.],
        radius: 2.,
        angle: 0.,
    });
    emit(
        &mut app,
        CreateBuildingFromPlacement {
            shape,
            placement: placement(2., 0., None),
        },
    );
    checked_step(&mut app, 0.5);
    emit(&mut app, RemoveAuthoredWall(WallId(100)));
    checked_step(&mut app, 0.5);
    assert!(
        !app.world()
            .resource::<WallAuthoringState>()
            .history_metadata
            .contains_key(&WallId(100))
    );
    assert_eq!(
        app.world()
            .resource::<roof_invalid::RoofInvalidNotificationState>()
            .history_pending
            .len(),
        1
    );
    emit(&mut app, HistoryAction::ResetAfterFutureClear);
    emit(&mut app, HistoryAction::Start);
    emit(
        &mut app,
        CreateBuildingFromPlacement {
            shape,
            placement: placement(3., 0., None),
        },
    );
    emit(&mut app, HistoryAction::Finish);
    checked_step(&mut app, 0.5);
    let history = app
        .world()
        .resource::<BuildingHistory>()
        .transactions
        .as_ref()
        .unwrap();
    assert_eq!(history.ledger().entries.len(), 1);
    assert_eq!(history.ledger().entries[0].edits.len(), 2);
    assert_eq!(
        app.world()
            .resource::<WallAuthoringState>()
            .pending_creation_history
            .len(),
        0
    );
    emit(&mut app, HistoryAction::Undo { instant: true });
    checked_step(&mut app, 0.5);
    checked_step(&mut app, 0.5);
    checked_step(&mut app, 0.5);
    assert!(
        app.world()
            .resource::<WallCollectionState>()
            .raw_sources
            .is_empty()
    );
    assert_eq!(
        app.world().resource::<WallAuthoringState>().id_counter,
        Some(101)
    );
}

#[test]
fn unresolved_history_owner_preserves_the_complete_queue_before_any_ledger_mutation() {
    use country_core::resources::history::building_transactions::BuildingEdit;
    use country_core::resources::history::building_transactions::RoofHistoryEdit;
    use tg_building_runtime::history::*;
    let mut app = building_app();
    configure(&mut app);
    checked_step(&mut app, 0.);
    emit(&mut app, HistoryAction::ResetAfterFutureClear);
    emit(&mut app, HistoryAction::Start);
    checked_step(&mut app, 0.);
    app.world_mut().resource_mut::<BuildingHistoryRequests>().0 = vec![
        BuildingEdit::Roof(RoofHistoryEdit::direction(7, 0, 1)),
        BuildingEdit::UnresolvedOwner { observed_tag: 9 },
        BuildingEdit::Roof(RoofHistoryEdit::direction(8, 0, 1)),
    ];
    emit(&mut app, HistoryAction::Finish);
    app.update();
    assert_eq!(app.world().resource::<BuildingHistoryRequests>().0.len(), 3);
    let history = app
        .world()
        .resource::<BuildingHistory>()
        .transactions
        .as_ref()
        .unwrap();
    assert!(history.pending().unwrap().edits.is_empty());
    assert!(history.ledger().entries.is_empty());
    assert_eq!(app.world().resource::<BuildingReport>().faults.len(), 1);
    app.world_mut()
        .resource_mut::<BuildingHistoryRequests>()
        .0
        .remove(1);
    emit(&mut app, HistoryAction::Finish);
    checked_step(&mut app, 0.);
    assert!(
        app.world()
            .resource::<BuildingHistoryRequests>()
            .0
            .is_empty()
    );
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
        2
    );
}

#[test]
fn actual_bulldoze_animation_and_shrink_delete_history_close_the_wall_roof_geometry_cycle() {
    use tg_building_runtime::{history::*, wall_bulldoze::*};
    let mut app = building_app();
    configure(&mut app);
    app.world_mut()
        .resource_mut::<WallAuthoringState>()
        .dev_override_is_demo_mode = Some(false);
    checked_step(&mut app, 0.);
    emit(&mut app, HistoryAction::ResetAfterFutureClear);
    emit(&mut app, HistoryAction::Start);
    emit(
        &mut app,
        CreateBuildingFromPlacement {
            shape: AuthoringShape::Rectangle(PreviewRectangleObserved {
                axes: [1., 0.],
                center: [0., 0.],
                dimensions: [4., 6.],
            }),
            placement: placement(3.5, 0.75, None),
        },
    );
    emit(&mut app, HistoryAction::Finish);
    checked_step(&mut app, 0.);
    let wall = WallId(100);
    let original_source = app.world().resource::<WallCollectionState>().raw_sources[&wall].clone();
    let original = app
        .world()
        .get::<WallCpuBricks>(app.world().resource::<WallBrickEntities>().0[&wall])
        .unwrap()
        .instances
        .clone();
    app.world_mut()
        .resource_mut::<WallBulldozeTiming>()
        .observed_game_time_58 = Some(0.05);
    emit(&mut app, HistoryAction::Start);
    emit(
        &mut app,
        BulldozeWall {
            wall,
            low: 0.1,
            high: 0.3,
            anchor: [2., 1., 3.],
        },
    );
    emit(&mut app, HistoryAction::Finish);
    checked_step(&mut app, 0.05);
    assert_eq!(
        app.world().resource::<WallBulldozeState>().commands[0].phase,
        0.25
    );
    assert_eq!(
        app.world().resource::<WallCollectionState>().raw_sources[&wall]
            .shape
            .tag(),
        2
    );
    assert!(
        app.world().resource::<WallCollectionState>().raw_sources[&wall]
            .curve
            .length
            < original_source.curve.length
    );
    assert_eq!(
        app.world()
            .resource::<BuildingHistory>()
            .transactions
            .as_ref()
            .unwrap()
            .ledger()
            .entries[1]
            .edits
            .len(),
        2
    );
    for _ in 0..3 {
        checked_step(&mut app, 0.05);
    }
    assert!(
        app.world()
            .resource::<WallBulldozeState>()
            .commands
            .is_empty()
    );
    assert!(
        !app.world()
            .resource::<WallCollectionState>()
            .raw_sources
            .contains_key(&wall)
    );
    assert!(
        !app.world()
            .resource::<WallBrickEntities>()
            .0
            .contains_key(&wall)
    );
    checked_step(&mut app, 0.5);
    emit(&mut app, HistoryAction::Undo { instant: false });
    checked_step(&mut app, 0.1);
    assert_eq!(
        app.world().resource::<WallCollectionState>().raw_sources[&wall]
            .shape
            .tag(),
        2
    );
    assert!(app.world().resource::<WallRoofMap>().entity(wall).is_none());
    for _ in 0..8 {
        if app
            .world()
            .resource::<RoofHistoryReplay>()
            .buckets
            .is_empty()
        {
            break;
        }
        checked_step(&mut app, 0.1);
    }
    assert!(
        app.world()
            .resource::<RoofHistoryReplay>()
            .buckets
            .is_empty()
    );
    assert!(app.world().resource::<WallRoofMap>().entity(wall).is_some());
    assert_eq!(
        app.world().resource::<WallCollectionState>().raw_sources[&wall]
            .curve
            .points,
        original_source.curve.points
    );
    assert_eq!(
        app.world()
            .get::<WallCpuBricks>(app.world().resource::<WallBrickEntities>().0[&wall])
            .unwrap()
            .instances,
        original
    );
    assert_eq!(
        app.world().resource::<WallAuthoringState>().id_counter,
        Some(100)
    );
    emit(&mut app, HistoryAction::Redo { instant: false });
    checked_step(&mut app, 0.1);
    assert!(
        app.world()
            .resource::<WallCollectionState>()
            .raw_sources
            .contains_key(&wall)
    );
    for _ in 0..8 {
        if app
            .world()
            .resource::<RoofHistoryReplay>()
            .buckets
            .is_empty()
        {
            break;
        }
        checked_step(&mut app, 0.1);
    }
    assert!(
        app.world()
            .resource::<RoofHistoryReplay>()
            .buckets
            .is_empty()
    );
    assert!(
        !app.world()
            .resource::<WallCollectionState>()
            .raw_sources
            .contains_key(&wall)
    );
    assert!(
        !app.world()
            .resource::<WallBrickEntities>()
            .0
            .contains_key(&wall)
    );
    assert!(
        app.world()
            .resource::<roof_invalid::RoofInvalidNotificationState>()
            .recording_pending
            .is_empty()
    );
}

#[test]
fn active_bulldoze_missing_metadata_preserves_the_command_and_actual_source() {
    use tg_building_runtime::{history::*, wall_bulldoze::*};
    let mut app = building_app();
    configure(&mut app);
    app.world_mut()
        .resource_mut::<WallAuthoringState>()
        .dev_override_is_demo_mode = Some(false);
    emit(&mut app, HistoryAction::ResetAfterFutureClear);
    emit(&mut app, HistoryAction::Start);
    emit(
        &mut app,
        CreateBuildingFromPlacement {
            shape: AuthoringShape::Circle(PreviewCircleObserved {
                center: [0., 0.],
                radius: 2.,
                angle: 0.,
            }),
            placement: placement(3., 0., None),
        },
    );
    emit(&mut app, HistoryAction::Finish);
    checked_step(&mut app, 0.);
    app.world_mut()
        .resource_mut::<WallBulldozeTiming>()
        .observed_game_time_58 = Some(0.05);
    emit(&mut app, HistoryAction::Start);
    emit(
        &mut app,
        BulldozeWall {
            wall: WallId(100),
            low: 0.1,
            high: 0.4,
            anchor: [0.; 3],
        },
    );
    emit(&mut app, HistoryAction::Finish);
    checked_step(&mut app, 0.05);
    let metadata = app
        .world_mut()
        .resource_mut::<WallAuthoringState>()
        .history_metadata
        .remove(&WallId(100))
        .unwrap();
    app.update();
    assert_eq!(app.world().resource::<BuildingReport>().faults.len(), 1);
    assert_eq!(
        app.world().resource::<WallBulldozeState>().commands.len(),
        1
    );
    assert_eq!(
        app.world().resource::<WallBulldozeState>().commands[0].phase,
        0.25
    );
    assert!(
        app.world()
            .resource::<WallCollectionState>()
            .raw_sources
            .contains_key(&WallId(100))
    );
    app.world_mut()
        .resource_mut::<WallAuthoringState>()
        .history_metadata
        .insert(WallId(100), metadata);
    checked_step(&mut app, 0.05);
    assert_eq!(
        app.world().resource::<WallBulldozeState>().commands[0].phase,
        0.5
    );
}

#[test]
fn multiple_free_wall_changes_retain_every_previous_and_next_snapshot() {
    use country_core::resources::history::wall_edit::WallAuthoritySnapshot;
    use country_core::resources::history::wall_edit::WallHistoryEdit;
    use tg_building_runtime::wall_pending_changed::*;
    let mut app = building_app();
    configure(&mut app);
    app.world_mut()
        .resource_mut::<WallAuthoringState>()
        .dev_override_is_demo_mode = Some(false);
    for x in [-8., 8.] {
        emit(
            &mut app,
            CreateBuildingFromPlacement {
                shape: AuthoringShape::Circle(PreviewCircleObserved {
                    center: [x, 0.],
                    radius: 2.,
                    angle: 0.,
                }),
                placement: placement(2., 0., None),
            },
        );
    }
    checked_step(&mut app, 0.5);
    for (wall, height) in [(WallId(100), 3.), (WallId(100), 4.), (WallId(101), 5.)] {
        let mut source = app.world().resource::<WallCollectionState>().raw_sources[&wall].clone();
        source.extra = height;
        let metadata = app
            .world()
            .resource::<WallAuthoringState>()
            .history_metadata[&wall];
        emit(
            &mut app,
            ApplyObservedFreeWallChange {
                wall,
                next: WallAuthoritySnapshot { source, metadata },
                extend_start: true,
            },
        );
    }
    checked_step(&mut app, 0.5);
    let state = app.world().resource::<WallPendingChangedState>();
    assert!(state.pending.is_none());
    let snapshots: Vec<_> = state
        .unrecorded_history
        .iter()
        .map(|edit| {
            let WallHistoryEdit::StateChange {
                wall,
                previous,
                next,
                ..
            } = &edit.groups[0][0]
            else {
                panic!("expected state change")
            };
            (*wall, previous.source.extra, next.source.extra)
        })
        .collect();
    assert_eq!(
        snapshots,
        vec![
            (WallId(100), 2., 3.),
            (WallId(100), 3., 4.),
            (WallId(101), 2., 5.)
        ]
    );
}

#[test]
fn rectangle_edits_process_each_wall_in_the_same_update() {
    let mut app = building_app();
    configure(&mut app);
    for x in [-8., 8.] {
        emit(
            &mut app,
            CreateBuildingFromPlacement {
                shape: AuthoringShape::Rectangle(PreviewRectangleObserved {
                    axes: [1., 0.],
                    center: [x, 0.],
                    dimensions: [4., 6.],
                }),
                placement: placement(2., 0., None),
            },
        );
    }
    checked_step(&mut app, 0.5);
    for (wall, center) in [(WallId(100), -10f32), (WallId(101), 10.)] {
        let mut event = [0; 64];
        event[8..16].copy_from_slice(&wall.0.to_le_bytes());
        for (offset, value) in [(16, 1f32), (24, center), (32, 6.), (36, 8.)] {
            event[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
        }
        emit(&mut app, EditAuthoredRectangle(event));
    }
    checked_step(&mut app, 0.5);
    let state = app.world().resource::<WallAuthoringState>();
    assert_eq!(state.rectangle_edits_this_frame.len(), 2);
    for (wall, center) in [(WallId(100), -10f32), (WallId(101), 10.)] {
        assert!(state.updated_sources.contains(&wall));
        let source = &app.world().resource::<WallCollectionState>().raw_sources[&wall];
        let bounds = source
            .curve
            .points
            .iter()
            .map(|p| p[0])
            .fold([f32::INFINITY, f32::NEG_INFINITY], |b, x| {
                [b[0].min(x), b[1].max(x)]
            });
        assert_eq!(bounds, [center - 3., center + 3.]);
    }
}

#[test]
fn observed_archive_keeps_the_supplied_culling_policy() {
    use country_core::resources::walls::decorator_archivist::DecoratorCullPolicy;
    use country_core::resources::walls::decorator_archivist::changes::DecoratorArchivist;
    use tg_building_runtime::wall_decorator_archivist::*;
    let mut app = building_app();
    emit(
        &mut app,
        ObserveDecoratorArchive(Arc::new(DecoratorArchivist {
            walls: Default::default(),
            special1: None,
            special2: None,
            collision_a: Default::default(),
            collision_b: Default::default(),
            culling: DecoratorCullPolicy {
                allowed_ids: [7].into(),
                allow_any: false,
            },
        })),
    );
    checked_step(&mut app, 0.);
    let archive = app
        .world()
        .resource::<WallDecoratorArchiveState>()
        .archive
        .as_ref()
        .unwrap();
    assert!(archive.is_decorator_cull_allowed(7));
    assert!(!archive.is_decorator_cull_allowed(8));
}

#[test]
fn native_opening_bindings_reject_shared_opening_identity() {
    use tg_building_runtime::{decorator_info_binding::*, openings::OpeningId};
    let mut app = building_app();
    for decorator_id in [7, 8] {
        emit(
            &mut app,
            ObserveNativeDecoratorOpeningId {
                decorator_id,
                opening_id: OpeningId(42),
            },
        );
    }
    app.update();
    assert_eq!(
        app.world().resource::<NativeDecoratorOpeningIds>().0.len(),
        1
    );
    assert_eq!(
        app.world().resource::<NativeDecoratorOpeningIds>().0[&7],
        OpeningId(42)
    );
    assert_eq!(app.world().resource::<BuildingReport>().faults.len(), 1);
}

#[test]
fn observed_places_publish_legacy_bindings_in_the_same_update() {
    use country_core::resources::walls::decorator::anchors::DecoratorPlaceRequestObserved;
    use country_core::resources::walls::decorator::anchors::decorator_source_from_place;
    use country_core::resources::walls::decorator::placement::WallDecoratorRequest;
    use country_core::resources::walls::decorator_archivist::changes::DecoratorStorage;
    use country_core::resources::walls::wall_holes::opening_rules::OpeningFamily;
    use tg_building_runtime::{
        decorator_assets::DecoratorAssetLibrary, decorator_info_binding::*, decorator_placement::*,
        openings::*, wall_decorator_archivist::*,
    };
    let mut app = building_app();
    configure(&mut app);
    app.world_mut()
        .resource_mut::<WallAuthoringState>()
        .dev_override_is_demo_mode = Some(false);
    emit(
        &mut app,
        CreateBuildingFromPlacement {
            shape: AuthoringShape::Circle(PreviewCircleObserved {
                center: [0., 0.],
                radius: 2.,
                angle: 0.,
            }),
            placement: placement(2., 0., None),
        },
    );
    checked_step(&mut app, 0.5);
    app.world_mut()
        .resource_mut::<DecoratorAssetLibrary>()
        .load_b_wall_opening_assets(
            &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/assets"),
        )
        .unwrap();
    let wall = WallId(100);
    let mut bytes = [0; 88];
    bytes[8..16].copy_from_slice(&wall.0.to_le_bytes());
    bytes[0x4c..0x50].copy_from_slice(&7u32.to_le_bytes());
    bytes[0x55] = 0x38;
    bytes[0x56] = 1;
    let request = DecoratorPlaceRequestObserved(bytes);
    let source = decorator_source_from_place(&request, false, 0, false).unwrap();
    emit(
        &mut app,
        ObserveDecoratorStorage(Arc::new(DecoratorStorage::default())),
    );
    emit(
        &mut app,
        ObserveDecoratorOpeningBinding {
            decorator_id: 7,
            observed_info: *source.info.bytes(),
            opening: PlaceWallOpening {
                id: OpeningId(42),
                wall,
                request: WallDecoratorRequest {
                    family: OpeningFamily::CottageWindow,
                    rank: 1,
                    world_position: [2., 1., 0.],
                },
            },
        },
    );
    emit(&mut app, ObserveDecoratorStorageReadonly(false));
    emit(
        &mut app,
        ApplyObservedDecoratorPlace {
            request,
            observed_caller_byte: 0,
        },
    );
    checked_step(&mut app, 0.);
    assert_eq!(
        app.world()
            .resource::<NativeDecoratorPlaceState>()
            .applied_this_frame,
        vec![7]
    );
    assert!(
        app.world()
            .resource::<WallOpeningIntents>()
            .0
            .contains_key(&OpeningId(42))
    );
    assert!(
        app.world()
            .resource::<OpeningEntities>()
            .0
            .contains_key(&OpeningId(42))
    );
    emit(
        &mut app,
        ObserveNativeDecoratorOpeningId {
            decorator_id: 7,
            opening_id: OpeningId(42),
        },
    );
    checked_step(&mut app, 0.);
    let native_entity = app.world().resource::<OpeningEntities>().0[&OpeningId(42)];
    let generation = app
        .world()
        .get::<OpeningState>(native_entity)
        .unwrap()
        .generation;
    checked_step(&mut app, 0.);
    assert_eq!(
        app.world()
            .get::<OpeningState>(native_entity)
            .unwrap()
            .generation,
        generation
    );
    emit(&mut app, RecalculateWallHoles(wall));
    checked_step(&mut app, 0.);
    assert_eq!(
        app.world()
            .get::<OpeningState>(native_entity)
            .unwrap()
            .generation,
        generation + 1
    );
    emit(
        &mut app,
        wall_height::SetWallSourceHeight { wall, height: 3. },
    );
    checked_step(&mut app, 0.);
    assert!(
        app.world()
            .get::<OpeningState>(native_entity)
            .unwrap()
            .generation
            > generation + 1
    );
    emit(&mut app, ForgetNativeDecoratorOpeningId(7));
    checked_step(&mut app, 0.);
    let mut conflicting = bytes;
    conflicting[0x4c..0x50].copy_from_slice(&8u32.to_le_bytes());
    let request = DecoratorPlaceRequestObserved(conflicting);
    let source = decorator_source_from_place(&request, false, 0, false).unwrap();
    emit(
        &mut app,
        ObserveDecoratorOpeningBinding {
            decorator_id: 8,
            observed_info: *source.info.bytes(),
            opening: PlaceWallOpening {
                id: OpeningId(42),
                wall,
                request: WallDecoratorRequest {
                    family: OpeningFamily::CottageWindow,
                    rank: 1,
                    world_position: [2., 1., 0.],
                },
            },
        },
    );
    emit(
        &mut app,
        ApplyObservedDecoratorPlace {
            request,
            observed_caller_byte: 0,
        },
    );
    app.update();
    assert_eq!(app.world().resource::<BuildingReport>().faults.len(), 2);
    assert!(
        !app.world()
            .resource::<OpeningEntities>()
            .0
            .contains_key(&OpeningId(42))
    );
}

#[test]
fn rejected_free_wall_source_does_not_commit_metadata_or_history() {
    use country_core::resources::history::wall_edit::WallAuthoritySnapshot;
    use tg_building_runtime::wall_pending_changed::*;
    let mut app = building_app();
    configure(&mut app);
    app.world_mut()
        .resource_mut::<WallAuthoringState>()
        .dev_override_is_demo_mode = Some(false);
    emit(
        &mut app,
        CreateBuildingFromPlacement {
            shape: AuthoringShape::Circle(PreviewCircleObserved {
                center: [0., 0.],
                radius: 2.,
                angle: 0.,
            }),
            placement: placement(2., 0., None),
        },
    );
    checked_step(&mut app, 0.5);
    let wall = WallId(100);
    let mut source = app.world().resource::<WallCollectionState>().raw_sources[&wall].clone();
    let original = app
        .world()
        .resource::<WallAuthoringState>()
        .history_metadata[&wall];
    let mut metadata = original;
    metadata.source_69 = 77;
    source.extra = f32::NAN;
    emit(
        &mut app,
        ApplyObservedFreeWallChange {
            wall,
            next: WallAuthoritySnapshot { source, metadata },
            extend_start: true,
        },
    );
    app.update();
    assert_eq!(app.world().resource::<BuildingReport>().faults.len(), 1);
    assert_eq!(
        app.world()
            .resource::<WallAuthoringState>()
            .history_metadata[&wall]
            .source_69,
        original.source_69
    );
    let state = app.world().resource::<WallPendingChangedState>();
    assert!(state.updated_this_frame.is_empty());
    assert!(state.unrecorded_history.is_empty());
    assert_eq!(
        app.world().resource::<WallCollectionState>().raw_sources[&wall].extra,
        2.
    );
}

#[test]
fn a_stale_creation_counter_cannot_overwrite_existing_authority() {
    let mut app = building_app();
    configure(&mut app);
    let create = |radius| CreateBuildingFromPlacement {
        shape: AuthoringShape::Circle(PreviewCircleObserved {
            center: [0., 0.],
            radius,
            angle: 0.,
        }),
        placement: placement(2., 0., None),
    };
    emit(&mut app, create(2.));
    checked_step(&mut app, 0.5);
    let original = app.world().resource::<WallCollectionState>().raw_sources[&WallId(100)]
        .curve
        .points
        .clone();
    app.world_mut()
        .resource_mut::<WallAuthoringState>()
        .id_counter = Some(99);
    emit(&mut app, create(4.));
    app.update();
    assert_eq!(app.world().resource::<BuildingReport>().faults.len(), 1);
    assert!(
        app.world()
            .resource::<WallAuthoringState>()
            .created_this_frame
            .is_empty()
    );
    assert_eq!(
        app.world().resource::<WallCollectionState>().raw_sources[&WallId(100)]
            .curve
            .points,
        original
    );
    app.world_mut()
        .resource_mut::<WallAuthoringState>()
        .id_counter = Some(u64::MAX);
    emit(&mut app, create(4.));
    app.update();
    assert_eq!(
        app.world().resource::<WallAuthoringState>().id_counter,
        Some(u64::MAX)
    );
    assert_eq!(
        app.world()
            .resource::<WallCollectionState>()
            .raw_sources
            .len(),
        1
    );
    assert!(
        app.world()
            .resource::<BuildingReport>()
            .faults
            .iter()
            .any(|fault| fault.contains("ID space is exhausted"))
    );
}

#[test]
fn bulldoze_copies_only_target_authority_and_no_sources_on_idle_updates() {
    use tg_building_runtime::{history::*, wall_bulldoze::*};
    let mut app = building_app();
    configure(&mut app);
    app.world_mut()
        .resource_mut::<WallAuthoringState>()
        .dev_override_is_demo_mode = Some(false);
    for x in [-8., 8.] {
        emit(
            &mut app,
            CreateBuildingFromPlacement {
                shape: AuthoringShape::Circle(PreviewCircleObserved {
                    center: [x, 0.],
                    radius: 2.,
                    angle: 0.,
                }),
                placement: placement(3., 0., None),
            },
        );
    }
    checked_step(&mut app, 0.);
    assert_eq!(
        app.world()
            .resource::<WallBulldozeState>()
            .authority_snapshots_this_frame,
        0
    );
    checked_step(&mut app, 0.);
    assert_eq!(
        app.world()
            .resource::<WallBulldozeState>()
            .authority_snapshots_this_frame,
        0
    );
    let unrelated = app.world().resource::<WallCollectionState>().raw_sources[&WallId(101)]
        .curve
        .points
        .clone();
    app.world_mut()
        .resource_mut::<WallBulldozeTiming>()
        .observed_game_time_58 = Some(0.05);
    emit(&mut app, HistoryAction::ResetAfterFutureClear);
    emit(&mut app, HistoryAction::Start);
    emit(
        &mut app,
        BulldozeWall {
            wall: WallId(100),
            low: 0.1,
            high: 0.4,
            anchor: [0.; 3],
        },
    );
    emit(&mut app, HistoryAction::Finish);
    checked_step(&mut app, 0.05);
    assert_eq!(
        app.world()
            .resource::<WallBulldozeState>()
            .authority_snapshots_this_frame,
        1
    );
    assert_eq!(
        app.world().resource::<WallCollectionState>().raw_sources[&WallId(101)]
            .curve
            .points,
        unrelated
    );
    checked_step(&mut app, 0.05);
    assert_eq!(
        app.world()
            .resource::<WallBulldozeState>()
            .authority_snapshots_this_frame,
        1
    );
    assert_eq!(
        app.world().resource::<WallBulldozeState>().commands[0].phase,
        0.5
    );
}

#[test]
fn incomplete_deletion_history_rejects_only_the_affected_wall() {
    use tg_building_runtime::history::*;
    let mut app = building_app();
    configure(&mut app);
    app.world_mut()
        .resource_mut::<WallAuthoringState>()
        .dev_override_is_demo_mode = Some(false);
    for x in [-8., 8.] {
        emit(
            &mut app,
            CreateBuildingFromPlacement {
                shape: AuthoringShape::Circle(PreviewCircleObserved {
                    center: [x, 0.],
                    radius: 2.,
                    angle: 0.,
                }),
                placement: placement(3., 0., None),
            },
        );
    }
    checked_step(&mut app, 0.5);
    app.world_mut()
        .resource_mut::<WallAuthoringState>()
        .history_metadata
        .remove(&WallId(100));
    emit(&mut app, HistoryAction::ResetAfterFutureClear);
    emit(&mut app, HistoryAction::Start);
    emit(&mut app, RemoveAuthoredWall(WallId(100)));
    emit(&mut app, RemoveAuthoredWall(WallId(101)));
    emit(&mut app, HistoryAction::Finish);
    app.world_mut().resource_mut::<BuildingFrame>().delta_time = 0.5;
    app.update();
    assert_eq!(app.world().resource::<BuildingReport>().faults.len(), 1);
    let source = app.world().resource::<WallCollectionState>();
    assert!(source.raw_sources.contains_key(&WallId(100)));
    assert!(!source.raw_sources.contains_key(&WallId(101)));
    assert!(
        app.world()
            .resource::<WallAuthoringState>()
            .appearance
            .contains_key(&WallId(100))
    );
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
        2
    );
}

#[test]
fn native_identity_changes_validate_final_ownership_and_removal_dominates() {
    use tg_building_runtime::{decorator_info_binding::*, openings::OpeningId};
    let mut app = building_app();
    emit(
        &mut app,
        ObserveNativeDecoratorOpeningId {
            decorator_id: 7,
            opening_id: OpeningId(42),
        },
    );
    checked_step(&mut app, 0.);
    emit(&mut app, ForgetNativeDecoratorOpeningId(7));
    emit(
        &mut app,
        ObserveNativeDecoratorOpeningId {
            decorator_id: 8,
            opening_id: OpeningId(42),
        },
    );
    emit(
        &mut app,
        ObserveNativeDecoratorOpeningId {
            decorator_id: 7,
            opening_id: OpeningId(99),
        },
    );
    emit(
        &mut app,
        ObserveNativeDecoratorOpeningId {
            decorator_id: 9,
            opening_id: OpeningId(43),
        },
    );
    checked_step(&mut app, 0.);
    let ids = &app.world().resource::<NativeDecoratorOpeningIds>().0;
    assert!(!ids.contains_key(&7));
    assert_eq!(ids[&8], OpeningId(42));
    emit(
        &mut app,
        ObserveNativeDecoratorOpeningId {
            decorator_id: 8,
            opening_id: OpeningId(43),
        },
    );
    emit(
        &mut app,
        ObserveNativeDecoratorOpeningId {
            decorator_id: 9,
            opening_id: OpeningId(42),
        },
    );
    checked_step(&mut app, 0.);
    let ids = &app.world().resource::<NativeDecoratorOpeningIds>().0;
    assert_eq!(ids[&8], OpeningId(43));
    assert_eq!(ids[&9], OpeningId(42));
}

#[test]
fn permanent_placement_errors_do_not_block_valid_requests_but_readonly_retries() {
    use country_core::resources::walls::decorator::anchors::DecoratorPlaceRequestObserved;
    use country_core::resources::walls::decorator_archivist::changes::DecoratorStorage;
    use tg_building_runtime::{decorator_info_binding::*, wall_decorator_archivist::*};
    let mut app = building_app();
    app.world_mut()
        .resource_mut::<WallAuthoringState>()
        .dev_override_is_demo_mode = Some(false);
    emit(
        &mut app,
        ObserveDecoratorStorage(Arc::new(DecoratorStorage::default())),
    );
    emit(&mut app, ObserveDecoratorStorageReadonly(false));
    let mut bytes = [0; 88];
    bytes[8..16].copy_from_slice(&100u64.to_le_bytes());
    bytes[0x4c..0x50].copy_from_slice(&9u32.to_le_bytes());
    bytes[0x55] = 0x38;
    bytes[0x56] = 1;
    let mut bad_address = bytes;
    bad_address[..8].copy_from_slice(&3u64.to_le_bytes());
    for (request, observed_caller_byte) in [(bad_address, 0), (bytes, 2), (bytes, 0)] {
        emit(
            &mut app,
            ApplyObservedDecoratorPlace {
                request: DecoratorPlaceRequestObserved(request),
                observed_caller_byte,
            },
        );
    }
    app.update();
    let state = app.world().resource::<NativeDecoratorPlaceState>();
    assert_eq!(state.rejected_this_frame, 2);
    assert_eq!(state.applied_this_frame, vec![9]);
    assert!(state.queued.is_empty());
    emit(&mut app, ObserveDecoratorStorageReadonly(true));
    bytes[0x4c..0x50].copy_from_slice(&10u32.to_le_bytes());
    emit(
        &mut app,
        ApplyObservedDecoratorPlace {
            request: DecoratorPlaceRequestObserved(bytes),
            observed_caller_byte: 0,
        },
    );
    app.update();
    assert_eq!(
        app.world()
            .resource::<NativeDecoratorPlaceState>()
            .queued
            .len(),
        1
    );
    assert_eq!(
        app.world()
            .resource::<NativeDecoratorPlaceState>()
            .rejected_this_frame,
        0
    );
    emit(&mut app, ObserveDecoratorStorageReadonly(false));
    app.update();
    let state = app.world().resource::<NativeDecoratorPlaceState>();
    assert_eq!(state.applied_this_frame, vec![10]);
    assert!(state.queued.is_empty());
}

#[test]
fn invalid_authored_creation_preserves_ids_metadata_and_history() {
    let mut app = building_app();
    configure(&mut app);
    app.world_mut()
        .resource_mut::<WallAuthoringState>()
        .dev_override_is_demo_mode = Some(false);
    let shape = AuthoringShape::Circle(PreviewCircleObserved {
        center: [0., 0.],
        radius: 2.,
        angle: 0.,
    });
    emit(
        &mut app,
        CreateBuildingFromPlacement {
            shape,
            placement: placement(f32::NAN, 0., None),
        },
    );
    app.update();
    let authoring = app.world().resource::<WallAuthoringState>();
    assert_eq!(authoring.id_counter, Some(99));
    assert!(authoring.created_this_frame.is_empty());
    assert!(authoring.appearance.is_empty());
    assert!(authoring.history_metadata.is_empty());
    assert!(authoring.pending_creation_history.is_empty());
    assert!(
        app.world()
            .resource::<WallCollectionState>()
            .raw_sources
            .is_empty()
    );
    assert!(
        app.world()
            .resource::<BuildingReport>()
            .faults
            .iter()
            .any(|fault| fault.contains("Invalid authored building geometry"))
    );
    emit(
        &mut app,
        CreateBuildingFromPlacement {
            shape,
            placement: placement(2., 0., None),
        },
    );
    checked_step(&mut app, 0.);
    assert!(
        app.world()
            .resource::<PublicWalls>()
            .walls
            .contains_key(&WallId(100))
    );
    assert!(
        app.world()
            .resource::<WallAuthoringState>()
            .pending_creation_history
            .is_empty()
    );
    for _ in 0..3 {
        emit(
            &mut app,
            CreateBuildingFromPlacement {
                shape,
                placement: placement(2., 0., None),
            },
        );
        checked_step(&mut app, 0.);
        assert!(
            app.world()
                .resource::<WallAuthoringState>()
                .pending_creation_history
                .is_empty()
        );
    }
}

#[test]
fn duplicate_bulldoze_and_invalid_inputs_do_not_add_owners_or_poison_animation() {
    use tg_building_runtime::{history::*, wall_bulldoze::*};
    let mut app = building_app();
    configure(&mut app);
    app.world_mut()
        .resource_mut::<WallAuthoringState>()
        .dev_override_is_demo_mode = Some(false);
    emit(
        &mut app,
        CreateBuildingFromPlacement {
            shape: AuthoringShape::Circle(PreviewCircleObserved {
                center: [0., 0.],
                radius: 2.,
                angle: 0.,
            }),
            placement: placement(3., 0., None),
        },
    );
    checked_step(&mut app, 0.);
    app.world_mut()
        .resource_mut::<WallBulldozeTiming>()
        .observed_game_time_58 = Some(0.05);
    emit(&mut app, HistoryAction::ResetAfterFutureClear);
    emit(&mut app, HistoryAction::Start);
    let request = BulldozeWall {
        wall: WallId(100),
        low: 0.1,
        high: 0.4,
        anchor: [0.; 3],
    };
    emit(
        &mut app,
        BulldozeWall {
            anchor: [f32::NAN, 0., 0.],
            ..request
        },
    );
    emit(&mut app, request);
    emit(&mut app, request);
    emit(&mut app, HistoryAction::Finish);
    app.update();
    assert_eq!(app.world().resource::<BuildingReport>().faults.len(), 2);
    let state = app.world().resource::<WallBulldozeState>();
    assert_eq!(state.commands.len(), 1);
    let phase = state.commands[0].phase;
    let ledger = app
        .world()
        .resource::<BuildingHistory>()
        .transactions
        .as_ref()
        .unwrap()
        .ledger();
    assert_eq!(ledger.entries.len(), 1);
    assert_eq!(ledger.entries[0].edits.len(), 2);
    let points = app.world().resource::<WallCollectionState>().raw_sources[&request.wall]
        .curve
        .points
        .clone();
    app.world_mut()
        .resource_mut::<WallBulldozeTiming>()
        .observed_game_time_58 = Some(f32::NAN);
    app.update();
    assert_eq!(
        app.world().resource::<WallBulldozeState>().commands[0].phase,
        phase
    );
    assert_eq!(
        app.world().resource::<WallCollectionState>().raw_sources[&request.wall]
            .curve
            .points,
        points
    );
    assert!(
        app.world()
            .resource::<BuildingReport>()
            .faults
            .iter()
            .any(|fault| fault.contains("finite nonnegative time step"))
    );
}

#[test]
fn recording_creation_requires_history_metadata_before_allocating_an_id() {
    use tg_building_runtime::history::*;
    let mut app = building_app();
    configure(&mut app);
    emit(&mut app, HistoryAction::ResetAfterFutureClear);
    emit(&mut app, HistoryAction::Start);
    emit(
        &mut app,
        CreateBuildingFromPlacement {
            shape: AuthoringShape::Circle(PreviewCircleObserved {
                center: [0., 0.],
                radius: 2.,
                angle: 0.,
            }),
            placement: placement(2., 0., None),
        },
    );
    app.update();
    let authoring = app.world().resource::<WallAuthoringState>();
    assert_eq!(authoring.id_counter, Some(99));
    assert!(authoring.appearance.is_empty());
    assert!(authoring.pending_creation_history.is_empty());
    assert!(
        app.world()
            .resource::<WallCollectionState>()
            .raw_sources
            .is_empty()
    );
    assert!(
        app.world()
            .resource::<BuildingReport>()
            .faults
            .iter()
            .any(|fault| fault.contains("creation history requires demo-mode metadata"))
    );
}

#[test]
fn failed_wall_replay_rolls_back_progress_and_keeps_completed_disjoint_groups() {
    use country_core::resources::history::building_transactions::BuildingReplayBucket;
    use country_core::resources::history::building_transactions::BuildingReplayBucketSelection;
    use country_core::resources::history::wall_edit::WallAuthoritySnapshot;
    use country_core::resources::history::wall_edit::WallHistoryEdit;
    use country_core::systems::history::wall_replay_interpreter::WallReplayCommand;
    use tg_building_runtime::history::*;
    let mut app = building_app();
    configure(&mut app);
    app.world_mut()
        .resource_mut::<WallAuthoringState>()
        .dev_override_is_demo_mode = Some(false);
    for x in [-8., 8.] {
        emit(
            &mut app,
            CreateBuildingFromPlacement {
                shape: AuthoringShape::Circle(PreviewCircleObserved {
                    center: [x, 0.],
                    radius: 2.,
                    angle: 0.,
                }),
                placement: placement(3., 0., None),
            },
        );
    }
    checked_step(&mut app, 0.);
    let snapshot = |wall| WallAuthoritySnapshot {
        source: app.world().resource::<WallCollectionState>().raw_sources[&wall].clone(),
        metadata: app
            .world()
            .resource::<WallAuthoringState>()
            .history_metadata[&wall],
    };
    let previous = snapshot(WallId(100));
    let mut next = previous.clone();
    next.source.extra = 4.;
    let malformed_shape = previous.source.shape.clone();
    let command = |edit| WallReplayCommand {
        edit,
        timed: false,
        duration: 0.2,
        elapsed: 0.,
        undo: false,
        finished: false,
    };
    app.world_mut()
        .resource_mut::<RoofHistoryReplay>()
        .buckets
        .push_back(BuildingReplayBucketSelection {
        entry_index: 42,
        owner: BuildingReplayBucket::Wall(std::collections::VecDeque::from([
            vec![command(WallHistoryEdit::StateChange {
                wall: WallId(100),
                previous,
                next,
                observed_operation: [0; 12],
            })],
            vec![command(WallHistoryEdit::MorphPremadeShape(
                country_core::resources::history::wall_edit::MorphPremadeShapeEdit::new(
                    WallId(101),
                    country_core::systems::wall::valid_enclosure::ShapeParameters::from_snapshot(
                        &malformed_shape,
                    ),
                    country_core::systems::wall::valid_enclosure::ShapeParameters::from_snapshot(
                        &malformed_shape,
                    ),
                ),
            ))],
        ])),
    });
    app.update();
    assert!(
        app.world()
            .resource::<BuildingReport>()
            .faults
            .iter()
            .any(|fault| fault.contains("Wall bucket replay"))
    );
    assert_eq!(
        app.world().resource::<WallCollectionState>().raw_sources[&WallId(100)].extra,
        3.
    );
    assert_eq!(
        app.world()
            .resource::<BuildingHistory>()
            .replayed_this_frame,
        0
    );
    {
        let mut replay = app.world_mut().resource_mut::<RoofHistoryReplay>();
        let BuildingReplayBucket::Wall(groups) = &mut replay.buckets[0].owner else {
            panic!("expected wall replay");
        };
        assert_eq!(groups.len(), 2);
        assert!(
            groups
                .iter()
                .flatten()
                .all(|command| !command.finished && command.elapsed == 0.)
        );
        groups[1][0].edit = WallHistoryEdit::AdjustHeight {
            wall: WallId(101),
            from: 3.,
            to: 4.,
        };
    }
    checked_step(&mut app, 0.);
    assert_eq!(
        app.world().resource::<WallCollectionState>().raw_sources[&WallId(100)].extra,
        4.
    );
    assert_eq!(
        app.world()
            .resource::<BuildingHistory>()
            .replayed_this_frame,
        2
    );
    assert!(
        app.world()
            .resource::<RoofHistoryReplay>()
            .buckets
            .is_empty()
    );
}
