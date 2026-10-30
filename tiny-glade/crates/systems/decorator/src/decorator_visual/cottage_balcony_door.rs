//! 原B1407f3da0农舍阳台门、1407f5e70自动门饰；只向canonical蓝图发布。
use super::{
    cottage_wall_window::{wall_space, window_builder},
    gothic_balcony_door::check_for_door_stairs,
};
use crate::blueprint::{DecoratorBlueprint, DecoratorBlueprints};
use bevy_ecs::prelude::*;
use country_core::{
    resources::{
        color_id::ColorId,
        session::ActiveSession,
        terrain_height::TerrainHeightsData,
        walls::{
            WallId,
            cached_decorator_transforms::CachedDecoratorTransform,
            decorator::{DecoratorId, DecoratorInfo},
            decorator_storage::{DecoratorStorage, DecoratorStorageAddr},
            decorator_subtype::{DecoratorLibKey, DerivedDecoratorInfoResources},
            public_walls::{PublicWallState, PublicWalls},
            wall_holes::RawWallHole,
        },
        world_raster_defs::WaterRaster,
    },
    systems::collision::{WindowAutoClutterCandidate, WindowAutoClutterKind},
    utils::components::transform::Transform,
};
use glam::{Vec2, Vec3};
use utils::{
    comparative_space::ComparativeSpace,
    curve::{CurveCoordSemantic3D, CurveInternalCoord, curve_slice::CurveSlice},
    wall_space::WallSpaceCoord,
};

pub const COTTAGE_DOOR_SUBTYPE: u8 = 4;
const DOORS: [&str; 3] = [
    "decorators/balcony_door_rank1",
    "decorators/balcony_door_rank2",
    "decorators/balcony_door_rank3",
];
const RAILS: [&str; 3] = [
    "decorators/balcony_door_rank1_rails",
    "decorators/balcony_door_rank2_rails",
    "decorators/balcony_door_rank3_rails",
];

/// 原fastrand混合保留完整128位乘积。随机选择读Info内id，候选键读外层id。
pub fn door_autoclutter_hash(id: u32) -> u64 {
    let a = utils::rng::init_rng(id).wrapping_add(0xa076_1d64_78bd_642f);
    let p = u128::from(a ^ 0xe703_7ed1_a0b4_28db) * u128::from(a);
    (p >> 30) as u64 ^ (p >> 94) as u64
}

/// 原1407f5e70六参：墙键、存储id和主题分别来自真实owner，不从Info猜测。
pub(super) fn add_door_autoclutter(
    info: &DecoratorInfo,
    pose: &Transform,
    blueprint: &mut DecoratorBlueprint,
    id: DecoratorId,
    wall: WallId,
    session: &ActiveSession,
) {
    if info.u45 <= 3 {
        return;
    }
    let (kind, name, offset) = match door_autoclutter_hash(info.decorator_id) & 3 {
        0 => (
            WindowAutoClutterKind::DoorBell,
            "doorbell",
            Vec3::new(-0.25, 0.74, 0.1),
        ),
        1 => (
            if session.is_autumn() {
                WindowAutoClutterKind::DoorKransAutumn
            } else {
                WindowAutoClutterKind::DoorKrans
            },
            "krans",
            Vec3::new(0.0, 0.5, 0.2),
        ),
        _ => return,
    };
    blueprint
        .window_auto_clutter_candidates
        .push(WindowAutoClutterCandidate {
            kind,
            rotation: pose.rotation,
            translation: pose.rotation * offset + pose.translation,
            scale: pose.scale,
            key: utils::calculate_hash(&(id, name)),
            wall,
        });
}

pub(super) fn rank_index(rank: u8) -> usize {
    let index = usize::from(rank.wrapping_sub(1));
    assert!(index < 3, "阳台门rank必须为1至3");
    index
}

