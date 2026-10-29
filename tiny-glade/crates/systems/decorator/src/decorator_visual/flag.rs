//! 原B `decorator_visual/flag.rs` 1407ff6c0，五类原子类型全部发布完整蓝图。
//! 原文件SHA 09e353aaade5769b9fa68f8e952ba9720a35420b5950f2cabe3dae1af50f5b33。
//! 墙旗保留邻装饰遮挡、墙顶与向下真实射线裁切；不把旗帜算法降为四角布片。
use super::cottage_wall_window::window_builder;
use crate::{
    blueprint::DecoratorBlueprints, decorator_visual_state::DecoratorTransientVisualState,
};
use bevy_ecs::prelude::*;
use country_core::resources::walls::{
    cached_decorator_transforms::{CachedDecoratorTransform, CachedDecoratorTransforms},
    decorator::{DecoratorId, DecoratorInfo},
    decorator_storage::{DecoratorStorage, DecoratorStorageAddr, Decorators},
    decorator_subtype::{DecoratorLibKey, DerivedDecoratorInfoResources, DerivedDecoratorInfoView},
    public_walls::{PublicWallState, PublicWalls},
};
use country_core::systems::collision::world::{RaycastMeshData, RaycastWorld};
use country_core::utils::components::transform::Transform;
use glam::{Quat, Vec2, Vec3};
use std::sync::Arc;
use utils::comparative_space::ComparativeSpace;

pub const FLAG_WALL_SUBTYPE: u8 = 13;
pub const FLAG_ROOF_SUBTYPE: u8 = 14;
pub const FLAG_CONNECTIONS_SUBTYPE: u8 = 15;
pub const FLAG_CLOTH_QUAT_XYZW: [f32; 4] = [-0.0, -0.707_106_77, -0.0, 0.707_106_77];
pub fn flag_cloth_quat() -> Quat {
    Quat::from_array(FLAG_CLOTH_QUAT_XYZW)
}

fn as_transform(pose: CachedDecoratorTransform) -> Transform {
    Transform {
        rotation: pose.rotation,
        translation: pose.translation,
        scale: pose.scale,
    }
}
fn cached(pose: Transform) -> CachedDecoratorTransform {
    CachedDecoratorTransform {
        rotation: pose.rotation,
        translation: pose.translation,
        scale: pose.scale,
    }
}
fn cloth_payload(seed: u32, offset: f32, height: f32, corner: bool) -> (u64, u64, u32) {
    (
        u64::from(seed),
        u64::from(offset.to_bits()) | (u64::from(height.to_bits()) << 32),
        u32::from(corner),
    )
}

/// 原0x1408000f4..02dd：墙挂和拐角挂的共享位姿链。
fn wall_flag_poses(original: Transform, corner: bool) -> (Transform, Transform) {
    let mut outline = original;
    outline.translation += outline.rotation * Vec3::new(0.0, 0.0, f32::from_bits(0x3ef5_c290));
    let mut pose = outline;
    if corner {
        pose.rotation *= Quat::from_xyzw(
            0.0,
            f32::from_bits(0x3f35_04f3),
            0.0,
            f32::from_bits(0x3f35_04f3),
        );
        pose.translation += pose.rotation * Vec3::new(-0.7, -0.0, -0.0);
    }
    (outline, pose)
}
fn terrain_flag_cloth_pose(original: Transform) -> (Transform, f32, f32) {
    let rotation = original.rotation * flag_cloth_quat();
    let translation =
        original.translation + Vec3::new(0.0, 3.8, 0.0) + rotation * Vec3::new(0.9, 0.0, 0.0);
    let height = translation.y.max(0.0).min(2.8);
    (
        Transform {
            rotation,
            translation,
            ..original
        },
        height,
        height + -1.2,
    )
}
fn cap_cloth_height(height: f32, world_y: f32) -> f32 {
    // 原MAXSS/MINSS的第二操作数选择规则包括NaN和相等，不是有序if的省略。
    let upper = if world_y > 0.5 { world_y } else { 0.5 };
    let height = if height < upper { height } else { upper };
    let height = if height > 1.5 { height } else { 1.5 };
    height + 0.01
}

