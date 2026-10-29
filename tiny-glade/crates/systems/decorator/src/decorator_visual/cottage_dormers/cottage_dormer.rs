//! B `cottage_dormers/cottage_dormer.rs` — 农舍老虎窗装饰（D1）。
//!
//! 证据：fresh RE-MCP 反编译 0x1407c7ac0（generate_cottage_dormers，55KB）/
//! 0x1407ca6c0（is_out_of_roof_bounds，7KB）+ 骨架 callees（O）：
//! cottage_dormer_dims（内联）、get_dormer_panel_xforms（内联）、
//! DormerHoleSsbo::cottage、Roof::get_roof_shape_from_pos_ws_clamped
//! （0x1408e4750）、DecoratorExtraInfo::as_window、Rectangle2d::perimeter ×2、
//! blueprint 链、WindowVisualEntityBuilder 链。
//!
//! is_out_of_roof_bounds uses the complete matched B1561B body, actual cached
//! transform and Roof sections。两类原入口只发布canonical蓝图；此入口保留
//! 原rank面板曲线、动态凸体缓存、窗框/玻璃/孔与花台候选。独立瓦片装配仍由下游owner恢复。

use bevy_ecs::prelude::*;
use country_core::resources::walls::decorator::DecoratorInfo;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::roofs::geometry::surface::RoofSection;
use country_core::resources::roofs::roof_shape::Roof;
use country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransform;
use country_core::resources::walls::decorator_subtype::{
    DerivedDecoratorSubtypes, DerivedDecoratorInfoResources,
};
use country_core::resources::walls::public_walls::PublicWalls;
use glam::{Vec2, Vec3};
use utils::geometry::Rectangle2d;

use crate::blueprint::DecoratorBlueprints;
#[cfg(test)]
use crate::decorator_visual_state::DecoratorTransientVisualState;
use country_core::{resources::{color_id::ColorId, walls::{decorator::DecoratorId,
    dormers::DormerHoleSsbo, decorator_storage::DecoratorStorageAddr,
    cached_decorator_transforms::CachedDecoratorTransforms}},
    systems::{ui::signifiers::UiSignifierStream, collision::{WindowAutoClutterCandidate, WindowAutoClutterKind}},
    utils::components::transform::Transform};
use crate::decorator_visual::cottage_wall_window::{window_builder, candidate_hash};

/// B1407ca6c0/1561B: true means out of the actual roof bounds.
/// The original stream argument is unused by this complete leaf.
pub fn is_out_of_roof_bounds(
    transform: &CachedDecoratorTransform,
    width: f32,
    height: f32,
    rank: u8,
    roof: &Roof,
) -> bool {
    // Native calls height before checking rank. Rank0 and ranks4+ are invalid;
    // table index0 is not a legal fallback mode.
    let roof_height = roof.height();
    assert!((1..=3).contains(&rank), "is_out_of_roof_bounds: rank out of range");
    let factor = [0.1, 0.15, 0.2][usize::from(rank - 1)];
    let position = transform.translation;
    let h = height * 0.5 + 0.1;
    let h = h + position.y;
    let top = roof_height * factor + h;
    let circle = u32::from_le_bytes(roof.observed()[8..12].try_into().unwrap()) == 0;
    let limit = (roof.height() + roof.floor_y()) - if circle { 0.5 } else { 0.1 };
    // Native setbe includes unordered values; do not replace with top<=limit.
    let height_permits = !(top > limit);
    if circle {
        return !height_permits;
    }
    if !height_permits {
        return true;
    }
    let half_width = width * 0.5 + 0.1;
    if !section_avoids_corners(roof, position, half_width) {
        return true;
    }
    !section_avoids_corners(roof, Vec3::new(position.x, top, position.z), half_width)
}

/// The original lower and upper sections use the same perimeter test, each
/// with that section's real size. The upper section is evaluated only if the
/// lower section permits it.
fn section_avoids_corners(roof: &Roof, position: Vec3, half_width: f32) -> bool {
    let rectangle = match roof.get_roof_shape_from_pos_ws_clamped(position) {
        RoofSection::Rectangle { frame, dimensions } => Rectangle2d {
            axis: Vec2::new(frame[0], frame[1]),
            center: Vec2::new(frame[2], frame[3]),
            size: Vec2::from_array(dimensions),
        },
        RoofSection::Circle { .. } => panic!("rectangle roof bounds expected a rectangle section"),
    };
    let center_u = rectangle.get_closest_u_from_pos(Vec2::new(position.x, position.z));
    let half_interval = half_width / rectangle.perimeter();
    let wrap = |value: f32| {
        let u = value % 1.0;
        if u < 0.0 { u + 1.0 } else { u }
    };
    let lower = wrap(center_u - half_interval);
    let upper = wrap(center_u + half_interval);
    if upper >= lower {
        for corner in 0..4 {
            let u = rectangle.get_point_u(corner);
            if u >= lower && upper >= u {
                return false;
            }
        }
    } else {
        let upper = upper + 1.0;
        for corner in 0..4 {
            let u = rectangle.get_point_u(corner);
            if (upper >= u && u >= lower) || (u + 1.0 >= lower && upper >= u + 1.0) {
                return false;
            }
        }
    }
    true
}

/// 农舍老虎窗子类型（O：0x1407c7ac0 行 402 `subtype != 0x02 → skip`）。
pub const COTTAGE_DORMER_SUBTYPE: u8 = 0x02;

/// 原7c81d3..81eb的rank1..3宽度表；高度固定为1，非法rank先失败。
pub fn cottage_dormer_dims(info: &DecoratorInfo) -> Option<(f32, f32)> {
    let rank = info.rank();
    assert!(
        (1..=3).contains(&rank),
        "cottage_dormer_dims: rank out of range"
    );
    Some(([0.8, 1.5, 2.2][usize::from(rank - 1)], 1.0))
}

