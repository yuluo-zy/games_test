//! B `ui_systems/decorator_interaction_intent.rs` — 装饰交互意图计算。
//!
//! - `calculate_decorator_interaction_intent` 0x14127cf70（原完整8546B；当前主body仍未恢复）：按
//!   目的 tag 分派（switch @+0x04）——case 0 墙：`PublicWalls::get`
//!   （0x140a91ef0）+ `derive_decorator_subtype`（0x140813d00）+ 部件位置
//!   （0x14125cf90）+ 合并位置（0x14126a040/0x142001880）+ 四角吸附
//!   （0x140c996d0(0..3)，`should_offer_snap_to_corner` 0x1408331c0 门控）+
//!   阳台门吸附（`snap_balcony_door` 0x14127f0e0）；case 1 屋顶：
//!   `can_place_on_roof` 门控；输出 0x98B 意图记录（字段 I，见
//!   [`DecoratorInteractionIntent`]）。
//! - `snap_balcony_door` B14127f0e0：type0x38/39 的五态真实support/flat-roof/
//!   finite-ray搜索；返回payload由真实caller按各自门处理。本轮仅恢复此依赖，
//!   `calculate_decorator_interaction_intent` 的旧近似输出不代表已恢复的UI入口。
use bevy_ecs::system::Query;
use country_core::resources::roofs::Roof;
use country_core::resources::walls::WallId;
use country_core::resources::walls::decorator::DecoratorInfo;
use country_core::resources::walls::public_wall_state::PublicWallState;
use country_core::systems::collision::world::RaycastWorld;
use country_core::resources::walls::decorator_subtype::{
    DecoratorLibKey, DecoratorMeshLibrary, DerivedDecoratorInfo,
};
use glam::{Vec2, Vec3};

use crate::merge_proposal_wall_decorators::find_best_wall_decorator_merge_position;

/// 0x98B 意图记录（I：字段语义按 0x14127cf70 输出写面；@0 u64 判别、
/// @0x10 位置簇、@0x28 墙信息、@0x40+ 吸附/合并结果）。
#[derive(Clone, Copy, Debug, Default)]
pub struct DecoratorInteractionIntent {
    /// @0：目的（param_12 观察；I）。
    pub dst_tag: u8,
    /// @0x10：墙空间位置（u64 对打包；I）。
    pub wall_pos: (u32, u32),
    /// @0x18：高度（I）。
    pub height: f32,
    /// @0x28..0x38：墙信息（I）。
    pub wall_info: [u64; 2],
    /// @0x40：有效标志（=1；I）。
    pub valid: u64,
    /// @0x50：世界位置（I）。
    pub pos: Vec3,
    /// @0x60：结果/合并（I）。
    pub result: u64,
    /// @0x70：u8（I）。
    pub sub: u8,
    /// @0x39..0x3a：标志位（I）。
    pub flags: [u8; 2],
    /// @0x80：f32（I）。
    pub f80: f32,
    /// @0x88：u64（I）。
    pub u88: u64,
    /// @0x98：f32（I）。
    pub f98: f32,
    /// @0xbc：rank（I）。
    pub rank: u8,
}

/// B 0x14127cf70（O 结构）。命中 + 装饰信息 → 交互意图。
/// 精确几何（四角吸附/合并/阳台门）I；输出记录字段见上。
pub fn calculate_decorator_interaction_intent(
    _hit: &crate::ui_systems::calculate_decorator_grab_offset::RaycastIntersection,
    info: &DecoratorInfo,
    subtype: &DerivedDecoratorInfo,
    wall: Option<&country_core::resources::walls::public_wall_state::PublicWallState>,
    storage: &country_core::resources::walls::decorator_storage::DecoratorStorage,
    walls: &country_core::resources::walls::public_walls::PublicWalls,
    cursor_ws: Vec3,
) -> DecoratorInteractionIntent {
    let mut out = DecoratorInteractionIntent::default();
    out.dst_tag = info.dst_tag();
    out.pos = cursor_ws;
    match info.dst_tag() {
        // case 0：墙（O：PublicWalls::get + subtype + 部件位置 + 合并 + 四角吸附）。
        0 => {
            let Some(wall) = wall else {
                return out;
            };
            let _ = subtype;
            // 合并位置（0x14126a040/0x142001880 族；I；D3 签名
            // `(&[WallId], &DecoratorInfo, &DecoratorStorage, &PublicWalls)`）。
            let _merge = find_best_wall_decorator_merge_position(&[], info, storage, walls);
            // 四角吸附（O：0x140c996d0(0..3) + should_offer_snap_to_corner）。
            if info.decorator_type().should_offer_snap_to_corner() {
                for corner in 0..4u8 {
                    let _ = corner_snap_candidate(corner, wall, cursor_ws);
                }
            }
            out.valid = 1;
        }
        // case 1：屋顶（O：can_place_on_roof 门控）。
        1 => {
            if !info.decorator_type().can_place_on_roof() {
                return out;
            }
            out.valid = 1;
        }
        // case 2/3/4：地形/楼梯（O 结构；位置由宿主投影）。
        2 | 3 | 4 => {
            out.valid = 1;
        }
        _ => {}
    }
    out
}

