//! 原B1407c21b0/12159B：两种墙烟囱的真实动态碰撞producer。
//! 19复用实际墙/路径/高度/邻居数据组成有序box，20变换真实vent_pipe_collision资产。
//! 此处只闭合碰撞/source/entry链；烟雾与视觉装配参数的既有缺口仍未全部恢复。

use bevy_ecs::prelude::*;
use ::utils::scalar::max_f32;
use country_core::asset_libraries::mesh_library::AssetMeshLibrary;
use country_core::resources::walls::{
    cached_decorator_transforms::{CachedDecoratorTransform, CachedDecoratorTransforms},
    decorator::{DecoratorId, DecoratorInfo},
    decorator_storage::{DecoratorStorage, DecoratorStorageAddr, Decorators},
    decorator_subtype::{DecoratorLibKey, DerivedDecoratorInfoResources, DerivedDecoratorInfoView},
    public_wall_state::PublicWallState,
    public_walls::PublicWalls,
};
use country_core::systems::collision::world::RaycastMeshData;
use country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::box_trigger_volume;
use country_core::utils::components::transform::Transform;
use glam::{Affine3A, Quat, Vec2, Vec3};
use std::sync::Arc;
use utils::{
    comparative_space::ComparativeSpace,
    curve::{
        CurveCoordSemantic3D, CurveInternalCoord, approx_curve_coord, curve_slice::CurveSlice,
    },
    geometry::intersect::Triangle,
    wall_space::WallSpace,
};

pub const WALL_CHIMNEY_SUBTYPE_LOW: u8 = 0x13;
pub const WALL_CHIMNEY_SUBTYPE_HIGH: u8 = 0x14;
pub const CHIMNEY_NANI_SMOKE: &str = "nani_smoke";
const VENT_PIPE_COLLISION: &str = "decorators/vent_pipe_collision";

pub fn is_wall_chimney_subtype(subtype: u8) -> bool {
    subtype.wrapping_sub(WALL_CHIMNEY_SUBTYPE_LOW) <= 1
}

pub fn generate_wall_chimneys(
    decorator_storage: Res<DecoratorStorage>,
    cached_transforms: Res<CachedDecoratorTransforms>,
    walls: Res<PublicWalls>,
    derived: DerivedDecoratorInfoResources,
    assets: Res<AssetMeshLibrary>,
    mut blueprints: ResMut<crate::blueprint::DecoratorBlueprints>,
    _transient: ResMut<crate::decorator_visual_state::DecoratorTransientVisualState>,
) {
    let derived = derived.as_view();
    for (wall_id, decorators) in decorator_storage.iter() {
        for (id, info) in decorators.iter_modified() {
            let subtype = derived.subtype(DecoratorId(id)).0;
            if !is_wall_chimney_subtype(subtype) {
                continue;
            }
            let cached = cached_transforms.transform(id);
            // 原2a19先取得唯一entry，再消费extra和动态几何的required输入。
            let blueprint =
                blueprints.insert_for(id, DecoratorStorageAddr::Wall(wall_id), info.state());
            let extra = info.extra().as_chimney();
            blueprint.set_cached_transform(cached);
            let triangles = if subtype == WALL_CHIMNEY_SUBTYPE_LOW {
                let wall = walls
                    .get(wall_id)
                    .expect("wall chimney requires its public wall");
                wall_chimney_triangles(id, info, extra.f, &cached, wall, decorators, &derived)
            } else {
                vent_pipe_triangles(&cached, &assets)
            };
            // 原4017..416b按面展开，不weld顶点；normals/UV均为None。
            let vertices: Vec<_> = triangles
                .into_iter()
                .flat_map(|triangle| triangle.points)
                .collect();
            let indices: Vec<_> = (0..vertices.len() / 3)
                .map(|triangle| {
                    let first = (triangle * 3) as u32;
                    [first, first.wrapping_add(1), first.wrapping_add(2)]
                })
                .collect();
            let mesh = Arc::new(RaycastMeshData::new_indexed(
                &indices, &vertices, None, None,
            ));
            // 点已经烘焙到世界空间；不能再次乘缓存位姿。
            blueprint.set_positioned_colliders(
                Arc::clone(&mesh),
                mesh,
                &Vec3::ZERO,
                &Quat::IDENTITY,
            );
            // 现有烟囱视觉参数接口保留；其完整原boxed装配载荷另有未恢复部分。
            blueprint.add_chimney(extra.f);
        }
    }
}