/// 原1407c82ec..8f04：rank1原姿态；rank2/3沿当前屋顶截面的真实曲线分割。
/// 先按原宽/周长求坐标，再从折线切向求正面，不能用中心线性平移替代。
pub fn get_dormer_panel_xforms(transform: &CachedDecoratorTransform, rank: u8,
    width: f32, roof: &Roof) -> Vec<Transform> {
    let shape = super::collision_geo::roof_section(roof, transform.translation);
    panel_xforms_in_section(transform, rank, width, shape)
}

fn panel_xforms_in_section(transform: &CachedDecoratorTransform, rank: u8, width: f32,
    shape: country_core::systems::wall::valid_enclosure::ShapeParameters) -> Vec<Transform> {
    let pose = Transform { rotation: transform.rotation, translation: transform.translation,
        scale: transform.scale };
    let center = shape.get_closest_u_from_pos(Vec2::new(transform.translation.x, transform.translation.z));
    if rank == 1 { return vec![pose]; }
    assert!(rank == 2 || rank == 3, "internal error: entered unreachable code");
    let extent = if rank == 2 { (width / shape.perimeter()) * 0.25 }
        else { (width / shape.perimeter()) / 3.0 };
    let wrap = |u: f32| { let u = u % 1.0; if u < 0.0 { u + 1.0 } else { u } };
    let lower = wrap(center - extent); let upper = wrap(extent + center);
    let curve = shape.as_curve3(transform.translation.y);
    let panel = |u: f32| {
        let coord = curve.get_coord_at_u(u);
        let tangent = curve.points[coord.segment + 1] - curve.points[coord.segment];
        let inverse = 1.0 / ((tangent.y * tangent.y + tangent.x * tangent.x) + tangent.z * tangent.z).sqrt();
        let front = Vec3::new(-tangent.z * inverse, 0.0, tangent.x * inverse);
        let inverse = 1.0 / ((front.y * front.y + front.x * front.x) + front.z * front.z).sqrt();
        Transform { rotation: utils::xz_projective_rotation_bad(front * inverse),
            translation: curve.get_pos_at_coord(coord), scale: Vec3::ONE }
    };
    if rank == 2 { vec![panel(lower), panel(upper)] }
    else { vec![panel(lower), Transform { rotation: transform.rotation,
        translation: curve.get_pos_at_u(center), scale: Vec3::ONE }, panel(upper)] }
}

/// 原8组SystemParam；动态凸体使用唯一Local缓存，视觉/孔交给唯一蓝图消费者。
pub fn generate_cottage_dormers(
    derived: DerivedDecoratorInfoResources,
    storage: Res<DecoratorStorage>, cached: Res<CachedDecoratorTransforms>,
    walls: Res<PublicWalls>, roofs: Query<&Roof>,
    mut blueprints: ResMut<DecoratorBlueprints>,
    mut collision_cache: Local<super::collision_geo::DormerCollisionCache>,
    _signifiers: Res<UiSignifierStream>,
) {
    let view = derived.as_view();
    for (wall_id, decorators) in storage.iter_walls() {
        let Some(roof_entity) = walls.roof_entity(wall_id) else { continue; };
        let Ok(roof) = roofs.get(roof_entity) else { continue; };
        if roof.is_flat() { continue; }
        for (modified, info) in decorators.iter_modified() {
            let id = DecoratorId(modified);
            if view.subtype(id).0 != COTTAGE_DORMER_SUBTYPE { continue; }
            let transform = cached.transform(modified);
            let (width, height) = cottage_dormer_dims(info).expect("原rank尺寸表");
            let blueprint = blueprints.insert_for(modified, DecoratorStorageAddr::Wall(wall_id), info.state());
            let mesh = super::collision_geo::get_collision_mesh(&transform, width, height, info.rank(),
                0.2, roof, &mut collision_cache);
            blueprint.set_positioned_colliders(mesh.clone(), mesh, &transform.translation, &transform.rotation);
            blueprint.set_cached_transform(transform);
            let panels = get_dormer_panel_xforms(&transform, info.rank(), width, roof);
            let rng = fastrand::Rng::with_seed(utils::rng::init_rng(info.decorator_id));
            let suppressed = info.decorator_id.wrapping_mul(0xaaaa_aab6).wrapping_add(0x5556_884b)
                <= 0x5555_5555 && info.rank() < 3;
            let flowerbed = !suppressed && rng.u32(..) >> 30 == 1;
            for (index, panel) in panels.into_iter().enumerate() {
                let semi = info.state() != 0;
                blueprint.visual_entities.push(window_builder(id, panel,
                    "decorators/window_cottage_1x1_dormer_frame", "nani_solid_vertex_color_window",
                    Some(ColorId(info.u28)), semi));
                let mut glass = window_builder(id, panel, "decorators/window_cottage_1x1_dormer_glass",
                    "nani_solid_vertex_color_window_glass", Some(ColorId(info.window_extra().0)), semi);
                glass.window_light = Some(u64::from(info.rank()) * 1000 + u64::from(modified));
                blueprint.visual_entities.push(glass);
                blueprint.dormer_holes.push(cottage_hole(wall_id.0 as u32, panel));
                if flowerbed {
                    let mut translation = panel.translation; translation.y += -0.1;
                    blueprint.window_auto_clutter_candidates.push(WindowAutoClutterCandidate {
                        kind: WindowAutoClutterKind::WindowFlowerbed { mesh_index: rng.usize(0..6) },
                        rotation: panel.rotation, translation, scale: panel.scale,
                        key: candidate_hash((id, "flower_bed", index)), wall: wall_id,
                    });
                }
            }
        }
    }
}

/// 原1407c962c..979f：面板的真实四元数偏置，不是轴对齐默认孔。
fn cottage_hole(wall: u32, panel: Transform) -> DormerHoleSsbo {
    let normal = panel.rotation * Vec3::Z;
    let point = panel.translation + panel.rotation * Vec3::new(0.05, 0.05, -0.05)
        + Vec3::new(0.0, 0.5, 0.0);
    DormerHoleSsbo::cottage(wall, point, Vec3::new(0.8, 1.0, 0.2), [normal.x, normal.z])
}