/// 原1408000514..080f9：邻装饰物按存储顺序逐个裁切，之后查询其它墙的真实碰撞。
fn wall_cloth_height(
    id: u32,
    info: &DecoratorInfo,
    corner: bool,
    pose: Transform,
    wall_id: country_core::resources::walls::WallId,
    wall: &PublicWallState,
    bucket: &Decorators,
    view: DerivedDecoratorInfoView<'_>,
    transforms: &CachedDecoratorTransforms,
    raycast: &RaycastWorld,
) -> f32 {
    let source = info.dst_as_wall().wall_space_coord();
    let path = wall
        .prepared_path
        .as_ref()
        .expect("wall path has not been prepared");
    let coord = utils::curve::approx_curve_coord(
        &path.positions,
        Vec2::new(pose.translation.x, pose.translation.z),
        |p| Vec2::new(p[0], p[2]),
    );
    let segment = coord.segment;
    let heights = wall
        .arch_segment_heights()
        .expect("wall heights have not been calculated yet");
    let a = heights[segment];
    let b = heights[segment + 1];
    let path_a = path.positions[segment][1];
    let path_b = path.positions[segment + 1][1];
    let wall_top = ((path_b - path_a) * coord.t + path_a) + ((b - a) * coord.t + a) + 0.5;
    let minimum = source.y + -4.0;
    // 原maxNum：第二数为NaN时取第一数，有限相等保留第一数。
    let mut lower = if minimum.is_nan() {
        wall_top
    } else if wall_top <= minimum {
        minimum
    } else {
        wall_top
    };
    let forward = pose.rotation * Vec3::Z;
    let space = super::cottage_wall_window::wall_space(wall);
    for (&other_id, other) in bucket.decorators() {
        if other_id == id {
            continue;
        }
        if wall.shape().tag() >= 2 {
            let other_forward = transforms.transform(other_id).rotation * Vec3::Z;
            if other_forward.z * forward.z
                + other_forward.y * forward.y
                + other_forward.x * forward.x
                < 0.0
            {
                continue;
            }
        }
        match other.dst_tag() {
            2 | 4 => continue,
            3 => panic!("internal error: entered unreachable code"),
            _ => {}
        }
        let other_coord = other.wall_space_coord();
        let key = DecoratorLibKey::new(view.subtype(DecoratorId(other_id)).0, other.rank());
        let bounds = view.mesh_library().collision_record(key).bounds;
        let half_height = (bounds.top - bounds.bottom) * 0.5;
        let floor = lower + -0.5;
        let middle = (source.y + floor) * 0.5;
        if (source.y - floor) * 0.5 + half_height <= (other_coord.y - middle).abs() {
            continue;
        }
        // 原调用前的空函数不改变坐标；必须保留环形WallSpace距离。
        if bounds.footprint.size.x * 0.5 + if corner { 0.05 } else { 0.25 }
            <= space.distance(source.u, other_coord.u)
        {
            continue;
        }
        lower = other_coord.y + half_height + 0.5;
    }
    let mut height = source.y - lower;
    if height > 1.5 {
        let hit = raycast.legacy_raycast_w_params_excluding_wall_id(
            pose.translation,
            Vec3::new(0.0, -1.0, -0.0),
            0,
            wall_id,
        );
        if hit.is_hit() && (hit.handle[0] as u8 & 0xfd) == 0 {
            let hit_floor = hit.position.y + 0.5;
            let clipped = if lower.is_nan() {
                hit_floor
            } else if hit_floor <= lower {
                lower
            } else {
                hit_floor
            };
            height = source.y - clipped;
        }
    }
    cap_cloth_height(height, pose.translation.y)
}