/// 四角吸附候选（I：0x140c996d0(corner) + 圆/矩形包含测试；几何 U）。
#[allow(clippy::unnecessary_wraps)]
fn corner_snap_candidate(
    corner: u8,
    _wall: &country_core::resources::walls::public_wall_state::PublicWallState,
    _cursor_ws: Vec3,
) -> Option<u8> {
    let _ = corner;
    None
}

/// Observed B Snap result; semantic Rust values carry no native ABI promise.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BalconyDoorSnap {
    AdjacentFlatRoof { wall: WallId, height: f32 },
    WallOutline { wall: WallId, height: f32 },
    /// The selected original ray result has source tag13; its name is unknown.
    ColliderSource13 { wall: WallId, height: f32 },
    Unavailable,
    RejectedUpperClearance,
}

impl BalconyDoorSnap {
    pub fn native_tag(self) -> u8 {
        match self {
            Self::AdjacentFlatRoof { .. } => 0,
            Self::WallOutline { .. } => 1,
            Self::ColliderSource13 { .. } => 2,
            Self::Unavailable => 3,
            Self::RejectedUpperClearance => 4,
        }
    }
}

/// B14127f0e0: preserve the five states, actual footprint/support sources and
/// finite source13 ray. The caller supplies the existing direction unchanged.
/// Current UI calculate is separately unrestored; location uses true/true and
/// observes only RejectedUpperClearance before continuing its other checks.
#[allow(clippy::too_many_arguments)]
pub fn snap_balcony_door(
    require_upper_clearance: bool,
    info: &DecoratorInfo,
    world_pos: Vec3,
    probe_direction_xz: Vec2,
    wall_distance: f32,
    wall_id: WallId,
    wall: &PublicWallState,
    roofs: &Query<&Roof>,
    meshes: &DecoratorMeshLibrary,
    prefer_wall_outline: bool,
    world: &RaycastWorld,
) -> BalconyDoorSnap {
    let (subtype, baseline_subtype) = match info.decorator_type().0 {
        0x38 => (4, 0),
        0x39 => (9, 6),
        _ => return BalconyDoorSnap::Unavailable,
    };
    // Both lookups precede all geometry/query work; missing original data is
    // an invariant failure, rather than a default box or unavailable result.
    let full_height = snap_collision_height(meshes, DecoratorLibKey::new(subtype, info.rank()));
    let baseline_height =
        snap_collision_height(meshes, DecoratorLibKey::new(baseline_subtype, info.rank()));
    let half_height = full_height * 0.5;
    let world_xz = Vec2::new(world_pos.x, world_pos.z);
    let probe = world_xz + probe_direction_xz * 0.2;
    // Public+280 comes from raw source+58, retained in this public snapshot.
    let elevation = wall.authority.elevation();
    let lower = snap_max_number(elevation - 0.4, world_pos.y - half_height - 0.4);
    let mut selected = None;
    for roof in roofs.iter() {
        if !roof.is_flat() || roof.wall_id() == wall_id {
            continue;
        }
        let support_y = roof.floor_y() - 0.79;
        if !(world_pos.y > support_y && support_y >= lower) {
            continue;
        }
        if !snap_roof_contains(roof, world_xz, probe) {
            continue;
        }
        selected = Some(BalconyDoorSnap::AdjacentFlatRoof {
            wall: roof.wall_id(),
            height: snap_max_number(elevation, support_y) + half_height,
        });
        break;
    }
    let selected = match selected {
        Some(selected) => selected,
        None => {
            let outline = wall.height_outline();
            let (i, t) = snap_curve_coord(outline, snap_nonnegative(wall_distance));
            let outline_y = snap_nonnegative(outline[i][1] + (outline[i + 1][1] - outline[i][1]) * t);
            let ray_base_y = if prefer_wall_outline {
                world_pos.y - half_height
            } else {
                world_pos.y + baseline_height * -0.5
            };
            let threshold = if prefer_wall_outline { ray_base_y - 0.1 } else { ray_base_y };
            if outline_y > threshold {
                BalconyDoorSnap::WallOutline {
                    wall: wall_id,
                    height: snap_max_number(half_height, outline_y + half_height + 0.05),
                }
            } else {
                let ray_xz = world_xz + probe_direction_xz * 0.5;
                let hit = world.raycast_simple_with_max_toi(
                    Vec3::new(ray_xz.x, ray_base_y + 0.5, ray_xz.y),
                    Vec3::new(-0.0, -1.0, -0.0),
                    1.0,
                    0x1000,
                );
                if hit.handle[0] as u8 != 13 {
                    return BalconyDoorSnap::Unavailable;
                }
                BalconyDoorSnap::ColliderSource13 {
                    wall: wall_id,
                    height: snap_max_number(half_height, hit.position.y + half_height + 0.05),
                }
            }
        }
    };
    if !require_upper_clearance {
        return selected;
    }
    let (i, t) = snap_curve_coord(wall.height_outline(), snap_nonnegative(wall_distance));
    let heights = wall.arch_segment_heights().expect("wall upper height cache is missing");
    // Original checks only the two selected indices, not whole-vector equality.
    let ceiling = heights[i] + (heights[i + 1] - heights[i]) * t;
    let mut roof_margin = 0.0;
    // A fresh query traversal is required even after selecting a foreign roof.
    for roof in roofs.iter() {
        if roof.is_flat() && roof.wall_id() == wall_id {
            roof_margin = -0.79;
        }
    }
    if world_pos.y + half_height >= ceiling + roof_margin {
        BalconyDoorSnap::RejectedUpperClearance
    } else {
        selected
    }
}

