//! B `decorator_visual/chimney_roof.rs` — 屋顶烟囱装饰视觉生成（D1）。
//!
//! 证据：fresh RE-MCP 反编译 0x1407c0910（generate_roof_chimneys，32KB）+ 骨架
//! callees（O）：DecoratorStorage::iter（0x140a1e5b0）、DecoratorStorageAddr
//! （0x140a1c2a0）、Decorators::iter_modified（0x140a1b070）、
//! DerivedDecoratorInfo::subtype（0x140b26970）、impl$8::from（0x140b26a70）、
//! DecoratorExtraInfo::as_chimney（0x1408392d0）、RaycastMeshData::new_cuboid、
//! PublicWallState::max_y、CachedDecoratorTransforms::transform、
//! blueprint {set_colliders, insert}。
//!
//! 2026-10-19 复核的新事实（O）：
//! - 屋顶烟囱子类型 = **0x15**（0x1407c0910 行 501 `subtype != 0x15 → skip`）；
//! - 烟囱高 = `as_chimney().f + 0.1`（0x142925984 = 0.1；行 519
//!   `fVar51 = *pfVar23 + fVar10`）；
//! - 立方体尺寸两轴 = 0.85（0x3f59999a；行 615-622 写入 new_cuboid 的
//!   `{fVar51, 0x3f59999a, 0x3f59999a}`）；
//! - 视觉 nani 名 `"nani_smoke"`（0x142c21224，len 10），Smoke 载荷
//!   `(id, 0, ?)` 与 `as_chimney().smoke`（字节 +4，行 520/687）；
//! - 蓝图 `+0x120`（0x28B=10×u32）烟囱参数块：`{2, …}`（行 690-710）。
//!
//! U 登记：① 烟囱高/位置几何的其余槽位 ② 参数排序 ③ 蓝图 +0x120 参数块
//! 各字段语义。

use bevy_ecs::prelude::*;
use country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes;
use country_core::resources::walls::public_walls::PublicWalls;

use crate::blueprint::{DecoratorBlueprint, DecoratorBlueprints};

/// 屋顶烟囱子类型（O：0x1407c0910 行 501）。
pub const ROOF_CHIMNEY_SUBTYPE: u8 = 0x15;
/// 烟囱高附加（O：0x142925984 = 0.1）。
pub const CHIMNEY_HEIGHT_EXTRA: f32 = 0.1;
/// 立方体截面/高（O：0x3f59999a = 0.85）。
pub const CHIMNEY_CUBOID_SIZE: f32 = 0.85;
/// 视觉 nani 名（O：0x142c21224，len 10）。
pub const CHIMNEY_NANI_SMOKE: &str = "nani_smoke";

/// 屋顶烟囱几何参数（O 公式）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RoofChimney {
    /// 立方体尺寸（两轴 0.85，高度轴另计）。
    pub cuboid: [f32; 3],
    /// 烟囱总高 = extra.f + 0.1。
    pub height: f32,
    /// 烟雾字节（`ChimneyExtra::smoke`，O：+4 字段）。
    pub smoke: u8,
}

/// 由 `ChimneyExtra` 计算屋顶烟囱几何（O）。
pub fn roof_chimney_geometry(extra_height: f32, smoke: u8) -> RoofChimney {
    RoofChimney {
        cuboid: [
            CHIMNEY_CUBOID_SIZE,
            CHIMNEY_CUBOID_SIZE,
            extra_height + CHIMNEY_HEIGHT_EXTRA,
        ],
        height: extra_height + CHIMNEY_HEIGHT_EXTRA,
        smoke,
    }
}

/// 0x1407c0910（O 结构）：生成屋顶烟囱装饰视觉。
pub fn generate_roof_chimneys(
    decorator_storage: Res<DecoratorStorage>,
    cached_transforms: Res<CachedDecoratorTransforms>,
    _walls: Res<PublicWalls>,
    derived: Res<DerivedDecoratorSubtypes>,
    mut blueprints: ResMut<DecoratorBlueprints>,
) {
    for (_wall, decorators) in decorator_storage.iter() {
        for (id, info) in decorators.iter_modified() {
            // O 判别：refusing 非屋顶目的地 / 非 0x15 子类型。
            if info.dst_tag() != 2 {
                continue;
            }
            if derived.get(id).subtype().0 != ROOF_CHIMNEY_SUBTYPE {
                continue;
            }
            let extra = info.extra().as_chimney();
            let chimney = roof_chimney_geometry(extra.f, extra.smoke);
            let _transform = cached_transforms.transform(id);
            let _subtype = derived.get(id).subtype();
            // O：立方体碰撞（RaycastMeshData::new_cuboid，尺寸 0.85/0.85/h）。
            let _collider = crate::utils::new_cuboid_collider(
                chimney.cuboid[0],
                chimney.cuboid[1],
                chimney.cuboid[2],
            );
            // O：蓝图（set_colliders + add_chimney + insert）。
            let mut blueprint = DecoratorBlueprint::new();
            blueprint.set_colliders(vec![derived.get(id).collision_mesh()]);
            blueprint.add_chimney(chimney.height);
            blueprints.insert(id, blueprint);
        }
    }
}

/// 判别（O：dst=屋顶 且 subtype=0x15）。
pub fn is_roof_chimney(dst_tag: u8, subtype: u8) -> bool {
    dst_tag == 2 && subtype == ROOF_CHIMNEY_SUBTYPE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn geometry_matches_machine_constants() {
        let c = roof_chimney_geometry(1.4, 3);
        assert!((c.height - 1.5).abs() < 1e-6);
        assert_eq!(c.cuboid, [0.85, 0.85, 1.5]);
        assert_eq!(c.smoke, 3);
    }

    #[test]
    fn predicate_matches_decomp() {
        assert!(is_roof_chimney(2, 0x15));
        assert!(!is_roof_chimney(2, 0x13));
        assert!(!is_roof_chimney(0, 0x15));
    }
}
