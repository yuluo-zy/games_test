use bevy_ecs::prelude::*;
use std::sync::Arc;
use tg_building_runtime::{wall_paths::*, wall_structures::*, *};
use tiny_glade_reconstruction::{
    building_wall_collection::{RawWallPublicationSource, TerrainHeightGrid512},
    building_wall_path_preparation::PathRasters512,
    building_wall_state::{WallId, WallShapeSnapshot},
    surface::Curve2,
};
fn emit<T: Event>(app: &mut bevy_app::App, event: T) {
    app.world_mut().resource_mut::<Events<T>>().send(event);
}
fn paths(app: &mut bevy_app::App, patch: bool) {
    let mut encoded = vec![0; 512 * 512];
    if patch {
        for z in 246..269 {
            for x in 246..265 {
                encoded[z * 512 + x] = 255;
            }
        }
    }
    emit(
        app,
        SetWallPathRasters(
            PathRasters512::from_encoded(
                [
                    Arc::from(encoded),
                    Arc::from(vec![0; 512 * 512]),
                    Arc::from(vec![0; 512 * 512]),
                    Arc::from(vec![0; 512 * 512]),
                ],
                true,
            )
            .unwrap(),
        ),
    );
}
fn source(scalar: f32) -> RawWallPublicationSource {
    let mut shape = [0; 28];
    shape[..4].copy_from_slice(&1u32.to_le_bytes());
    for (i, value) in [1f32, 0., 0., 0., 4., 4.].iter().enumerate() {
        shape[4 + i * 4..8 + i * 4].copy_from_slice(&value.to_le_bytes());
    }
    RawWallPublicationSource {
        curve: Curve2::try_new(
            vec![[-2., -2.], [-2., 2.], [2., 2.], [2., -2.], [-2., -2.]],
            false,
        )
        .unwrap(),
        curve_kind_tag: 1,
        terrain_floor: 0.,
        shape: WallShapeSnapshot::from_observed(shape),
        style: 0,
        extra: 0.8,
        special: true,
        observed_path_scalar: scalar,
        clutter_version: 0,
        observed_source_flag_64:0,
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
fn cpu(app: &bevy_app::App, wall: WallId) -> (u64, Vec<[u8; 64]>) {
    let entity = app.world().resource::<WallLintelEntities>().0[&wall];
    let output = app.world().get::<WallCpuLintels>(entity).unwrap();
    (
        output.generation,
        output
            .instances
            .iter()
            .map(|instance| instance.observed_bytes())
            .collect(),
    )
}
#[test]
fn actual_masks_generate_lintels_and_lowering_source_resets_third_then_removal_cleans_all() {
    let mut app = building_app();
    let wall = WallId(100011);
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
    paths(&mut app, false);
    emit(
        &mut app,
        UpsertRawWall {
            wall,
            source: source(2.8),
        },
    );
    step(&mut app);
    assert!(cpu(&app, wall).1.is_empty());
    paths(&mut app, true);
    step(&mut app);
    let produced = cpu(&app, wall);
    assert!(!produced.1.is_empty());
    let high = app.world().resource::<PublicWalls>().walls[&wall]
        .segment_heights
        .clone();
    assert!(high.iter().any(|height| *height > 0.));
    step(&mut app);
    assert_eq!(cpu(&app, wall), produced);
    assert_eq!(
        app.world().resource::<PublicWalls>().walls[&wall].segment_heights,
        high
    );
    emit(
        &mut app,
        UpsertRawWall {
            wall,
            source: source(1.1),
        },
    );
    step(&mut app);
    let low = app.world().resource::<PublicWalls>().walls[&wall]
        .segment_heights
        .clone();
    assert!(low.iter().zip(&high).any(|(low, high)| low < high));
    assert_ne!(cpu(&app, wall).1, produced.1);
    paths(&mut app, false);
    step(&mut app);
    assert!(cpu(&app, wall).1.is_empty());
    assert!(
        app.world().resource::<PublicWalls>().walls[&wall]
            .segment_heights
            .iter()
            .all(|height| *height == 0.)
    );
    emit(&mut app, RemoveWall(wall));
    step(&mut app);
    assert!(
        !app.world()
            .resource::<WallLintelEntities>()
            .0
            .contains_key(&wall)
    );
    paths(&mut app, true);
    emit(
        &mut app,
        UpsertRawWall {
            wall,
            source: source(2.8),
        },
    );
    step(&mut app);
    assert_eq!(cpu(&app, wall).1, produced.1);
    assert_eq!(
        app.world().resource::<PublicWalls>().walls[&wall].segment_heights,
        high
    );
    let mut normal = source(2.8);
    normal.special = false;
    emit(
        &mut app,
        UpsertRawWall {
            wall,
            source: normal,
        },
    );
    step(&mut app);
    assert!(
        !app.world()
            .resource::<WallLintelEntities>()
            .0
            .contains_key(&wall)
    );
    assert!(
        app.world()
            .resource::<wall_bricks::WallBrickEntities>()
            .0
            .contains_key(&wall)
    );
}
