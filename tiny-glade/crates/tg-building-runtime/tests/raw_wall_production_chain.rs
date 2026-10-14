use bevy_ecs::prelude::*;
use country_core::recovered_building::{
    building_wall_collection::{RawWallPublicationSource, TerrainHeightGrid512},
    building_wall_path_preparation::PathRasters512,
    building_wall_state::{WallId, WallShapeSnapshot},
    surface::Curve2,
};
use std::sync::Arc;
use tg_building_runtime::{wall_bricks::*, wall_paths::*, *};
fn emit<T: Event>(app: &mut bevy_app::App, event: T) {
    app.world_mut().resource_mut::<Events<T>>().send(event);
}
fn clear_paths(app: &mut bevy_app::App) {
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
}
fn terrain(app: &mut bevy_app::App, shifted: f32, affected_walls: Vec<WallId>) {
    let encoded = (0..512 * 512)
        .map(|index| ((index % 512) as u32 * 65535 / 511) as u16)
        .collect::<Vec<_>>();
    emit(
        app,
        SetTerrainGrid {
            grid: TerrainHeightGrid512 {
                encoded: Arc::from(encoded),
                minimum_y: shifted,
                maximum_y: shifted + 4.,
                sampling_fix: true,
            },
            affected_walls,
        },
    );
}
fn raw(shape_tag: u32) -> RawWallPublicationSource {
    let points = if shape_tag >= 2 {
        vec![[-4., -1.], [-1., 0.], [0., 2.], [3., -1.]]
    } else {
        vec![
            [-3., -4.],
            [-3., -4.],
            [-3., 4.],
            [3., 4.],
            [3., -4.],
            [-3., -4.],
        ]
    };
    let mut shape = [0; 28];
    shape[..4].copy_from_slice(&shape_tag.to_le_bytes());
    if shape_tag == 1 {
        for (index, value) in [1f32, 0., 0., 0., 6., 8.].iter().enumerate() {
            shape[4 + index * 4..8 + index * 4].copy_from_slice(&value.to_le_bytes());
        }
    }
    RawWallPublicationSource {
        curve: Curve2::try_new(points, false).unwrap(),
        curve_kind_tag: shape_tag,
        terrain_floor: 0.,
        shape: WallShapeSnapshot::from_observed(shape),
        style: 0,
        extra: 2.5,
        special: false,
        observed_path_scalar: -0.,
        clutter_version: 0,
        observed_source_flag_64: 0,
    }
}
fn step(app: &mut bevy_app::App) {
    app.update();
    assert!(
        app.world().resource::<BuildingReport>().faults.is_empty(),
        "{:?}",
        app.world().resource::<BuildingReport>().faults
    );
}
fn records(app: &bevy_app::App, wall: WallId) -> Vec<[u8; 96]> {
    let entity = app.world().resource::<WallBrickEntities>().0[&wall];
    app.world()
        .get::<WallCpuBricks>(entity)
        .unwrap()
        .instances
        .iter()
        .map(|record| record.observed_bytes())
        .collect()
}
#[test]
fn actual_preparation_deduplicates_and_keeps_initial_seed_count_and_owner_identity() {
    let mut app = building_app();
    let wall = WallId(100011);
    clear_paths(&mut app);
    terrain(&mut app, 0., vec![]);
    emit(
        &mut app,
        UpsertRawWall {
            wall,
            source: raw(1),
        },
    );
    step(&mut app);
    let host = &app.world().resource::<PublicWalls>().walls[&wall];
    let prepared = app
        .world()
        .resource::<WallPreparedPaths>()
        .current(wall, host.publication_revision)
        .unwrap();
    assert!(host.source.outline.len() > prepared.path.positions.len());
    assert_eq!(
        host.arch_segment_heights.len(),
        prepared.path.positions.len()
    );
    assert_eq!(
        host.segment_heights.len(),
        prepared.path.segment_values.len()
    );
    assert!(
        host.segment_heights
            .iter()
            .all(|height| height.to_bits() == 0)
    );
    let entity = app.world().resource::<WallBrickEntities>().0[&wall];
    let output = app.world().get::<WallCpuBricks>(entity).unwrap();
    assert!(!output.instances.is_empty());
    assert!(
        output
            .instances
            .iter()
            .all(|record| record.material == 0x8000000b)
    );
    let before = records(&app, wall);
    step(&mut app);
    assert_eq!(records(&app, wall), before);
    assert_eq!(
        app.world().get::<WallCpuBricks>(entity).unwrap().generation,
        1
    );
    emit(&mut app, RemoveWall(wall));
    step(&mut app);
    assert!(app.world().get_entity(entity).is_err());
    emit(
        &mut app,
        UpsertRawWall {
            wall,
            source: raw(1),
        },
    );
    step(&mut app);
    assert_eq!(records(&app, wall), before);
    assert_ne!(app.world().resource::<WallBrickEntities>().0[&wall], entity);
}
#[test]
fn missing_path_resources_never_fabricate_prepared_geometry_or_bricks() {
    for shape_tag in [1, 2] {
        let mut app = building_app();
        let wall = WallId(32);
        terrain(&mut app, 0., vec![]);
        emit(
            &mut app,
            UpsertRawWall {
                wall,
                source: raw(shape_tag),
            },
        );
        step(&mut app);
        assert!(
            app.world()
                .resource::<WallPreparedPaths>()
                .records
                .is_empty()
        );
        assert_eq!(
            app.world().resource::<WallPreparedPaths>().pending[&wall],
            PendingWallPathReason::MissingPathResources
        );
        assert!(app.world().resource::<WallBrickEntities>().0.is_empty());
        assert_eq!(
            app.world().resource::<WallBrickBuildState>().pending[&wall],
            PendingWallProductionReason::MissingPreparedPath
        );
        clear_paths(&mut app);
        step(&mut app);
        assert!(!records(&app, wall).is_empty());
        assert!(
            app.world()
                .resource::<WallPreparedPaths>()
                .pending
                .is_empty()
        );
        assert!(
            app.world()
                .resource::<WallBrickBuildState>()
                .pending
                .is_empty()
        );
    }
}
#[test]
fn freeform_cache_and_bricks_follow_terrain_height_and_clutter_policy() {
    let mut app = building_app();
    let wall = WallId(u64::MAX);
    clear_paths(&mut app);
    terrain(&mut app, 0., vec![]);
    emit(
        &mut app,
        UpsertRawWall {
            wall,
            source: raw(2),
        },
    );
    step(&mut app);
    let cache = &app.world().resource::<PublicWalls>().walls[&wall].arch_segment_heights;
    assert!(cache.windows(2).any(|values| values[0] != values[1]));
    let initial = records(&app, wall);
    assert!(!initial.is_empty());
    terrain(&mut app, 1., vec![wall]);
    step(&mut app);
    assert_ne!(records(&app, wall), initial);
    emit(
        &mut app,
        wall_height::SetWallSourceHeight { wall, height: 3. },
    );
    step(&mut app);
    let shifted = records(&app, wall);
    assert_ne!(shifted, initial);
    let mut uniform = raw(2);
    uniform.clutter_version = 1;
    emit(
        &mut app,
        UpsertRawWall {
            wall,
            source: uniform,
        },
    );
    step(&mut app);
    let host = &app.world().resource::<PublicWalls>().walls[&wall];
    assert!(
        host.arch_segment_heights
            .windows(2)
            .all(|values| values[0] == values[1])
    );
    assert_eq!(host.segment_heights.len(), host.arch_segment_heights.len());
    emit(&mut app, RemoveWall(wall));
    step(&mut app);
    assert!(
        app.world()
            .resource::<WallPreparedPaths>()
            .records
            .is_empty()
    );
    assert!(app.world().resource::<WallBrickEntities>().0.is_empty());
}

