use bevy_ecs::prelude::*;
use country_core::resources::walls::wall_holes::opening_rules::*;
use country_core::resources::roofs::creation::RoofCreateRequestObserved;
use country_core::resources::roofs::customization::FlatRoofRimRequest;
use country_core::resources::walls::publication_fields::WallId;
use country_core::resources::walls::publication_fields::WallShapeSnapshot;
use std::sync::Arc;
use tg_building_runtime::{history::*, openings::*, *};
fn emit<T: Event>(app: &mut bevy_app::App, event: T) {
    app.world_mut().resource_mut::<Events<T>>().send(event);
}
fn step(app: &mut bevy_app::App) {
    app.world_mut().resource_mut::<BuildingFrame>().delta_time = 0.;
    app.update();
    assert!(
        app.world().resource::<BuildingReport>().faults.is_empty(),
        "{:?}",
        app.world().resource::<BuildingReport>()
    );
}
fn wall(app: &mut bevy_app::App, id: WallId, tag: u32) {
    let mut shape = [0; 28];
    shape[..4].copy_from_slice(&tag.to_le_bytes());
    for (i, v) in [1f32, 0., 0., 0., 6., 8.].iter().enumerate() {
        shape[4 + i * 4..8 + i * 4].copy_from_slice(&v.to_le_bytes());
    }
    emit(
        app,
        PublishWall {
            wall: id,
            input: WallPublicationInput {
                outline: Arc::new(vec![
                    [-3., 2., -4.],
                    [-3., 2., 4.],
                    [3., 2., 4.],
                    [3., 2., -4.],
                    [-3., 2., -4.],
                ]),
                terrain_y: vec![0.; 5],
                source_curve_length: 28.,
                source_point_count: 5,
                shape: WallShapeSnapshot::from_observed(shape),
                style: 0,
                extra: 3.,
                special: false,
            },
        },
    );
}
fn roof(app: &mut bevy_app::App, id: WallId) {
    let mut request = [0; 64];
    request[8..16].copy_from_slice(&id.0.to_le_bytes());
    request[40..44].copy_from_slice(&0.5f32.to_le_bytes());
    request[48..52].copy_from_slice(&1f32.to_le_bytes());
    emit(
        app,
        CreateRoof(RoofCreateRequestObserved::from_observed(request)),
    );
}
fn input(family: OpeningFamily, x: f32) -> DecoratorOpeningInput {
    DecoratorOpeningInput {
        family,
        size_variant: 2,
        center: [x, 2.],
        collision: DecoratorCollisionMetrics {
            extent_xz: [1.5, 0.4],
            vertical_bounds: [-1., 1.],
        },
    }
}
fn upsert(app: &mut bevy_app::App, id: u64, host: WallId, input: DecoratorOpeningInput) {
    emit(
        app,
        UpsertOpening(OpeningAuthority {
            id: OpeningId(id),
            wall: host,
            input,
        }),
    );
}
fn rim(app: &mut bevy_app::App, id: WallId, from: u8, to: u8) {
    emit(
        app,
        SetFlatRoofRimStyle(FlatRoofRimRequest {
            wall_id: id,
            from,
            to,
            observed_record_history: 1,
        }),
    );
}
#[test]
fn actual_recalculation_reaches_window_door_geometry_and_cleanup() {
    let mut app = building_app();
    let host = WallId(u64::MAX);
    wall(&mut app, host, 1);
    roof(&mut app, host);
    upsert(&mut app, 1, host, input(OpeningFamily::CottageWindow, 0.1));
    upsert(
        &mut app,
        2,
        host,
        input(OpeningFamily::GothicBalconyDoor, 10.),
    );
    step(&mut app);
    let e = app.world().resource::<OpeningEntities>().0[&OpeningId(1)];
    let storage = &app.world().resource::<WallOpenings>().storage[&host];
    assert_eq!(storage.raw_holes().len(), 4);
    assert_eq!(storage.finalized_holes().unwrap().len(), 5);
    assert_eq!(storage.wall_length().unwrap(), 28.);
    let original = storage.finalized_holes().unwrap().to_vec();
    step(&mut app);
    assert!(
        app.world()
            .resource::<WallOpenings>()
            .geometry_rebuilt
            .is_empty()
    );
    assert_eq!(
        app.world().resource::<WallOpenings>().storage[&host]
            .finalized_holes()
            .unwrap(),
        &original
    );
    let generation = app.world().get::<OpeningState>(e).unwrap().generation;
    emit(&mut app, RecalculateWallHoles(host));
    emit(&mut app, RecalculateWallHoles(host));
    emit(&mut app, RecalculateWallHoles(WallId(404)));
    step(&mut app);
    assert_eq!(
        app.world()
            .resource::<WallOpenings>()
            .forwarded_recalculations,
        vec![host, host]
    );
    assert_eq!(
        app.world().get::<OpeningState>(e).unwrap().generation,
        generation + 1
    );
    assert!(
        app.world()
            .resource::<WallOpenings>()
            .geometry_rebuilt
            .is_empty()
    );
    // Native CottageWindow policy shrinks width on a circle host; this is a real
    // recomputation of the stored opening, not just an invalidation counter.
    wall(&mut app, host, 0);
    step(&mut app);
    assert_eq!(
        app.world().get::<OpeningState>(e).unwrap().raw_holes[0].dimensions[0],
        1.4
    );
    assert_eq!(
        app.world().resource::<WallOpenings>().geometry_rebuilt,
        vec![host]
    );
    emit(&mut app, RemoveOpening(OpeningId(2)));
    step(&mut app);
    assert_eq!(
        app.world().resource::<WallOpenings>().storage[&host]
            .raw_holes()
            .len(),
        1
    );
    emit(&mut app, RemoveOpening(OpeningId(1)));
    step(&mut app);
    assert!(
        app.world().resource::<WallOpenings>().storage[&host]
            .finalized_holes()
            .unwrap()
            .is_empty()
    );
    assert!(app.world().get_entity(e).is_err());
    upsert(
        &mut app,
        3,
        host,
        input(OpeningFamily::CottageBalconyDoor, 5.),
    );
    step(&mut app);
    let orphan = app.world().resource::<OpeningEntities>().0[&OpeningId(3)];
    emit(&mut app, RemoveWall(host));
    step(&mut app);
    assert!(
        !app.world()
            .resource::<WallOpenings>()
            .storage
            .contains_key(&host)
    );
    assert!(
        app.world()
            .get::<OpeningState>(orphan)
            .unwrap()
            .resolved_host
            .is_none()
    );
    assert!(
        app.world()
            .get::<OpeningState>(orphan)
            .unwrap()
            .raw_holes
            .is_empty()
    );
    wall(&mut app, host, 1);
    step(&mut app);
    assert_eq!(
        app.world()
            .get::<OpeningState>(orphan)
            .unwrap()
            .resolved_host,
        Some(host)
    );
}
#[test]
fn rim_transaction_commit_undo_redo_and_branch_drive_opening_state() {
    let mut app = building_app();
    let a = WallId(31);
    let b = WallId(32);
    wall(&mut app, a, 1);
    wall(&mut app, b, 1);
    roof(&mut app, a);
    roof(&mut app, b);
    upsert(&mut app, 1, a, input(OpeningFamily::CottageWindow, 5.));
    step(&mut app);
    let ea = app.world().resource::<WallRoofMap>().entity(a).unwrap();
    let eb = app.world().resource::<WallRoofMap>().entity(b).unwrap();
    let opening = app.world().resource::<OpeningEntities>().0[&OpeningId(1)];
    emit(&mut app, HistoryAction::ResetAfterFutureClear);
    emit(&mut app, HistoryAction::Start);
    rim(&mut app, a, 0, 1);
    rim(&mut app, b, 0, 2);
    rim(&mut app, a, 1, 3);
    emit(&mut app, HistoryAction::Finish);
    step(&mut app);
    assert_eq!(
        app.world()
            .resource::<BuildingHistory>()
            .committed_this_frame,
        vec![0]
    );
    assert!(
        app.world()
            .resource::<RoofRimHistoryRequests>()
            .0
            .is_empty()
    );
    let generation = app.world().get::<OpeningState>(opening).unwrap().generation;
    emit(&mut app, HistoryAction::Undo { instant: true });
    for (ar, br, changed) in [(1, 2, a), (1, 0, b), (0, 0, a)] {
        step(&mut app);
        assert_eq!(app.world().get::<Roof>(ea).unwrap().0[0x3d], ar);
        assert_eq!(app.world().get::<Roof>(eb).unwrap().0[0x3d], br);
        assert_eq!(
            app.world()
                .resource::<WallOpenings>()
                .forwarded_recalculations,
            vec![changed]
        );
    }
    assert!(app.world().get::<OpeningState>(opening).unwrap().generation > generation);
    emit(&mut app, HistoryAction::Redo { instant: true });
    for (ar, br) in [(1, 0), (1, 2), (3, 2)] {
        step(&mut app);
        assert_eq!(app.world().get::<Roof>(ea).unwrap().0[0x3d], ar);
        assert_eq!(app.world().get::<Roof>(eb).unwrap().0[0x3d], br);
    }
    assert_eq!(app.world().get::<Roof>(ea).unwrap().0[0x3d], 3);
    assert_eq!(app.world().get::<Roof>(eb).unwrap().0[0x3d], 2);
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
    emit(&mut app, HistoryAction::Undo { instant: true });
    for _ in 0..3 {
        step(&mut app);
    }
    emit(&mut app, HistoryAction::Start);
    rim(&mut app, a, 0, 2);
    emit(&mut app, HistoryAction::Finish);
    step(&mut app);
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
    step(&mut app);
    assert_eq!(app.world().get::<Roof>(ea).unwrap().0[0x3d], 2);
    assert_eq!(
        app.world()
            .resource::<BuildingHistory>()
            .replayed_this_frame,
        0
    );
    emit(&mut app, HistoryAction::Start);
    emit(&mut app, HistoryAction::Finish);
    step(&mut app);
    assert!(
        app.world()
            .resource::<BuildingHistory>()
            .committed_this_frame
            .is_empty()
    );
}