#[cfg(test)]
mod original_bounds_tests {
    use super::*;

    fn roof(rectangle: bool) -> Roof {
        let mut bytes = [0u8; 88];
        let mut put = |offset: usize, value: f32| {
            bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
        };
        put(0x0c, 1.0);
        put(0x14, if rectangle { 0.0 } else { 3.0 });
        put(0x1c, 8.0);
        put(0x20, 6.0);
        put(0x2c, 0.5);
        put(0x30, 0.5);
        put(0x34, 0.5);
        bytes[8..12].copy_from_slice(&u32::from(rectangle).to_le_bytes());
        Roof::from_observed(bytes)
    }

    fn transform(position: Vec3) -> CachedDecoratorTransform {
        CachedDecoratorTransform {
            rotation: glam::Quat::IDENTITY,
            translation: position,
            scale: Vec3::ONE,
        }
    }

    #[test]
    fn circle_has_out_of_bounds_polarity_and_original_unordered_comparison() {
        let roof = roof(false);
        assert!(!is_out_of_roof_bounds(&transform(Vec3::ZERO), 1.0, 2.0, 1, &roof));
        assert!(is_out_of_roof_bounds(&transform(Vec3::Y * 10.0), 1.0, 2.0, 1, &roof));
        // Direct leaf comparison witness only: the real cache producer rejects
        // a non-finite transform before storing it.
        assert!(!is_out_of_roof_bounds(&transform(Vec3::new(0.0, f32::NAN, 0.0)), 1.0, 2.0, 1, &roof));
    }

    #[test]
    fn zero_and_four_are_rejected_ranks_instead_of_default_modes() {
        let roof = roof(false);
        for rank in [0, 4] {
            assert!(std::panic::catch_unwind(|| {
                is_out_of_roof_bounds(&transform(Vec3::ZERO), 1.0, 2.0, rank, &roof)
            }).is_err());
        }
    }

    #[test]
    fn rectangle_checks_real_corner_intervals_and_collision_width() {
        let roof = roof(true);
        let side = transform(Vec3::new(-4.0, 0.0, 0.0));
        for rank in 1..=3 {
            assert!(!is_out_of_roof_bounds(&side, 0.5, 2.0, rank, &roof));
        }
        assert!(is_out_of_roof_bounds(&side, 8.0, 2.0, 1, &roof));
        assert!(is_out_of_roof_bounds(&transform(Vec3::new(-4.0, 0.0, -3.0)), 2.0, 2.0, 1, &roof));
    }

    #[test]
    fn upper_actual_section_can_reject_after_lower_section_accepts() {
        let roof = roof(true);
        let position = Vec3::new(-4.0, 0.1, 0.0);
        let top = roof.height() * 0.2 + ((4.0 * 0.5 + 0.1) + position.y);
        assert!(section_avoids_corners(&roof, position, 2.6));
        assert!(!section_avoids_corners(&roof, Vec3::new(position.x, top, position.z), 2.6));
        assert!(is_out_of_roof_bounds(&transform(position), 5.0, 4.0, 3, &roof));
    }
}


#[cfg(test)]
mod canonical_dynamic_parent_tests {
    use super::*;
    use crate::blueprint::BlueprintMeshSlot;
    use country_core::resources::walls::{
        WallId,
        cached_decorator_transforms::{CachedDecoratorTransform, CachedDecoratorTransforms},
        decorator::{DecoratorId, DecoratorInfo},
        decorator_storage::{DecoratorStorage, DecoratorStorageAddr, Decorators},
        decorator_subtype::{DecoratorSubtype, DerivedDecoratorSubtypes},
    };
    use glam::Quat;
    use std::{
        panic::{AssertUnwindSafe, catch_unwind},
        sync::Arc,
    };

    fn roof() -> Roof {
        let mut bytes = [0u8; 88];
        for (offset, value) in [
            (0x0c, 0.0f32),
            (0x10, 0.0),
            (0x14, 3.0),
            (0x18, 0.0),
            (0x1c, 8.0),
            (0x20, 6.0),
            (0x2c, 0.5),
            (0x30, 0.5),
            (0x34, 0.5),
        ] {
            bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
        }
        Roof::from_observed(bytes)
    }

    pub(super) fn fixture(rank: u8, transform: CachedDecoratorTransform) -> (World, Schedule) {
        let mut world = World::new();
        let roof_entity = world.spawn(roof()).id();
        let mut walls = PublicWalls::default();
        walls.associate_roof(WallId(7), roof_entity);
        world.insert_resource(walls);
        let mut bucket = Decorators::default();
        bucket.insert(
            DecoratorId(41),
            DecoratorInfo {
                decorator_id: 41,
                rank,
                state: 1,
                u40: 6 << 8,
                ..Default::default()
            },
        );
        let mut storage = DecoratorStorage::default();
        storage.per_wall.insert(WallId(7), bucket);
        world.insert_resource(storage);
        let mut subtypes = DerivedDecoratorSubtypes::default();
        subtypes.insert(41, DecoratorSubtype(2));
        world.insert_resource(subtypes);
        let mut cached = CachedDecoratorTransforms::default();
        cached.insert(41, transform);
        world.insert_resource(cached);
        world.insert_resource(DecoratorBlueprints::default());
        world.insert_resource(DecoratorTransientVisualState::default());
        // 原SystemParam借用真实两资源；动态凸体本身不读取库记录。
        world.init_resource::<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>();
        world.init_resource::<UiSignifierStream>();
        let mut schedule = Schedule::default();
        schedule.add_systems(generate_cottage_dormers);
        (world, schedule)
    }

    fn transform() -> CachedDecoratorTransform {
        CachedDecoratorTransform {
            rotation: Quat::IDENTITY,
            translation: Vec3::new(-3.0, 0.25, 0.0),
            scale: Vec3::new(2.0, 1.0, 2.0),
        }
    }

