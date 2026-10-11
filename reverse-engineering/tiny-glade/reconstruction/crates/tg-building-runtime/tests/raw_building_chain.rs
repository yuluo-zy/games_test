use bevy_ecs::prelude::*;
use std::sync::Arc;
use tg_building_runtime::{nani::*, *};
use tiny_glade_reconstruction::{
    building_roof_creation::RoofCreateRequestObserved,
    building_wall_collection::{
        DirtyPublicationSubscription, RawWallPublicationSource, TerrainHeightGrid512,
    },
    building_wall_state::{WallId, WallShapeSnapshot},
    surface::Curve2,
};
fn emit<T: Event>(app: &mut bevy_app::App, event: T) {
    app.world_mut().resource_mut::<Events<T>>().send(event);
}
fn grid(height: f32) -> TerrainHeightGrid512 {
    TerrainHeightGrid512 {
        encoded: Arc::from(vec![0; 512 * 512]),
        minimum_y: height,
        maximum_y: height,
        sampling_fix: true,
    }
}
fn source(style: u8, floor: f32) -> RawWallPublicationSource {
    let mut shape = [0; 28];
    shape[..4].copy_from_slice(&1u32.to_le_bytes());
    for (i, v) in [1f32, 0., 0., 0., 6., 8.].iter().enumerate() {
        shape[4 + i * 4..8 + i * 4].copy_from_slice(&v.to_le_bytes());
    }
    RawWallPublicationSource {
        curve: Curve2::try_new(
            vec![[-3., -4.], [-3., 4.], [3., 4.], [3., -4.], [-3., -4.]],
            false,
        )
        .unwrap(),
        curve_kind_tag: 1,
        terrain_floor: floor,
        shape: WallShapeSnapshot::from_observed(shape),
        style,
        extra: 3.,
        special: false,
        observed_path_scalar: 0.,
        clutter_version: 0,
        observed_source_flag_64:0,
    }
}
fn roof_request(id: WallId, direction: u8) -> CreateRoof {
    let mut b = [0; 64];
    b[8..16].copy_from_slice(&id.0.to_le_bytes());
    for (o, v) in [(40, 0.5f32), (44, 0.5), (48, 1.), (52, 0.5)] {
        b[o..o + 4].copy_from_slice(&v.to_le_bytes());
    }
    b[56] = direction;
    CreateRoof(RoofCreateRequestObserved::from_observed(b))
}
fn clean(app: &bevy_app::App) {
    assert!(
        app.world().resource::<BuildingReport>().faults.is_empty(),
        "{:?}",
        app.world().resource::<BuildingReport>()
    );
}
fn configure_clear_path_resources(app: &mut bevy_app::App) {
    use tiny_glade_reconstruction::building_wall_path_preparation::PathRasters512;
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
fn step(app: &mut bevy_app::App, dt: f32) {
    app.world_mut().resource_mut::<BuildingFrame>().delta_time = dt;
    app.update();
    clean(app);
}
#[test]
fn raw_curve_terrain_gable_nani_edit_and_delete_are_connected() {
    let mut app = building_app();
    configure_clear_path_resources(&mut app);
    let id = WallId(u64::MAX);
    emit(
        &mut app,
        SetTerrainGrid {
            grid: grid(0.),
            affected_walls: vec![],
        },
    );
    emit(
        &mut app,
        UpsertRawWall {
            wall: id,
            source: source(0, 2.),
        },
    );
    emit(&mut app, roof_request(id, 1));
    step(&mut app, 0.5);
    let world = app.world();
    let entity = world.resource::<WallRoofMap>().entity(id).unwrap();
    let wall = &world.resource::<PublicWalls>().walls[&id];
    assert_eq!(wall.state.source_point_count(), 5);
    assert_eq!(wall.state.source_curve_length(), 28.);
    assert_eq!(wall.state.published_source_curve_length(), 28.);
    assert!(wall.source.outline.len() > 5);
    assert_eq!(wall.source.outline.first(), wall.source.outline.last());
    assert_eq!(
        wall.base_arch_segment_heights.len(),
        wall.source.outline.len()
    );
    assert!(!wall.gable_profile_pending);
    assert!(
        wall.arch_segment_heights
            .iter()
            .zip(&wall.base_arch_segment_heights)
            .any(|(h, b)| h > b)
    );
    assert!(wall.arch_segment_heights.windows(2).any(|p| p[0] != p[1]));
    assert_eq!(
        world.get::<NaniInstanceDesc>(entity).unwrap().mesh,
        NaniInstanceMeshDesc::AtlasPart(0)
    );
    assert_eq!(
        world.get::<NaniInstanceDesc>(entity).unwrap().subset_kind,
        9
    );
    assert!(world.get::<StoneFloorInstanceDataMarker>(entity).is_some());
    assert!(world.resource::<NaniCpuStagingState>().contains(entity));
    assert!(
        world
            .resource::<DeferredNaniRequests>()
            .0
            .iter()
            .any(|r| matches!(r,DeferredNaniRequest::Create{entity:e,..} if *e==entity))
    );
    let baseline = wall.arch_segment_heights.clone();
    step(&mut app, 0.);
    assert_eq!(
        app.world().resource::<PublicWalls>().walls[&id].arch_segment_heights,
        baseline
    );
    assert!(app.world().resource::<DeferredNaniRequests>().0.is_empty());
    emit(
        &mut app,
        UpsertRawWall {
            wall: id,
            source: source(2, 4.),
        },
    );
    step(&mut app, 0.);
    assert_eq!(
        app.world().resource::<PublicWalls>().walls[&id].publication_revision,
        2
    );
    assert!(app.world().resource::<DeferredNaniRequests>().0.iter().any(
        |r| matches!(r,DeferredNaniRequest::Update{entity:e,mesh_changed:false,..} if *e==entity)
    ));
    emit(&mut app, DeleteRoof(id));
    step(&mut app, 0.05);
    assert!(app.world().get::<RoofMarkedForDeletion>(entity).is_some());
    step(&mut app, 0.5);
    assert!(app.world().get_entity(entity).is_err());
    assert!(
        !app.world()
            .resource::<NaniCpuStagingState>()
            .contains(entity)
    );
    assert_eq!(
        app.world()
            .resource::<DeferredNaniRequests>()
            .0
            .iter()
            .filter(|r| matches!(r,DeferredNaniRequest::Remove{entity:e} if *e==entity))
            .count(),
        1
    );
    let wall = &app.world().resource::<PublicWalls>().walls[&id];
    assert_eq!(wall.arch_segment_heights, wall.base_arch_segment_heights);
    assert_eq!(wall.state.max_y_gable(), wall.state.max_y());
    step(&mut app, 0.);
    assert!(app.world().resource::<DeferredNaniRequests>().0.is_empty());
}
#[test]
fn terrain_dirty_policy_filters_unchanged_and_floor_suppressed_samples() {
    let mut app = building_app();
    configure_clear_path_resources(&mut app);
    let id = WallId(13);
    emit(
        &mut app,
        SetTerrainGrid {
            grid: grid(0.),
            affected_walls: vec![],
        },
    );
    emit(
        &mut app,
        UpsertRawWall {
            wall: id,
            source: source(0, 2.),
        },
    );
    step(&mut app, 0.);
    emit(
        &mut app,
        SetTerrainGrid {
            grid: grid(1.),
            affected_walls: vec![id],
        },
    );
    step(&mut app, 0.);
    assert_eq!(
        app.world().resource::<PublicWalls>().walls[&id].publication_revision,
        1
    );
    emit(
        &mut app,
        SetTerrainGrid {
            grid: grid(5.),
            affected_walls: vec![id],
        },
    );
    step(&mut app, 0.);
    assert_eq!(
        app.world().resource::<PublicWalls>().walls[&id].publication_revision,
        2
    );
    assert!(
        app.world().resource::<PublicWalls>().walls[&id]
            .source
            .outline
            .iter()
            .all(|p| p[1] == 5.)
    );
    emit(
        &mut app,
        TerrainDirtyCandidates {
            subscription: DirtyPublicationSubscription::Secondary,
            walls: vec![id],
            already_lazy_dirty: Default::default(),
        },
    );
    step(&mut app, 0.);
    assert_eq!(
        app.world().resource::<PublicWalls>().walls[&id].publication_revision,
        2
    );
    emit(
        &mut app,
        TerrainDirtyCandidates {
            subscription: DirtyPublicationSubscription::Primary,
            walls: vec![id],
            already_lazy_dirty: Default::default(),
        },
    );
    step(&mut app, 0.);
    assert_eq!(
        app.world().resource::<PublicWalls>().walls[&id].publication_revision,
        3
    );
}
#[test]
fn both_gable_directions_reset_from_base_instead_of_accumulating() {
    for direction in [0, 1] {
        let mut app = building_app();
        configure_clear_path_resources(&mut app);
        let id = WallId(20 + direction as u64);
        emit(
            &mut app,
            SetTerrainGrid {
                grid: grid(0.),
                affected_walls: vec![],
            },
        );
        emit(
            &mut app,
            UpsertRawWall {
                wall: id,
                source: source(0, 2.),
            },
        );
        emit(&mut app, roof_request(id, direction));
        step(&mut app, 0.5);
        let initial = app.world().resource::<PublicWalls>().walls[&id]
            .arch_segment_heights
            .clone();
        emit(
            &mut app,
            UpsertRawWall {
                wall: id,
                source: source(0, 2.),
            },
        );
        step(&mut app, 0.);
        assert_eq!(
            initial,
            app.world().resource::<PublicWalls>().walls[&id].arch_segment_heights
        );
    }
}

#[test]
fn terrain_mask_resolves_raw_memberships_and_cleans_moves_and_removal() {
    let mut app = building_app();
    configure_clear_path_resources(&mut app);
    let id = WallId(71);
    let far = WallId(72);
    let mut far_source = source(0, 2.);
    far_source.curve = Curve2::try_new(
        vec![[27., 26.], [27., 34.], [33., 34.], [33., 26.], [27., 26.]],
        false,
    )
    .unwrap();
    emit(
        &mut app,
        SetTerrainGrid {
            grid: grid(0.),
            affected_walls: vec![],
        },
    );
    emit(
        &mut app,
        UpsertRawWall {
            wall: id,
            source: source(0, 2.),
        },
    );
    emit(
        &mut app,
        UpsertRawWall {
            wall: far,
            source: far_source.clone(),
        },
    );
    step(&mut app, 0.);
    // Cell (30,30) intersects the original x=-3 wall. The far wall is outside.
    let mut mask = [0u64; 64];
    mask[30] = 1 << 30;
    assert_eq!(
        app.world()
            .resource::<WallCollectionState>()
            .spatial_index
            .candidates(&mask),
        vec![id]
    );
    emit(
        &mut app,
        SetTerrainGrid {
            grid: grid(5.),
            affected_walls: vec![],
        },
    );
    emit(
        &mut app,
        TerrainDirtyTiles {
            subscription: DirtyPublicationSubscription::Secondary,
            mask,
            already_lazy_dirty: Default::default(),
        },
    );
    step(&mut app, 0.);
    let walls = &app.world().resource::<PublicWalls>().walls;
    assert_eq!(walls[&id].publication_revision, 2);
    assert_eq!(walls[&far].publication_revision, 1);
    assert!(walls[&id].source.outline.iter().all(|p| p[1] == 5.));
    emit(
        &mut app,
        UpsertRawWall {
            wall: id,
            source: far_source,
        },
    );
    step(&mut app, 0.);
    assert!(
        app.world()
            .resource::<WallCollectionState>()
            .spatial_index
            .candidates(&mask)
            .is_empty()
    );
    emit(&mut app, RemoveWall(id));
    step(&mut app, 0.);
    assert!(
        app.world()
            .resource::<WallCollectionState>()
            .spatial_index
            .wall_cells(id)
            .is_none()
    );
    assert!(
        !app.world()
            .resource::<WallCollectionState>()
            .raw_sources
            .contains_key(&id)
    );
}