fn vent_pipe_triangles(
    cached: &CachedDecoratorTransform,
    assets: &AssetMeshLibrary,
) -> Vec<Triangle> {
    let position =
        cached.translation + cached.rotation * Vec3::new(0.0, 0.0, f32::from_bits(0x3e8f5c29));
    let affine = Affine3A::from_scale_rotation_translation(cached.scale, cached.rotation, position);
    let mesh = assets
        .get_by_name(VENT_PIPE_COLLISION)
        .expect("vent pipe collision mesh");
    mesh.as_triangles()
        .into_iter()
        .map(|triangle| {
            Triangle::new(
                affine.transform_point3(triangle.points[0]),
                affine.transform_point3(triangle.points[1]),
                affine.transform_point3(triangle.points[2]),
            )
        })
        .collect()
}

/// 借真实点表重用原XZ距离采样，不创建另一份曲线或累计长度缓存。
fn coord_at_distance(points: &[[f32; 3]], distance: f32) -> CurveInternalCoord {
    let start = CurveInternalCoord { segment: 0, t: 0.0 };
    let mut slice =
        CurveSlice::<[f32; 3], _, CurveCoordSemantic3D, Vec2, _>::from_coord_range_with_remap(
            points,
            start,
            start,
            |_, point| Vec2::new(point[0], point[2]),
        );
    slice.extend_right(distance);
    slice.end()
}

fn scalar_at(values: &[f32], coord: CurveInternalCoord) -> f32 {
    let first = values[coord.segment];
    let second = values[coord.segment + 1];
    (second - first) * coord.t + first
}

fn nearest_coord(points: &[[f32; 3]], position: Vec3) -> CurveInternalCoord {
    approx_curve_coord(points, Vec2::new(position.x, position.z), |point| {
        Vec2::new(point[0], point[2])
    })
}