/// 原6组参数。所有地址桶都参与原iter；类型门保持先于缓存和subtype查表。
pub fn generate_flags(
    storage: Res<DecoratorStorage>,
    walls: Res<PublicWalls>,
    derived: DerivedDecoratorInfoResources,
    transforms: Res<CachedDecoratorTransforms>,
    mut blueprints: ResMut<DecoratorBlueprints>,
    raycast: Res<RaycastWorld>,
) {
    let view = derived.as_view();
    let buckets = storage
        .iter_walls()
        .map(|(wall, bucket)| (DecoratorStorageAddr::Wall(wall), bucket))
        .chain(
            storage
                .iter_terrain_decorators()
                .map(|bucket| (DecoratorStorageAddr::Terrain, bucket)),
        )
        .chain(
            storage
                .iter_stair_decorators()
                .map(|bucket| (DecoratorStorageAddr::Stair, bucket)),
        );
    for (address, bucket) in buckets {
        for id in bucket.iter_modified_ids() {
            let info = bucket
                .get(DecoratorId(id))
                .expect("modified decorator is missing from its storage");
            if info.decorator_type().0 != 0x3d {
                continue;
            }
            let original = as_transform(transforms.transform(id));
            let subtype = view.subtype(DecoratorId(id)).0;
            let key = DecoratorLibKey::new(subtype, info.rank());
            let semi = info.state() != 0;
            let seed = id.wrapping_add(info.decorator_id);
            if matches!(subtype, 11 | 12) {
                let wall_id = match address {
                    DecoratorStorageAddr::Wall(wall) => wall,
                    _ => panic!("expected a wall decorator storage address"),
                };
                let (outline, pose) = wall_flag_poses(original, subtype == 12);
                let wall = walls
                    .get(wall_id)
                    .expect("called `Option::unwrap()` on a `None` value");
                let height = wall_cloth_height(
                    id,
                    info,
                    subtype == 12,
                    pose,
                    wall_id,
                    wall,
                    bucket,
                    view,
                    &transforms,
                    &raycast,
                );
                if height <= 1.5 {
                    continue;
                }
                let offset = height + -1.2;
                let custom = Arc::new(RaycastMeshData::new_cuboid_at(
                    Vec3::new(1.0, offset, 0.6),
                    Vec3::new(offset * -0.0 * 0.5, offset * -0.5, offset * -0.0 * 0.5),
                ));
                let blueprint = blueprints.insert_for(id, address, info.state());
                blueprint.set_cached_transform(cached(outline));
                blueprint.set_positioned_colliders(
                    Arc::clone(&view.mesh_library().collision_record(key).mesh),
                    custom,
                    &pose.translation,
                    &pose.rotation,
                );
                let connected = info.extra().as_flag().0 != 0;
                let beam = match (subtype, connected) {
                    (11, false) => "decorators/flag_wall_beam_no_connection",
                    (11, true) => "decorators/flag_wall_beam",
                    (12, false) => "decorators/flag_corner_beam_no_connection",
                    (12, true) => "decorators/flag_corner_beam",
                    _ => unreachable!(),
                };
                blueprint.visual_entities.push(window_builder(
                    DecoratorId(id),
                    outline,
                    beam,
                    "nani_solid_vertex_color",
                    None,
                    semi,
                ));
                let mut cloth = window_builder(
                    DecoratorId(id),
                    pose,
                    "decorators/flag_wall",
                    "flag_wall_cloth",
                    Some(country_core::resources::color_id::ColorId(info.u28)),
                    semi,
                );
                cloth.wall_flag_cloth = Some(cloth_payload(seed, offset, height, subtype == 12));
                blueprint.visual_entities.push(cloth);
            } else if matches!(subtype, 13 | 14) {
                let pose = Transform {
                    rotation: flag_cloth_quat(),
                    ..original
                };
                let blueprint = blueprints.insert_for(id, address, info.state());
                blueprint.set_cached_transform(cached(pose));
                blueprint.set_positioned_colliders(
                    Arc::clone(&view.mesh_library().collision_record(key).mesh),
                    Arc::clone(&view.mesh_library().interaction_record(key).mesh),
                    &pose.translation,
                    &pose.rotation,
                );
                blueprint.visual_entities.push(window_builder(
                    DecoratorId(id),
                    pose,
                    "decorators/flag_roof_beam",
                    "nani_solid_vertex_color",
                    None,
                    semi,
                ));
                let cloth_pose = Transform {
                    translation: pose.translation + Vec3::new(0.0, 1.6, 0.0),
                    ..pose
                };
                let mut cloth = window_builder(
                    DecoratorId(id),
                    cloth_pose,
                    "decorators/flag_roof_cloth",
                    "flag_roof_cloth",
                    Some(country_core::resources::color_id::ColorId(info.u28)),
                    semi,
                );
                cloth.roof_flag_cloth = Some(seed);
                blueprint.visual_entities.push(cloth);
            } else if subtype == 15 {
                let blueprint = blueprints.insert_for(id, address, info.state());
                blueprint.set_cached_transform(cached(original));
                blueprint.set_positioned_colliders(
                    Arc::clone(&view.mesh_library().collision_record(key).mesh),
                    Arc::clone(&view.mesh_library().interaction_record(key).mesh),
                    &original.translation,
                    &original.rotation,
                );
                let mut support = window_builder(
                    DecoratorId(id),
                    original,
                    "decorators/flag_terrain_support",
                    "nani_uber",
                    None,
                    semi,
                );
                support.albedo_map = Some(0xe14f_8a4b_e230_cb70);
                support.sittable_for_birds = Some((u64::from(0x4079_999au32) << 32, 0x3ecc_cccd));
                blueprint.visual_entities.push(support);
                let (pose, height, offset) = terrain_flag_cloth_pose(original);
                let mut cloth = window_builder(
                    DecoratorId(id),
                    pose,
                    "decorators/flag_wall",
                    "flag_wall_cloth",
                    Some(country_core::resources::color_id::ColorId(info.u28)),
                    semi,
                );
                cloth.wall_flag_cloth = Some(cloth_payload(seed, offset, height, false));
                blueprint.visual_entities.push(cloth);
            } else {
                panic!("generate_flags: unexpected decorator subtype {subtype}");
            }
        }
    }
}

