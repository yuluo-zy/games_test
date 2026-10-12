use bevy_ecs::prelude::*;
use std::sync::Arc;
use tg_building_runtime::{wall_authoring::*, wall_bricks::*, wall_paths::*, *};
use tiny_glade_reconstruction::{
    building_wall_authoring::*, building_wall_collection::TerrainHeightGrid512,
    building_wall_freeform_cache::*, building_wall_path_preparation::PathRasters512,
    building_wall_state::WallId,
};
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
        1
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
        1
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
    use tg_building_runtime::wall_platform_stairs::*;
    use tiny_glade_reconstruction::{
        building_roof_creation::RoofCreateRequestObserved,
        building_wall_collection::RawWallPublicationSource, building_wall_state::WallShapeSnapshot,
        surface::Curve2,
    };
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
    emit(&mut app, SetPlatformDecoratorRows { wall, rows: vec![] });
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
    let mut row = [0; 80];
    row[0x4e] = 0x3f;
    row[0xc..0x10].copy_from_slice(&1f32.to_le_bytes());
    row[0x3c..0x40].copy_from_slice(&100f32.to_le_bytes());
    emit(&mut app,SetPlatformDecoratorRows{wall,rows:vec![tiny_glade_reconstruction::building_wall_platform_stairs::WallPlatformDecoratorObserved(row)]});
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
        1
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
    use tg_building_runtime::history::*;
    use tiny_glade_reconstruction::building_history_transactions::{BuildingEdit, RoofHistoryEdit};
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
