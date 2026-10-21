//! 原 B1407f66b0 农舍角窗：保留真实角几何、蓝图副作用与随机支路。
use bevy_ecs::prelude::*;
use country_core::{
    asset_libraries::mesh_library::AssetMeshLibrary,
    resources::{color_id::ColorId, walls::{
        cached_decorator_transforms::{CachedDecoratorTransform, CachedDecoratorTransforms},
        decorator::{DecoratorId, DecoratorInfo},
        decorator_storage::{DecoratorStorage, DecoratorStorageAddr},
        decorator_subtype::{DecoratorLibKey, DerivedDecoratorInfoResources},
        public_walls::{PublicWallState, PublicWalls},
        wall_holes::RawWallHole,
    }},
    systems::collision::{WindowAutoClutterCandidate, WindowAutoClutterKind},
    utils::components::transform::Transform,
};
use glam::{Vec2, Vec3};
use utils::wall_space::WallSpaceCoord;

use crate::{blueprint::DecoratorBlueprints, decorator_visual_state::ClothInfo};
use super::cottage_wall_window::{add_lintels, candidate_hash, flowerbed_xforms,
    wall_space, window_builder, LINTEL_SHADER};

const FRAME_NAMES: [&str; 3] = ["decorators/window_cottage_corner_1x1",
    "decorators/window_cottage_corner_2x1", "decorators/window_cottage_corner_3x1"];
const GLASS_NAMES: [&str; 3] = ["decorators/window_cottage_corner_1x1_glass",
    "decorators/window_cottage_corner_2x1_glass", "decorators/window_cottage_corner_3x1_glass"];
const SQRT_TWO: f32 = f32::from_bits(0x3fb5_04f3);
const FRAME_FORWARD: f32 = f32::from_bits(0x3eb6_77ae);
const CORNER_RETAIN: f32 = f32::from_bits(0x3f26_6666);
const EDGE_BLEND: f32 = f32::from_bits(0x3eb3_3333);

/// 原版在非矩形曲线上允许归一化失败，并在蓝图发布前跳过该装饰。
fn corner_directions(wall: &PublicWallState, info: &DecoratorInfo) -> Option<(Vec2, [Vec2; 2])> {
    let wall_info = info.dst_as_wall();
    if let Some(rectangle) = wall.is_rectangle() {
        // 原14B叶仅将非零 attachment 转为 attachment-1；真正角序由矩形检查。
        let attachment = wall_info.u20 as u8;
        let corner = attachment.checked_sub(1)
            .expect("called `Option::unwrap()` on a `None` value");
        let index = usize::from(corner);
        Some((rectangle.get_corner_normal(index), rectangle.get_edge_dirs_from_corner_id(index)))
    } else {
        let coord = wall_info.wall_space_coord();
        let space = wall_space(wall);
        let first = WallSpaceCoord::new_unchecked(
            space.project_clamp(coord.u + f32::from_bits(0xbdcc_cccd)), coord.y);
        let second = WallSpaceCoord::new_unchecked(
            space.project_clamp(coord.u + f32::from_bits(0x3dcc_cccd)), coord.y);
        let curve = wall.simplified_curve_2d();
        let center = coord.to_world_space(curve);
        let first = first.to_world_space(curve);
        let second = second.to_world_space(curve);
        let left = Vec2::new(first.x - center.x, first.z - center.z).try_normalize()?;
        let right = Vec2::new(second.x - center.x, second.z - center.z).try_normalize()?;
        let direction = (right - left).try_normalize()?;
        Some((Vec2::new(-direction.y, direction.x), [left, right]))
    }
}