/// 原两端采样，不能改读旧缓存；即使墙形状是圆也走同一墙空间投影。
fn door_pose(info: &DecoratorInfo, wall: &PublicWallState, width: f32) -> Transform {
    let coord = info.dst_as_wall().wall_space_coord();
    let space = wall_space(wall);
    let first = WallSpaceCoord::new_unchecked(space.project_clamp(coord.u - width * 0.5), coord.y)
        .to_world_space(wall.simplified_curve_2d());
    let second = WallSpaceCoord::new_unchecked(space.project_clamp(coord.u + width * 0.5), coord.y)
        .to_world_space(wall.simplified_curve_2d());
    cottage_pose_from_samples(first, second, coord.y)
}

/// 原1407f459b..1407f4648；真实墙空间采样后的数值段，复用供原指令差分。
fn cottage_pose_from_samples(first: Vec3, second: Vec3, y: f32) -> Transform {
    let dx = second.x - first.x;
    let dz = second.z - first.z;
    let reciprocal = 1.0 / (dx * dx + dz * dz).sqrt();
    Transform {
        rotation: utils::xz_projective_rotation_bad(Vec3::new(
            -reciprocal * dz,
            0.0,
            reciprocal * dx,
        )),
        translation: Vec3::new(
            second.x * 0.5 + first.x * 0.5,
            y,
            second.z * 0.5 + first.z * 0.5,
        ),
        scale: Vec3::ONE,
    }
}

/// 原门底栏杆读mappedCurve3的XZ弧长；高度是mix，不是差值lerp。
pub(super) fn needs_bottom_rail(
    wall: &PublicWallState,
    info: &DecoratorInfo,
    pose: &Transform,
    terrain: &TerrainHeightsData,
) -> bool {
    let points = wall.geometry.outline.as_slice();
    assert!(points.len() >= 2, "阳台门必须有有效mappedCurve3");
    let u = info.dst_as_wall().wall_space_coord().u;
    let start = CurveInternalCoord { segment: 0, t: 0.0 };
    let mut curve =
        CurveSlice::<[f32; 3], _, CurveCoordSemantic3D, Vec2, _>::from_coord_range_with_remap(
            points,
            start,
            start,
            |_, p| Vec2::new(p[0], p[2]),
        );
    curve.extend_right(if u > 0.0 { u } else { 0.0 });
    let end = curve.end();
    let profile = end.t * points[end.segment + 1][1] + (1.0 - end.t) * points[end.segment][1];
    // 原ucomiss/jbe在无序时跳过，必须为严格有序大于。
    profile > terrain.sample_at_world_xz(pose.translation.x, pose.translation.z) + 0.15
}

pub fn generate_cottage_balcony_doors(
    session: Res<ActiveSession>,
    walls: Res<PublicWalls>,
    storage: Res<DecoratorStorage>,
    derived: DerivedDecoratorInfoResources,
    mut blueprints: ResMut<DecoratorBlueprints>,
    terrain: Res<TerrainHeightsData>,
    water: Res<WaterRaster>,
) {
    let view = derived.as_view();
    let library = view.mesh_library();
    for (wall_id, decorators) in storage.iter_walls() {
        for modified in decorators.iter_modified_ids() {
            let id = DecoratorId(modified);
            let info = decorators.get(id).expect("modified装饰记录缺失");
            if view.subtype(id).0 != COTTAGE_DOOR_SUBTYPE {
                continue;
            }
            // 原entry早于required墙、库查询，不能将失败路径改成事务式预计算。
            let blueprint =
                blueprints.insert_for(modified, DecoratorStorageAddr::Wall(wall_id), info.state());
            let wall = walls.get(wall_id).expect("阳台门所属墙缺失");
            let coord = info.dst_as_wall().wall_space_coord();
            let key = DecoratorLibKey::new(COTTAGE_DOOR_SUBTYPE, info.rank());
            let bounds = library.collision_record(key).bounds;
            let pose = door_pose(info, wall, bounds.footprint.size.x);
            blueprint.set_positioned_colliders(
                library.collision_record(key).mesh.clone(),
                library.interaction_record(key).mesh.clone(),
                &pose.translation,
                &pose.rotation,
            );
            let cached = CachedDecoratorTransform {
                rotation: pose.rotation,
                translation: pose.translation,
                scale: pose.scale,
            };
            blueprint.set_cached_transform(cached);
            let bounds = library.collision_record(key).bounds;
            let rank = rank_index(info.rank());
            blueprint.wall_holes.push(RawWallHole {
                center: [coord.u, coord.y],
                dimensions: [bounds.footprint.size.x, bounds.top - bounds.bottom],
                kind: 0,
            });
            let color = Some(ColorId(info.u28));
            let semi = info.state() != 0;
            let mut door = window_builder(id, pose, DOORS[rank], "nani_balcony_door", color, semi);
            door.balcony_door = true;
            blueprint.visual_entities.push(door);
            add_door_autoclutter(info, &pose, blueprint, id, wall_id, &session);
            if (info.u20 >> 8) as u8 == 1
                && !check_for_door_stairs(&terrain, &water, &cached, &bounds, blueprint)
                && needs_bottom_rail(wall, info, &pose, &terrain)
            {
                blueprint.visual_entities.push(window_builder(
                    id,
                    pose,
                    RAILS[rank],
                    "nani_solid_vertex_color_window",
                    color,
                    semi,
                ));
            }
        }
    }
}