#[test]
fn special_source_waits_for_its_original_construction_branch() {
    let mut app = building_app();
    let wall = WallId(42);
    clear_paths(&mut app);
    terrain(&mut app, 0., vec![]);
    let mut source = raw(1);
    source.special = true;
    emit(&mut app, UpsertRawWall { wall, source });
    step(&mut app);
    assert!(app.world().resource::<WallBrickEntities>().0.is_empty());
    assert_eq!(
        app.world().resource::<WallBrickBuildState>().pending[&wall],
        PendingWallProductionReason::SpecialConstruction
    );
    emit(
        &mut app,
        UpsertRawWall {
            wall,
            source: raw(1),
        },
    );
    step(&mut app);
    assert!(!records(&app, wall).is_empty());
    assert!(
        app.world()
            .resource::<WallBrickBuildState>()
            .pending
            .is_empty()
    );
}

#[test]
fn encoded_path_updates_rebuild_actual_preparation_and_return_to_identical_bricks() {
    let mut app = building_app();
    let wall = WallId(94);
    clear_paths(&mut app);
    terrain(&mut app, 0., vec![]);
    let mut source = raw(2);
    source.observed_path_scalar = 2.8;
    emit(&mut app, UpsertRawWall { wall, source });
    step(&mut app);
    let initial = records(&app, wall);
    let initial_positions = app.world().resource::<WallPreparedPaths>().records[&wall]
        .path
        .positions
        .clone();
    let initial_publication =
        app.world().resource::<PublicWalls>().walls[&wall].publication_revision;
    let mut encoded: [Arc<[u8]>; 4] = std::array::from_fn(|_| Arc::from(vec![0; 512 * 512]));
    let mut patch = vec![0; 512 * 512];
    for z in 246..268 {
        patch[z * 512 + 246..z * 512 + 264].fill(255);
    }
    encoded[0] = Arc::from(patch);
    emit(
        &mut app,
        SetWallPathRasters(PathRasters512::from_encoded(encoded, true).unwrap()),
    );
    step(&mut app);
    let path = &app.world().resource::<WallPreparedPaths>().records[&wall];
    assert_eq!(path.generation, 2);
    let masks = &app
        .world()
        .resource::<wall_segments::WallPathMaskRegistry>()
        .records[&wall];
    assert!(!masks.masks.is_empty());
    assert_eq!(masks.prepared_generation, path.generation);
    assert_eq!(
        app.world()
            .resource::<wall_segments::WallPathMaterialStats>()
            .0[&wall]
            .segmentation_count as usize,
        masks.masks.len()
    );
    assert!(path.path.segment_values.iter().any(|value| *value > 0.));
    assert_ne!(path.path.positions, initial_positions);
    assert_eq!(
        app.world().resource::<PublicWalls>().walls[&wall].publication_revision,
        initial_publication
    );
    let entity = app.world().resource::<WallBrickEntities>().0[&wall];
    assert_eq!(
        app.world().get::<WallCpuBricks>(entity).unwrap().generation,
        2
    );
    clear_paths(&mut app);
    step(&mut app);
    assert_eq!(
        app.world().resource::<WallPreparedPaths>().records[&wall]
            .path
            .positions,
        initial_positions
    );
    assert!(
        app.world()
            .resource::<wall_segments::WallPathMaskRegistry>()
            .records[&wall]
            .masks
            .is_empty()
    );
    assert_eq!(records(&app, wall), initial);
    assert_eq!(
        app.world().get::<WallCpuBricks>(entity).unwrap().generation,
        3
    );
}