/// 原内联几何恒产生两份侧边姿态；用定长数组避免重新引入临时堆分配。
fn corner_poses(cached: CachedDecoratorTransform, width: f32, normal: Vec2,
    directions: [Vec2; 2], rank: u8) -> (Transform, [Transform; 2]) {
    let span = width / SQRT_TWO;
    let center = Vec2::new(cached.translation.x, cached.translation.z);
    let first = directions[0] * span + center;
    let second = directions[1] * span + center;
    let midpoint = second * 0.5 + first * 0.5;
    let mut main = Transform { rotation: utils::xz_rotation_bad(normal),
        translation: Vec3::new(midpoint.x, cached.translation.y, midpoint.y), scale: Vec3::ONE };
    assert!((1..=3).contains(&rank), "农舍角窗 rank 必须为1至3");
    let side = |point: Vec2, direction: Vec2| {
        let position = point * EDGE_BLEND + center * CORNER_RETAIN;
        Transform { rotation: utils::xz_rotation_bad(Vec2::new(-direction.y, direction.x)),
            translation: Vec3::new(position.x, cached.translation.y, position.y), scale: Vec3::ONE }
    };
    let sides = [side(first, directions[0]), side(second, directions[1])];
    main.translation += main.rotation * Vec3::new(0.0, 0.0, FRAME_FORWARD);
    (main, sides)
}

/// 原六参系统面：使用同一 required 派生表、墙、缓存、资产与蓝图 owner。
pub fn generate_cottage_corner_windows(derived: DerivedDecoratorInfoResources,
    storage: Res<DecoratorStorage>, cached: Res<CachedDecoratorTransforms>,
    walls: Res<PublicWalls>, mut blueprints: ResMut<DecoratorBlueprints>,
    meshes: Res<AssetMeshLibrary>) {
    let view = derived.as_view();
    for (wall_id, decorators) in storage.iter_walls() {
        for (modified, info) in decorators.iter_modified() {
            let id = DecoratorId(modified);
            if view.subtype(id).0 != 3 { continue; }
            let wall = walls.get(wall_id).expect("called `Option::unwrap()` on a `None` value");
            let transform = cached.transform(modified);
            let Some((normal, directions)) = corner_directions(wall, info) else { continue; };
            let key = DecoratorLibKey::new(3, info.rank());
            let bounds = view.mesh_library().collision_record(key).bounds;
            let width = bounds.footprint.size.x;
            let height = bounds.top - bounds.bottom;
            let span = width / SQRT_TWO;
            let (pose, sources) = corner_poses(transform, width, normal, directions, info.rank());
            let blueprint = blueprints.insert_for(modified, DecoratorStorageAddr::Wall(wall_id), info.state());
            let index = usize::from(info.rank() - 1);
            let color = ColorId(info.u28);
            let semi = info.state() != 0;
            blueprint.visual_entities.push(window_builder(id, pose, FRAME_NAMES[index],
                LINTEL_SHADER, Some(color), semi));
            blueprint.set_cached_transform(CachedDecoratorTransform { rotation: pose.rotation,
                translation: pose.translation, scale: pose.scale });
            // 原版先追加主框，再查询两张真实碰撞表；失败时不能假称整个操作原子成功。
            let collision = view.mesh_library().collision_record(key).mesh.clone();
            let interaction = view.mesh_library().interaction_record(key).mesh.clone();
            blueprint.set_positioned_colliders(collision, interaction, &pose.translation, &pose.rotation);
            let premade = wall.shape().tag() < 2;
            if premade {
                let mut glass = window_builder(id, pose, GLASS_NAMES[index],
                    "nani_solid_vertex_color_window_glass", Some(ColorId(info.window_extra().0)), semi);
                glass.window_light = Some(1000 * u64::from(info.rank()) + u64::from(modified));
                blueprint.visual_entities.push(glass);
            }
            // 原主墙洞与 lintel 墙洞分别追加；前者保留角窗的专用宽度收缩。
            let coord = info.dst_as_wall().wall_space_coord();
            blueprint.wall_holes.push(RawWallHole { center: [coord.u, coord.y],
                dimensions: [(span + span) + f32::from_bits(0xbf01_0625), height], kind: 0 });
            add_lintels(id, info, &sources, semi, color, blueprint, view, 0.0, 1, span);
            if !premade { continue; }
            let has_cloth = info.decorator_id.wrapping_mul(0xaaaa_aab6)
                .wrapping_add(0x5556_884b) <= 0x5555_5555;
            if has_cloth && info.rank() < 3 {
                let mut cloth_pose = pose;
                cloth_pose.translation += pose.rotation * Vec3::new(0.0, 0.0, 0.3);
                let cloth_width = if info.rank() == 1 { span } else { width };
                blueprint.cloth_info = Some(Box::new(ClothInfo { transform: cloth_pose, wall: wall_id,
                    dimensions: [cloth_width, height], decorator_id: id }));
            } else {
                let rng = fastrand::Rng::with_seed(utils::rng::init_rng(info.decorator_id));
                if rng.u32(..) >> 30 != 1 { continue; }
                for (location, transform) in flowerbed_xforms(info.rank(), 3, &pose, &meshes)
                    .into_iter().enumerate() {
                    let key = candidate_hash((id, "flower_bed", location));
                    let mesh_index = rng.usize(0..6);
                    blueprint.window_auto_clutter_candidates.push(WindowAutoClutterCandidate {
                        kind: WindowAutoClutterKind::WindowFlowerbed { mesh_index },
                        rotation: transform.rotation, translation: transform.translation,
                        scale: transform.scale, key, wall: wall_id });
                }
            }
        }
    }
}