#[cfg(test)]
pub(crate) mod canonical_mixed_input_gate_tests {
    use super::*;
    use crate::blueprint::{BlueprintMeshSlot, DecoratorBlueprints};
    use crate::decorator_visual::{
        arrow_slit::generate_arrow_slits, chimney_roof::generate_roof_chimneys,
    };
    use bevy_ecs::system::RunSystemOnce;
    use country_core::resources::walls::{
        WallId,
        cached_decorator_transforms::{CachedDecoratorTransform, CachedDecoratorTransforms},
        decorator::{
            ChimneyExtra, DecoratorExtraInfo, DecoratorId, DecoratorInfo, FlagExtra, WindowExtra,
        },
        decorator_storage::{DecoratorStorage, DecoratorStorageAddr, Decorators},
        decorator_subtype::{
            DecoratorCollision, DecoratorCollisionOutline, DecoratorLibKey, DecoratorMeshLibrary,
            DecoratorOutline, DecoratorSubtype, DerivedDecoratorSubtypes,
        },
        public_walls::PublicWalls,
    };
    use country_core::systems::collision::world::RaycastMeshData;
    use country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::box_trigger_volume;
    use country_core::utils::components::transform::Transform;
    use std::{
        any::Any,
        panic::{AssertUnwindSafe, catch_unwind},
        sync::Arc,
    };

    pub(crate) fn fixture() -> World {
        let mut world = World::new();
        let mut bucket = Decorators::default();
        for (id, ty, rank) in [(41, 0x3d, 2), (42, 0x3b, 1), (43, 0x38, 2), (44, 0x3a, 1)] {
            let mut info = DecoratorInfo {
                decorator_id: id,
                ty,
                rank,
                state: 1,
                ..Default::default()
            };
            match id {
                41 => info.set_extra(DecoratorExtraInfo::Flag(FlagExtra(0))),
                42 => {
                    info.u20 = 2 << 16;
                    info.set_extra(DecoratorExtraInfo::Chimney(ChimneyExtra {
                        f: 1.0,
                        smoke: 3,
                    }));
                }
                43 | 44 => info.set_extra(DecoratorExtraInfo::Window(WindowExtra(0))),
                _ => unreachable!(),
            }
            bucket.insert(DecoratorId(id), info);
        }
        let mut storage = DecoratorStorage::default();
        storage.per_wall.insert(WallId(7), bucket);
        world.insert_resource(storage);
        let mut derived = DerivedDecoratorSubtypes::default();
        for (id, subtype) in [(41, 13), (42, 21), (43, 4), (44, 23)] {
            derived.insert(id, DecoratorSubtype(subtype));
        }
        world.insert_resource(derived);
        let mut cached = CachedDecoratorTransforms::default();
        for id in [41, 42] {
            cached.insert(
                id,
                CachedDecoratorTransform {
                    rotation: Quat::from_rotation_y(0.5),
                    translation: Vec3::new(id as f32, 2.0, -3.0),
                    scale: Vec3::ONE,
                },
            );
        }
        // 普通门与23/rank1没有cache；真前门必须先拒绝，不凭默认位姿继续。
        world.insert_resource(cached);
        let collision = DecoratorCollision {
            triangles: box_trigger_volume(1.0, 1.0, 1.0, &Transform::identity()),
            mesh: Arc::new(RaycastMeshData::new_cuboid_at(Vec3::ONE, Vec3::ZERO)),
            bounds: utils::geometry::Rectangle3d::new(Vec3::ZERO, Vec3::ONE, glam::Vec2::X),
        };
        let mut library = DecoratorMeshLibrary::default();
        library.insert_collision_and_interaction_as_same(
            DecoratorLibKey::new(13, 2),
            collision,
            DecoratorOutline::None,
            DecoratorCollisionOutline::None,
        );
        world.insert_resource(library);
        world.insert_resource(PublicWalls::default());
        world.init_resource::<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>();
        world.insert_resource(RaycastWorld::new());
        world.insert_resource(DecoratorBlueprints::default());
        world.insert_resource(DecoratorTransientVisualState::default());
        world
    }

    pub(crate) fn published_test_wall() -> PublicWalls {
        use country_core::{
            resources::roofs::geometry::surface::Curve2,
            resources::walls::publication_source::TerrainHeightGrid512,
            resources::walls::{
                inner_wall_state::WallState, inner_walls::InnerWalls,
                publication_fields::WallShapeSnapshot,
            },
        };
        let points = vec![[0., 0.], [8., 0.], [8., 8.], [0., 8.], [0., 0.]];
        let mut snapshot = WallState::new(Curve2::try_new(points, true).unwrap(), 3., 0, 0, false)
            .snapshot()
            .clone();
        let mut bytes = [0u8; 28];
        bytes[..4].copy_from_slice(&1u32.to_le_bytes());
        for (i, value) in [1f32, 0., 0., 0., 8., 8.].into_iter().enumerate() {
            bytes[4 + i * 4..8 + i * 4].copy_from_slice(&value.to_le_bytes());
        }
        snapshot.source.shape = WallShapeSnapshot::from_observed(bytes);
        snapshot.source.curve_kind_tag = 1;
        let mut inner = InnerWalls::new();
        inner.new_wall_with_id(WallId(7), WallState::from_snapshot(snapshot, 0));
        let mut walls = PublicWalls::default();
        walls.consume_with_paths(&mut inner,&TerrainHeightGrid512 {encoded:vec![0;512*512].into(),minimum_y:0.,maximum_y:0.,sampling_fix:false},Some(&country_core::systems::wall::wall_state::path_preparation::PathRasters512::from_encoded(
                std::array::from_fn(|_| vec![0u8;512*512].into()),false).unwrap()),
            &country_core::resources::walls::public_walls::WallPublicationDirty::default(),
            &country_core::resources::walls::public_walls::WallSpatialHash::default());
        walls
    }