#[allow(clippy::too_many_arguments)]
fn wall_chimney_triangles(
    id: u32,
    info: &DecoratorInfo,
    extra_height: f32,
    cached: &CachedDecoratorTransform,
    wall: &PublicWallState,
    decorators: &Decorators,
    derived: &DerivedDecoratorInfoView<'_>,
) -> Vec<Triangle> {
    let max_y = wall.max_y();
    let dst = info.dst_as_wall().wall_space_coord();
    let original_points = wall.geometry.outline.as_slice();
    let coord = coord_at_distance(original_points, max_f32(dst.u, 0.0));
    let ground_y = (1.0 - coord.t) * original_points[coord.segment][1]
        + coord.t * original_points[coord.segment + 1][1];
    let height = extra_height + (max_f32(max_y - dst.y, 0.0) + 3.0);
    let first_offset = cached.rotation * Vec3::new(0.0, 0.0, f32::from_bits(0x3eaaaaab));
    let second_offset = cached.rotation * Vec3::new(0.0, 0.0, f32::from_bits(0x3e19999a));
    let position = Vec3::new(
        cached.translation.x,
        cached.translation.y + height * 0.5,
        cached.translation.z,
    ) + first_offset
        + second_offset;
    let body = Transform {
        rotation: cached.rotation,
        translation: position,
        scale: Vec3::new(0.85, height, 0.85),
    };
    let mut triangles = Vec::with_capacity(24);
    let mut lower_valid = false;
    let relative_height_blocks = wall.authority.elevation() > 0.0
        && scalar_at(
            &wall.geometry.relative_heights,
            nearest_coord(original_points, position),
        ) > 0.5;
    let difference = dst.y - ground_y;
    // 原2dee有序<5；NaN和>=5都不进入lower候选，不能写成 !(difference>=5)。
    if !relative_height_blocks && difference < 5.0 {
        let lower_height = max_f32(difference, 1.5);
        let lower = Transform {
            rotation: body.rotation,
            translation: Vec3::new(
                body.translation.x,
                (body.translation.y - height * 0.5) - lower_height * 0.5,
                body.translation.z,
            ),
            scale: Vec3::new(1.75, lower_height, 1.2),
        };
        let space = WallSpace {
            is_loop: wall.shape().tag() != 2,
            length: wall.wall_space_length(),
        };
        let lower_u = space.project_unbounded(dst.u);
        let prepared = wall
            .prepared_path
            .as_ref()
            .expect("wall chimney requires prepared wall paths");
        let coord = coord_at_distance(&prepared.positions, lower_u);
        let arch_height = scalar_at(
            wall.arch_segment_heights()
                .expect("wall chimney requires arch segment heights"),
            coord,
        );
        let center_y = dst.y - lower_height * 0.5;
        lower_valid = is_valid_body_placement(
            id,
            decorators,
            derived,
            &space,
            lower_u,
            center_y,
            lower.scale,
            arch_height,
            &prepared.positions,
            &prepared.segment_values,
            body.translation,
        );
        if lower_valid {
            triangles.extend(box_trigger_volume(1.0, 1.0, 1.0, &lower));
        }
    }
    // 原有效lower先追加，再追加主体；无效lower在主体之后追加内缩fallback。
    triangles.extend(box_trigger_volume(1.0, 1.0, 1.0, &body));
    if !lower_valid {
        let fallback = Transform {
            rotation: body.rotation,
            translation: Vec3::new(
                body.translation.x,
                (body.translation.y - height * 0.5) + (-0.2),
                body.translation.z,
            ),
            scale: Vec3::new(body.scale.x + (-0.2), 0.4, body.scale.z + (-0.2)),
        };
        triangles.extend(box_trigger_volume(1.0, 1.0, 1.0, &fallback));
    }
    triangles
}

/// 原37c2..43cc的完整lower判定；真实邻居/高度/路径缺失不能默认成功。
#[allow(clippy::too_many_arguments)]
fn is_valid_body_placement(
    id: u32,
    decorators: &Decorators,
    derived: &DerivedDecoratorInfoView<'_>,
    space: &WallSpace,
    lower_u: f32,
    center_y: f32,
    dimensions: Vec3,
    arch_height: f32,
    path_points: &[[f32; 3]],
    path_values: &[f32],
    body_position: Vec3,
) -> bool {
    for (other_id, other) in decorators.iter_maybe_mut() {
        let tag = other.dst_tag();
        // 原wrapped分派：2为Roof，3/4不可达，其余raw tag走墙坐标读面。
        let selector = if tag.wrapping_sub(2) >= 3 {
            0
        } else {
            tag.wrapping_sub(1)
        };
        match selector {
            0 => {
                let coord = other.wall_space_coord();
                let subtype = derived.subtype(DecoratorId(other_id));
                let key = DecoratorLibKey::new(subtype.0, other.rank());
                let record = derived.mesh_library().collision_record(key);
                let overlaps_y = ((record.bounds.top - record.bounds.bottom) * 0.5
                    + dimensions.y * 0.5)
                    > (coord.y - center_y).abs();
                let overlaps = overlaps_y
                    && space.distance(coord.u, lower_u)
                        < record.bounds.footprint.size.x * 0.5 + dimensions.x * 0.5;
                // 原self也先完成required表读取；仅最后排除其重叠结果。
                if other_id != id && overlaps {
                    return false;
                }
            }
            1 => {}
            _ => panic!("internal error: entered unreachable code"),
        }
    }
    let limit = (center_y + dimensions.y * 0.5) + f32::from_bits(0x3e99999a);
    if !(arch_height > limit) {
        return false;
    }
    scalar_at(path_values, nearest_coord(path_points, body_position)) == 0.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subtype_range_is_0x13_0x14() {
        assert!(is_wall_chimney_subtype(0x13));
        assert!(is_wall_chimney_subtype(0x14));
        assert!(!is_wall_chimney_subtype(0x15));
        assert!(!is_wall_chimney_subtype(0x12));
        assert!(!is_wall_chimney_subtype(0x00));
    }
}