pub fn is_balcony_door(subtype: u8) -> bool {
    subtype == COTTAGE_DOOR_SUBTYPE
}

#[cfg(test)]
pub(super) mod original_balcony_tests {
    use super::*;
    use bevy_ecs::system::RunSystemOnce;
    use country_core::resources::{
        session::{GladeType, SessionId, SessionMeta},
        walls::{
            cached_decorator_transforms::CachedDecoratorTransforms,
            decorator_subtype::DecoratorMeshLibrary,
        },
        world_raster::DirtyTilePublisher,
    };
    use glam::Quat;

    fn session(theme: u32) -> ActiveSession {
        ActiveSession::new(
            SessionId::new(),
            SessionMeta {
                glade: if theme == 3 {
                    GladeType::Autumn
                } else {
                    GladeType::Summer
                },
                ..SessionMeta::default()
            },
        )
    }
    pub(in crate::decorator_visual) fn world(subtype: u8, rank: u8) -> World {
        let mut world =
            super::super::cottage_wall_window::original_window_helper_tests::producer_world(
                subtype, rank,
            );
        // 原系统不依赖CachedDecoratorTransforms，删除以防误用旧生产者合同。
        world.remove_resource::<CachedDecoratorTransforms>();
        world.insert_resource(session(0));
        world.insert_resource(TerrainHeightsData::<
            country_core::resources::terrain_height::TerrainHeightSsbo,
        >::new());
        world.insert_resource(WaterRaster::new(DirtyTilePublisher::unique()));
        world
    }

    #[test]
    fn original_autoclutter_4096_native_cases_full_rng_hash_pose_theme_and_outer_ids() {
        let fixture = include_str!(
            "../../../../../reports/module-recovery-2026-10-23/1407f5e70-autoclutter-native.txt"
        );
        let sessions = [session(0), session(3)];
        for (case, line) in fixture.lines().enumerate() {
            let row: Vec<u32> = line
                .split_ascii_whitespace()
                .map(|word| u32::from_str_radix(word, 16).unwrap())
                .collect();
            let info = DecoratorInfo {
                decorator_id: row[0],
                u45: row[1] as u8,
                ..Default::default()
            };
            let pose = Transform {
                rotation: Quat::from_array(std::array::from_fn(|i| f32::from_bits(row[3 + i]))),
                translation: Vec3::from_array(std::array::from_fn(|i| f32::from_bits(row[7 + i]))),
                scale: Vec3::from_array(std::array::from_fn(|i| f32::from_bits(row[10 + i]))),
            };
            let mut blueprint = DecoratorBlueprint::new();
            add_door_autoclutter(
                &info,
                &pose,
                &mut blueprint,
                DecoratorId(row[0] + 700),
                WallId(0x1_0000_0007),
                &sessions[usize::from(row[2] == 3)],
            );
            assert_eq!(
                blueprint.window_auto_clutter_candidates.len(),
                row[13] as usize,
                "case {case}"
            );
            if row[13] == 0 {
                continue;
            }
            let actual = blueprint.window_auto_clutter_candidates[0];
            let kind = match actual.kind {
                WindowAutoClutterKind::DoorBell => 3,
                WindowAutoClutterKind::DoorKrans => 4,
                WindowAutoClutterKind::DoorKransAutumn => 5,
                _ => unreachable!(),
            };
            let mut words = vec![kind];
            words.extend(actual.rotation.to_array().map(f32::to_bits));
            words.extend(actual.translation.to_array().map(f32::to_bits));
            words.extend(actual.scale.to_array().map(f32::to_bits));
            words.extend([
                actual.key as u32,
                (actual.key >> 32) as u32,
                actual.wall.0 as u32,
                (actual.wall.0 >> 32) as u32,
            ]);
            assert_eq!(words, row[14..], "原完整helper case {case}");
        }
    }

