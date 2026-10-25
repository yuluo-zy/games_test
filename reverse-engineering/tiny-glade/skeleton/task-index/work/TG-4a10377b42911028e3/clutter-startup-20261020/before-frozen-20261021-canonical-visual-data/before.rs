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
//! its bounds caller now uses genuine geometry inputs, and is not registered
//! as a recovered full visual generator by this leaf.

use bevy_ecs::prelude::*;
use country_core::resources::walls::decorator::DecoratorInfo;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::roofs::geometry::surface::RoofSection;
use country_core::resources::roofs::roof_shape::Roof;
use country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransform;
use country_core::resources::walls::decorator_subtype::{
    DecoratorLibKey, DecoratorMeshLibrary, DerivedDecoratorSubtypes,
};
use country_core::resources::walls::public_walls::PublicWalls;
use glam::{Vec2, Vec3};
use utils::geometry::Rectangle2d;

use crate::blueprint::{DecoratorBlueprint, DecoratorBlueprints};
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

/// 农舍老虎窗尺寸（内联，I；U：维度来源）。未恢复返回 None。
pub fn cottage_dormer_dims(_info: &DecoratorInfo) -> Option<(f32, f32)> {
    None // U。
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
    cached: Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,
    mesh_lib: Res<DecoratorMeshLibrary>,
    walls: Res<PublicWalls>,
    roofs: Query<&Roof>,
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
            if derived.get(id).subtype().0 != COTTAGE_DORMER_SUBTYPE {
                continue;
            }
            let transform = cached.transform(id);
            let key = DecoratorLibKey::new(COTTAGE_DORMER_SUBTYPE, info.rank());
            let (minimum, maximum) = mesh_lib.get_collision_bbx_ls(key).unwrap_or_else(|| {
                panic!("The decorator mesh library doesn't contain {key:?}")
            });
            if is_out_of_roof_bounds(
                &transform,
                maximum.x - minimum.x,
                maximum.y - minimum.y,
                info.rank(),
                roof,
            ) {
                continue;
            }
            let d = derived.get(id);
            // O：蓝图 + 视觉实体。
            let mut blueprint = DecoratorBlueprint::new();
            blueprint.set_colliders(vec![d.collision_mesh()]);
            blueprint.set_outline((0.0, 0.0, 0.0));
            blueprints.insert(id, blueprint);
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
