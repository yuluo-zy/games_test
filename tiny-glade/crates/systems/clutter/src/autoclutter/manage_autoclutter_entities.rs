//! B1422d04e0/10301B：使用唯一 canonical AutoClutter 和实际 Nani/Transform。
//! Window selector0/1/2 与具备真实来源的 selector3/5/6 可消费；
//! 被旧生产者抹去 owner 的输出仍在 ECS 修改前明确失败。

use bevy_ecs::prelude::*;
use country_core::{
    components::drawable::{AutoClutter, AutoClutterConvertableToDecoClutter, NaniBundle, NaniInstanceDesc, NaniInstanceMeshDesc},
    render::nani::{
        gpu_world::{GpuWorldSubsetKind, InstanceDataMarker},
        instance_data::{PlantInstanceData, SolidAlbedoMapEmissiveInstanceData, SolidAlbedoMapInstanceData,
            SolidVertexColorInstanceData, SolidVertexColorWindowInstanceData},
    },
    resources::{clutter_meshes::ClutterMarkedForDeletion, color_id::ColorId,
        critters::sheep::Sheep, time::GameTime},
    utils::components::transform::Transform,
};
use glam::{Mat3, Quat, Vec3};
use std::collections::{HashMap, HashSet};

use super::{AutoClutterSpawn, AutoClutterState, ResolvedClutterMesh, SpawnShared, WindowShutter};

fn resolved(spawn: &AutoClutterSpawn) -> (&ResolvedClutterMesh, Option<ColorId>) {
    match &spawn.shared {
        SpawnShared::Mesh { metadata, color } => {
            assert!(spawn.marker <= 2, "window source has invalid selector");
            assert!(spawn.marker == 2 || color.is_some(), "autoclutter shutter output is missing its actual ColorId payload");
            (metadata, *color)
        }
        SpawnShared::Object(object) => {
            assert!(matches!(spawn.marker, 3 | 5 | 6), "ORIGINAL_IMPLEMENTATION_PENDING: non-window selector {}", spawn.marker);
            (&object.metadata, None)
        }
        _ => panic!("ORIGINAL_IMPLEMENTATION_PENDING: autoclutter selected mesh/source owner erased (kind={}, key={:#x})", spawn.kind, spawn.key),
    }
}

/// B1422d2d20: two normalized crosses, then Quat::from_mat3.
fn orientation(spawn: &AutoClutterSpawn) -> Quat {
    let forward = Vec3::new(f32::from_bits(spawn.f54),
        f32::from_bits(spawn.f58), f32::from_bits(spawn.f5c));
    let right = Vec3::Y.cross(forward).normalize();
    let up = forward.cross(right).normalize();
    Quat::from_mat3(&Mat3::from_cols(right, up, forward))
}

fn insert_nani<T: Default + Send + Sync + 'static>(
    entity: &mut EntityCommands<'_>, metadata: &ResolvedClutterMesh,
    transform: Transform, subset_kind: GpuWorldSubsetKind,
) {
    entity.insert(NaniBundle::<T> {
        desc: NaniInstanceDesc { mesh: NaniInstanceMeshDesc::Named(metadata.name_hash), subset_kind },
        transform,
        marker: InstanceDataMarker::default(),
    });
}

fn emissive(metadata: &ResolvedClutterMesh) -> bool {
    ["clutter/pumpkin_w_face", "clutter/cart_w_pumpkins",
        "clutter/crates_w_pumpkins", "clutter/barrel_w_candles"].into_iter()
        .any(|name| metadata.name_hash == country_core::name_hash::NameHash::compute(name))
}