    #[test]
    fn actual_cottage_all_ranks_publish_builders_holes_real_pose_and_two_mesh_owners() {
        use crate::blueprint::BlueprintMeshSlot;
        for rank in 1..=3 {
            let mut world = world(4, rank);
            world
                .run_system_once(generate_cottage_balcony_doors)
                .unwrap();
            let blueprint = world.resource::<DecoratorBlueprints>().get(7).unwrap();
            assert_eq!(
                blueprint.storage_addr,
                Some(DecoratorStorageAddr::Wall(WallId(42)))
            );
            let pose = blueprint.transform.as_ref().unwrap();
            assert_eq!(pose.scale, Vec3::ONE);
            assert_eq!(pose.translation, Vec3::new(-4., 2., 0.));
            assert_eq!(blueprint.visual_entities.len(), 1);
            let door = &blueprint.visual_entities[0];
            assert_eq!(door.mesh_name, DOORS[rank as usize - 1]);
            assert_eq!(door.nani_name, "nani_balcony_door");
            assert!(door.balcony_door && door.semi_transparent);
            assert_eq!(door.color_id, Some(ColorId(0x55667788)));
            assert_eq!(blueprint.wall_holes.len(), 1);
            assert_eq!(blueprint.wall_holes[0].center, [4., 2.]);
            assert_eq!(blueprint.wall_holes[0].dimensions, [4., 2.]);
            let BlueprintMeshSlot::Mesh(collision) = &blueprint.collision_mesh else {
                panic!("无碰撞")
            };
            let BlueprintMeshSlot::Mesh(interaction) = &blueprint.interaction_mesh else {
                panic!("无交互")
            };
            assert_eq!(collision.position, pose.translation);
            assert_eq!(interaction.position, pose.translation);
            let library = world.resource::<DecoratorMeshLibrary>();
            let key = DecoratorLibKey::new(4, rank);
            assert!(std::sync::Arc::ptr_eq(
                &collision.mesh,
                &library.collision_record(key).mesh
            ));
            assert!(std::sync::Arc::ptr_eq(
                &interaction.mesh,
                &library.interaction_record(key).mesh
            ));
        }
    }