    fn panic_text(payload: Box<dyn Any + Send>) -> String {
        if let Some(text) = payload.downcast_ref::<String>() {
            return text.clone();
        }
        if let Some(text) = payload.downcast_ref::<&str>() {
            return (*text).to_owned();
        }
        panic!("预期原required失败文字");
    }

    #[test]
    fn three_actual_generators_share_a_mixed_modified_world_without_consuming_unrelated_inputs() {
        let mut world = fixture();
        world.insert_resource(published_test_wall());
        world.run_system_once(generate_flags).unwrap();
        world.run_system_once(generate_roof_chimneys).unwrap();
        world.run_system_once(generate_arrow_slits).unwrap();
        let blueprints = world.resource::<DecoratorBlueprints>();
        assert_eq!(blueprints.map.len(), 2);
        for id in [41, 42] {
            let blueprint = blueprints.get(id).unwrap();
            assert_eq!(
                blueprint.storage_addr,
                Some(DecoratorStorageAddr::Wall(WallId(7)))
            );
            assert_eq!(blueprint.interaction_source_flags, 0);
            assert_eq!(blueprint.u138, 1);
            assert!(matches!(
                blueprint.collision_mesh,
                BlueprintMeshSlot::Mesh(_)
            ));
            assert!(matches!(
                blueprint.interaction_mesh,
                BlueprintMeshSlot::Mesh(_)
            ));
        }
        assert!(blueprints.get(43).is_none());
        assert!(blueprints.get(44).is_none());
    }

    #[test]
    fn flags_skip_non_flag_types_before_id_cache_and_keep_their_own_required_order() {
        let mut world = fixture();
        let mut only_flag = DerivedDecoratorSubtypes::default();
        only_flag.insert(41, DecoratorSubtype(13));
        world.insert_resource(only_flag);
        world.run_system_once(generate_flags).unwrap();
        assert_eq!(world.resource::<DecoratorBlueprints>().map.len(), 1);
        let mut missing_both = fixture();
        missing_both.insert_resource(CachedDecoratorTransforms::default());
        missing_both.insert_resource(DerivedDecoratorSubtypes::default());
        let failure = catch_unwind(AssertUnwindSafe(|| {
            missing_both.run_system_once(generate_flags).unwrap()
        }))
        .unwrap_err();
        assert!(panic_text(failure).contains("CachedDecoratorTransforms::transform"));
        assert!(
            missing_both
                .resource::<DecoratorBlueprints>()
                .map
                .is_empty()
        );
        let mut missing_id = fixture();
        missing_id.insert_resource(DerivedDecoratorSubtypes::default());
        let failure = catch_unwind(AssertUnwindSafe(|| {
            missing_id.run_system_once(generate_flags).unwrap()
        }))
        .unwrap_err();
        assert!(panic_text(failure).contains("subtype library doesn't contain subtype 41"));
    }

    #[test]
    fn arrow_id_and_real_wall_are_required_after_type_and_excluded_rank_guard() {
        let mut excluded = fixture();
        excluded.run_system_once(generate_arrow_slits).unwrap();
        assert!(excluded.resource::<DecoratorBlueprints>().map.is_empty());
        let mut missing_id = fixture();
        missing_id.insert_resource(DerivedDecoratorSubtypes::default());
        let failure = catch_unwind(AssertUnwindSafe(|| {
            missing_id.run_system_once(generate_arrow_slits).unwrap()
        }))
        .unwrap_err();
        assert!(panic_text(failure).contains("subtype library doesn't contain subtype 44"));
        let mut missing_wall = fixture();
        missing_wall
            .resource_mut::<DerivedDecoratorSubtypes>()
            .insert(44, DecoratorSubtype(22));
        missing_wall
            .resource_mut::<DecoratorStorage>()
            .per_wall
            .get_mut(&WallId(7))
            .unwrap()
            .get_mut(DecoratorId(44))
            .unwrap()
            .rank = 2;
        let failure = catch_unwind(AssertUnwindSafe(|| {
            missing_wall.run_system_once(generate_arrow_slits).unwrap()
        }))
        .unwrap_err();
        assert!(panic_text(failure).contains("called `Option::unwrap()`"));
        assert!(
            missing_wall
                .resource::<DecoratorBlueprints>()
                .map
                .is_empty()
        );
    }