#[cfg(test)]
mod canonical_collision_chain_tests {
    use super::*;
    use crate::blueprint::{BlueprintMeshSlot, DecoratorBlueprints};
    use bevy_ecs::system::RunSystemOnce;
    use country_core::asset_libraries::mesh_library::MeshAsset;
    use country_core::render::mesh::{ATTR_POSITION, Mesh};
    use country_core::resources::roofs::geometry::surface::Curve2;
    use country_core::resources::walls::{
        InnerWalls, WallId, WallState,
        decorator::{ChimneyExtra, DecoratorExtraInfo},
        decorator_subtype::{
            DecoratorCollision, DecoratorCollisionOutline, DecoratorMeshLibrary, DecoratorOutline,
            DecoratorSubtype, DerivedDecoratorSubtypes,
        },
        publication_source::TerrainHeightGrid512,
    };
    use country_core::systems::wall::wall_state::path_preparation::PathRasters512;
    use std::panic::{AssertUnwindSafe, catch_unwind};

    fn info(id: u32, u: f32, y: f32) -> DecoratorInfo {
        let mut info = DecoratorInfo {
            decorator_id: id,
            dst_wall: u64::from(u.to_bits()) | (u64::from(y.to_bits()) << 32),
            rank: 1,
            state: 1,
            ..Default::default()
        };
        info.set_extra(DecoratorExtraInfo::Chimney(ChimneyExtra {
            f: 1.0,
            smoke: 3,
        }));
        info
    }

    fn record(size: Vec3) -> DecoratorCollision {
        DecoratorCollision {
            triangles: box_trigger_volume(size.x, size.y, size.z, &Transform::identity()),
            mesh: Arc::new(RaycastMeshData::new_cuboid_at(size, Vec3::ZERO)),
            bounds: utils::geometry::Rectangle3d::new(Vec3::ZERO, size, Vec2::X),
        }
    }

    fn base_world(subtype: u8, cached: CachedDecoratorTransform) -> World {
        let mut world = World::new();
        let mut bucket = Decorators::default();
        bucket.insert(DecoratorId(41), info(41, 4.0, 2.0));
        let mut storage = DecoratorStorage::default();
        storage.per_wall.insert(WallId(7), bucket);
        world.insert_resource(storage);
        let mut subtypes = DerivedDecoratorSubtypes::default();
        subtypes.insert(41, DecoratorSubtype(subtype));
        world.insert_resource(subtypes);
        let mut transforms = CachedDecoratorTransforms::default();
        transforms.insert(41, cached);
        world.insert_resource(transforms);
        let mut library = DecoratorMeshLibrary::default();
        library.insert(
            DecoratorLibKey::new(19, 1),
            record(Vec3::ONE),
            record(Vec3::ONE),
            DecoratorOutline::None,
            DecoratorCollisionOutline::None,
        );
        library.insert(
            DecoratorLibKey::new(4, 2),
            record(Vec3::splat(2.0)),
            record(Vec3::splat(2.0)),
            DecoratorOutline::None,
            DecoratorCollisionOutline::None,
        );
        world.insert_resource(library);
        world.insert_resource(AssetMeshLibrary::new());
        world.insert_resource(PublicWalls::default());
        world.insert_resource(DecoratorBlueprints::default());
        world.insert_resource(
            crate::decorator_visual_state::DecoratorTransientVisualState::default(),
        );
        world
    }