fn snap_collision_height(meshes: &DecoratorMeshLibrary, key: DecoratorLibKey) -> f32 {
    let (minimum, maximum) = meshes.get_collision_bbx_ls(key)
        .unwrap_or_else(|| panic!("The decorator mesh library doesn't contain {key:?}"));
    maximum.y - minimum.y
}

fn snap_roof_contains(roof: &Roof, point: Vec2, probe: Vec2) -> bool {
    let bytes = roof.observed();
    let f = |at| f32::from_le_bytes(bytes[at..at + 4].try_into().unwrap());
    if bytes[8] & 1 != 0 {
        // Borrow the actual footprint. Rectangle::new would add unrelated
        // validation that the original contains_point_padding call lacks.
        let footprint = utils::geometry::Rectangle2d {
            axis: Vec2::new(f(12), f(16)),
            center: Vec2::new(f(20), f(24)),
            size: Vec2::new(f(28), f(32)),
        };
        footprint.contains_point_padding(point, Vec2::splat(0.8))
            && footprint.contains_point_padding(probe, Vec2::splat(0.8))
    } else {
        let footprint = utils::geometry::Circle2d::new(Vec2::new(f(12), f(16)), f(20));
        footprint.signed_distance(point) < 0.4 && footprint.signed_distance(probe) < 0.0
    }
}

// MAXSS's right operand wins ties/unordered; the native NaN select keeps the
// numeric left operand when the right one isNaN.
fn snap_max_number(left: f32, right: f32) -> f32 {
    if right.is_nan() || left > right { left } else { right }
}
fn snap_nonnegative(value: f32) -> f32 {
    if value > 0.0 { value } else { 0.0 }
}

// The original921B CurveSlice specialization over projected [[f32;3]] input.
// Keep its f32 residual/endpoint order, without allocating Vec<Vec3> or casting
// array storage. The same borrowed publication feeds both samplings.
fn snap_curve_coord(points: &[[f32; 3]], distance: f32) -> (usize, f32) {
    assert!(!points.is_empty(), "original curve is empty");
    points.get(1).expect("original curve needs a second point");
    assert!(distance >= 0.0, "invalid extension distance");
    let length = |i: usize| {
        let x = points[i + 1][0] - points[i][0];
        let z = points[i + 1][2] - points[i][2];
        (z * z + x * x).sqrt()
    };
    let mut i = 0;
    let mut segment_length = length(i);
    let mut balance = segment_length - distance;
    loop {
        if balance >= 0.0 {
            return (i, 1.0 - balance / segment_length);
        }
        if i + 2 >= points.len() {
            return (i, 1.0);
        }
        i += 1;
        segment_length = length(i);
        balance += segment_length;
    }
}

