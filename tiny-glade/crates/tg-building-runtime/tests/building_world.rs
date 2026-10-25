use bevy_ecs::prelude::*;
use country_core::resources::roofs::creation::RoofCreateRequestObserved;
use country_core::resources::walls::publication_fields::WallId;
use country_core::resources::walls::publication_fields::WallShapeSnapshot;
use std::sync::Arc;
use tg_building_runtime::*;
fn input(tag: u32, style: u8, y: f32) -> WallPublicationInput {
    let mut shape = [0; 28];
    shape[..4].copy_from_slice(&tag.to_le_bytes());
    let floats = if tag == 0 {
        [0., 0., 3., 0., 0., 0.]
    } else {
        [1., 0., 0., 0., 6., 8.]
    };
    for (i, v) in floats.iter().enumerate() {
        shape[4 + i * 4..8 + i * 4].copy_from_slice(&f32::to_le_bytes(*v));
    }
    WallPublicationInput {
        outline: Arc::new(vec![[0., y, 0.], [1., y + 1., 1.], [2., y, 0.]]),
        terrain_y: vec![0.; 3],
        source_curve_length: 3.,
        source_point_count: 3,
        shape: WallShapeSnapshot::from_observed(shape),
        style,
        extra: 3.,
        special: false,
    }
}
fn create(id: WallId, height: f32) -> CreateRoof {
    let mut record = [0; 64];
    record[8..16].copy_from_slice(&id.0.to_le_bytes());
    for (offset, v) in [(40, 0.5f32), (44, height), (48, 0.5), (52, 0.5)] {
        record[offset..offset + 4].copy_from_slice(&v.to_le_bytes());
    }
    CreateRoof(RoofCreateRequestObserved::from_observed(record))
}
fn emit<T: Event>(app: &mut bevy_app::App, event: T) {
    app.world_mut().resource_mut::<Events<T>>().send(event);
}
fn dt(app: &mut bevy_app::App, time: f32) {
    app.world_mut().resource_mut::<BuildingFrame>().delta_time = time;
}
fn clean(app: &bevy_app::App) {
    assert!(
        app.world().resource::<BuildingReport>().faults.is_empty(),
        "{:?}",
        app.world().resource::<BuildingReport>()
    );
}
#[test]
fn real_entities_publish_rebuild_edit_fade_despawn_and_recreate() {
    let mut app = building_app();
    let id = WallId(u64::MAX);
    emit(
        &mut app,
        PublishWall {
            wall: id,
            input: input(1, 0, 2.),
        },
    );
    emit(&mut app, create(id, 0.5));
    dt(&mut app, 0.5);
    app.update();
    clean(&app);
    let entity = app.world().resource::<WallRoofMap>().entity(id).unwrap();
    let world = app.world();
    assert!(world.get::<RoofSpire>(entity).is_some());
    assert!(world.get::<RoofSupportBeams>(entity).is_some());
    assert!(world.get::<RoofTiles>(entity).unwrap().allocation.is_none());
    assert!(
        world
            .get::<TriggerVolume>(entity)
            .unwrap()
            .0
            .triangles()
            .is_empty()
    );
    assert!(
        !world
            .get::<RoofCpuTiles>(entity)
            .unwrap()
            .0
            .records
            .is_empty()
    );
    assert_eq!(world.resource::<BuildingReport>().created, vec![entity]);
    assert_eq!(
        world.resource::<BuildingReport>().presence_changes,
        vec![id]
    );
    let maximum = world.resource::<PublicWalls>().walls[&id]
        .state
        .max_y()
        .unwrap();
    assert_eq!(maximum.to_bits(), (3f32 + 3f32 - 0.46f32).to_bits());
    assert_eq!(
        world.resource::<PublicWalls>().walls[&id].arch_segment_heights,
        vec![maximum; 3]
    );
    let initial_height = f32::from_le_bytes(
        world.get::<Roof>(entity).unwrap().0[0x4c..0x50]
            .try_into()
            .unwrap(),
    );
    assert_eq!(initial_height, maximum);
    emit(
        &mut app,
        PublishWall {
            wall: id,
            input: input(0, 2, 4.),
        },
    );
    dt(&mut app, 0.);
    app.update();
    clean(&app);
    let changes = &app.world().resource::<BuildingReport>().geometry_changes;
    assert_eq!(changes.len(), 2);
    assert_eq!(changes.iter().map(|c| c.kind).collect::<Vec<_>>(), [2, 2]);
    assert!(changes.iter().all(|c| c.entity == entity));
    let roof = app.world().get::<Roof>(entity).unwrap();
    assert_eq!(roof.0[8], 0);
    assert_eq!(roof.0[0x54], 1);
    let tile = app.world().get::<TileSimulation>(entity).unwrap();
    let delta = f32::from_le_bytes(tile.0.bytes()[..4].try_into().unwrap());
    assert_eq!(delta, 2.);
    emit(&mut app, DeleteRoof(id));
    emit(&mut app, DeleteRoof(id));
    dt(&mut app, 0.05);
    app.update();
    clean(&app);
    assert!(app.world().get::<RoofMarkedForDeletion>(entity).is_some());
    assert_eq!(
        app.world().resource::<WallRoofMap>().entity(id),
        Some(entity)
    );
    assert_eq!(app.world().resource::<BuildingReport>().sounds.len(), 1);
    assert!(
        app.world()
            .resource::<BuildingReport>()
            .presence_changes
            .is_empty()
    );
    dt(&mut app, 0.5);
    app.update();
    clean(&app);
    assert!(app.world().get_entity(entity).is_err());
    assert_eq!(app.world().resource::<WallRoofMap>().entity(id), None);
    assert_eq!(
        app.world().resource::<BuildingReport>().despawned,
        vec![entity]
    );
    assert_eq!(
        app.world().resource::<BuildingReport>().presence_changes,
        vec![id]
    );
    app.update();
    assert!(
        app.world()
            .resource::<BuildingReport>()
            .presence_changes
            .is_empty()
    );
    emit(&mut app, create(id, 0.));
    dt(&mut app, 0.5);
    app.update();
    clean(&app);
    let flat = app.world().resource::<WallRoofMap>().entity(id).unwrap();
    assert_ne!(flat, entity);
    assert!(app.world().get::<RoofCpuTiles>(flat).is_none());
}
#[test]
fn missing_wall_drives_actual_delete_consumer_and_no_ghost_mapping() {
    let mut app = building_app();
    let id = WallId(7);
    emit(
        &mut app,
        PublishWall {
            wall: id,
            input: input(1, 0, 2.),
        },
    );
    emit(&mut app, create(id, 0.5));
    dt(&mut app, 0.5);
    app.update();
    clean(&app);
    let entity = app.world().resource::<WallRoofMap>().entity(id).unwrap();
    emit(&mut app, RemoveWall(id));
    dt(&mut app, 0.05);
    app.update();
    clean(&app);
    assert!(app.world().get::<RoofMarkedForDeletion>(entity).is_some());
    assert_eq!(app.world().resource::<WallRoofMap>().entity(id), None);
    dt(&mut app, 0.5);
    app.update();
    clean(&app);
    assert!(app.world().get_entity(entity).is_err());
}
#[test]
fn unsupported_shape_never_receives_fabricated_cache_or_spawns_roof() {
    let mut app = building_app();
    let id = WallId(9);
    emit(
        &mut app,
        PublishWall {
            wall: id,
            input: input(2, 0, 2.),
        },
    );
    emit(&mut app, create(id, 0.5));
    app.update();
    assert!(
        app.world().resource::<PublicWalls>().walls[&id]
            .state
            .max_y()
            .is_err()
    );
    assert!(app.world().resource::<BuildingReport>().created.is_empty());
    assert!(!app.world().resource::<BuildingReport>().faults.is_empty());
}
#[test]
fn zero_time_does_not_change_animation_tick() {
    let mut app = building_app();
    let id = WallId(5);
    emit(
        &mut app,
        PublishWall {
            wall: id,
            input: input(1, 0, 2.),
        },
    );
    emit(&mut app, create(id, 0.5));
    app.update();
    clean(&app);
    let entity = app.world().resource::<WallRoofMap>().entity(id).unwrap();
    let tick = app
        .world()
        .get_entity(entity)
        .unwrap()
        .get_change_ticks::<Animation>()
        .unwrap()
        .changed;
    app.update();
    assert_eq!(
        app.world()
            .get_entity(entity)
            .unwrap()
            .get_change_ticks::<Animation>()
            .unwrap()
            .changed,
        tick
    );
}
#[test]
fn gable_creation_updates_second_cache_and_removal_reverts_it() {
    let mut app = building_app();
    let id = WallId(19);
    emit(
        &mut app,
        PublishWall {
            wall: id,
            input: input(1, 0, 2.),
        },
    );
    let CreateRoof(request) = create(id, 0.5);
    let mut record = *request.record();
    record[48..52].copy_from_slice(&1f32.to_le_bytes());
    emit(
        &mut app,
        CreateRoof(RoofCreateRequestObserved::from_observed(record)),
    );
    dt(&mut app, 0.5);
    app.update();
    clean(&app);
    let entity = app.world().resource::<WallRoofMap>().entity(id).unwrap();
    let wall = &app.world().resource::<PublicWalls>().walls[&id];
    let roof = &app.world().get::<Roof>(entity).unwrap().0;
    let expected = wall.state.max_y().unwrap()
        + country_core::resources::roofs::geometry::surface::height(roof)
        + 0.46f32;
    assert_eq!(
        wall.state.max_y_gable().unwrap().to_bits(),
        expected.to_bits()
    );
    assert!(!wall.gable_profile_pending);
    emit(&mut app, DeleteRoof(id));
    dt(&mut app, 0.5);
    app.update();
    app.update();
    clean(&app);
    let wall = &app.world().resource::<PublicWalls>().walls[&id];
    assert_eq!(wall.state.max_y_gable(), wall.state.max_y());
    assert!(!wall.gable_profile_pending);
}