    fn body_world(elevation: f32) -> World {
        let mut world = base_world(
            19,
            CachedDecoratorTransform {
                rotation: Quat::IDENTITY,
                translation: Vec3::new(4.0, 2.0, 0.0),
                scale: Vec3::ONE,
            },
        );
        let mut inner = InnerWalls::new();
        let authority = WallState::new(
            Curve2::try_new(vec![[0.0, 0.0], [8.0, 0.0]], false).unwrap(),
            8.0,
            0,
            0,
            false,
        );
        inner.new_wall_with_id(WallId(7), authority);
        if elevation != 0.0 {
            inner.update_elevation(WallId(7), elevation);
        }
        let terrain = TerrainHeightGrid512 {
            encoded: vec![0; 512 * 512].into(),
            minimum_y: 0.0,
            maximum_y: 0.0,
            sampling_fix: false,
        };
        let paths = PathRasters512::from_encoded(
            std::array::from_fn(|_| Arc::from(vec![0u8; 512 * 512])),
            false,
        )
        .unwrap();
        let mut walls = PublicWalls::default();
        walls.consume_with_paths(&mut inner, &terrain, None, &country_core::resources::walls::public_walls::WallPublicationDirty::default(), &country_core::resources::walls::public_walls::WallSpatialHash::default());
        walls
            .get_mut(WallId(7))
            .unwrap()
            .prepare_paths(&paths)
            .unwrap();
        world.insert_resource(walls);
        world
    }

    fn mesh(world: &World) -> Arc<RaycastMeshData> {
        let blueprint = world.resource::<DecoratorBlueprints>().get(41).unwrap();
        assert_eq!(
            blueprint.storage_addr,
            Some(DecoratorStorageAddr::Wall(WallId(7)))
        );
        assert_eq!(blueprint.interaction_source_flags, 1);
        assert!(blueprint.colliders.is_empty());
        let BlueprintMeshSlot::Mesh(collision) = &blueprint.collision_mesh else {
            panic!("实际碰撞网格")
        };
        let BlueprintMeshSlot::Mesh(interaction) = &blueprint.interaction_mesh else {
            panic!("实际交互网格")
        };
        assert!(Arc::ptr_eq(&collision.mesh, &interaction.mesh));
        assert_eq!(collision.position, Vec3::ZERO);
        assert_eq!(collision.rotation, Quat::IDENTITY);
        assert_eq!(interaction.position, Vec3::ZERO);
        assert_eq!(interaction.rotation, Quat::IDENTITY);
        assert!(collision.mesh.shape_ref().unwrap().as_trimesh().is_some());
        assert!(collision.mesh.normals().is_empty());
        assert!(collision.mesh.uvs().is_empty());
        Arc::clone(&collision.mesh)
    }

    fn bounds(points: &[Vec3]) -> (Vec3, Vec3) {
        points.iter().fold(
            (Vec3::splat(f32::INFINITY), Vec3::splat(f32::NEG_INFINITY)),
            |(lo, hi), &point| (lo.min(point), hi.max(point)),
        )
    }

    fn close(actual: Vec3, expected: Vec3) {
        assert!(
            (actual - expected).abs().max_element() < 2.0e-6,
            "{actual:?} != {expected:?}"
        );
    }