#[cfg(test)]
mod snap_balcony_door_regressions {
    use super::*;
    use bevy_ecs::world::World;
    use country_core::resources::roofs::geometry::surface::Curve2;
    use country_core::resources::walls::WallState;
    use country_core::resources::walls::decorator_subtype::DecoratorSubtype;
    use country_core::resources::walls::height_cache::RegularWallHeightCache;
    use country_core::resources::walls::publication_fields::WallShapeSnapshot;
    use country_core::resources::walls::publication_source::TerrainHeightGrid512;
    use country_core::systems::collision::world::{
        ColliderMeshInstance, ColliderUpsert, RaycastGeoSource, RaycastMeshData, upsert_collider,
    };
    use std::collections::HashMap;
    use std::sync::Arc;

    // Actual same-World Bevy Roof query, mesh resource and real Rapier world.
    // Populated fixture data is not the missing game mesh-loader integration.
    fn scene() -> World {
        let mut ecs = World::new();
        let mut meshes = DecoratorMeshLibrary::default();
        for (subtype, height) in [(4, 2.0), (0, 4.0), (9, 4.0), (6, 6.0)] {
            meshes.insert_collision_and_interaction_as_same(
                DecoratorLibKey::new(subtype, 1),
                Arc::new(DerivedDecoratorInfo::new(
                    DecoratorSubtype(subtype), Vec3::ZERO, Vec3::new(1.0, height, 1.0))),
            );
        }
        ecs.insert_resource(meshes);
        ecs.insert_resource(RaycastWorld::new());
        ecs
    }
    fn info(dtype: u8) -> DecoratorInfo {
        DecoratorInfo { ty: dtype, rank: 1, ..Default::default() }
    }
    fn published_wall(elevation: f32, ceiling: f32) -> PublicWallState {
        let points = (0..=16).map(|i| {
            let angle = i as f32 * std::f32::consts::TAU / 16.0;
            [-4.0 + 4.0 * angle.cos(), 4.0 * angle.sin()]
        }).collect();
        let authority = WallState::new(Curve2::try_new(points, false).unwrap(), 3.0, 0, 2, false);
        let (mut snapshot, freehand) = authority.into_snapshot();
        let mut shape = [0u8; 28];
        shape[4..8].copy_from_slice(&(-4.0f32).to_le_bytes());
        shape[12..16].copy_from_slice(&4.0f32.to_le_bytes());
        snapshot.source.shape = WallShapeSnapshot::from_observed(shape);
        snapshot.source.terrain_floor = elevation;
        let authority = WallState::from_snapshot(snapshot, freehand);
        let terrain = TerrainHeightGrid512 {
            encoded: vec![0; 512 * 512].into(), minimum_y: 0.0, maximum_y: 0.0,
            sampling_fix: false,
        };
        let mut wall = PublicWallState::collect(&authority, &terrain).unwrap();
        wall.regular_heights = Some(RegularWallHeightCache {
            maximum_y: ceiling,
            arch_segment_heights: vec![ceiling; wall.height_outline().len()],
        });
        wall
    }
    fn roof(wall: u64, floor: f32, rectangle: bool) -> Roof {
        let mut shape = [0u8; 28];
        if rectangle {
            shape[..4].copy_from_slice(&1u32.to_le_bytes());
            shape[4..8].copy_from_slice(&1.0f32.to_le_bytes());
            shape[20..24].copy_from_slice(&1.0f32.to_le_bytes());
            shape[24..28].copy_from_slice(&1.0f32.to_le_bytes());
        } else {
            shape[12..16].copy_from_slice(&1.0f32.to_le_bytes());
        }
        Roof::new(wall.to_le_bytes(), [0; 28], shape, floor.to_le_bytes(), [0; 4], 0, [0; 12])
    }
    fn invoke(
        ecs: &mut World, wall: &PublicWallState, info: &DecoratorInfo,
        pos: Vec3, upper: bool, prefer_outline: bool,
    ) -> BalconyDoorSnap {
        let mut state = ecs.query::<&Roof>();
        let roofs = state.query(&*ecs);
        snap_balcony_door(upper, info, pos, Vec2::X, 0.0, WallId(1), wall, &roofs,
            ecs.resource::<DecoratorMeshLibrary>(), prefer_outline, ecs.resource::<RaycastWorld>())
    }
    fn insert_surface(ecs: &mut World, height: f32) -> HashMap<u32, ColliderMeshInstance> {
        let mut instances = HashMap::new();
        let mut physics = ecs.resource_mut::<RaycastWorld>();
        assert!(matches!(upsert_collider(&mut physics, &mut instances, 1,
            RaycastGeoSource::Slot13,
            Arc::new(RaycastMeshData::new_cuboid_at(Vec3::splat(0.2), Vec3::ZERO)),
            Vec3::new(0.5, height, 0.0), glam::Quat::IDENTITY), ColliderUpsert::Inserted { .. }));
        physics.flush_colliders();
        instances
    }

