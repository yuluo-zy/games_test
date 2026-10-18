use bevy_ecs::prelude::*;
use tg_building_runtime::nani::*;
use tg_building_runtime::{
    Animation, ObjectTransform, Roof, RoofSpire, RoofSupportBeams, RoofTiles,
};
use tg_building_runtime::{TileSimulation, TriggerVolume};

#[test]
fn explicit_stone_floor_bundle_has_three_components_and_empty_required_graph() {
    let mut world = World::new();
    register_nani_components(&mut world);
    let entity = world.spawn(NaniStoneFloorBundle::default()).id();
    let row = world.entity(entity);
    assert_eq!(row.archetype().components().count(), 3);
    assert_eq!(
        *row.get::<NaniInstanceDesc>().unwrap(),
        NaniInstanceDesc {
            mesh: NaniInstanceMeshDesc::AtlasPart(0),
            subset_kind: 9,
        }
    );
    assert!(row.contains::<StoneFloorInstanceDataMarker>());
    assert_eq!(
        row.get::<ObjectTransform>().unwrap().0,
        ObjectTransform::default().0
    );
    for id in [
        world.register_component::<NaniInstanceDesc>(),
        world.register_component::<ObjectTransform>(),
        world.register_component::<StoneFloorInstanceDataMarker>(),
        world.register_component::<RoofTiles>(),
        world.register_component::<RoofSpire>(),
        world.register_component::<RoofSupportBeams>(),
        world.register_component::<Roof>(),
        world.register_component::<TileSimulation>(),
        world.register_component::<Animation>(),
        world.register_component::<TriggerVolume>(),
    ] {
        let required = world
            .components()
            .get_info(id)
            .unwrap()
            .required_components();
        assert_eq!(required.iter_ids().count(), 0);
    }
}

#[test]
fn initial_and_changed_named_meshes_use_different_original_selection_rules() {
    let descriptor = NaniInstanceDesc {
        mesh: NaniInstanceMeshDesc::Named(0xfeed),
        subset_kind: 9,
    };
    let entries = [
        NaniMeshAtlasEntry {
            atlas_part: 3,
            subset_kind: 7,
        },
        NaniMeshAtlasEntry {
            atlas_part: 8,
            subset_kind: 9,
        },
        NaniMeshAtlasEntry {
            atlas_part: 11,
            subset_kind: 9,
        },
    ];
    assert_eq!(resolve_nani_mesh(descriptor, &entries, false), Ok(8));
    assert_eq!(
        resolve_nani_mesh(descriptor, &entries, true),
        Err(NaniAtlasError::ChangedNamedMeshMustHaveOnePart)
    );
    assert_eq!(
        resolve_nani_mesh(descriptor, &entries[..1], false),
        Err(NaniAtlasError::MissingMatchingSubset)
    );
    assert_eq!(
        resolve_nani_mesh(descriptor, &entries[..1], true),
        Err(NaniAtlasError::ChangedNamedMeshSubsetMismatch)
    );
    assert_eq!(resolve_nani_mesh(descriptor, &entries[1..2], true), Ok(8));
    assert_eq!(
        resolve_nani_mesh(NaniInstanceDesc::default(), &[], false),
        Ok(0)
    );
}