    #[test]
    fn vent_pipe_actual_asset_is_baked_in_native_triangle_order_and_reaches_physics() {
        let cached = CachedDecoratorTransform {
            rotation: Quat::from_xyzw(0.0, 1.0, 0.0, 0.0),
            translation: Vec3::new(4.0, 2.0, -3.0),
            scale: Vec3::new(2.0, 3.0, 4.0),
        };
        let mut world = base_world(20, cached);
        let mut asset = Mesh::new("非规则四面体原输入夹具");
        asset.set_attribute(
            ATTR_POSITION,
            vec![Vec3::ZERO, Vec3::X, Vec3::Y * 2.0, Vec3::Z * 3.0],
        );
        asset.set_indices(vec![0, 1, 2, 0, 3, 1, 0, 2, 3, 1, 3, 2]);
        world
            .resource::<AssetMeshLibrary>()
            .add(MeshAsset::new(asset, Some(VENT_PIPE_COLLISION.to_owned())));
        world.run_system_once(generate_wall_chimneys).unwrap();
        let actual = mesh(&world);
        // 独立列出180度Y旋转、非单位缩放、原.28局部偏移后的全部原面顺序。
        let a = Vec3::new(4.0, 2.0, -3.28);
        let b = Vec3::new(2.0, 2.0, -3.28);
        let c = Vec3::new(4.0, 8.0, -3.28);
        let d = Vec3::new(4.0, 2.0, -15.28);
        let expected = [a, b, c, a, d, b, a, c, d, b, d, c];
        assert_eq!(actual.vertices().len(), expected.len());
        for (&actual, expected) in actual.vertices().iter().zip(expected) {
            close(actual, expected);
        }
        assert_eq!(
            actual.indices(),
            &[[0, 1, 2], [3, 4, 5], [6, 7, 8], [9, 10, 11]]
        );
        assert_eq!(
            world
                .resource::<DecoratorBlueprints>()
                .get(41)
                .unwrap()
                .transform,
            Some(cached)
        );
        use crate::blueprint::{PrevDecoratorBlueprints, instantiate_decorator_colliders};
        use country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders;
        use country_core::systems::collision::world::{RaycastGeoSource, RaycastWorld};
        let mut maps = DecoratorPhysicsColliders::default();
        let mut physics = RaycastWorld::new();
        instantiate_decorator_colliders(
            &mut maps,
            world.resource::<DecoratorBlueprints>(),
            &PrevDecoratorBlueprints::default(),
            &mut physics,
        );
        for (instance, source) in [
            (
                &maps.first[&DecoratorId(41)],
                RaycastGeoSource::decorator(DecoratorId(41), DecoratorStorageAddr::Wall(WallId(7))),
            ),
            (
                &maps.second[&DecoratorId(41)],
                RaycastGeoSource::decorator_interaction(
                    DecoratorId(41),
                    DecoratorStorageAddr::Wall(WallId(7)),
                    1,
                ),
            ),
        ] {
            let registered =
                physics.get_collider_mesh(instance.registry_index, instance.generation);
            assert!(Arc::ptr_eq(&registered.data, &actual));
            assert_eq!(registered.source, source);
        }
    }

    #[test]
    fn actual_published_wall_lower_and_main_order_changes_to_main_fallback_only_on_strict_overlap()
    {
        let mut world = body_world(0.0);
        assert_eq!(
            world
                .resource::<PublicWalls>()
                .get(WallId(7))
                .unwrap()
                .max_y(),
            8.0
        );
        world.run_system_once(generate_wall_chimneys).unwrap();
        let actual = mesh(&world);
        assert_eq!(actual.indices().len(), 24);
        let (lower_lo, lower_hi) = bounds(&actual.vertices()[..36]);
        let (body_lo, body_hi) = bounds(&actual.vertices()[36..]);
        let z = f32::from_bits(0x3eaaaaab) + f32::from_bits(0x3e19999a);
        close(lower_lo, Vec3::new(3.125, 0.0, z - 0.6));
        close(lower_hi, Vec3::new(4.875, 2.0, z + 0.6));
        close(body_lo, Vec3::new(3.575, 2.0, z - 0.425));
        close(body_hi, Vec3::new(4.425, 12.0, z + 0.425));
        let mut neighbor = info(42, 5.875, 1.0);
        neighbor.rank = 2;
        world
            .resource_mut::<DecoratorStorage>()
            .per_wall
            .get_mut(&WallId(7))
            .unwrap()
            .insert(DecoratorId(42), neighbor);
        world
            .resource_mut::<DerivedDecoratorSubtypes>()
            .insert(42, DecoratorSubtype(4));
        // 接触边界distance=1+1.75/2，严格比较不视为重叠。
        world.run_system_once(generate_wall_chimneys).unwrap();
        close(bounds(&mesh(&world).vertices()[..36]).0, lower_lo);
        neighbor.dst_wall = u64::from(5.865f32.to_bits()) | (u64::from(1.0f32.to_bits()) << 32);
        world
            .resource_mut::<DecoratorStorage>()
            .per_wall
            .get_mut(&WallId(7))
            .unwrap()
            .insert(DecoratorId(42), neighbor);
        world.run_system_once(generate_wall_chimneys).unwrap();
        let actual = mesh(&world);
        close(bounds(&actual.vertices()[..36]).0, body_lo);
        let (fallback_lo, fallback_hi) = bounds(&actual.vertices()[36..]);
        close(fallback_lo, Vec3::new(3.675, 1.6, z - 0.325));
        close(fallback_hi, Vec3::new(4.325, 2.0, z + 0.325));
    }