    #[test]
    fn roof_generator_does_not_hide_missing_id_behind_a_synthetic_destination_gate() {
        let mut world = fixture();
        {
            let mut storage = world.resource_mut::<DecoratorStorage>();
            storage
                .per_wall
                .get_mut(&WallId(7))
                .unwrap()
                .decorators
                .retain(|&id, _| id == 43);
            storage
                .per_wall
                .get_mut(&WallId(7))
                .unwrap()
                .modified_ids
                .retain(|&id| id == 43);
        }
        world.insert_resource(DerivedDecoratorSubtypes::default());
        let failure = catch_unwind(AssertUnwindSafe(|| {
            world.run_system_once(generate_roof_chimneys).unwrap()
        }))
        .unwrap_err();
        assert!(panic_text(failure).contains("subtype library doesn't contain subtype 43"));
        assert!(world.resource::<DecoratorBlueprints>().map.is_empty());
    }
}

#[cfg(test)]
mod native_geometry_tests {
    use super::*;
    #[test]
    fn original_flag_sse_geometry_and_height_caps() {
        type PoseCase = (u8, u8, [u32; 7], [u32; 14]);
        type CapCase = ([u32; 2], u32);
        let (poses, caps): (&[PoseCase], &[CapCase]) = include!(
            "../../../../../reports/module-recovery-2026-10-23/m01-flag-geometry-native-cases.rs"
        );
        let pose_bits = |pose: Transform| {
            [
                pose.rotation.x,
                pose.rotation.y,
                pose.rotation.z,
                pose.rotation.w,
                pose.translation.x,
                pose.translation.y,
                pose.translation.z,
            ]
            .map(f32::to_bits)
        };
        for (index, (kind, corner, input, expected)) in poses.iter().enumerate() {
            let original = Transform {
                rotation: Quat::from_array(std::array::from_fn(|i| f32::from_bits(input[i]))),
                translation: Vec3::from_array(std::array::from_fn(|i| {
                    f32::from_bits(input[i + 4])
                })),
                scale: Vec3::new(1., 2., 3.),
            };
            let mut got = [0u32; 14];
            if *kind == 0 {
                let (outline, pose) = wall_flag_poses(original, *corner != 0);
                got[..7].copy_from_slice(&pose_bits(outline));
                got[7..].copy_from_slice(&pose_bits(pose));
            } else {
                let (pose, height, offset) = terrain_flag_cloth_pose(original);
                got[..7].copy_from_slice(&pose_bits(pose));
                got[7] = height.to_bits();
                got[8] = offset.to_bits();
            }
            assert_eq!(got, *expected, "原旗位姿case {index}");
        }
        for (input, expected) in caps {
            assert_eq!(
                cap_cloth_height(f32::from_bits(input[0]), f32::from_bits(input[1])).to_bits(),
                *expected
            );
        }
    }
}