pub fn manage_autoclutter_entities(
    state: Res<AutoClutterState>,
    mut query: Query<(Entity, &AutoClutter, Option<&mut WindowShutter>, ComponentWrite<Transform>),
        Without<ClutterMarkedForDeletion>>,
    time: Res<GameTime>,
    mut commands: Commands,
    sheep: Query<&Transform, (With<Sheep>, Without<AutoClutter>)>,
) {
    // Native target map has last-value semantics for duplicate full u64 keys.
    let targets: HashMap<u64, &AutoClutterSpawn> = state.spawn_candidates.iter()
        .map(|spawn| (spawn.key, spawn)).collect();
    // Review requirement: preflight retained values only, after deduplication.
    // An overwritten unresolved source row is not a consumer input.
    for spawn in targets.values() { resolved(spawn); }

    let mut live = HashSet::new();
    for (entity, clutter, shutter, mut transform) in &mut query {
        live.insert(clutter.key);
        let Some(spawn) = targets.get(&clutter.key) else {
            commands.entity(entity).insert(ClutterMarkedForDeletion);
            continue;
        };
        transform.translation = spawn.position;
        let rotation = orientation(spawn);
        if let Some(mut shutter) = shutter {
            shutter.base_rotation = rotation;
            let (_, color) = resolved(spawn);
            // Every ordinary Window writer explicitly stores native+34=0;
            // this is a proven payload, not ColorId::default() for missing data.
            let color = if spawn.marker == 2 { ColorId(0) }
                else { color.expect("preflight shutter ColorId") };
            commands.entity(entity).insert(color);
        } else {
            transform.rotation = rotation;
        }
        // Transform.scale belongs to the actual animation, not to this update.
    }

    // 原版先完成差集及羊距离筛选，再按实际待建数量选择延迟区间。
    let pending: Vec<_> = targets.iter().filter_map(|(&key, &spawn)| {
        if live.contains(&key) { return None; }
        let (metadata, _) = resolved(spawn);
        sheep.iter().all(|transform|
            metadata.radius_xz + 1.0 <= spawn.position.distance(transform.translation))
            .then_some((key, spawn))
    }).collect();
    let mut rng = fastrand::Rng::with_seed(utils::rng::init_rng(time.frame_count));
    for (key, spawn) in &pending {
        let key = *key;
        let (metadata, color) = resolved(spawn);
        let r = rng.f32();
        let selector_offset = spawn.marker.wrapping_sub(3);
        let transform = Transform { rotation: orientation(spawn),
            translation: spawn.position, scale: Vec3::ZERO };
        let timer = match spawn.marker {
            6 => (1.0 - r) * 0.2 + r * 0.3,
            3 | 5 if pending.len() > 4 => (1.0 - r) * 0.3 + r * 0.8,
            3 | 5 => (1.0 - r) * 0.1 + r * 0.3,
            _ => r * 0.1,
        };
        let mut entity = commands.spawn(AutoClutter {
            key,
            timer,
            progress: 0.0,
            target_scale: spawn.scale,
            native_group: metadata.native_group,
            native_flag_15: selector_offset >= 4 || selector_offset == 1,
        });
        // B 1422d1be9：仅 marker >= 3 且 (marker & 6) != 4 插入。
        if spawn.marker >= 3 && (spawn.marker & 6) != 4 {
            let SpawnShared::Object(object) = &spawn.shared else { unreachable!() };
            entity.insert(AutoClutterConvertableToDecoClutter {
                source: object.source.clone(), destination: object.destination,
                mesh_name: object.metadata.mesh_name, bounds: object.metadata.bounds,
            });
        }
        if let Some(texture) = metadata.solid_texture {
            if emissive(metadata) {
                insert_nani::<SolidAlbedoMapEmissiveInstanceData>(&mut entity, metadata,
                    transform, GpuWorldSubsetKind::SolidAlbedoMapEmissive);
            } else {
                insert_nani::<SolidAlbedoMapInstanceData>(&mut entity, metadata,
                    transform, GpuWorldSubsetKind::SolidAlbedoMap);
            }
            entity.insert(texture);
        } else if spawn.marker == 5 {
            insert_nani::<PlantInstanceData>(&mut entity, metadata,
                transform, GpuWorldSubsetKind::Plant);
        } else if matches!(spawn.marker, 2 | 3 | 6) {
            insert_nani::<SolidVertexColorInstanceData>(&mut entity, metadata,
                transform, GpuWorldSubsetKind::SolidVertexColor);
        } else {
            insert_nani::<SolidVertexColorWindowInstanceData>(&mut entity, metadata,
                transform, GpuWorldSubsetKind::SolidVertexColorWindow);
        }
        if spawn.marker <= 1 {
            entity.insert((WindowShutter { base_rotation: transform.rotation,
                side: spawn.marker != 0 }, color.expect("preflight shutter ColorId")));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::autoclutter::{add_autoclutter_on_windows::add_autoclutter_on_windows,
        autoclutter_animation::autoclutter_animation, window_test_helpers::{candidate, world}};
    use country_core::systems::collision::{WindowAutoClutterCandidates, WindowAutoClutterKind};

    fn run(world: &mut World) {
        let mut schedule = Schedule::default();
        schedule.add_systems((add_autoclutter_on_windows, manage_autoclutter_entities).chain());
        schedule.run(world);
    }

    #[test]
    fn resolved_candidate_reaches_nani_conversion_and_deletion() {
        use crate::autoclutter::{AutoClutterCandidate, ResolvedAutoClutterObject,
            process_autoclutter_candidates::process_autoclutter_candidates};
        use country_core::{components::drawable::{AutoClutterDestination, AutoClutterSource},
            resources::clutter_meshes::{ClutterMeshes, ClutterMeshTy},
            systems::ui_clutter::DeletedAutoClutter};
        let mut world = world(vec![]);
        world.insert_resource(DeletedAutoClutter::default());
        let metadata = ResolvedClutterMesh::from(world.resource::<ClutterMeshes>().mesh(ClutterMeshTy::DoorBell));
        world.resource_mut::<AutoClutterState>().region_candidates.push(AutoClutterCandidate {
            resolved: Some(ResolvedAutoClutterObject { metadata,
                source: AutoClutterSource::None, destination: AutoClutterDestination::Terrain }),
            kind: 5, forced: 6, f40: 0x1234_5678_8765_4321,
            f48: u64::from(3.0f32.to_bits()) | (u64::from(7.0f32.to_bits()) << 32),
            f50: (-2.0f32).to_bits(), f5c: 1.0f32.to_bits(), f60: 1.0f32.to_bits(),
            ..Default::default()
        });
        let mut schedule = Schedule::default();
        schedule.add_systems((process_autoclutter_candidates, manage_autoclutter_entities).chain());
        schedule.run(&mut world);
        let entity = entity(&mut world);
        let source = world.get::<AutoClutterConvertableToDecoClutter>(entity).unwrap();
        assert!(matches!(source.source, AutoClutterSource::None));
        assert!(matches!(source.destination, AutoClutterDestination::Terrain));
        assert_eq!(source.mesh_name, metadata.mesh_name);
        assert_eq!(source.bounds, metadata.bounds);
        assert!(world.get::<InstanceDataMarker<SolidVertexColorInstanceData>>(entity).is_some());
        assert_eq!(world.get::<Transform>(entity).unwrap().translation, Vec3::new(3.0, 7.0, -2.0));
        world.get_mut::<AutoClutter>(entity).unwrap().progress = 1.0;
        {
            let mut state = world.resource_mut::<AutoClutterState>();
            state.region_candidates.clear();
            state.spawn_candidates.clear();
        }
        schedule.run(&mut world);
        assert!(world.get::<ClutterMarkedForDeletion>(entity).is_some());
        world.resource_mut::<GameTime>().delta = 0.3;
        let mut animation = Schedule::default();
        animation.add_systems(autoclutter_animation);
        animation.run(&mut world);
        assert!(world.get_entity(entity).is_err());
    }

    #[test]
    fn plant_selector_uses_plant_nani_and_is_not_convertible() {
        use crate::autoclutter::ResolvedAutoClutterObject;
        use country_core::{components::drawable::{AutoClutterDestination, AutoClutterSource},
            resources::clutter_meshes::{ClutterMeshes, ClutterMeshTy}};
        let mut world = world(vec![]);
        let metadata = ResolvedClutterMesh::from(world.resource::<ClutterMeshes>().mesh(ClutterMeshTy::DoorBell));
        world.resource_mut::<AutoClutterState>().spawn_candidates.push(AutoClutterSpawn {
            key: 33, marker: 5, f5c: 1.0f32.to_bits(),
            shared: SpawnShared::Object(ResolvedAutoClutterObject { metadata,
                source: AutoClutterSource::None, destination: AutoClutterDestination::Terrain }),
            ..Default::default()
        });
        run(&mut world);
        let entity = entity(&mut world);
        assert!(world.get::<InstanceDataMarker<PlantInstanceData>>(entity).is_some());
        assert!(world.get::<AutoClutterConvertableToDecoClutter>(entity).is_none());
        assert!(world.get::<WindowShutter>(entity).is_none());
    }

    #[test]
    fn convertible_component_retains_and_releases_the_actual_arc() {
        use crate::autoclutter::{AutoClutterCandidate, ResolvedAutoClutterObject,
            process_autoclutter_candidates::process_autoclutter_candidates};
        use country_core::{components::drawable::{AutoClutterDestination, AutoClutterSource},
            resources::clutter_meshes::{ClutterMeshes, ClutterMeshTy},
            systems::ui_clutter::DeletedAutoClutter};
        let mut world = world(vec![]);
        world.insert_resource(DeletedAutoClutter::default());
        let mesh = ResolvedClutterMesh::from(world.resource::<ClutterMeshes>().mesh(ClutterMeshTy::DoorBell));
        let points: std::sync::Arc<[Vec3]> = vec![Vec3::ZERO, Vec3::X].into();
        let weak = std::sync::Arc::downgrade(&points);
        world.resource_mut::<AutoClutterState>().region_candidates.push(AutoClutterCandidate {
            resolved: Some(ResolvedAutoClutterObject { metadata: mesh,
                source: AutoClutterSource::Points(points), destination: AutoClutterDestination::Terrain }),
            kind: 4, forced: 3, f40: 59, f5c: 1.0f32.to_bits(), f60: 1.0f32.to_bits(),
            ..Default::default()
        });
        let mut schedule = Schedule::default();
        schedule.add_systems((process_autoclutter_candidates, manage_autoclutter_entities).chain());
        schedule.run(&mut world);
        let entity = entity(&mut world);
        {
            let mut state = world.resource_mut::<AutoClutterState>();
            state.region_candidates.clear();
            state.spawn_candidates.clear();
        }
        assert!(weak.upgrade().is_some());
        assert!(world.get::<AutoClutterConvertableToDecoClutter>(entity).is_some());
        world.despawn(entity);
        assert!(weak.upgrade().is_none());
    }
    fn entity(world: &mut World) -> Entity {
        let mut query = world.query_filtered::<Entity, With<AutoClutter>>();
        let entities: Vec<_> = query.iter(world).collect();
        assert_eq!(entities.len(), 1);
        entities[0]
    }

    #[test]
    fn actual_drain_to_manager_uses_canonical_nani_and_retained_last_key_value() {
        let key = 0xabcd_ef01_0000_0001;
        let mut world = world(vec![candidate(key, WindowAutoClutterKind::DoorBell),
            candidate(key, WindowAutoClutterKind::WindowShutter { side: false, native_u32_04: 0xfedc_ba98 })]);
        // Overwritten erased metadata is not an actual retained consumer row.
        world.resource_mut::<AutoClutterState>().spawn_candidates.push(AutoClutterSpawn {
            key, shared: SpawnShared::Prefab("erased legacy source".into()), ..Default::default() });
        run(&mut world);
        let entity = entity(&mut world);
        let clutter = world.get::<AutoClutter>(entity).unwrap();
        assert_eq!(clutter.key, key);
        assert_eq!(clutter.progress, 0.0);
        assert_eq!(clutter.target_scale, 1.0);
        assert!(clutter.native_flag_15);
        assert_eq!(world.get::<Transform>(entity).unwrap().scale, Vec3::ZERO);
        assert_eq!(world.get::<Transform>(entity).unwrap().translation, Vec3::new(3.0, 7.0, -2.0));
        assert_eq!(world.get::<ColorId>(entity).unwrap().0, 0xfedc_ba98);
        assert!(world.get::<WindowShutter>(entity).unwrap().side);
        assert!(world.get::<InstanceDataMarker<SolidVertexColorWindowInstanceData>>(entity).is_some());
        assert_eq!(world.get::<NaniInstanceDesc>(entity).unwrap().subset_kind, GpuWorldSubsetKind::SolidVertexColorWindow);
        assert!(matches!(world.get::<NaniInstanceDesc>(entity).unwrap().mesh, NaniInstanceMeshDesc::Named(_)));

        world.component_mut::<Transform>(entity).unwrap().scale = Vec3::splat(0.37);
        let mut updated = candidate(key, WindowAutoClutterKind::DoorBell);
        updated.translation = Vec3::new(-8.0, 5.0, 2.0);
        world.resource_mut::<WindowAutoClutterCandidates>().0.push(updated);
        run(&mut world);
        assert_eq!(self::entity(&mut world), entity);
        assert_eq!(world.get::<Transform>(entity).unwrap().scale, Vec3::splat(0.37));
        assert_eq!(world.get::<Transform>(entity).unwrap().translation, Vec3::new(-8.0, 5.0, 2.0));
        assert_eq!(world.get::<ColorId>(entity).unwrap().0, 0,
            "ordinary writer's native+34 payload is explicitly zero");
    }

    #[test]
    fn missing_target_marks_then_actual_animation_despawns_the_same_entity() {
        let mut world = world(vec![candidate(4, WindowAutoClutterKind::DoorBell)]);
        run(&mut world);
        let entity = entity(&mut world);
        assert!(world.get::<InstanceDataMarker<SolidVertexColorInstanceData>>(entity).is_some());
        world.get_mut::<AutoClutter>(entity).unwrap().progress = 1.0;
        world.resource_mut::<AutoClutterState>().spawn_candidates.clear();
        run(&mut world);
        assert!(world.get::<ClutterMarkedForDeletion>(entity).is_some());
        world.resource_mut::<GameTime>().delta = 0.3;
        let mut animation = Schedule::default();
        animation.add_systems(autoclutter_animation);
        animation.run(&mut world);
        assert!(world.get_entity(entity).is_err());
    }

    #[test]
    fn unresolved_retained_source_fails_before_existing_entity_is_changed() {
        let mut world = world(vec![candidate(7, WindowAutoClutterKind::DoorBell)]);
        run(&mut world);
        let entity = entity(&mut world);
        let before = *world.get::<Transform>(entity).unwrap();
        world.resource_mut::<AutoClutterState>().spawn_candidates.clear();
        world.resource_mut::<AutoClutterState>().spawn_candidates.push(AutoClutterSpawn {
            key: 8, shared: SpawnShared::ArcStrong(123), ..Default::default() });
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| run(&mut world)));
        assert!(result.is_err());
        assert_eq!(*world.get::<Transform>(entity).unwrap(), before);
        assert!(world.get::<ClutterMarkedForDeletion>(entity).is_none());
    }

    #[test]
    fn sheep_guard_uses_actual_transform_and_full_3d_distance_with_nan_rejection() {
        use country_core::resources::critters::sheep::SheepMoveState;
        for (offset, accepted) in [(Vec3::Y * 3.0, true), (Vec3::Y, false), (Vec3::splat(f32::NAN), false)] {
            let mut world = world(vec![candidate(9, WindowAutoClutterKind::DoorBell)]);
            world.spawn((Sheep { behavior_state: 0, goal_timer: 0.0, pet_phase: 0,
                pet_progress: 0.0, pet_target: 0, move_state: SheepMoveState::Idle { timeout_s: 0.0 },
                amplitude: 0.0, control_flag: 0 }, Transform::from_translation(Vec3::new(3.0, 7.0, -2.0) + offset)));
            run(&mut world);
            let count = world.query_filtered::<Entity, With<AutoClutter>>().iter(&world).count();
            assert_eq!(count, usize::from(accepted));
        }
    }
    #[test]
    fn actual_metadata_texture_is_attached_to_the_matching_nani_type() {
        use crate::autoclutter::window_test_helpers::textured_meshes;
        use country_core::components::drawable::NaniSolidTexture;
        for (name, kind) in [("unit/nonemissive", GpuWorldSubsetKind::SolidAlbedoMap),
            ("clutter/pumpkin_w_face", GpuWorldSubsetKind::SolidAlbedoMapEmissive)] {
            let mut world = world(vec![candidate(10, WindowAutoClutterKind::WindowFlowerpot)]);
            let texture = NaniSolidTexture::try_from(4u8).unwrap();
            world.insert_resource(textured_meshes(name, texture));
            run(&mut world);
            let entity = entity(&mut world);
            assert_eq!(world.get::<NaniInstanceDesc>(entity).unwrap().subset_kind, kind);
            assert_eq!(*world.get::<NaniSolidTexture>(entity).unwrap(), texture);
            if kind == GpuWorldSubsetKind::SolidAlbedoMap {
                assert!(world.get::<InstanceDataMarker<SolidAlbedoMapInstanceData>>(entity).is_some());
            } else {
                assert!(world.get::<InstanceDataMarker<SolidAlbedoMapEmissiveInstanceData>>(entity).is_some());
            }
        }
    }

}

use country_core::render::extract::WorldComponentWrite as _;

use country_core::render::extract::ComponentWrite;