#[cfg(test)]
mod original_corner_tests {
    use super::*;
    use country_core::resources::{roofs::geometry::surface::Curve2,
        walls::{WallId, WallState, decorator::{DecoratorDst, WallDecoratorDst},
            publication_source::TerrainHeightGrid512}};

    fn close_vec(actual: Vec3, expected: Vec3) {
        assert!((actual - expected).abs().max_element() < 0.000003,
            "actual={actual:?}, expected={expected:?}");
    }

    #[test]
    fn actual_corner_geometry_keeps_two_sides_native_blend_and_forward_offset() {
        let cached = CachedDecoratorTransform { translation: Vec3::new(10.0, 3.0, -4.0),
            rotation: glam::Quat::from_rotation_x(0.7), scale: Vec3::new(2.0, 3.0, 4.0) };
        let normal = Vec2::new(-1.0, -1.0).normalize();
        let (main, sides) = corner_poses(cached, SQRT_TWO * 2.0, normal,
            [Vec2::X, Vec2::Y], 1);
        close_vec(main.translation, Vec3::new(10.748, 3.0, -3.252));
        close_vec(main.rotation * Vec3::Z, Vec3::new(-std::f32::consts::FRAC_1_SQRT_2,
            0.0, -std::f32::consts::FRAC_1_SQRT_2));
        close_vec(sides[0].translation, Vec3::new(10.7, 3.0, -4.0));
        close_vec(sides[1].translation, Vec3::new(10.0, 3.0, -3.3));
        close_vec(sides[0].rotation * Vec3::Z, Vec3::Z);
        close_vec(sides[1].rotation * Vec3::Z, -Vec3::X);
        assert_eq!(main.scale, Vec3::ONE);
        assert!(sides.iter().all(|side| side.scale == Vec3::ONE));
        let (rank_three_main, rank_three_sides) = corner_poses(cached, SQRT_TWO * 2.0,
            normal, [Vec2::X, Vec2::Y], 3);
        assert_eq!(rank_three_main.translation, main.translation);
        assert_eq!(rank_three_sides.map(|pose| pose.translation), sides.map(|pose| pose.translation));
    }

    fn straight_open_wall() -> PublicWallState {
        let authority = WallState::new(Curve2::try_new(vec![[0.0, 0.0], [8.0, 0.0]], false)
            .unwrap(), 3.0, 0, 0, false);
        PublicWallState::collect(&authority, &TerrainHeightGrid512 {
            encoded: vec![0; 512 * 512].into(), minimum_y: 0.0, maximum_y: 0.0,
            sampling_fix: false }).unwrap()
    }

    fn wall_info(u: f32) -> DecoratorInfo {
        let mut info = DecoratorInfo::default();
        info.set_destination(DecoratorDst::Wall(WallDecoratorDst {
            coord: WallSpaceCoord::new_unchecked(u, 2.0), platform: WallId(17),
            relative_y: 0.0, position: Vec3::ZERO, corner_attachment: 0, bottom_as_door: false }));
        info
    }