    #[test]
    fn actual_system_uses_native_rank_sizes_convex_local_cache_and_same_arc_in_both_slots() {
        for (rank, width) in [(1, 0.8), (2, 1.5), (3, 2.2)] {
            assert_eq!(
                cottage_dormer_dims(&DecoratorInfo {
                    rank,
                    ..Default::default()
                }),
                Some((width, 1.0))
            );
            let transform = transform();
            let (mut world, mut schedule) = fixture(rank, transform);
            schedule.run(&mut world);
            let blueprint = world.resource::<DecoratorBlueprints>().get(41).unwrap();
            assert_eq!(
                blueprint.storage_addr,
                Some(DecoratorStorageAddr::Wall(WallId(7)))
            );
            assert_eq!(blueprint.interaction_source_flags, 0);
            assert_eq!(blueprint.u138, 1);
            assert_eq!(blueprint.transform, Some(transform));
            assert!(blueprint.colliders.is_empty());
            let BlueprintMeshSlot::Mesh(collision) = &blueprint.collision_mesh else {
                panic!("真实collision槽")
            };
            let BlueprintMeshSlot::Mesh(interaction) = &blueprint.interaction_mesh else {
                panic!("真实interaction槽")
            };
            assert!(Arc::ptr_eq(&collision.mesh, &interaction.mesh));
            assert_eq!(collision.position, transform.translation);
            assert_eq!(collision.rotation, transform.rotation);
            assert!(
                collision
                    .mesh
                    .shape_ref()
                    .unwrap()
                    .as_convex_polyhedron()
                    .unwrap()
                    .points()
                    .len()
                    >= 4
            );
            let previous = Arc::clone(&collision.mesh);
            // 同一Schedule保留原Local；key不含scale，命中必须先于奇异inverse。
            world.resource_mut::<CachedDecoratorTransforms>().insert(
                41,
                CachedDecoratorTransform {
                    scale: Vec3::ZERO,
                    ..transform
                },
            );
            schedule.run(&mut world);
            let blueprint = world.resource::<DecoratorBlueprints>().get(41).unwrap();
            let BlueprintMeshSlot::Mesh(collision) = &blueprint.collision_mesh else {
                panic!("缓存命中实际网格")
            };
            assert!(Arc::ptr_eq(&previous, &collision.mesh));
        }
    }

    #[test]
    fn old_bounds_leaf_is_not_an_extra_gate_in_the_actual_dynamic_parent() {
        let transform = CachedDecoratorTransform {
            translation: Vec3::new(-0.5, 5.0, 0.0),
            ..transform()
        };
        assert!(is_out_of_roof_bounds(&transform, 0.8, 1.0, 1, &roof()));
        let (mut world, mut schedule) = fixture(1, transform);
        schedule.run(&mut world);
        let blueprint = world.resource::<DecoratorBlueprints>().get(41).unwrap();
        let BlueprintMeshSlot::Mesh(collision) = &blueprint.collision_mesh else {
            panic!("原caller仍构造真实动态凸体")
        };
        assert!(
            collision
                .mesh
                .shape_ref()
                .unwrap()
                .as_convex_polyhedron()
                .is_some()
        );
    }

    #[test]
    fn invalid_rank_fails_before_entry_but_failed_hull_keeps_the_native_entry_side_effect() {
        for rank in [0, 4] {
            let (mut world, mut schedule) = fixture(rank, transform());
            assert!(catch_unwind(AssertUnwindSafe(|| schedule.run(&mut world))).is_err());
            assert!(world.resource::<DecoratorBlueprints>().map.is_empty());
        }
        let transform = CachedDecoratorTransform {
            scale: Vec3::ZERO,
            ..transform()
        };
        let (mut world, mut schedule) = fixture(1, transform);
        assert!(catch_unwind(AssertUnwindSafe(|| schedule.run(&mut world))).is_err());
        let blueprint = world.resource::<DecoratorBlueprints>().get(41).unwrap();
        assert_eq!(
            blueprint.storage_addr,
            Some(DecoratorStorageAddr::Wall(WallId(7)))
        );
        assert_eq!(blueprint.interaction_source_flags, 0);
            assert_eq!(blueprint.u138, 1);
        assert!(matches!(
            blueprint.collision_mesh,
            BlueprintMeshSlot::Unspecified
        ));
        assert!(matches!(
            blueprint.interaction_mesh,
            BlueprintMeshSlot::Unspecified
        ));
        assert!(blueprint.transform.is_none());
    }
}