    #[test]
    fn required_wall_or_interaction_failure_retains_original_partial_entry() {
        for missing_wall in [false, true] {
            let mut world = world(4, 1);
            if missing_wall {
                world.insert_resource(PublicWalls::default());
            } else {
                world
                    .resource_mut::<DecoratorMeshLibrary>()
                    .interaction
                    .clear();
            }
            assert!(
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| world
                    .run_system_once(generate_cottage_balcony_doors)
                    .unwrap()))
                .is_err()
            );
            let entry = world.resource::<DecoratorBlueprints>().get(7).unwrap();
            assert_eq!(
                entry.storage_addr,
                Some(DecoratorStorageAddr::Wall(WallId(42)))
            );
            assert!(entry.transform.is_none() && entry.visual_entities.is_empty());
        }
    }
    #[test]
    fn actual_doors_fullvisual_multiframe_edit_recollect_and_unload() {
        use crate::{
            blueprint::{PrevDecoratorBlueprints, instantiate_blueprint_visual_state},
            decorator_visual::deco_differential_state::{
                ChimneyAssemblyParams, DoorStairsAssemblyParams, WallAttachedDecoTracker,
            },
            decorator_visual_state::DecoratorVisualState,
        };
        use country_core::{
            asset_libraries::{
                mesh_library::AssetMeshLibrary,
                shader_library::{AssetShaderLibrary, SharedAssetShaderLibrary},
            },
            render::nani::systems::NaniInstanceDesc,
            resources::walls::{
                decorator_collision::DecoratorPhysicsColliders, public_walls::OnWallDeleted,
                wall_holes::WallHoles,
            },
            systems::collision::{WindowAutoClutterCandidates, world::RaycastWorld},
        };
        use std::sync::Arc;
        for (subtype, state) in [4, 9].into_iter().flat_map(|subtype| [0_u8, 1].map(move |state| (subtype, state))) {
            let mut world = world(subtype, 1);
            world.insert_resource(PrevDecoratorBlueprints::default());
            world.insert_resource(DecoratorVisualState::default());
            world.insert_resource(WallHoles::default());
            world.insert_resource(RaycastWorld::new());
            world.insert_resource(DecoratorPhysicsColliders::default());
            world.insert_resource(WindowAutoClutterCandidates::default());
            world.insert_resource(WallAttachedDecoTracker::<ChimneyAssemblyParams>::default());
            world.insert_resource(WallAttachedDecoTracker::<DoorStairsAssemblyParams>::default());
            world.insert_resource(AssetMeshLibrary::new());
            world.insert_resource(SharedAssetShaderLibrary(
                Arc::new(AssetShaderLibrary::new()),
            ));
            world.insert_resource(bevy_ecs::event::Events::<OnWallDeleted>::default());
            // 选择原Bell分支，使真实候选重收集路径非空；Info.id和map id仍不同。
            let seed = (0..512)
                .find(|id| door_autoclutter_hash(*id) & 3 == 0)
                .unwrap();
            {
                let mut storage = world.resource_mut::<DecoratorStorage>();
                let info = storage
                    .per_wall
                    .get_mut(&WallId(42))
                    .unwrap()
                    .get_mut(DecoratorId(7))
                    .unwrap();
                info.decorator_id = seed;
                info.u45 = 4;
                info.state = state;
            }
            let generate = |world: &mut World| {
                if subtype == 4 {
                    world
                        .run_system_once(generate_cottage_balcony_doors)
                        .unwrap();
                } else {
                    world
                        .run_system_once(
                            super::super::gothic_balcony_door::generate_gothic_balcony_doors,
                        )
                        .unwrap();
                }
            };
            generate(&mut world);
            world
                .run_system_once(instantiate_blueprint_visual_state)
                .unwrap();
            let old = world
                .resource::<DecoratorVisualState>()
                .persistent_entities_of(7)
                .unwrap()
                .to_vec();
            assert_eq!(old.len(), if subtype == 4 { 1 } else { 2 });
            for entity in &old {
                assert!(world.get::<NaniInstanceDesc>(*entity).is_some());
            }
            let snapshot = |world: &mut World| {
                let mut params = bevy_ecs::system::SystemState::<(
                    Res<DecoratorBlueprints>,
                    ResMut<PrevDecoratorBlueprints>,
                )>::new(world);
                let (current, mut previous) = params.get_mut(world);
                current.copy_blueprints_to_prev(&mut previous);
            };
            snapshot(&mut world);
            *world.resource_mut::<WallHoles>() = WallHoles::default();
            world
                .resource_mut::<WindowAutoClutterCandidates>()
                .0
                .clear();
            generate(&mut world);
            world
                .run_system_once(instantiate_blueprint_visual_state)
                .unwrap();
            assert_eq!(
                world
                    .resource::<DecoratorVisualState>()
                    .persistent_entities_of(7)
                    .unwrap(),
                old
            );
            assert_eq!(world.resource::<WindowAutoClutterCandidates>().0.len(), 1);
            assert_eq!(
                world.resource::<WallHoles>().storage[&WallId(42)]
                    .raw_holes()
                    .len(),
                if subtype == 4 { 1 } else { 3 }
            );
            // 改真实墙空间目标，不改builder或缓存；新生产者应替换实际ECS实体和碰撞。
            {
                let mut storage = world.resource_mut::<DecoratorStorage>();
                let info = storage
                    .per_wall
                    .get_mut(&WallId(42))
                    .unwrap()
                    .get_mut(DecoratorId(7))
                    .unwrap();
                let coord = info.wall_space_coord();
                let mut dst = info.destination().as_wall().clone();
                dst.coord = WallSpaceCoord::new_unchecked(coord.u, coord.y + 0.5);
                info.set_destination(
                    country_core::resources::walls::decorator::DecoratorDst::Wall(dst),
                );
            }
            *world.resource_mut::<WallHoles>() = WallHoles::default();
            world
                .resource_mut::<WindowAutoClutterCandidates>()
                .0
                .clear();
            generate(&mut world);
            world
                .run_system_once(instantiate_blueprint_visual_state)
                .unwrap();
            let edited = world
                .resource::<DecoratorVisualState>()
                .persistent_entities_of(7)
                .unwrap()
                .to_vec();
            for entity in &edited {
                assert_eq!(world.get::<Transform>(*entity).unwrap().translation.y, 2.5);
            }
            let physics = world.resource::<DecoratorPhysicsColliders>();
            if state == 0 {
                let handle = physics.first[&DecoratorId(7)].handle();
                assert_eq!(world.resource::<RaycastWorld>().get_collider(handle).position.y, 2.5);
                assert!(physics.second.contains_key(&DecoratorId(7)));
            } else {
                assert!(physics.first.is_empty() && physics.second.is_empty(), "原preview状态不注册正式碰撞");
            }
            assert_ne!(old, edited);
            for entity in old {
                assert!(!world.entities().contains(entity));
            }
            snapshot(&mut world);
            world.resource_mut::<DecoratorBlueprints>().map.remove(&7);
            *world.resource_mut::<WallHoles>() = WallHoles::default();
            world
                .resource_mut::<WindowAutoClutterCandidates>()
                .0
                .clear();
            world
                .run_system_once(instantiate_blueprint_visual_state)
                .unwrap();
            for entity in edited {
                assert!(!world.entities().contains(entity));
            }
            assert!(
                world
                    .resource::<DecoratorVisualState>()
                    .persistent_entities_of(7)
                    .is_none()
            );
            let physics = world.resource::<DecoratorPhysicsColliders>();
            assert!(physics.first.is_empty() && physics.second.is_empty());
            assert!(world.resource::<WindowAutoClutterCandidates>().0.is_empty());
        }
    }

    #[test]
    fn original_door_pose_2080_native_cases_include_actual_projective_rotation() {
        for (case, line) in include_str!(
            "../../../../../reports/module-recovery-2026-10-23/1407f0900-1407f3da0-pose-native.txt"
        )
        .lines()
        .enumerate()
        {
            let row: Vec<u32> = line
                .split_ascii_whitespace()
                .map(|v| u32::from_str_radix(v, 16).unwrap())
                .collect();
            let first = Vec3::new(
                f32::from_bits(row[1]),
                f32::from_bits(row[2]),
                f32::from_bits(row[3]),
            );
            let second = Vec3::new(
                f32::from_bits(row[4]),
                f32::from_bits(row[5]),
                f32::from_bits(row[6]),
            );
            let actual = if row[0] == 0 {
                cottage_pose_from_samples(first, second, first.y)
            } else {
                super::super::gothic_balcony_door::gothic_pose_from_samples(first, second)
            };
            let mut words = actual.translation.to_array().map(f32::to_bits).to_vec();
            words.extend(actual.rotation.to_array().map(f32::to_bits));
            assert_eq!(words, row[7..], "原数值段case {case}");
        }
    }

    #[test]
    fn actual_named_water_controls_low_sill_stairs_and_path_cannot_substitute() {
        use country_core::resources::{
            walls::decorator::DecoratorDst, world_raster_defs::PathRaster,
        };
        for subtype in [4, 9] {
            for covered in [false, true] {
                let mut world = world(subtype, 1);
                let mut path = PathRaster::new(DirtyTilePublisher::unique());
                path.composite_mut().fill(255);
                world.insert_resource(path);
                if covered {
                    world
                        .resource_mut::<WaterRaster>()
                        .composite_mut()
                        .fill(255);
                }
                {
                    let mut terrain = world.resource_mut::<TerrainHeightsData>();
                    terrain.set_height_range(0.0, 2.0);
                    for row in terrain.heights_mut().chunks_exact_mut(512) {
                        row[239..242].fill(u16::MAX);
                    }
                }
                {
                    let mut storage = world.resource_mut::<DecoratorStorage>();
                    let info = storage
                        .per_wall
                        .get_mut(&WallId(42))
                        .unwrap()
                        .get_mut(DecoratorId(7))
                        .unwrap();
                    let mut dst = info.destination().as_wall().clone();
                    dst.coord = WallSpaceCoord::new_unchecked(4.0, 0.5);
                    dst.bottom_as_door = true;
                    info.set_destination(DecoratorDst::Wall(dst));
                }
                if subtype == 4 {
                    world
                        .run_system_once(generate_cottage_balcony_doors)
                        .unwrap();
                } else {
                    world
                        .run_system_once(
                            super::super::gothic_balcony_door::generate_gothic_balcony_doors,
                        )
                        .unwrap();
                }
                let bp = world.resource::<DecoratorBlueprints>().get(7).unwrap();
                assert_eq!(
                    bp.door_stairs_assembly.is_some(),
                    covered,
                    "subtype {subtype}"
                );
                if let Some(stairs) = &bp.door_stairs_assembly {
                    let values = stairs.words.map(f32::from_bits);
                    assert_eq!(
                        [
                            values[0], values[1], values[4], values[5], values[6], values[7]
                        ],
                        [-4., 0., 2., -0.8, 0., 4.]
                    );
                    assert!((values[2] + 1.0).abs() < f32::EPSILON);
                    assert!(values[3].abs() < f32::EPSILON);
                }
            }
        }
    }

    #[test]
    fn original_gothic_placeholder_color_only_and_strict_missing_modified_record() {
        for subtype in [4, 9] {
            let mut world = world(subtype, 1);
            world
                .resource_mut::<DecoratorStorage>()
                .per_wall
                .get_mut(&WallId(42))
                .unwrap()
                .get_mut(DecoratorId(7))
                .unwrap()
                .u28 = 999;
            if subtype == 4 {
                world
                    .run_system_once(generate_cottage_balcony_doors)
                    .unwrap();
            } else {
                world
                    .run_system_once(
                        super::super::gothic_balcony_door::generate_gothic_balcony_doors,
                    )
                    .unwrap();
            }
            let bp = world.resource::<DecoratorBlueprints>().get(7).unwrap();
            for visual in &bp.visual_entities {
                assert_eq!(
                    visual.color_id,
                    Some(ColorId(if subtype == 9 { 0 } else { 999 }))
                );
            }
            world.resource_mut::<DecoratorBlueprints>().map.clear();
            world
                .resource_mut::<DecoratorStorage>()
                .per_wall
                .get_mut(&WallId(42))
                .unwrap()
                .remove(DecoratorId(7));
            assert!(
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    if subtype == 4 {
                        world
                            .run_system_once(generate_cottage_balcony_doors)
                            .unwrap();
                    } else {
                        world
                            .run_system_once(
                                super::super::gothic_balcony_door::generate_gothic_balcony_doors,
                            )
                            .unwrap();
                    }
                }))
                .is_err()
            );
            assert!(world.resource::<DecoratorBlueprints>().map.is_empty());
        }
    }
}
