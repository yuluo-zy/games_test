use bevy_ecs::prelude::*;
use country_core::resources::roofs::creation::RoofCreateRequestObserved;
use country_core::resources::walls::publication_source::RawWallPublicationSource;
use country_core::resources::walls::publication_source::TerrainHeightGrid512;
use country_core::systems::wall::wall_state::path_preparation::PathRasters512;
use country_core::resources::walls::publication_fields::WallId;
use country_core::resources::walls::publication_fields::WallShapeSnapshot;
use country_core::resources::roofs::geometry::surface::Curve2;
use std::sync::Arc;
use tg_building_runtime::{wall_arches::*, wall_paths::*, wall_structures::*, *};
fn emit<T: Event>(app: &mut bevy_app::App, value: T) {
    app.world_mut().resource_mut::<Events<T>>().send(value);
}
fn paths(app: &mut bevy_app::App, patch: bool) {
    let mut bytes = vec![0; 512 * 512];
    if patch {
        for z in 246..269 {
            for x in 246..265 {
                bytes[z * 512 + x] = 255;
            }
        }
    }
    emit(
        app,
        SetWallPathRasters(
            PathRasters512::from_encoded(
                [
                    Arc::from(bytes),
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
fn raw(tag: u32, scalar: f32, special: bool) -> RawWallPublicationSource {
    let mut shape = [0; 28];
    shape[..4].copy_from_slice(&tag.to_le_bytes());
    if tag == 1 {
        for (i, v) in [1f32, 0., 0., 0., 4., 4.].iter().enumerate() {
            shape[4 + i * 4..8 + i * 4].copy_from_slice(&v.to_le_bytes());
        }
    }
    let points = if tag == 2 {
        vec![[-2., -2.], [-2., 0.], [0., 1.], [2., 2.]]
    } else {
        vec![[-2., -2.], [-2., 2.], [2., 2.], [2., -2.], [-2., -2.]]
    };
    RawWallPublicationSource {
        curve: Curve2::try_new(points, false).unwrap(),
        curve_kind_tag: tag,
        terrain_floor: 0.,
        shape: WallShapeSnapshot::from_observed(shape),
        style: 0,
        extra: scalar,
        special,
        observed_path_scalar: 2.8,
        clutter_version: 0,
        observed_source_flag_64: 0,
    }
}
fn setup() -> bevy_app::App {
    let mut app = building_app();
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
    app
}
fn step(app: &mut bevy_app::App) {
    app.update();
    assert!(
        app.world().resource::<BuildingReport>().faults.is_empty(),
        "{:?}",
        app.world().resource::<BuildingReport>().faults
    );
}
fn output(app: &bevy_app::App, id: WallId) -> (u64, Vec<[u8; 96]>) {
    let e = app.world().resource::<WallArchEntities>().0[&id];
    let c = app.world().get::<WallCpuArches>(e).unwrap();
    (
        c.generation,
        c.instances.iter().map(|i| i.observed_bytes()).collect(),
    )
}
#[test]
fn raw_paths_regular_and_freehand_rebuild_without_feedback_clear_and_remove() {
    for tag in [1, 2] {
        let mut app = setup();
        let wall = WallId(100011);
        emit(
            &mut app,
            UpsertRawWall {
                wall,
                source: raw(tag, 2.8, false),
            },
        );
        step(&mut app);
        assert!(output(&app, wall).1.is_empty());
        paths(&mut app, true);
        step(&mut app);
        let high = output(&app, wall);
        assert!(!high.1.is_empty());
        let arch = app.world().resource::<PublicWalls>().walls[&wall]
            .arch_segment_heights
            .clone();
        let third = app.world().resource::<PublicWalls>().walls[&wall]
            .segment_heights
            .clone();
        step(&mut app);
        assert_eq!(output(&app, wall), high);
        assert_eq!(
            app.world().resource::<PublicWalls>().walls[&wall].arch_segment_heights,
            arch
        );
        assert_eq!(
            app.world().resource::<PublicWalls>().walls[&wall].segment_heights,
            third
        );
        emit(&mut app, WallSpatialDirty(wall));
        step(&mut app);
        assert_eq!(output(&app, wall), high);
        assert_eq!(
            app.world().resource::<PublicWalls>().walls[&wall].arch_segment_heights,
            arch
        );
        assert_eq!(
            app.world().resource::<PublicWalls>().walls[&wall].segment_heights,
            third
        );
        emit(
            &mut app,
            UpsertRawWall {
                wall,
                source: raw(tag, 1.1, false),
            },
        );
        step(&mut app);
        let low = output(&app, wall);
        assert_ne!(low.1, high.1);
        let lower = app.world().resource::<PublicWalls>().walls[&wall]
            .segment_heights
            .clone();
        let mut fresh = setup();
        paths(&mut fresh, true);
        emit(
            &mut fresh,
            UpsertRawWall {
                wall,
                source: raw(tag, 1.1, false),
            },
        );
        step(&mut fresh);
        assert_eq!(
            lower,
            fresh.world().resource::<PublicWalls>().walls[&wall].segment_heights
        );
        assert_eq!(low.1, output(&fresh, wall).1);
        paths(&mut app, false);
        step(&mut app);
        assert!(output(&app, wall).1.is_empty());
        assert!(
            app.world().resource::<PublicWalls>().walls[&wall]
                .segment_heights
                .iter()
                .all(|h| *h == 0.)
        );
        emit(&mut app, RemoveWall(wall));
        step(&mut app);
        assert!(
            !app.world()
                .resource::<WallArchEntities>()
                .0
                .contains_key(&wall)
        );
        paths(&mut app, true);
        emit(
            &mut app,
            UpsertRawWall {
                wall,
                source: raw(tag, 2.8, false),
            },
        );
        step(&mut app);
        assert_eq!(output(&app, wall).1, high.1);
        emit(
            &mut app,
            UpsertRawWall {
                wall,
                source: raw(tag, 2.8, true),
            },
        );
        step(&mut app);
        assert!(
            !app.world()
                .resource::<WallArchEntities>()
                .0
                .contains_key(&wall)
        );
        assert!(
            app.world()
                .resource::<WallLintelEntities>()
                .0
                .contains_key(&wall)
        );
        assert!(
            app.world().resource::<PublicWalls>().walls[&wall]
                .segment_heights
                .iter()
                .any(|h| *h > 0.)
        );
    }
}
#[test]
fn actual_roof_relationship_emits_gate_payloads_via_bevy_and_changes_with_source() {
    let mut app = setup();
    let wall = WallId(12345);
    app.world_mut().resource_mut::<wall_authoring::WallAuthoringState>().history_metadata.insert(
        wall,
        country_core::resources::history::wall_edit::WallHistoryMetadata::for_new_wall(false),
    );
    let mut strip = vec![0; 512 * 512];
    for z in 240..274 {
        for x in 249..254 {
            strip[z * 512 + x] = 255;
        }
    }
    emit(
        &mut app,
        SetWallPathRasters(
            PathRasters512::from_encoded(
                [
                    Arc::from(strip),
                    Arc::from(vec![0; 512 * 512]),
                    Arc::from(vec![0; 512 * 512]),
                    Arc::from(vec![0; 512 * 512]),
                ],
                true,
            )
            .unwrap(),
        ),
    );
    emit(
        &mut app,
        UpsertRawWall {
            wall,
            source: raw(1, 2.8, false),
        },
    );
    step(&mut app);
    let mut cursor = app
        .world()
        .resource::<Events<ConstructGateCmd>>()
        .get_cursor();
    assert_eq!(
        cursor
            .read(app.world().resource::<Events<ConstructGateCmd>>())
            .count(),
        0
    );
    let mut request = [0; 64];
    request[8..16].copy_from_slice(&wall.0.to_le_bytes());
    for (offset, v) in [(40, 0.5f32), (44, 2.), (48, 1.), (52, 0.5)] {
        request[offset..offset + 4].copy_from_slice(&v.to_le_bytes());
    }
    emit(
        &mut app,
        CreateRoof(RoofCreateRequestObserved::from_observed(request)),
    );
    step(&mut app);
    assert!(app.world().resource::<WallRoofMap>().entity(wall).is_some());
    let commands: Vec<_> = cursor
        .read(app.world().resource::<Events<ConstructGateCmd>>())
        .cloned()
        .collect();
    assert!(!commands.is_empty());
    assert!(
        commands
            .iter()
            .all(|c| c.0.wall_id == wall && c.0.ordinal > 0 && !c.0.segment.positions.is_empty())
    );
    // The producer emits commands only on rebuild. Missing dependencies must
    // retain that batch so completing the dependency can resume construction.
    let terrain = app
        .world_mut()
        .resource_mut::<WallCollectionState>()
        .terrain
        .take()
        .unwrap();
    for command in &commands {
        emit(&mut app, command.clone());
    }
    step(&mut app);
    assert_eq!(
        app.world()
            .resource::<gates::GateBuildState>()
            .pending
            .get(&wall),
        Some(&gates::PendingGateReason::MissingTerrain)
    );
    app.world_mut()
        .resource_mut::<WallCollectionState>()
        .terrain = Some(terrain);
    step(&mut app);
    assert!(
        app.world()
            .resource::<gates::GateBuildState>()
            .cpu_outputs
            .contains_key(&wall),
        "retained gate commands must resume without another producer event"
    );
    // Terrain can change outside the wall's sampled outline. The gate still
    // samples that terrain for support geometry and must refresh independently.
    let gate_generation_before_terrain = app.world().resource::<gates::Gates>().0[&wall].clone();
    {
        let mut collection = app.world_mut().resource_mut::<WallCollectionState>();
        let terrain = collection.terrain.as_mut().unwrap();
        terrain.maximum_y = 0.1;
    }
    step(&mut app);
    assert!(
        app.world()
            .resource::<gates::GateBuildState>()
            .rebuilt
            .contains(&wall)
    );
    assert_ne!(
        app.world().resource::<gates::Gates>().0[&wall],
        gate_generation_before_terrain
    );
    // Mutating the public Arc uses COW because the cached fingerprint retains
    // its allocation; replacing terrain cells invalidates the support meshes.
    {
        let mut collection = app.world_mut().resource_mut::<WallCollectionState>();
        let terrain = collection.terrain.as_mut().unwrap();
        Arc::make_mut(&mut terrain.encoded).fill(1);
    }
    step(&mut app);
    assert!(
        app.world()
            .resource::<gates::GateBuildState>()
            .rebuilt
            .contains(&wall)
    );
    let gate_entities = app.world().resource::<gates::Gates>().0[&wall].clone();
    assert!(
        app.world()
            .resource::<gates::GateBuildState>()
            .cpu_outputs
            .contains_key(&wall)
    );
    let mut keep = [0; 64];
    keep[32..40].copy_from_slice(&wall.0.to_le_bytes());
    emit(
        &mut app,
        roof_edits::EditRoof(
            country_core::resources::roofs::geometry::edit_rules::RoofEditEvent::from_observed(&keep),
        ),
    );
    emit(&mut app, WallSpatialDirty(wall));
    step(&mut app);
    assert_eq!(
        app.world().resource::<gates::Gates>().0[&wall],
        gate_entities
    );
    assert!(
        app.world()
            .resource::<gates::GateBuildState>()
            .cpu_outputs
            .contains_key(&wall)
    );
    assert!(
        app.world()
            .resource::<gates::GateBuildState>()
            .rebuilt
            .is_empty()
    );
    assert_eq!(
        cursor
            .read(app.world().resource::<Events<ConstructGateCmd>>())
            .count(),
        0
    );
    emit(
        &mut app,
        UpsertRawWall {
            wall,
            source: raw(1, 1.1, false),
        },
    );
    step(&mut app);
    let changed: Vec<_> = cursor
        .read(app.world().resource::<Events<ConstructGateCmd>>())
        .cloned()
        .collect();
    assert!(!changed.is_empty());
    assert_ne!(changed[0].0.segment.heights, commands[0].0.segment.heights);
    assert!(
        app.world()
            .resource::<gates::GateBuildState>()
            .cpu_outputs
            .contains_key(&wall)
    );
    emit(&mut app, DeleteRoof(wall));
    for command in &changed {
        emit(&mut app, command.clone());
    }
    step(&mut app);
    assert!(
        !app.world()
            .resource::<gates::GateBuildState>()
            .cpu_outputs
            .contains_key(&wall)
    );
    assert!(!app.world().resource::<gates::Gates>().0.contains_key(&wall));
    emit(&mut app, RemoveWall(wall));
    step(&mut app);
    assert!(
        !app.world()
            .resource::<WallArchEntities>()
            .0
            .contains_key(&wall)
    );
    assert_eq!(
        cursor
            .read(app.world().resource::<Events<ConstructGateCmd>>())
            .count(),
        0
    );
}