    #[test]
    fn unsupported_type_does_not_consult_missing_keys() {
        let mut ecs = scene();
        ecs.resource_mut::<DecoratorMeshLibrary>().collision.clear();
        let out = invoke(&mut ecs, &published_wall(0.0, 10.0), &info(0), Vec3::Y * 3.0, true, true);
        assert_eq!(out, BalconyDoorSnap::Unavailable);
    }

    #[test]
    fn dtype_rank_and_second_bbox_lookup_are_strict() {
        let mut ecs = scene();
        ecs.spawn(roof(2, 3.0, false));
        let wall = published_wall(0.0, 10.0);
        let a = invoke(&mut ecs, &wall, &info(0x38), Vec3::Y * 3.0, false, true);
        let b = invoke(&mut ecs, &wall, &info(0x39), Vec3::Y * 3.0, false, true);
        let BalconyDoorSnap::AdjacentFlatRoof { height: a, .. } = a else { panic!("cottage"); };
        let BalconyDoorSnap::AdjacentFlatRoof { height: b, .. } = b else { panic!("gothic"); };
        assert!((b - a - 1.0).abs() < 1e-6);
        let mut rank2 = info(0x38);
        rank2.rank = 2;
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            invoke(&mut ecs, &wall, &rank2, Vec3::Y * 3.0, false, true)
        })).is_err());
        ecs.resource_mut::<DecoratorMeshLibrary>().collision.remove(&DecoratorLibKey::new(0, 1));
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            invoke(&mut ecs, &wall, &info(0x38), Vec3::Y * 3.0, false, true)
        })).is_err());
    }

    #[test]
    fn first_other_flat_roof_preserves_id_and_elevation_not_max_y() {
        let mut ecs = scene();
        ecs.spawn(roof(1, 3.0, false));
        let mut pitched = roof(9, 3.0, false);
        pitched.observed_mut()[0x30..0x34].copy_from_slice(&0.2f32.to_le_bytes());
        ecs.spawn(pitched);
        ecs.spawn(roof(2, 3.0, false));
        ecs.spawn(roof(3, 3.2, false));
        let wall = published_wall(0.0, 20.0);
        let out = invoke(&mut ecs, &wall, &info(0x38), Vec3::Y * 3.0, false, true);
        assert_eq!(out, BalconyDoorSnap::AdjacentFlatRoof { wall: WallId(2), height: 3.0 - 0.79 + 1.0 });
    }

    #[test]
    fn rectangle_padding_is_inclusive_and_circle_probe_is_strict() {
        let rectangle = roof(2, 3.0, true);
        assert!(snap_roof_contains(&rectangle, Vec2::new(0.9, 0.0), Vec2::new(0.9, 0.0)));
        assert!(!snap_roof_contains(&rectangle, Vec2::new(0.9001, 0.0), Vec2::ZERO));
        let circle = roof(2, 3.0, false);
        assert!(snap_roof_contains(&circle, Vec2::new(1.25, 0.0), Vec2::new(0.9, 0.0)));
        assert!(!snap_roof_contains(&circle, Vec2::new(1.5, 0.0), Vec2::ZERO));
        assert!(!snap_roof_contains(&circle, Vec2::new(0.8, 0.0), Vec2::X));
    }

    #[test]
    fn upper_compare_uses_input_y_halfheight_and_fresh_same_wall_roof_query() {
        let mut ecs = scene();
        let mut current_roof = roof(1, 50.0, false);
        current_roof.observed_mut()[0x30..0x34].copy_from_slice(&0.2f32.to_le_bytes());
        let current_roof = ecs.spawn(current_roof).id();
        ecs.spawn(roof(2, 3.0, false));
        let mut wall = published_wall(0.0, 4.125);
        let expected = BalconyDoorSnap::AdjacentFlatRoof { wall: WallId(2), height: 3.0 - 0.79 + 1.0 };
        assert_eq!(invoke(&mut ecs, &wall, &info(0x38), Vec3::Y * 3.0, true, true), expected);
        wall.regular_heights.as_mut().unwrap().arch_segment_heights.fill(4.0);
        assert_eq!(invoke(&mut ecs, &wall, &info(0x38), Vec3::Y * 3.0, true, true),
            BalconyDoorSnap::RejectedUpperClearance);
        wall.regular_heights.as_mut().unwrap().arch_segment_heights.fill(4.5);
        assert_eq!(invoke(&mut ecs, &wall, &info(0x38), Vec3::Y * 3.0, true, true), expected);
        ecs.get_mut::<Roof>(current_roof).unwrap().observed_mut()[0x30..0x34]
            .copy_from_slice(&0.0f32.to_le_bytes());
        assert_eq!(invoke(&mut ecs, &wall, &info(0x38), Vec3::Y * 3.0, true, true),
            BalconyDoorSnap::RejectedUpperClearance);
        wall.regular_heights = None;
        assert_eq!(invoke(&mut ecs, &wall, &info(0x38), Vec3::Y * 3.0, false, true), expected);
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            invoke(&mut ecs, &wall, &info(0x38), Vec3::Y * 3.0, true, true)
        })).is_err());
    }

    #[test]
    fn baseline_bbox_and_preference_select_different_real_branches() {
        let mut ecs = scene();
        let wall = published_wall(0.0, 10.0);
        assert_eq!(invoke(&mut ecs, &wall, &info(0x38), Vec3::Y * 1.5, false, false),
            BalconyDoorSnap::WallOutline { wall: WallId(1), height: 1.05 });
        assert_eq!(invoke(&mut ecs, &wall, &info(0x38), Vec3::Y * 1.5, false, true),
            BalconyDoorSnap::Unavailable);
    }

    #[test]
    fn real_source13_pose_is_bounded_to_one_unit() {
        let mut ecs = scene();
        let _instances = insert_surface(&mut ecs, 1.75);
        let wall = published_wall(0.0, 10.0);
        let out = invoke(&mut ecs, &wall, &info(0x38), Vec3::Y * 3.0, false, true);
        let BalconyDoorSnap::ColliderSource13 { wall, height } = out else { panic!("actual source13"); };
        assert_eq!(wall, WallId(1));
        assert!((height - 2.9).abs() < 1e-5);
        let mut beyond = scene();
        let _instances = insert_surface(&mut beyond, 0.25);
        let wall = published_wall(0.0, 10.0);
        assert!(!beyond.resource::<RaycastWorld>()
            .raycast_simple(Vec3::new(0.5, 2.5, 0.0), Vec3::NEG_Y, 0x1000).is_miss());
        assert_eq!(invoke(&mut beyond, &wall, &info(0x38), Vec3::Y * 3.0, false, true),
            BalconyDoorSnap::Unavailable);
    }

    #[test]
    fn projected_distance_keeps_f32_endpoint_zero_and_overrun_policy() {
        let points = [[0.0, 0.0, 0.0], [3.0, 100.0, 4.0], [6.0, 200.0, 8.0]];
        assert_eq!(snap_curve_coord(&points, 2.5), (0, 0.5));
        assert_eq!(snap_curve_coord(&points, 5.0), (0, 1.0));
        assert_eq!(snap_curve_coord(&points, 50.0), (1, 1.0));
        assert_eq!(snap_curve_coord(&points, snap_nonnegative(f32::NAN)), (0, 0.0));
        assert_eq!(snap_curve_coord(&points, snap_nonnegative(-2.0)), (0, 0.0));
        let zero = [[0.0, 0.0, 0.0], [0.0, 4.0, 0.0], [1.0, 5.0, 0.0]];
        let (i, t) = snap_curve_coord(&zero, 0.0);
        assert_eq!(i, 0);
        assert!(t.is_nan());
        assert_eq!(snap_curve_coord(&zero, 0.5), (1, 0.5));
        for count in [0, 1] {
            assert!(std::panic::catch_unwind(|| snap_curve_coord(&points[..count], 0.0)).is_err());
        }
        assert_eq!(snap_max_number(-0.0, 0.0).to_bits(), 0.0f32.to_bits());
        assert_eq!(snap_max_number(2.0, f32::NAN), 2.0);
    }
}
