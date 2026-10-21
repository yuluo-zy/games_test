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
//! transform and Roof sections. The visual generator below remains partial;
//! bounds叶仍独立保留。generator的动态碰撞分支按原尺寸表调用唯一
//! collision_geo并消费真实Local缓存；面板/瓦片等其余视觉装配仍未完整恢复。

use bevy_ecs::prelude::*;
use country_core::resources::walls::decorator::DecoratorInfo;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::roofs::geometry::surface::RoofSection;
use country_core::resources::roofs::roof_shape::Roof;
use country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransform;
use country_core::resources::walls::decorator_subtype::{
    DerivedDecoratorSubtypes,
};
use country_core::resources::walls::public_walls::PublicWalls;
use glam::{Vec2, Vec3};
use utils::geometry::Rectangle2d;

use crate::blueprint::DecoratorBlueprints;
use crate::decorator_visual_state::DecoratorTransientVisualState;
use crate::utils::visual_entity_builder::WindowVisualEntityBuilder;

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

/// 老虎窗面板变换（内联，I；U：面板几何）。未恢复返回 None。
pub fn get_dormer_panel_xforms(_info: &DecoratorInfo) -> Option<Vec<crate::utils::IsoTransform>> {
    None // U。
}

/// 0x1407c7ac0（O 结构）：生成农舍老虎窗装饰视觉。
pub fn generate_cottage_dormers(
    decorator_storage: Res<DecoratorStorage>,
    derived: Res<DerivedDecoratorSubtypes>,
    mut blueprints: ResMut<DecoratorBlueprints>,
    mut transient: ResMut<DecoratorTransientVisualState>,
    cached: Res<
        country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms,
    >,
    walls: Res<PublicWalls>,
    roofs: Query<&Roof>,
    mut collision_cache: Local<super::collision_geo::DormerCollisionCache>,
) {
    for (wall, decorators) in decorator_storage.iter_walls() {
        // This is the original generator's real roof precondition; do not
        // substitute a default or synthetic mode when the roof is unavailable.
        let Some(roof_entity) = walls.roof_entity(wall) else {
            continue;
        };
        let Ok(roof) = roofs.get(roof_entity) else {
            continue;
        };
        if roof.is_flat() {
            continue;
        }
        for (id, info) in decorators.iter_modified() {
            // O：农舍老虎窗子类型 = 2。
            if derived.get(id).0 != COTTAGE_DORMER_SUBTYPE {
                continue;
            }
            let transform = cached.transform(id);
            let (width, height) = cottage_dormer_dims(info).expect("原rank尺寸表");
            // 原8229先取得唯一entry；helper失败也保留source/flags及原部分mutation。
            let blueprint = blueprints.insert_for(
                id,
                country_core::resources::walls::decorator_storage::DecoratorStorageAddr::Wall(wall),
                info.state(),
            );
            let mesh = super::collision_geo::get_collision_mesh(
                &transform,
                width,
                height,
                info.rank(),
                0.2,
                roof,
                &mut collision_cache,
            );
            blueprint.set_positioned_colliders(
                std::sync::Arc::clone(&mesh),
                mesh,
                &transform.translation,
                &transform.rotation,
            );
            // 原82c1..82e7的唯一变换槽，不以legacy outline元组替代。
            blueprint.set_cached_transform(transform);
            blueprint.set_outline((0.0, 0.0, 0.0));
            let builder =
                WindowVisualEntityBuilder::new(transform.translation, transform.rotation, id);
            let _ = &mut transient;
            let _ = &builder;
            let _ = info;
        }
    }
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

    fn fixture(rank: u8, transform: CachedDecoratorTransform) -> (World, Schedule) {
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
        // 不插DecoratorMeshLibrary：原动态几何不依赖loader的Sphere记录。
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
            assert_eq!(blueprint.interaction_source_flags, 1);
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
        assert_eq!(blueprint.interaction_source_flags, 1);
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
