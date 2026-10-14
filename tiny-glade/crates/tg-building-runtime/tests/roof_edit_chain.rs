use bevy_ecs::prelude::*;
use country_core::recovered_building::{
    building_roof_creation::RoofCreateRequestObserved,
    building_roof_direction::RoofDirectionRequest,
    building_wall_state::{WallId, WallShapeSnapshot},
};
use std::sync::Arc;
use tg_building_runtime::*;
fn emit<T: Event>(app: &mut bevy_app::App, event: T) {
    app.world_mut().resource_mut::<Events<T>>().send(event);
}
fn scene() -> (bevy_app::App, WallId, Entity) {
    scene_with_height(0.5)
}
fn scene_with_height(height: f32) -> (bevy_app::App, WallId, Entity) {
    let mut app = building_app();
    let wall = WallId(40);
    let mut shape = [0; 28];
    shape[..4].copy_from_slice(&1u32.to_le_bytes());
    for (i, value) in [1f32, 0., 0., 0., 6., 8.].iter().enumerate() {
        shape[4 + i * 4..8 + i * 4].copy_from_slice(&value.to_le_bytes());
    }
    emit(
        &mut app,
        PublishWall {
            wall,
            input: WallPublicationInput {
                outline: Arc::new(vec![
                    [-3., 2., -4.],
                    [-3., 2., 0.],
                    [-3., 2., 4.],
                    [0., 2., 4.],
                    [3., 2., 4.],
                    [3., 2., 0.],
                    [3., 2., -4.],
                    [0., 2., -4.],
                    [-3., 2., -4.],
                ]),
                terrain_y: vec![0.; 9],
                source_curve_length: 28.,
                source_point_count: 5,
                shape: WallShapeSnapshot::from_observed(shape),
                style: 0,
                extra: 3.,
                special: false,
            },
        },
    );
    let mut request = [0; 64];
    request[8..16].copy_from_slice(&wall.0.to_le_bytes());
    for (offset, value) in [(40, 0.5f32), (44, height), (48, 1.), (52, 0.5)] {
        request[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
    }
    emit(
        &mut app,
        CreateRoof(RoofCreateRequestObserved::from_observed(request)),
    );
    app.world_mut().resource_mut::<BuildingFrame>().delta_time = 0.5;
    app.update();
    let entity = app.world().resource::<WallRoofMap>().entity(wall).unwrap();
    (app, wall, entity)
}
#[test]
fn direction_switch_updates_gable_tiles_feedback_and_preserves_duplicate_events() {
    let (mut app, wall, entity) = scene();
    let initial = app.world().resource::<PublicWalls>().walls[&wall]
        .arch_segment_heights
        .clone();
    let tiles: Vec<_> = app
        .world()
        .get::<RoofCpuTiles>(entity)
        .unwrap()
        .0
        .records
        .iter()
        .map(|r| r.0)
        .collect();
    emit(
        &mut app,
        SwitchRoofDirection(RoofDirectionRequest { wall, direction: 1 }),
    );
    app.world_mut().resource_mut::<BuildingFrame>().delta_time = 0.;
    app.update();
    assert_eq!(app.world().get::<Roof>(entity).unwrap().0[0x3c], 1);
    assert_ne!(
        app.world().resource::<PublicWalls>().walls[&wall].arch_segment_heights,
        initial
    );
    assert_ne!(
        app.world()
            .get::<RoofCpuTiles>(entity)
            .unwrap()
            .0
            .records
            .iter()
            .map(|r| r.0)
            .collect::<Vec<_>>(),
        tiles
    );
    assert_eq!(
        app.world().resource::<BuildingReport>().geometry_changes[0].kind,
        1
    );
    assert_eq!(app.world().resource::<BuildingReport>().sounds[0].kind, 7);
    let heights = app.world().resource::<PublicWalls>().walls[&wall]
        .arch_segment_heights
        .clone();
    for _ in 0..2 {
        emit(
            &mut app,
            SwitchRoofDirection(RoofDirectionRequest { wall, direction: 1 }),
        );
    }
    app.update();
    assert_eq!(
        app.world()
            .resource::<BuildingReport>()
            .geometry_changes
            .len(),
        2
    );
    assert_eq!(app.world().resource::<BuildingReport>().sounds.len(), 2);
    assert_eq!(
        app.world().resource::<PublicWalls>().walls[&wall].arch_segment_heights,
        heights
    );
    assert!(app.world().resource::<BuildingReport>().faults.is_empty());
    emit(&mut app, DeleteRoof(wall));
    app.world_mut().resource_mut::<BuildingFrame>().delta_time = 0.05;
    app.update();
    assert!(app.world().get::<RoofMarkedForDeletion>(entity).is_some());
    emit(
        &mut app,
        SwitchRoofDirection(RoofDirectionRequest { wall, direction: 0 }),
    );
    app.world_mut().resource_mut::<BuildingFrame>().delta_time = 0.;
    app.update();
    assert_eq!(app.world().get::<Roof>(entity).unwrap().0[0x3c], 0);
    assert_eq!(
        app.world()
            .resource::<BuildingReport>()
            .geometry_changes
            .len(),
        1
    );
    assert!(app.world().resource::<BuildingReport>().faults.is_empty());
}
#[test]
fn direction_missing_association_reports_original_failure_boundary() {
    let (mut app, wall, entity) = scene();
    emit(
        &mut app,
        SwitchRoofDirection(RoofDirectionRequest {
            wall: WallId(404),
            direction: 1,
        }),
    );
    emit(
        &mut app,
        SwitchRoofDirection(RoofDirectionRequest { wall, direction: 1 }),
    );
    app.update();
    assert_eq!(app.world().resource::<BuildingReport>().faults.len(), 1);
    assert_eq!(app.world().get::<Roof>(entity).unwrap().0[0x3c], 0);
    assert!(
        app.world()
            .resource::<BuildingReport>()
            .geometry_changes
            .is_empty()
    );
}

#[test]
fn flat_style_rim_and_history_are_separate_consumers() {
    use country_core::recovered_building::building_roof_customization::*;
    let (mut app, wall, entity) = scene_with_height(0.);
    assert!(app.world().get::<RoofCpuTiles>(entity).is_none());
    let mut style = [0x5a; 12];
    style[0] = 2;
    style[1] = 1;
    style[4..8].copy_from_slice(&0.4f32.to_le_bytes());
    style[8..12].copy_from_slice(&0.8f32.to_le_bytes());
    emit(
        &mut app,
        SetFlatRoofStyle(FlatRoofStyleRequest {
            wall_id: wall,
            style: FlatRoofStyleObserved::from_observed(style),
            observed_play_sound: 1,
        }),
    );
    emit(
        &mut app,
        SetFlatRoofRimStyle(FlatRoofRimRequest {
            wall_id: wall,
            from: 7,
            to: 2,
            observed_record_history: 1,
        }),
    );
    app.world_mut().resource_mut::<BuildingFrame>().delta_time = 0.;
    app.update();
    let roof = &app.world().get::<Roof>(entity).unwrap().0;
    assert_eq!(&roof[0x40..0x4c], &style);
    assert_eq!(roof[0x3d], 2);
    let report = app.world().resource::<BuildingReport>();
    assert_eq!(report.wall_holes_requests, vec![wall, wall]);
    assert_eq!(report.sounds[0].kind, 11);
    let mutation = app.world().resource::<RoofRimHistoryRequests>().0[0];
    assert_eq!(
        (mutation.observed_tag, mutation.from, mutation.to),
        (4, 7, 2)
    );
    assert!(
        app.world()
            .resource::<nani::DeferredNaniRequests>()
            .0
            .iter()
            .any(|r| matches!(r,nani::DeferredNaniRequest::Update{entity:e,..} if *e==entity))
    );
    // Padding is assigned, but it is not semantic style equality for holes.
    style[2] = 0x11;
    style[3] = 0x22;
    emit(
        &mut app,
        SetFlatRoofStyle(FlatRoofStyleRequest {
            wall_id: wall,
            style: FlatRoofStyleObserved::from_observed(style),
            observed_play_sound: 2,
        }),
    );
    emit(
        &mut app,
        SetFlatRoofRimStyle(FlatRoofRimRequest {
            wall_id: wall,
            from: 9,
            to: 2,
            observed_record_history: 1,
        }),
    );
    app.update();
    assert_eq!(
        &app.world().get::<Roof>(entity).unwrap().0[0x40..0x4c],
        &style
    );
    let report = app.world().resource::<BuildingReport>();
    assert!(report.wall_holes_requests.is_empty());
    assert!(report.sounds.is_empty());
    assert_eq!(report.rim_history_requests.len(), 1);
    assert_eq!(app.world().resource::<RoofRimHistoryRequests>().0.len(), 2);
    emit(
        &mut app,
        SetFlatRoofRimStyle(FlatRoofRimRequest {
            wall_id: WallId(404),
            from: 0,
            to: 1,
            observed_record_history: 1,
        }),
    );
    app.update();
    assert_eq!(app.world().resource::<RoofRimHistoryRequests>().0.len(), 2);
    assert!(app.world().resource::<BuildingReport>().faults.is_empty());
    emit(&mut app, DeleteRoof(wall));
    app.world_mut().resource_mut::<BuildingFrame>().delta_time = 0.05;
    app.update();
    emit(
        &mut app,
        SetFlatRoofRimStyle(FlatRoofRimRequest {
            wall_id: wall,
            from: 2,
            to: 3,
            observed_record_history: 0,
        }),
    );
    app.world_mut().resource_mut::<BuildingFrame>().delta_time = 0.;
    app.update();
    assert_eq!(app.world().get::<Roof>(entity).unwrap().0[0x3d], 3);
    app.world_mut().resource_mut::<BuildingFrame>().delta_time = 0.5;
    app.update();
    assert!(app.world().get_entity(entity).is_err());
    assert_eq!(app.world().resource::<RoofRimHistoryRequests>().0.len(), 2);
}
#[test]
fn original_discrete_replay_producer_routes_direction_rim_and_undo_to_world() {
    use country_core::recovered_building::roof_replay::VisualReplayState;
    let (mut app, wall, entity) = scene();
    app.world_mut().resource_mut::<BuildingFrame>().delta_time = 0.;
    for (undo, desired) in [(0, 1), (1, 0)] {
        let mut state = [0; 24];
        state[20] = undo;
        let mut replay = VisualReplayState::from_observed(state);
        let mut payload = [0; 64];
        payload[0] = 3;
        payload[1] = 0;
        payload[2] = 1;
        payload[8..16].copy_from_slice(&wall.0.to_le_bytes());
        let output = replay
            .advance_discrete_observed(&payload, 0.)
            .unwrap()
            .unwrap();
        assert!(replay.finished());
        emit(&mut app, ReplayRoofEvent(output));
        app.update();
        assert_eq!(app.world().get::<Roof>(entity).unwrap().0[0x3c], desired);
        let mut replay = VisualReplayState::from_observed(state);
        payload[0] = 4;
        emit(
            &mut app,
            ReplayRoofEvent(
                replay
                    .advance_discrete_observed(&payload, 0.)
                    .unwrap()
                    .unwrap(),
            ),
        );
        app.update();
        assert_eq!(app.world().get::<Roof>(entity).unwrap().0[0x3d], desired);
        assert!(
            app.world()
                .resource::<RoofRimHistoryRequests>()
                .0
                .is_empty()
        );
    }
    assert!(app.world().resource::<BuildingReport>().faults.is_empty());
}
#[test]
fn original_delete_replay_and_undo_restore_connect_to_entity_lifecycle() {
    use country_core::recovered_building::roof_replay::VisualReplayState;
    let (mut app, wall, old_entity) = scene();
    let mut request = [0; 64];
    request[8..16].copy_from_slice(&wall.0.to_le_bytes());
    for (offset, value) in [(40, 0.5f32), (44, 0.5), (48, 1.), (52, 0.5)] {
        request[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
    }
    let mut payload = [0; 64];
    payload[0] = 1;
    payload[4..48].copy_from_slice(&request[16..60]);
    payload[48..56].copy_from_slice(&wall.0.to_le_bytes());
    let mut replay = VisualReplayState::from_observed([0; 24]);
    emit(
        &mut app,
        ReplayRoofEvent(
            replay
                .advance_discrete_observed(&payload, 0.)
                .unwrap()
                .unwrap(),
        ),
    );
    app.world_mut().resource_mut::<BuildingFrame>().delta_time = 0.5;
    app.update();
    assert!(app.world().get_entity(old_entity).is_err());
    assert!(app.world().resource::<WallRoofMap>().entity(wall).is_none());
    let mut state = [0; 24];
    state[20] = 1;
    let mut replay = VisualReplayState::from_observed(state);
    emit(
        &mut app,
        ReplayRoofEvent(
            replay
                .advance_discrete_observed(&payload, 0.)
                .unwrap()
                .unwrap(),
        ),
    );
    app.update();
    let restored = app.world().resource::<WallRoofMap>().entity(wall).unwrap();
    assert_ne!(restored, old_entity);
    assert!(
        app.world()
            .resource::<nani::NaniCpuStagingState>()
            .contains(restored)
    );
    assert_eq!(app.world().resource::<BuildingReport>().created.len(), 1);
    assert!(app.world().resource::<BuildingReport>().faults.is_empty());
}