#[test]
fn actual_world_changes_stage_upload_and_despawn_releases_cpu_registration_once() {
    let mut world = World::new();
    world.init_resource::<DeferredNaniRequests>();
    world.init_resource::<NaniCpuStagingState>();
    world.init_resource::<NaniMeshAtlas>();
    world.init_resource::<NaniBackendConfirmations>();
    world.init_resource::<NaniCpuDiagnostics>();
    let mut roof = [0; 88];
    roof[8..12].copy_from_slice(&1_u32.to_le_bytes());
    for (offset, value) in [(0x0c, 1_f32), (0x1c, 4_f32), (0x20, 3_f32), (0x4c, 2_f32)] {
        roof[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
    }
    let entity = world
        .spawn((
            Roof(roof),
            Animation::default(),
            NaniStoneFloorBundle::default(),
        ))
        .id();
    let mut schedule = Schedule::default();
    schedule.add_systems(stage_roof_nani_updates);
    schedule.run(&mut world);
    assert!(world.resource::<NaniCpuStagingState>().contains(entity));
    assert!(
        matches!(world.resource::<DeferredNaniRequests>().0.as_slice(),
        [DeferredNaniRequest::Create { entity: created, .. }] if *created == entity)
    );
    world.resource_mut::<DeferredNaniRequests>().0.clear();
    schedule.run(&mut world);
    assert!(world.resource::<DeferredNaniRequests>().0.is_empty());
    world.entity_mut(entity).get_mut::<Roof>().unwrap().0[0x4c..0x50]
        .copy_from_slice(&5_f32.to_le_bytes());
    schedule.run(&mut world);
    assert!(
        matches!(world.resource::<DeferredNaniRequests>().0.as_slice(),
        [DeferredNaniRequest::Update { mesh_changed: false, snapshot, .. }]
        if snapshot.stone_floor.bounds_center[1] == 5_f32 + f32::from_bits(0xbf30a3d7))
    );
    world.resource_mut::<DeferredNaniRequests>().0.clear();
    world.despawn(entity);
    schedule.run(&mut world);
    assert!(world.resource::<NaniCpuStagingState>().is_empty());
    assert!(
        matches!(world.resource::<DeferredNaniRequests>().0.as_slice(),
        [DeferredNaniRequest::Remove { entity: removed }] if *removed == entity)
    );
    world.resource_mut::<DeferredNaniRequests>().0.clear();
    schedule.run(&mut world);
    assert!(world.resource::<DeferredNaniRequests>().0.is_empty());
}

#[test]
fn named_atlas_resolution_is_consumed_by_the_real_staging_system() {
    let mut world = World::new();
    world.init_resource::<DeferredNaniRequests>();
    world.init_resource::<NaniCpuStagingState>();
    world.init_resource::<NaniMeshAtlas>();
    world.init_resource::<NaniBackendConfirmations>();
    world.init_resource::<NaniCpuDiagnostics>();
    let mut roof = [0; 88];
    roof[8..12].copy_from_slice(&1u32.to_le_bytes());
    for (o, v) in [(0x0c, 1f32), (0x1c, 6.), (0x20, 8.), (0x4c, 3.)] {
        roof[o..o + 4].copy_from_slice(&v.to_le_bytes());
    }
    let entity = world
        .spawn((
            Roof(roof),
            Animation::default(),
            NaniStoneFloorBundle::default(),
        ))
        .id();
    world
        .entity_mut(entity)
        .get_mut::<NaniInstanceDesc>()
        .unwrap()
        .mesh = NaniInstanceMeshDesc::Named(42);
    let mut schedule = Schedule::default();
    schedule.add_systems(stage_roof_nani_updates);
    schedule.run(&mut world);
    assert!(world.resource::<DeferredNaniRequests>().0.is_empty());
    assert_eq!(
        world.resource::<NaniCpuDiagnostics>().0,
        vec![(entity, NaniAtlasError::MissingMatchingSubset)]
    );
    world.resource_mut::<NaniMeshAtlas>().0.insert(
        42,
        vec![
            NaniMeshAtlasEntry {
                atlas_part: 3,
                subset_kind: 7,
            },
            NaniMeshAtlasEntry {
                atlas_part: 17,
                subset_kind: 9,
            },
        ],
    );
    schedule.run(&mut world);
    assert!(world.resource::<NaniCpuDiagnostics>().0.is_empty());
    assert!(
        matches!(&world.resource::<DeferredNaniRequests>().0[0],DeferredNaniRequest::Create{snapshot,..} if snapshot.resolved_atlas_part==17)
    );
    assert!(world.resource::<NaniBackendConfirmations>().0.is_empty());
    world.resource_mut::<DeferredNaniRequests>().0.clear();
    world.despawn(entity);
    schedule.run(&mut world);
    assert!(world.resource::<NaniCpuStagingState>().is_empty());
    assert!(
        matches!(&world.resource::<DeferredNaniRequests>().0[0],DeferredNaniRequest::Remove{entity:e} if *e==entity)
    );
}

#[test]
fn atlas_remapping_marks_mesh_updates_for_both_staging_consumers() {
    let mut app = bevy_app::App::new();
    app.init_resource::<DeferredNaniRequests>()
        .init_resource::<NaniCpuStagingState>()
        .init_resource::<NaniMeshAtlas>()
        .init_resource::<NaniBackendConfirmations>()
        .init_resource::<NaniCpuDiagnostics>()
        .init_resource::<SolidNaniCpuChanges>()
        .init_resource::<SolidNaniCpuStaging>()
        .init_resource::<SolidNaniCpuDiagnostics>()
        .add_systems(
            bevy_app::Update,
            (stage_roof_nani_updates, stage_solid_nani_updates).chain(),
        );
    let mut roof = [0; 88];
    roof[8..12].copy_from_slice(&1u32.to_le_bytes());
    let descriptor = NaniInstanceDesc {
        mesh: NaniInstanceMeshDesc::Named(42),
        subset_kind: 9,
    };
    let floor = app
        .world_mut()
        .spawn((
            Roof(roof),
            Animation::default(),
            NaniStoneFloorBundle {
                descriptor,
                ..Default::default()
            },
        ))
        .id();
    let solid = app
        .world_mut()
        .spawn(NaniSolidVertexColorBundle {
            descriptor,
            transform: ObjectTransform::default(),
            marker: SolidVertexColorInstanceDataMarker,
        })
        .id();
    app.world_mut().resource_mut::<NaniMeshAtlas>().0.insert(
        42,
        vec![NaniMeshAtlasEntry {
            atlas_part: 1,
            subset_kind: 9,
        }],
    );
    app.update();
    app.world_mut()
        .resource_mut::<DeferredNaniRequests>()
        .0
        .clear();
    app.world_mut()
        .resource_mut::<SolidNaniCpuChanges>()
        .0
        .clear();
    app.world_mut()
        .resource_mut::<NaniMeshAtlas>()
        .0
        .get_mut(&42)
        .unwrap()[0]
        .atlas_part = 2;
    app.update();
    assert!(app.world().resource::<DeferredNaniRequests>().0.iter().any(|change| matches!(change, DeferredNaniRequest::Update { entity, mesh_changed: true, .. } if *entity == floor)));
    assert!(app.world().resource::<SolidNaniCpuChanges>().0.iter().any(|change| matches!(change, SolidNaniCpuChange::Update { entity, mesh_changed: true, .. } if *entity == solid)));
}

#[test]
fn nonfinite_roof_uploads_are_rejected_before_staging() {
    let mut roof = [0; 88];
    roof[8..12].copy_from_slice(&1u32.to_le_bytes());
    roof[0x4c..0x50].copy_from_slice(&f32::INFINITY.to_le_bytes());
    assert_eq!(
        project_roof_stone_floor(&roof, 0.),
        Err(StoneFloorProjectionError::NonfiniteGeometry)
    );
    roof[0x4c..0x50].fill(0);
    assert_eq!(
        project_roof_stone_floor(&roof, f32::NAN),
        Err(StoneFloorProjectionError::NonfiniteGeometry)
    );
}