/// 三个原生成器共享的显式CPU验收装配；只提供测试资产，不替换任何业务函数。
#[cfg(test)]
pub(crate) mod full_visual_test_support {
    use super::*;
    use crate::{
        blueprint::PrevDecoratorBlueprints,
        decorator_visual::deco_differential_state::{
            ChimneyAssemblyParams, DoorStairsAssemblyParams, WallAttachedDecoTracker,
        },
        decorator_visual_state::DecoratorVisualState,
    };
    use bevy_ecs::system::RunSystemOnce;
    use country_core::{
        asset_libraries::{
            mesh_library::{AssetMeshLibrary, MeshAsset},
            shader_library::{AssetShaderLibrary, SharedAssetShaderLibrary},
        },
        render::mesh::{ATTR_POSITION, Mesh},
        resources::walls::{
            decorator_collision::DecoratorPhysicsColliders, public_walls::OnWallDeleted,
            wall_holes::WallHoles,
        },
        systems::collision::WindowAutoClutterCandidates,
    };
    pub(crate) fn scene(info: DecoratorInfo, subtype: u8, address: DecoratorStorageAddr) -> World {
        let mut world = World::new();
        let id = DecoratorId(41);
        let mut storage = DecoratorStorage::default();
        storage.get_or_insert_default(address).insert(id, info);
        storage.decorator_to_storage.insert(id, address);
        world.insert_resource(storage);
        let mut subtypes =
            country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes::default();
        subtypes.insert(
            id.0,
            country_core::resources::walls::decorator_subtype::DecoratorSubtype(subtype),
        );
        world.insert_resource(subtypes);
        let mut transforms = CachedDecoratorTransforms::default();
        transforms.insert(
            41,
            CachedDecoratorTransform {
                rotation: Quat::from_rotation_y(0.4),
                translation: Vec3::new(4., 6., 1.),
                scale: Vec3::ONE,
            },
        );
        world.insert_resource(transforms);
        let mut lib =
            country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::default();
        let collision = country_core::resources::walls::decorator_subtype::DecoratorCollision {
            triangles: Vec::new(),
            mesh: Arc::new(RaycastMeshData::new_cuboid_at(
                Vec3::new(0.8, 2., 0.6),
                Vec3::ZERO,
            )),
            bounds: utils::geometry::Rectangle3d::new(Vec3::ZERO, Vec3::new(0.8, 2., 0.6), Vec2::X),
        };
        lib.insert_collision_and_interaction_as_same(
            DecoratorLibKey::new(subtype, info.rank()),
            collision,
            country_core::resources::walls::decorator_subtype::DecoratorOutline::None,
            country_core::resources::walls::decorator_subtype::DecoratorCollisionOutline::None,
        );
        world.insert_resource(lib);
        world.insert_resource(super::canonical_mixed_input_gate_tests::published_test_wall());
        world.init_resource::<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>();
        world.insert_resource(RaycastWorld::new());
        world.init_resource::<DecoratorBlueprints>();
        world.init_resource::<PrevDecoratorBlueprints>();
        world.init_resource::<DecoratorVisualState>();
        world.init_resource::<WallHoles>();
        world.init_resource::<DecoratorPhysicsColliders>();
        world.init_resource::<WindowAutoClutterCandidates>();
        world.init_resource::<WallAttachedDecoTracker<ChimneyAssemblyParams>>();
        world.init_resource::<WallAttachedDecoTracker<DoorStairsAssemblyParams>>();
        world.init_resource::<Events<OnWallDeleted>>();
        let meshes = AssetMeshLibrary::new();
        for name in ["decorators/flag_roof_cloth", "decorators/flag_wall"] {
            let mut mesh = Mesh::new(name);
            mesh.set_attribute(ATTR_POSITION, vec![Vec3::ZERO, Vec3::X, Vec3::Y]);
            mesh.set_indices(vec![0, 1, 2]);
            meshes.add(MeshAsset::new(mesh, Some(name.into())));
        }
        let shaders = AssetShaderLibrary::new();
        for name in ["flag_roof_cloth", "flag_wall_cloth"] {
            shaders.add_raster(
                country_core::render::shader::RasterShaderProgram::new_draw(
                    name,
                    Default::default(),
                    &country_core::render::test_device::TestDevice(1),
                ),
                false,
            );
        }
        world.insert_resource(meshes);
        world.insert_resource(SharedAssetShaderLibrary(Arc::new(shaders)));
        world
    }
    pub(crate) fn instantiate(world: &mut World) -> Vec<Entity> {
        world
            .run_system_once(crate::blueprint::instantiate_blueprint_visual_state)
            .unwrap();
        let formal = world
            .resource::<DecoratorBlueprints>()
            .get(41)
            .is_some_and(|bp| bp.u138 == 0);
        let physics = world.resource::<DecoratorPhysicsColliders>();
        assert_eq!(physics.first.contains_key(&DecoratorId(41)), formal);
        assert_eq!(physics.second.contains_key(&DecoratorId(41)), formal);
        world
            .resource::<DecoratorVisualState>()
            .persistent_entities_of(41)
            .unwrap_or(&[])
            .to_vec()
    }
    pub(crate) fn snapshot(world: &mut World) {
        world
            .run_system_once(crate::blueprint::copy_blueprints_to_prev)
            .unwrap();
    }
    pub(crate) fn assert_unload(world: &mut World, old: &[Entity]) {
        snapshot(world);
        world.resource_mut::<DecoratorBlueprints>().remove(41);
        instantiate(world);
        for &entity in old {
            assert!(world.get_entity(entity).is_err());
        }
        let physics = world.resource::<DecoratorPhysicsColliders>();
        assert!(physics.first.is_empty() && physics.second.is_empty());
    }
}
#[cfg(test)]
mod actual_flags_world_tests {
    use super::full_visual_test_support::*;
    use super::*;
    use bevy_ecs::system::RunSystemOnce;
    fn wall_info() -> DecoratorInfo {
        let mut info = DecoratorInfo {
            ty: 0x3d,
            rank: 2,
            u28: 17,
            decorator_id: 19,
            ..Default::default()
        };
        info.dst_wall = u64::from(4f32.to_bits()) | (u64::from(8f32.to_bits()) << 32);
        info.set_extra(
            country_core::resources::walls::decorator::DecoratorExtraInfo::Flag(
                country_core::resources::walls::decorator::FlagExtra(1),
            ),
        );
        info
    }
    fn generated_height(world: &World) -> f32 {
        let words = world
            .resource::<DecoratorBlueprints>()
            .get(41)
            .unwrap()
            .visual_entities[1]
            .wall_flag_cloth
            .unwrap()
            .1;
        f32::from_bits((words >> 32) as u32)
    }
    #[test]
    fn wall_banner_uses_other_decorators_and_true_excluding_wall_raycast_before_height_caps() {
        use country_core::{
            resources::walls::{
                WallId,
                decorator_subtype::{
                    DecoratorMeshLibrary, DecoratorSubtype, DerivedDecoratorSubtypes,
                },
            },
            systems::collision::{
                query_policy::RaycastGeoSourceProjection, world::upsert_collider,
            },
        };
        let address = DecoratorStorageAddr::Wall(WallId(7));
        let mut world = scene(wall_info(), 11, address);
        world.run_system_once(generate_flags).unwrap();
        assert_eq!(generated_height(&world).to_bits(), 4.01f32.to_bits());
        // 当前墙较高的碰撞必须被原Option<WallId>拒绝，另一墙的topY=4真实截断。
        let mut map = std::collections::HashMap::new();
        {
            let mut ray = world.resource_mut::<RaycastWorld>();
            for (id, wall, y) in [(1, 7u64, 5f32), (2, 8u64, 3f32)] {
                let mut source = [0u8; 24];
                source[8..16].copy_from_slice(&wall.to_le_bytes());
                upsert_collider(
                    &mut ray,
                    &mut map,
                    id,
                    RaycastGeoSourceProjection(source),
                    Arc::new(RaycastMeshData::new_cuboid_at(
                        Vec3::new(100., 2., 100.),
                        Vec3::ZERO,
                    )),
                    Vec3::new(0., y, 0.),
                    Quat::IDENTITY,
                );
            }
            ray.flush_colliders();
        }
        world.run_system_once(generate_flags).unwrap();
        assert_eq!(generated_height(&world).to_bits(), 3.51f32.to_bits());
        let mut other = DecoratorInfo {
            ty: 0x3a,
            rank: 2,
            ..Default::default()
        };
        other.dst_wall = u64::from(4f32.to_bits()) | (u64::from(6f32.to_bits()) << 32);
        world
            .resource_mut::<DecoratorStorage>()
            .get_mut(address)
            .unwrap()
            .insert(DecoratorId(42), other);
        world
            .resource_mut::<DerivedDecoratorSubtypes>()
            .insert(42, DecoratorSubtype(22));
        let copy = world
            .resource::<DecoratorMeshLibrary>()
            .collision_record(DecoratorLibKey::new(11, 2))
            .clone();
        world
            .resource_mut::<DecoratorMeshLibrary>()
            .insert_collision_and_interaction_as_same(
                DecoratorLibKey::new(22, 2),
                copy,
                country_core::resources::walls::decorator_subtype::DecoratorOutline::None,
                country_core::resources::walls::decorator_subtype::DecoratorCollisionOutline::None,
            );
        world.run_system_once(generate_flags).unwrap();
        assert_eq!(generated_height(&world).to_bits(), 1.51f32.to_bits());
    }