#[cfg(test)]
mod native_panel_tests {
    use super::*;
    use country_core::systems::wall::valid_enclosure::{ShapeParameters, Shape};
    #[test]
    fn all_rank_panels_match_original_curve_and_rotation_instructions() {
        // 原Unicorn输出12组；完整原输入、指纹和指令边界在m01-dormer-panels-native.json。
        let rows: &[(u8, [u32; 7], [u32; 3], &[[u32; 10]])] = &[
            (1, [1, 1065353216, 0, 0, 0, 1090519040, 1086324736], [3229614080, 1067450368, 0], &[[0, 0, 0, 1065353216, 3229614080, 1067450368, 0, 1065353216, 1065353216, 1065353216]]),
            (1, [1, 1065353216, 0, 0, 0, 1090519040, 1086324736], [3229614080, 1067450368, 3225000346], &[[0, 0, 0, 1065353216, 3229614080, 1067450368, 3225000346, 1065353216, 1065353216, 1065353216]]),
            (1, [1, 1058642330, 1061997773, 1077936128, 3221225472, 1091567616, 1088421888], [1048576000, 1075838976, 3229614080], &[[0, 0, 0, 1065353216, 1048576000, 1075838976, 3229614080, 1065353216, 1065353216, 1065353216]]),
            (1, [0, 0, 0, 1077936128, 0, 0, 0], [3225419776, 1067450368, 0], &[[0, 0, 0, 1065353216, 3225419776, 1067450368, 0, 1065353216, 1065353216, 1065353216]]),
            (2, [1, 1065353216, 0, 0, 0, 1090519040, 1086324736], [3229614080, 1067450368, 0], &[[0, 1060439283, 0, 3207922931, 3229614080, 1067450368, 3200253952, 1065353216, 1065353216, 1065353216], [0, 1060439283, 0, 3207922931, 3229614080, 1067450368, 1052770304, 1065353216, 1065353216, 1065353216]]),
            (2, [1, 1065353216, 0, 0, 0, 1090519040, 1086324736], [3229614080, 1067450368, 3225000346], &[[0, 1065353216, 0, 2147483648, 3228460648, 1067450368, 3225419776, 1065353216, 1065353216, 1065353216], [0, 1060439283, 0, 3207922931, 3229614080, 1067450368, 3223427482, 1065353216, 1065353216, 1065353216]]),
            (2, [1, 1058642330, 1061997773, 1077936128, 3221225472, 1091567616, 1088421888], [1048576000, 1075838976, 3229614080], &[[0, 1064492266, 0, 3198281885, 3192704216, 1075838976, 3232183091, 1065353216, 1065353216, 1065353216], [0, 1064492266, 0, 3198281885, 3209481423, 1075838976, 3231239372, 1065353216, 1065353216, 1065353216]]),
            (2, [0, 0, 0, 1077936128, 0, 0, 0], [3225419776, 1067450368, 0], &[[0, 1061018850, 0, 3207313557, 3225318440, 1067450368, 3200218581, 1065353216, 1065353216, 1065353216], [0, 3207313550, 2147483648, 1061018856, 3225318444, 1067450368, 1052734654, 1065353216, 1065353216, 1065353216]]),
            (3, [1, 1065353216, 0, 0, 0, 1090519040, 1086324736], [3229614080, 1067450368, 0], &[[0, 1060439283, 0, 3207922931, 3229614080, 1067450368, 3208362938, 1065353216, 1065353216, 1065353216], [0, 0, 0, 1065353216, 3229614080, 1067450368, 0, 1065353216, 1065353216, 1065353216], [0, 1060439283, 0, 3207922931, 3229614080, 1067450368, 1060879296, 1065353216, 1065353216, 1065353216]]),
            (3, [1, 1065353216, 0, 0, 0, 1090519040, 1086324736], [3229614080, 1067450368, 3225000346], &[[0, 1065353216, 0, 2147483648, 3226957684, 1067450368, 3225419776, 1065353216, 1065353216, 1065353216], [0, 0, 0, 1065353216, 3229614080, 1067450368, 3225000346, 1065353216, 1065353216, 1065353216], [0, 1060439283, 0, 3207922931, 3229614080, 1067450368, 3221924524, 1065353216, 1065353216, 1065353216]]),
            (3, [1, 1058642330, 1061997773, 1077936128, 3221225472, 1091567616, 1088421888], [1048576000, 1075838976, 3229614080], &[[0, 1064492266, 0, 3198281885, 1035042368, 1075838976, 3232633979, 1065353216, 1065353216, 1065353216], [0, 0, 0, 1065353216, 3204448256, 1075838976, 3231711232, 1065353216, 1065353216, 1065353216], [0, 1064492266, 0, 3198281885, 3213563880, 1075838976, 3230788485, 1065353216, 1065353216, 1065353216]]),
            (3, [0, 0, 0, 1077936128, 0, 0, 0], [3225419776, 1067450368, 0], &[[0, 1061742749, 0, 3206457325, 3225039552, 1067450368, 3208234880, 1065353216, 1065353216, 1065353216], [0, 0, 0, 1065353216, 3225419775, 1067450368, 3061031014, 1065353216, 1065353216, 1065353216], [0, 3206457325, 2147483648, 1061742749, 3225039558, 1067450368, 1060751095, 1065353216, 1065353216, 1065353216]]),
        ];
        for (index, &(rank, shape, position, expected)) in rows.iter().enumerate() {
            let f = f32::from_bits;
            let section = ShapeParameters::from_shape(if shape[0] == 0 {
                Shape::Circle(utils::geometry::Circle2d { center: Vec2::new(f(shape[1]), f(shape[2])),
                    radius: f(shape[3]), rotation: f(shape[4]) })
            } else { Shape::Rectangle(Rectangle2d {
                axis: Vec2::new(f(shape[1]), f(shape[2])),
                center: Vec2::new(f(shape[3]), f(shape[4])),
                size: Vec2::new(f(shape[5]), f(shape[6])),
            }) });
            let transform = CachedDecoratorTransform { rotation: glam::Quat::IDENTITY,
                translation: Vec3::new(f(position[0]), f(position[1]), f(position[2])),
                scale: Vec3::ONE };
            let actual = panel_xforms_in_section(&transform, rank, [0.8, 1.5, 2.2][usize::from(rank - 1)], section);
            assert_eq!(actual.len(), expected.len());
            for (panel, expected) in actual.iter().zip(expected) {
                let actual = [panel.rotation.x, panel.rotation.y, panel.rotation.z, panel.rotation.w,
                    panel.translation.x, panel.translation.y, panel.translation.z,
                    panel.scale.x, panel.scale.y, panel.scale.z].map(f32::to_bits);
                assert_eq!(&actual, expected, "原面板案例{index}");
            }
        }
    }
}