    #[test]
    fn actual_open_curve_has_real_skip_at_clamped_endpoint_and_real_interior_directions() {
        let wall = straight_open_wall();
        assert!(corner_directions(&wall, &wall_info(0.0)).is_none());
        let (normal, [left, right]) = corner_directions(&wall, &wall_info(4.0)).unwrap();
        assert_eq!(left, -Vec2::X);
        assert_eq!(right, Vec2::X);
        assert_eq!(normal, Vec2::Y);
    }

    #[test]
    #[should_panic(expected = "农舍角窗 rank 必须为1至3")]
    fn actual_corner_rank_failure_precedes_blueprint_publication() {
        corner_poses(CachedDecoratorTransform { translation: Vec3::ZERO,
            rotation: glam::Quat::IDENTITY, scale: Vec3::ONE }, SQRT_TWO,
            Vec2::Y, [Vec2::X, Vec2::Y], 0);
    }
    #[test]
    fn actual_corner_producer_publishes_same_pose_glass_two_lintels_and_authoritative_cloth() {
        use bevy_ecs::system::RunSystemOnce;
        use super::super::cottage_wall_window::original_window_helper_tests::producer_world;
        let mut world = producer_world(3, 2);
        world.run_system_once(generate_cottage_corner_windows).unwrap();
        let blueprints = world.resource::<DecoratorBlueprints>();
        let blueprint = blueprints.get(7).unwrap();
        assert_eq!(blueprint.visual_entities.len(), 4);
        let frame = &blueprint.visual_entities[0]; let glass = &blueprint.visual_entities[1];
        assert_eq!(frame.mesh_name, "decorators/window_cottage_corner_2x1");
        assert_eq!(glass.mesh_name, "decorators/window_cottage_corner_2x1_glass");
        assert_eq!(frame.translation, glass.translation);
        assert_eq!(glass.window_light, Some(2007));
        assert_eq!(glass.color_id, Some(ColorId(0x1020_3040)));
        assert_eq!(frame.color_id, Some(ColorId(0x5566_7788)));
        assert_eq!(frame.scale, Vec3::ONE);
        assert_eq!(blueprint.wall_holes.len(), 2);
        assert!((blueprint.wall_holes[0].dimensions[0] - 5.1528544).abs() < 0.000003);
        assert_eq!(blueprint.storage_addr, Some(DecoratorStorageAddr::Wall(WallId(42))));
        let cloth = blueprint.cloth_info.as_ref().unwrap();
        assert_eq!(cloth.wall, WallId(42)); assert_eq!(cloth.decorator_id, DecoratorId(7));
        assert_eq!(cloth.dimensions, [4.0, 2.0]);
        close_vec(cloth.transform.translation - frame.translation,
            frame.rotation * Vec3::new(0.0, 0.0, 0.3));
    }

    #[test]
    fn actual_corner_missing_interaction_keeps_already_written_frame_and_outline() {
        use bevy_ecs::system::RunSystemOnce;
        use country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary;
        use super::super::cottage_wall_window::original_window_helper_tests::producer_world;
        let mut world = producer_world(3, 2);
        world.resource_mut::<DecoratorMeshLibrary>().interaction.clear();
        let failure = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            world.run_system_once(generate_cottage_corner_windows).unwrap();
        }));
        assert!(failure.is_err());
        let blueprints = world.resource::<DecoratorBlueprints>();
        let blueprint = blueprints.get(7).unwrap();
        assert_eq!(blueprint.visual_entities.len(), 1);
        assert!(blueprint.transform.is_some());
        assert!(blueprint.wall_holes.is_empty());
    }

    #[test]
    fn actual_corner_invalid_rank_fails_before_insert_when_the_required_key_exists() {
        use bevy_ecs::system::RunSystemOnce;
        use super::super::cottage_wall_window::original_window_helper_tests::producer_world;
        let mut world = producer_world(3, 0);
        let failure = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            world.run_system_once(generate_cottage_corner_windows).unwrap();
        }));
        assert!(failure.is_err());
        assert!(world.resource::<DecoratorBlueprints>().get(7).is_none());
    }

}