    #[test]
    fn actual_path_arch_and_relative_height_gates_keep_distinct_owner_inputs() {
        let mut world = body_world(0.0);
        world
            .resource_mut::<PublicWalls>()
            .get_mut(WallId(7))
            .unwrap()
            .prepared_path
            .as_mut()
            .unwrap()
            .segment_values
            .fill(1.0);
        world.run_system_once(generate_wall_chimneys).unwrap();
        assert_eq!(bounds(&mesh(&world).vertices()[..36]).0.y, 2.0);
        let mut world = body_world(0.0);
        world
            .resource_mut::<PublicWalls>()
            .get_mut(WallId(7))
            .unwrap()
            .freeform_heights
            .as_mut()
            .unwrap()
            .arch_segment_heights
            .fill(2.3);
        world.run_system_once(generate_wall_chimneys).unwrap();
        // 原限值为2+.3，恰相等不能放lower。
        assert_eq!(bounds(&mesh(&world).vertices()[..36]).0.y, 2.0);
        let elevated = body_world(1.0);
        assert!(
            elevated
                .resource::<PublicWalls>()
                .get(WallId(7))
                .unwrap()
                .geometry
                .relative_heights
                .iter()
                .all(|&height| height == 1.0)
        );
        world = elevated;
        world.run_system_once(generate_wall_chimneys).unwrap();
        assert_eq!(bounds(&mesh(&world).vertices()[..36]).0.y, 2.0);
    }

    #[test]
    fn own_record_is_required_before_self_exclusion_and_failures_keep_entry_source() {
        let mut world = body_world(0.0);
        world
            .resource_mut::<DecoratorMeshLibrary>()
            .collision
            .remove(&DecoratorLibKey::new(19, 1));
        assert!(
            catch_unwind(AssertUnwindSafe(|| world
                .run_system_once(generate_wall_chimneys)
                .unwrap()))
            .is_err()
        );
        let blueprint = world.resource::<DecoratorBlueprints>().get(41).unwrap();
        assert_eq!(
            blueprint.storage_addr,
            Some(DecoratorStorageAddr::Wall(WallId(7)))
        );
        assert_eq!(blueprint.interaction_source_flags, 1);
        assert!(blueprint.transform.is_some());
        assert!(matches!(
            blueprint.collision_mesh,
            BlueprintMeshSlot::Unspecified
        ));
        let mut missing_asset = base_world(
            20,
            CachedDecoratorTransform {
                rotation: Quat::IDENTITY,
                translation: Vec3::ZERO,
                scale: Vec3::ONE,
            },
        );
        assert!(
            catch_unwind(AssertUnwindSafe(|| missing_asset
                .run_system_once(generate_wall_chimneys)
                .unwrap()))
            .is_err()
        );
        assert!(
            missing_asset
                .resource::<DecoratorBlueprints>()
                .get(41)
                .unwrap()
                .transform
                .is_some()
        );
    }
}
