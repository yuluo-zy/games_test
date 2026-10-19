//! B `decorator_visual/chimney_wall.rs` — 墙烟囱装饰视觉生成（D1）。
//!
//! 证据：fresh RE-MCP 反编译 0x1407c21b0（generate_wall_chimneys，63KB）+ 骨架
//! callees（O）：DecoratorStorage::iter、iter_modified、
//! DerivedDecoratorInfo::subtype、DecoratorLibKey::new、wall_space::distance
//! （0x140c91df0）、DecoratorExtraInfo::as_chimney、is_valid_body_placement
//! （本文件子函数）、blueprint 链、RaycastMeshData::new_cuboid。
//!
//! 2026-10-19 复核的新事实（O）：
//! - 墙烟囱子类型 ∈ **{0x13, 0x14}**（0x1407c21b0 行 539
//!   `1 < (subtype - 0x13) → skip`）；
//! - 视觉 nani 名 `"nani_smoke"`（0x142c21224，len 10，行 808/1513）；
//! - `as_chimney` 载荷（`{f32, u8}`，行 554）驱动烟囱高与烟雾；
//! - 墙空间距离 `wall_space::distance`（0x140c91df0，行 1385）。
//!
//! U 登记：① is_valid_body_placement 准则 ② 参数排序 ③ wall_space::distance
//! 结果消费面。

use bevy_ecs::prelude::*;
use country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms;
use country_core::resources::walls::decorator::DecoratorInfo;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes;
use country_core::resources::walls::public_walls::PublicWalls;

use crate::blueprint::{DecoratorBlueprint, DecoratorBlueprints};

/// 墙烟囱子类型下界（O：0x13）。
pub const WALL_CHIMNEY_SUBTYPE_LOW: u8 = 0x13;
/// 墙烟囱子类型上界（O：0x13/0x14，`subtype-0x13 <= 1`）。
pub const WALL_CHIMNEY_SUBTYPE_HIGH: u8 = 0x14;
/// 视觉 nani 名（O：0x142c21224，len 10）。
pub const CHIMNEY_NANI_SMOKE: &str = "nani_smoke";

/// 墙烟囱子类型判别（O：`wrapping_sub(0x13) <= 1`）。
pub fn is_wall_chimney_subtype(subtype: u8) -> bool {
    subtype.wrapping_sub(WALL_CHIMNEY_SUBTYPE_LOW) <= 1
}

/// 0x1407c21b0（O 结构）：生成墙烟囱装饰视觉。
pub fn generate_wall_chimneys(
    decorator_storage: Res<DecoratorStorage>,
    cached_transforms: Res<CachedDecoratorTransforms>,
    _walls: Res<PublicWalls>,
    derived: Res<DerivedDecoratorSubtypes>,
    mut blueprints: ResMut<DecoratorBlueprints>,
    _transient: ResMut<crate::decorator_visual_state::DecoratorTransientVisualState>,
) {
    for (_wall, decorators) in decorator_storage.iter() {
        for (id, info) in decorators.iter_modified() {
            if info.dst_tag() >= 2 {
                continue; // 墙目的地（O 分支）。
            }
            // O：墙烟囱子类型 0x13/0x14。
            if !is_wall_chimney_subtype(derived.get(id).subtype().0) {
                continue;
            }
            // 放置合法性（O callee：is_valid_body_placement；准则 U）。
            if !is_valid_body_placement(info) {
                continue;
            }
            let extra = info.extra().as_chimney();
            let _height = extra.f;
            let _smoke = extra.smoke;
            let _transform = cached_transforms.transform(id);
            // O：wall_space::distance（0x140c91df0）在墙空间内求落点距离；
            // 输入墙空间对象由 PublicWalls 提供（U：PublicWallState raw 面）。
            let _wall_space = 0.0f32; // U：distance 消费面。
            let _collider = crate::utils::new_cuboid_collider(1.0, 1.0, 1.0);
            let mut blueprint = DecoratorBlueprint::new();
            blueprint.set_colliders(vec![derived.get(id).collision_mesh()]);
            blueprint.add_chimney(extra.f);
            blueprints.insert(id, blueprint);
        }
    }
}

/// 墙烟囱体放置合法性（O callee，I 语义：墙高/边界校验；精确准则 U）。
pub fn is_valid_body_placement(_info: &DecoratorInfo) -> bool {
    // U：校验准则待取证（以未过滤承载，宁可真生成面）。
    true
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