#[cfg(test)]
mod native_hole_tests {
    use super::*;
    #[test]
    fn original_hole_words_preserve_rank_rotation_and_all_twelve_words() {
        // 原B完整孔写面9组输入，不只是比较构造器或默认轴向孔。
        let rows: &[(u8, [u32; 4], [u32; 12])] = &[
        (1, [0, 0, 0, 1065353216], [1075000115, 1071015526, 3227726643, 7, 1061997773, 1065353216, 1045220557, 0, 0, 1065353216, 1065353216, 3259498496]),
        (1, [0, 1058642330, 0, 1061997773], [1074647794, 1071015526, 3227776975, 7, 1061997773, 1065353216, 1045220557, 0, 1064682128, 1049582634, 1065353216, 3259498496]),
        (1, [1058642330, 0, 0, 1061997773], [1075000115, 1071116190, 3227374322, 7, 1061997773, 1065353216, 1045220557, 0, 0, 1049582634, 1065353216, 3259498496]),
        (2, [0, 0, 0, 1065353216], [1075000115, 1071015526, 3227726643, 7, 1061997773, 1065353216, 1045220557, 0, 0, 1065353216, 1065353216, 3259498496]),
        (2, [0, 1058642330, 0, 1061997773], [1074647794, 1071015526, 3227776975, 7, 1061997773, 1065353216, 1045220557, 0, 1064682128, 1049582634, 1065353216, 3259498496]),
        (2, [1058642330, 0, 0, 1061997773], [1075000115, 1071116190, 3227374322, 7, 1061997773, 1065353216, 1045220557, 0, 0, 1049582634, 1065353216, 3259498496]),
        (3, [0, 0, 0, 1065353216], [1075000115, 1071015526, 3227726643, 7, 1061997773, 1065353216, 1045220557, 0, 0, 1065353216, 1065353216, 3259498496]),
        (3, [0, 1058642330, 0, 1061997773], [1074647794, 1071015526, 3227776975, 7, 1061997773, 1065353216, 1045220557, 0, 1064682128, 1049582634, 1065353216, 3259498496]),
        (3, [1058642330, 0, 0, 1061997773], [1075000115, 1071116190, 3227374322, 7, 1061997773, 1065353216, 1045220557, 0, 0, 1049582634, 1065353216, 3259498496]),
        ];
        for &(rank, rotation, expected) in rows {
            let rotation = glam::Quat::from_array(rotation.map(f32::from_bits));
            let transform = CachedDecoratorTransform { rotation,
                translation: Vec3::new(2.25, 1.125, -3.5), scale: Vec3::ONE };
            let _ = rank;
            assert_eq!(cottage_hole(7, Transform { rotation, translation: transform.translation,
                scale: transform.scale }).0, expected);
        }
    }
}

#[cfg(test)]
mod producer_fullvisual_tests {
    use super::*;
    use bevy_ecs::system::RunSystemOnce;
    use crate::{blueprint::{PrevDecoratorBlueprints, clear_blueprints, copy_blueprints_to_prev,
        instantiate_blueprints}, decorator_visual_state::DecoratorVisualState,
        decorator_visual::deco_differential_state::{WallAttachedDecoTracker, ChimneyAssemblyParams,
            DoorStairsAssemblyParams}};
    use country_core::{asset_libraries::{mesh_library::AssetMeshLibrary,
        shader_library::{AssetShaderLibrary, SharedAssetShaderLibrary}, texture_library::AssetTextureLibrary},
        resources::{roofs::geometry::surface::Curve2, walls::{InnerWalls, WallId, WallState,
            decorator_subtype::{DecoratorMeshLibrary,
                DecoratorCollision, DecoratorLibKey, DecoratorSubtype},
            decorator_collision::DecoratorPhysicsColliders,
            public_walls::{OnWallDeleted, WallPublicationDirty, WallSpatialHash},
            publication_source::TerrainHeightGrid512, wall_holes::WallHoles}},
        render::test_device::TestDevice,
        components::drawable::NaniInstanceDesc,
        systems::collision::{WindowAutoClutterCandidates, world::{RaycastWorld, RaycastMeshData}}};
    use std::sync::Arc;

    fn run_producer(world: &mut World, gothic: bool) {
        if gothic { world.run_system_once(crate::decorator_visual::gothic_dormer::generate_gothic_dormers).unwrap(); }
        else { world.run_system_once(generate_cottage_dormers).unwrap(); }
    }
    fn snapshot(world: &mut World) { world.run_system_once(copy_blueprints_to_prev).unwrap(); }
    fn instantiate(world: &mut World) { world.run_system_once(instantiate_blueprints).unwrap(); }
    fn entities(world: &World) -> Vec<Entity> {
        world.resource::<DecoratorVisualState>().persistent_entities_of(41).unwrap().to_vec()
    }
    fn world(gothic: bool, rank: u8) -> World {
        let pose = CachedDecoratorTransform { rotation: glam::Quat::IDENTITY,
            translation: Vec3::new(-3.0, 0.25, 0.0), scale: Vec3::ONE };
        let (mut world, _) = super::canonical_dynamic_parent_tests::fixture(rank, pose);
        let roof_entity = world.resource::<PublicWalls>().roof_entity(WallId(7)).unwrap();
        let curve = Curve2::try_new(vec![[-4.,-3.],[-4.,3.],[4.,3.],[4.,-3.],[-4.,-3.]], true).unwrap();
        let mut inner = InnerWalls::new();
        inner.new_wall_with_id(WallId(7), WallState::new(curve, 3.0, 2, 0, false));
        let mut walls = PublicWalls::default();
        walls.consume_with_paths(&mut inner, &TerrainHeightGrid512 {
            encoded: vec![0;512*512].into(), minimum_y: 0., maximum_y: 0., sampling_fix: false,
        }, None, &WallPublicationDirty::default(), &WallSpatialHash::default());
        assert!(walls.get(WallId(7)).is_some());
        walls.associate_roof(WallId(7), roof_entity); world.insert_resource(walls);
        let addr = DecoratorStorageAddr::Wall(WallId(7));
        {
            let mut storage = world.resource_mut::<DecoratorStorage>();
            storage.decorator_to_storage.insert(DecoratorId(41), addr);
            let info = storage.get_mut(addr).unwrap().decorators.get_mut(&41).unwrap();
            info.u20 = 2 << 16; info.u28 = 0x1234; info.u30 = 0x5678;
        }
        if gothic {
            world.resource_mut::<DerivedDecoratorSubtypes>().insert(41, DecoratorSubtype(8));
            let mesh = Arc::new(RaycastMeshData::new_cuboid_at(Vec3::new(0.,0.55,-0.35), Vec3::new(0.4,0.55,0.35)));
            let record = DecoratorCollision { triangles: Vec::new(), mesh,
                bounds: utils::geometry::Rectangle3d::new(Vec3::new(0.,0.55,-0.35), Vec3::new(0.8,1.1,0.7), Vec2::X) };
            let mut library = world.resource_mut::<DecoratorMeshLibrary>();
            library.collision.insert(DecoratorLibKey::new(8,rank), record.clone());
            library.interaction.insert(DecoratorLibKey::new(8,rank), record);
        }
        world.init_resource::<PrevDecoratorBlueprints>();
        world.init_resource::<DecoratorVisualState>(); world.init_resource::<WallHoles>();
        world.insert_resource(RaycastWorld::new()); world.init_resource::<DecoratorPhysicsColliders>();
        world.init_resource::<WindowAutoClutterCandidates>();
        world.init_resource::<WallAttachedDecoTracker<ChimneyAssemblyParams>>();
        world.init_resource::<WallAttachedDecoTracker<DoorStairsAssemblyParams>>();
        world.insert_resource(AssetMeshLibrary::new());
        world.insert_resource(SharedAssetShaderLibrary(Arc::new(AssetShaderLibrary::new())));
        world.insert_resource(AssetTextureLibrary::new(&TestDevice(1)));
        world.init_resource::<Events<OnWallDeleted>>();
        world
    }