    #[test]
    fn all_five_original_flag_variants_reach_fullvisual_and_release_real_collision_and_entities() {
        for (subtype, state) in (11..=15).flat_map(|subtype| [0, 1].map(|state| (subtype, state))) {
            let mut info = DecoratorInfo {
                ty: 0x3d,
                rank: 2,
                state,
                u28: 17,
                decorator_id: 19,
                ..Default::default()
            };
            info.dst_wall = u64::from(4f32.to_bits()) | (u64::from(8f32.to_bits()) << 32);
            info.set_extra(
                country_core::resources::walls::decorator::DecoratorExtraInfo::Flag(
                    country_core::resources::walls::decorator::FlagExtra(1),
                ),
            );
            let address = if subtype == 15 {
                DecoratorStorageAddr::Terrain
            } else {
                DecoratorStorageAddr::Wall(country_core::resources::walls::WallId(7))
            };
            let mut world = scene(info, subtype, address);
            world.run_system_once(generate_flags).unwrap();
            let blueprint = world.resource::<DecoratorBlueprints>().get(41).unwrap();
            assert!(blueprint.transform.is_some());
            assert_eq!(blueprint.visual_entities.len(), 2);
            if subtype <= 12 {
                assert!(blueprint.visual_entities[1].wall_flag_cloth.is_some());
            }
            let entities = instantiate(&mut world);
            assert_eq!(entities.len(), 2);
            if subtype == 13 || subtype == 14 {
                assert_eq!(
                    world
                        .get::<country_core::components::drawable::RoofFlagCloth>(entities[1])
                        .unwrap()
                        .0,
                    60
                );
            } else {
                let cloth = world
                    .get::<country_core::components::drawable::WallFlagCloth>(entities[1])
                    .unwrap();
                assert_eq!(cloth.id, 60);
                assert_eq!(cloth.flag, subtype == 12);
            }
            snapshot(&mut world);
            assert_eq!(instantiate(&mut world), entities, "Arc相同帧不重新建实体");
            assert_unload(&mut world, &entities);
        }
    }
}