#[test]
fn duplicate_create_requests_preserve_one_live_roof_and_future_associations() {
    let mut app = building_app();
    let id = WallId(90);
    emit(
        &mut app,
        PublishWall {
            wall: id,
            input: input(1, 0, 2.),
        },
    );
    emit(&mut app, create(id, 0.5));
    emit(&mut app, create(id, 0.5));
    app.update();
    assert_eq!(app.world().resource::<BuildingReport>().faults.len(), 1);
    let entity = app.world().resource::<WallRoofMap>().entity(id).unwrap();
    assert_eq!(
        app.world_mut().query::<&Roof>().iter(app.world()).count(),
        1
    );
    emit(&mut app, create(id, 0.5));
    app.update();
    assert_eq!(app.world().resource::<BuildingReport>().faults.len(), 1);
    assert_eq!(
        app.world().resource::<WallRoofMap>().entity(id),
        Some(entity)
    );
    app.update();
    clean(&app);
}

#[test]
fn invalid_publications_do_not_replace_the_last_valid_wall_or_block_valid_inputs() {
    let mut app = building_app();
    let id = WallId(91);
    emit(
        &mut app,
        PublishWall {
            wall: id,
            input: input(1, 0, 2.),
        },
    );
    app.update();
    clean(&app);
    for case in 0..7 {
        let mut invalid = input(1, 0, 2.);
        match case {
            0 => {
                invalid.outline = Arc::new(Vec::new());
                invalid.terrain_y.clear();
            }
            1 => invalid.source_curve_length = f32::INFINITY,
            2 => invalid.extra = f32::NAN,
            3 => Arc::make_mut(&mut invalid.outline)[0][0] = f32::INFINITY,
            4 => invalid.terrain_y[0] = f32::NAN,
            5 => {
                Arc::make_mut(&mut invalid.outline)[0][1] = f32::MAX;
                invalid.terrain_y[0] = -f32::MAX;
            }
            _ => {
                let mut bytes = *invalid.shape.bytes();
                bytes[4..8].copy_from_slice(&f32::NAN.to_le_bytes());
                invalid.shape = WallShapeSnapshot::from_observed(bytes);
            }
        }
        emit(
            &mut app,
            PublishWall {
                wall: id,
                input: invalid,
            },
        );
    }
    emit(
        &mut app,
        PublishWall {
            wall: WallId(92),
            input: input(0, 0, 2.),
        },
    );
    app.update();
    assert_eq!(app.world().resource::<BuildingReport>().faults.len(), 7);
    let walls = app.world().resource::<PublicWalls>();
    assert_eq!(walls.walls[&id].publication_revision, 1);
    assert!(walls.walls.contains_key(&WallId(92)));
}