    #[test]
    fn both_dormer_families_reach_real_fullvisual_hole_grid_change_and_delete() {
        for gothic in [false, true] {
            for rank in 1..=3 {
                let mut world = world(gothic, rank);
                run_producer(&mut world, gothic);
                let panels = if gothic { 1 } else { usize::from(rank) };
                let blueprint = world.resource::<DecoratorBlueprints>().get(41).unwrap();
                assert_eq!(blueprint.visual_entities.len(), panels * 2);
                assert_eq!(blueprint.dormer_holes.len(), panels);
                assert!(blueprint.transform.is_some()); assert!(blueprint.outline.is_none());
                let holes = blueprint.dormer_holes.clone();
                let grid = crate::dormer_holes::build_dormer_hole_grid(world.resource::<DecoratorBlueprints>());
                assert!(holes.iter().all(|hole| grid.cells.iter().any(|cell|cell.contains(hole))));
                instantiate(&mut world);
                let initial = entities(&world); assert_eq!(initial.len(), panels * 2);
                for pair in initial.chunks_exact(2) {
                    assert_eq!(world.get::<ColorId>(pair[0]), Some(&ColorId(0x1234)));
                    assert_eq!(world.get::<ColorId>(pair[1]), Some(&ColorId(0x5678)));
                    assert!(world.get::<NaniInstanceDesc>(pair[0]).is_some());
                    assert!(world.get::<NaniInstanceDesc>(pair[1]).is_some());
                    assert!(world.get::<country_core::resources::window_lights::WindowLight>(pair[1]).is_some());
                    assert!(world.get::<Transform>(pair[0]).is_some());
                }
                snapshot(&mut world);
                world.resource_mut::<DecoratorStorage>().per_wall.get_mut(&WallId(7)).unwrap().modified_ids.clear();
                run_producer(&mut world, gothic); instantiate(&mut world);
                assert_eq!(entities(&world), initial, "稳定原Arc保留真实实体");
                // 真修改通知+Blueprint清理+同一producer+fullvisual，旧实体必须卸载。
                world.resource_mut::<DecoratorStorage>().per_wall.get_mut(&WallId(7)).unwrap().modified_ids.insert(41);
                world.run_system_once(clear_blueprints).unwrap();
                let mut pose = world.resource::<CachedDecoratorTransforms>().transform(41);
                pose.translation.y += 0.125; world.resource_mut::<CachedDecoratorTransforms>().insert(41,pose);
                run_producer(&mut world, gothic); instantiate(&mut world);
                let changed = entities(&world); assert_eq!(changed.len(), panels * 2);
                assert!(initial.iter().all(|e| !world.entities().contains(*e)));
                assert!(changed.iter().all(|e| world.entities().contains(*e)));
                let new_holes = &world.resource::<DecoratorBlueprints>().get(41).unwrap().dormer_holes;
                assert_ne!(new_holes, &holes);
                snapshot(&mut world);
                world.resource_mut::<DecoratorStorage>().apply_decorator_remove(
                    DecoratorStorageAddr::Wall(WallId(7)), DecoratorId(41));
                world.run_system_once(clear_blueprints).unwrap();
                run_producer(&mut world, gothic); instantiate(&mut world);
                assert!(world.resource::<DecoratorBlueprints>().get(41).is_none());
                assert!(world.resource::<DecoratorVisualState>().persistent_entities_of(41).is_none());
                assert!(changed.iter().all(|e| !world.entities().contains(*e)));
                assert!(world.resource::<DecoratorPhysicsColliders>().first.is_empty());
                assert!(world.resource::<DecoratorPhysicsColliders>().second.is_empty());
                assert!(crate::dormer_holes::build_dormer_hole_grid(world.resource::<DecoratorBlueprints>()).counts.iter().all(|n|n.0==0));
            }
        }
    }
    #[test]
    fn both_dormer_families_fullvisual_to_actual_sparse_tiles_dirty_upload_cache_edit_and_unload() {
        use country_core::{asset_libraries::ssbo_library::AssetSsboLibrary,
            render::{sparse_instance_buffer::{SparseInstanceBuffer,SparseSsbo}, ssbo::rhapsody_boundary::{GraphicsDevice,Buffer,BufferDesc,BackendError}},
            resources::{roof_tile_ssbo::RoofTileSsbo,roofs::{dormer_tiles::DormerTiles,commands::DeleteRoofCmd}}};
        use super::super::{assemble_dormer_roof_tiles::generate_dormer_roofs,maintain_dormer_tiles_draw::maintain_dormer_tiles};
        use std::{ops::Range,sync::atomic::{AtomicUsize,Ordering}};
        #[derive(Default)] struct UploadDevice { calls: AtomicUsize, bytes: AtomicUsize }
        impl GraphicsDevice for UploadDevice {
            fn create_buffer(&self, d:&BufferDesc,_:&str,_:Option<&[u8]>) -> Result<Buffer,BackendError> {
                Ok(Buffer { byte_size:d.size as u64,..Buffer::default() })
            }
            fn update_buffer_bytes(&self,_:&Buffer,data:&[u8],ranges:&[Range<usize>],offset:usize)->Result<(),BackendError> {
                assert_eq!(offset,0);
                for r in ranges { assert!(r.start<r.end && r.end<=data.len()); assert_eq!(r.start%64,0); }
                self.calls.fetch_add(1,Ordering::Relaxed);
                self.bytes.fetch_add(ranges.iter().map(|r|r.end-r.start).sum::<usize>(),Ordering::Relaxed); Ok(())
            }
        }
        fn records(world:&World)->Vec<RoofTileSsbo> {
            let tiles=world.resource::<DormerTiles>();
            let Some(allocation)=tiles.allocations.get(&WallId(7)) else { return Vec::new(); };
            let buffer=world.resource::<AssetSsboLibrary>().get::<RoofTileSsbo>().unwrap();
            assert!(buffer.owns_allocation(allocation));
            let chunks=allocation.handle.0.as_ref().unwrap().chunk_ids.lock();
            chunks.iter().flat_map(|&chunk| {
                let start=chunk as usize*32; buffer.data[start..start+usize::from(buffer.byte_map[chunk as usize])].iter().copied()
            }).collect()
        }
        for gothic in [false,true] {
            for rank in 1..=3 {
                let mut world=world(gothic,rank);
                let roof_entity=world.resource::<PublicWalls>().roof_entity(WallId(7)).unwrap();
                world.get_mut::<Roof>(roof_entity).unwrap().observed_mut()[..8].copy_from_slice(&7u64.to_le_bytes());
                let device=UploadDevice::default();
                let ssbos=AssetSsboLibrary::new();
                ssbos.register::<RoofTileSsbo>(SparseInstanceBuffer::<RoofTileSsbo>::new_storage_buffer(&device,None));
                world.insert_resource(ssbos);world.init_resource::<DormerTiles>();world.init_resource::<Events<DeleteRoofCmd>>();
                let mut schedule=Schedule::default();schedule.add_systems((generate_dormer_roofs,maintain_dormer_tiles).chain());
                run_producer(&mut world,gothic);instantiate(&mut world);let initial_entities=entities(&world);
                let blueprint_count=world.resource::<DecoratorBlueprints>().get(41).unwrap().visual_entities.len();
                schedule.run(&mut world);let initial=records(&world);assert!(!initial.is_empty(),"family {gothic} rank {rank}");
                assert!(initial.iter().all(|r|r.roof_id==7 && r.animation_t==1. && r.current_height==0. && r.is_circle==0 && r.is_main_roof_body==0));
                assert_eq!(world.resource::<DecoratorBlueprints>().get(41).unwrap().visual_entities.len(),blueprint_count);
                world.resource_mut::<AssetSsboLibrary>().get_mut::<RoofTileSsbo>().unwrap().gpu_flush(&device).unwrap();
                assert_eq!(device.calls.load(Ordering::Relaxed),1);assert!(device.bytes.load(Ordering::Relaxed)>=initial.len()*64);
                for _ in 0..3 { schedule.run(&mut world);world.resource_mut::<AssetSsboLibrary>().get_mut::<RoofTileSsbo>().unwrap().gpu_flush(&device).unwrap(); }
                assert_eq!(records(&world),initial);assert_eq!(device.calls.load(Ordering::Relaxed),1);
                snapshot(&mut world);let mut pose=world.resource::<CachedDecoratorTransforms>().transform(41);pose.translation.x+=0.37;
                world.resource_mut::<CachedDecoratorTransforms>().insert(41,pose);run_producer(&mut world,gothic);instantiate(&mut world);schedule.run(&mut world);
                let edited=records(&world);assert!(!edited.is_empty());assert_ne!(edited,initial);
                world.resource_mut::<AssetSsboLibrary>().get_mut::<RoofTileSsbo>().unwrap().gpu_flush(&device).unwrap();assert_eq!(device.calls.load(Ordering::Relaxed),2);
                assert!(initial_entities.iter().all(|e|!world.entities().contains(*e)));
                snapshot(&mut world);world.resource_mut::<DecoratorStorage>().apply_decorator_remove(DecoratorStorageAddr::Wall(WallId(7)),DecoratorId(41));
                world.run_system_once(clear_blueprints).unwrap();instantiate(&mut world);schedule.run(&mut world);
                assert!(records(&world).is_empty());assert!(world.resource::<DecoratorVisualState>().persistent_entities_of(41).is_none());
                let buffer=world.resource::<AssetSsboLibrary>().get::<RoofTileSsbo>().unwrap();
                assert!(buffer.active_words().all(|word|word==0));assert_eq!(buffer.free_chunks.read().len(),RoofTileSsbo::CHUNK_COUNT);
                world.resource_mut::<Events<DeleteRoofCmd>>().send(DeleteRoofCmd(WallId(7)));schedule.run(&mut world);
                assert!(world.resource::<DormerTiles>().allocations.is_empty());
                let mut library=world.resource_mut::<AssetSsboLibrary>();
                let buffer=library.get_mut::<RoofTileSsbo>().unwrap();
                buffer.gpu_flush(&device).unwrap();assert!(buffer.waste.lock().is_empty());
            }
        }
    }

}
