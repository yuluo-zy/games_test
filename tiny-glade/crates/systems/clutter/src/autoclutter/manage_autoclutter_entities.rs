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

use super::{
    AutoClutterPayload, AutoClutterSpawn, AutoClutterState, ResolvedClutterMesh, WindowShutter,
};

fn resolved(spawn: &AutoClutterSpawn) -> &ResolvedClutterMesh {
    match &spawn.payload {
        AutoClutterPayload::Mesh { metadata } => {
            assert!(
                matches!(spawn.marker, 0 | 1 | 2),
                "window mesh output has invalid selector {}",
                spawn.marker
            );
            metadata
        }
        AutoClutterPayload::Object(object) => {
            // The original consumer branches on the raw selector byte and
            // covers every 0..=255 value; the typed object payload is the
            // non-window source projection, so selector 0..=2 stays with the
            // window Mesh payload and all higher selectors route here.
            assert!(
                spawn.marker >= 3,
                "window selector {} cannot use an object payload",
                spawn.marker
            );
            &object.metadata
        }
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

    let mut live = HashSet::new();
    for (entity, clutter, shutter, mut transform) in &mut query {
        live.insert(clutter.key);
        let Some(spawn) = targets.get(&clutter.key) else {
            commands.entity(entity).insert(ClutterMarkedForDeletion);
            continue;
        };
        // B 1422d0f57..0x1422d10ff: a live entity is only rewritten when the
        // target moved more than 0.1 units. The original evaluates
        // `0.1 < sqrt(dz*dz + dy*dy + dx*dx)` in exactly this accumulation
        // order; unordered operands keep the comparison false and skip the
        // update, mirroring the native branch.
        let dx = spawn.position.x - transform.translation.x;
        let dy = spawn.position.y - transform.translation.y;
        let dz = spawn.position.z - transform.translation.z;
        if !(0.1 < (dz * dz + dy * dy + dx * dx).sqrt()) {
            continue;
        }
        transform.translation = spawn.position;
        let rotation = orientation(spawn);
        if let Some(mut shutter) = shutter {
            shutter.base_rotation = rotation;
            commands.entity(entity).insert(ColorId(spawn.selector_value));
        } else {
            transform.rotation = rotation;
        }
        // Transform.scale belongs to the actual animation, not to this update.
    }

    // 原版先完成差集及羊距离筛选，再按实际待建数量选择延迟区间。
    let pending: Vec<_> = targets.iter().filter_map(|(&key, &spawn)| {
        if live.contains(&key) { return None; }
        let metadata = resolved(spawn);
        sheep.iter().all(|transform|
            metadata.radius_xz + 1.0 <= spawn.position.distance(transform.translation))
            .then_some((key, spawn))
    }).collect();
    let rng = fastrand::Rng::with_seed(utils::rng::init_rng(time.frame_count));
    for (key, spawn) in &pending {
        let key = *key;
        let metadata = resolved(spawn);
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
            let AutoClutterPayload::Object(object) = &spawn.payload else {
                unreachable!()
            };
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
        // O@1422d1133..1160: `marker != 2 && (marker-3 > 3 || marker-3 == 1)`
        // in raw byte arithmetic receives WindowShutter and the source+34
        // ColorId. The component's byte is tested only for zero/nonzero by
        // animation, hence the retained boolean projection.
        if spawn.marker != 2
            && (spawn.marker.wrapping_sub(3) > 3 || spawn.marker.wrapping_sub(3) == 1)
        {
            entity.insert((WindowShutter { base_rotation: transform.rotation,
                side: spawn.marker != 0 }, ColorId(spawn.selector_value)));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::autoclutter::{add_autoclutter_on_windows::add_autoclutter_on_windows,
        autoclutter_animation::autoclutter_animation, window_test_helpers::{candidate, world}};
    use country_core::render::extract::WorldComponentWrite as _;
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
            kind: 5,
            payload: AutoClutterPayload::Object(ResolvedAutoClutterObject {
                metadata,
                source: AutoClutterSource::None,
                destination: AutoClutterDestination::Terrain,
            }),
            f18: 2,
            key: 33,
            marker: 5,
            selector_value: 0,
            position: Vec3::ZERO,
            host_id: 0,
            f48: 0,
            f50: 0,
            f54: 0,
            f58: 0,
            f5c: 1.0f32.to_bits(),
            scale: 1.0,
        });
        run(&mut world);
        let entity = entity(&mut world);
        assert!(world.get::<InstanceDataMarker<PlantInstanceData>>(entity).is_some());
        assert!(world.get::<AutoClutterConvertableToDecoClutter>(entity).is_none());
        assert!(world.get::<WindowShutter>(entity).is_none());
    }

    #[test]
    fn selector_four_keeps_the_original_shutter_and_color_branch() {
        use crate::autoclutter::ResolvedAutoClutterObject;
        use country_core::{components::drawable::{AutoClutterDestination, AutoClutterSource},
            resources::clutter_meshes::{ClutterMeshes, ClutterMeshTy}};
        let mut world = world(vec![]);
        let metadata =
            ResolvedClutterMesh::from(world.resource::<ClutterMeshes>().mesh(ClutterMeshTy::DoorBell));
        world
            .resource_mut::<AutoClutterState>()
            .spawn_candidates
            .push(AutoClutterSpawn {
                kind: 4,
                payload: AutoClutterPayload::Object(ResolvedAutoClutterObject {
                    metadata,
                    source: AutoClutterSource::Points(vec![Vec3::ZERO].into()),
                    destination: AutoClutterDestination::Terrain,
                }),
                f18: 1,
                key: 44,
                marker: 4,
                selector_value: 0x0f0e_0d0c,
                position: Vec3::ZERO,
                host_id: 0,
                f48: 0,
                f50: 0,
                f54: 0,
                f58: 0,
                f5c: 1.0f32.to_bits(),
                scale: 1.0,
            });
        run(&mut world);
        let entity = entity(&mut world);
        assert!(world.get::<AutoClutterConvertableToDecoClutter>(entity).is_none());
        assert!(world.get::<InstanceDataMarker<SolidVertexColorWindowInstanceData>>(entity).is_some());
        let shutter = world.get::<WindowShutter>(entity).expect("selector 4 must install a shutter");
        assert!(shutter.side);
        assert_eq!(world.get::<ColorId>(entity).unwrap().0, 0x0f0e_0d0c);
    }

    #[test]
    fn selectors_above_six_follow_the_native_byte_branches() {
        use crate::autoclutter::ResolvedAutoClutterObject;
        use country_core::{components::drawable::{AutoClutterDestination, AutoClutterSource},
            resources::clutter_meshes::{ClutterMeshes, ClutterMeshTy}};
        for (marker, convertible) in [(7u8, true), (12u8, false), (255u8, true)] {
            let mut world = world(vec![]);
            let metadata =
                ResolvedClutterMesh::from(world.resource::<ClutterMeshes>().mesh(ClutterMeshTy::DoorBell));
            world
                .resource_mut::<AutoClutterState>()
                .spawn_candidates
                .push(AutoClutterSpawn {
                    kind: 5,
                    payload: AutoClutterPayload::Object(ResolvedAutoClutterObject {
                        metadata,
                        source: AutoClutterSource::None,
                        destination: AutoClutterDestination::Terrain,
                    }),
                    f18: 1,
                    key: u64::from(marker) + 0x100,
                    marker,
                    selector_value: 0x0a0b_0c0d,
                    position: Vec3::ZERO,
                    host_id: 0,
                    f48: 0,
                    f50: 0,
                    f54: 0,
                    f58: 0,
                    f5c: 1.0f32.to_bits(),
                    scale: 1.0,
                });
            run(&mut world);
            let entity = entity(&mut world);
            assert_eq!(
                world.get::<AutoClutterConvertableToDecoClutter>(entity).is_some(),
                convertible,
                "marker {marker} convertable branch"
            );
            assert!(world
                .get::<InstanceDataMarker<SolidVertexColorWindowInstanceData>>(entity)
                .is_some());
            let shutter = world
                .get::<WindowShutter>(entity)
                .expect("selectors above six with (marker-3)>3 install a shutter");
            assert!(shutter.side);
            assert_eq!(world.get::<ColorId>(entity).unwrap().0, 0x0a0b_0c0d);
        }
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
    fn existing_entities_keep_the_native_point_one_rewrite_gate() {
        // B 1422d0f57: the rewrite branch is guarded by
        // `0.1 < sqrt(dz^2 + dy^2 + dx^2)`. Nothing is written inside the
        // gate, including ColorId and the shutter base rotation.
        for (offset, rewritten) in [
            (Vec3::new(0.0, 0.0, 0.0), false),
            (Vec3::new(0.093_75, 0.0, 0.0), false),
            (Vec3::new(0.0, 0.093_75, 0.0), false),
            (Vec3::new(0.0, 0.0, 0.093_75), false),
            (Vec3::new(0.125, 0.0, 0.0), true),
            (Vec3::splat(f32::NAN), false),
        ] {
            let mut world = world(vec![candidate(17, WindowAutoClutterKind::WindowShutter {
                side: true,
                native_u32_04: 0x1234_5678,
            })]);
            run(&mut world);
            let entity = entity(&mut world);
            let base = Vec3::new(3.0, 7.0, -2.0);
            let before = *world.get::<Transform>(entity).unwrap();
            let before_shutter_rotation =
                world.get::<WindowShutter>(entity).unwrap().base_rotation;
            let before_color = world.get::<ColorId>(entity).unwrap().0;
            let target = world
                .resource::<AutoClutterState>()
                .spawn_candidates
                .last()
                .expect("drained window candidate")
                .clone();
            assert_eq!(target.position, base);
            let mut moved = target.clone();
            moved.position = base + offset;
            moved.selector_value = 0;
            moved.f5c = (-1.0f32).to_bits();
            world
                .resource_mut::<AutoClutterState>()
                .spawn_candidates
                .push(moved);
            run(&mut world);
            let after = *world.get::<Transform>(entity).unwrap();
            if rewritten {
                assert_eq!(after.translation, base + offset);
                assert_ne!(
                    world.get::<WindowShutter>(entity).unwrap().base_rotation,
                    before_shutter_rotation
                );
                assert_eq!(world.get::<ColorId>(entity).unwrap().0, 0);
            } else {
                assert_eq!(after, before);
                assert_eq!(
                    world.get::<WindowShutter>(entity).unwrap().base_rotation,
                    before_shutter_rotation
                );
                assert_eq!(world.get::<ColorId>(entity).unwrap().0, before_color);
            }
        }
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


use country_core::render::extract::ComponentWrite;