#[test]
fn invalid_roof_creation_does_not_admit_nonfinite_geometry_or_consume_valid_requests() {
    let mut app = building_app();
    let wall = WallId(93);
    emit(
        &mut app,
        PublishWall {
            wall,
            input: input(1, 0, 2.),
        },
    );
    let CreateRoof(valid) = create(wall, 0.5);
    for offset in [32, 36, 40, 44, 48, 52] {
        for invalid in [f32::NAN, f32::INFINITY] {
            let mut record = *valid.record();
            record[offset..offset + 4].copy_from_slice(&invalid.to_le_bytes());
            emit(
                &mut app,
                CreateRoof(RoofCreateRequestObserved::from_observed(record)),
            );
        }
    }
    let mut bad_direction = *valid.record();
    bad_direction[56] = 2;
    emit(
        &mut app,
        CreateRoof(RoofCreateRequestObserved::from_observed(bad_direction)),
    );
    emit(&mut app, CreateRoof(valid));
    app.update();
    assert_eq!(app.world().resource::<BuildingReport>().faults.len(), 13);
    assert_eq!(app.world().resource::<BuildingReport>().created.len(), 1);
    let entity = app.world().resource::<WallRoofMap>().entity(wall).unwrap();
    assert!(app.world().get::<RoofCpuTiles>(entity).is_some());
    assert!(
        app.world()
            .resource::<nani::NaniCpuStagingState>()
            .contains(entity)
    );
}

#[test]
fn roofs_created_and_deleted_in_one_update_do_not_allocate_backend_instances() {
    let mut app = building_app();
    let wall = WallId(94);
    emit(
        &mut app,
        PublishWall {
            wall,
            input: input(1, 0, 2.),
        },
    );
    emit(&mut app, create(wall, 0.5));
    emit(&mut app, DeleteRoof(wall));
    dt(&mut app, 1.);
    app.update();
    clean(&app);
    assert_eq!(app.world().resource::<BuildingReport>().created.len(), 1);
    assert_eq!(app.world().resource::<BuildingReport>().despawned.len(), 1);
    assert!(
        app.world()
            .resource::<nani::DeferredNaniRequests>()
            .0
            .is_empty()
    );
    assert!(
        app.world()
            .resource::<nani::NaniCpuStagingState>()
            .is_empty()
    );
    assert!(
        app.world()
            .resource::<nani::NaniBackendConfirmations>()
            .0
            .is_empty()
    );
}
