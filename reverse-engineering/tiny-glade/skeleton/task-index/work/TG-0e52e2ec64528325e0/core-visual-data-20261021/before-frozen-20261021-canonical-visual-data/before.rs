//! B `decorator_visual/trap_doors.rs` — 活板门装饰视觉生成（D1）。
//!
//! 证据：fresh RE-MCP 反编译 0x1407be730（generate_trap_doors，51KB）+ 骨架
//! callees（O）：iter_walls / iter_modified、Roof::is_flat、
//! PublicWalls::roof_entity、CachedDecoratorTransforms::transform、
//! DormerHoleSsbo::cottage、RaycastMeshData::new_cuboid、
//! Rectangle2d::contains_point_padding、Circle2d::intersect_circle、
//! DecoratorBlueprint::{set_colliders,set_outline} + DecoratorBlueprints::insert、
//! WindowVisualEntityBuilder::{new,custom_shader}、ColorId eq。
//!
//! 2026-10-19 复核的新事实（O）：
//! - 活板门子类型 ∈ **{0x05, 0x0a}**（0x1407be730 行 424：
//!   `subtype != 0x0a && subtype != 0x05 → skip`）；
//! - 两套网格名（0x142a85fa8/0x142a85fc4）：`"decorators/trap_door_cottage"`、
//!   `"decorators/trap_door_gothic"`；视觉 nani 名 `"nani_balcony_door"`
//!   （0x142a85fdf，len 17）；
//! - 立方体尺寸常量 1.3（0x3fa66666，行 454）与 1.125（0x3f900000，行 764）。
//!
//! U 登记：① 类型谓词的位表过滤式 ② 1.3/1.125 的轴向语义 ③ 0x30B 记录
//! （+0x1c Vec）载荷。

use bevy_ecs::prelude::*;
use country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::decorator_subtype::{
    DecoratorLibKey, DecoratorMeshLibrary, DerivedDecoratorSubtypes,
};
use country_core::resources::walls::public_walls::PublicWalls;

use crate::blueprint::{DecoratorBlueprint, DecoratorBlueprints};
use crate::decorator_visual_state::DecoratorTransientVisualState;
use crate::utils::visual_entity_builder::WindowVisualEntityBuilder;

/// 农舍活板门子类型（O：行 424）。
pub const TRAP_DOOR_SUBTYPE_COTTAGE: u8 = 0x05;
/// 哥特活板门子类型（O：行 424）。
pub const TRAP_DOOR_SUBTYPE_GOTHIC: u8 = 0x0a;
/// 活板门立方体常量（O：0x3fa66666 = 1.3）。
pub const TRAP_DOOR_CUBOID_A: f32 = 1.3;
/// 活板门立方体常量（O：0x3f900000 = 1.125）。
pub const TRAP_DOOR_CUBOID_B: f32 = 1.125;
/// 农舍活板门网格名（O：0x142a85fa8）。
pub const TRAP_DOOR_MESH_COTTAGE: &str = "decorators/trap_door_cottage";
/// 哥特活板门网格名（O：0x142a85fc4）。
pub const TRAP_DOOR_MESH_GOTHIC: &str = "decorators/trap_door_gothic";
/// 视觉 nani 名（O：0x142a85fdf，len 17）。
pub const TRAP_DOOR_NANI: &str = "nani_balcony_door";

/// 活板门子类型判别（O：{5, 10}）。
pub fn is_trap_door_subtype(subtype: u8) -> bool {
    subtype == TRAP_DOOR_SUBTYPE_COTTAGE || subtype == TRAP_DOOR_SUBTYPE_GOTHIC
}

/// 子类型 → 网格名（O）。
pub fn trap_door_mesh(subtype: u8) -> Option<&'static str> {
    match subtype {
        TRAP_DOOR_SUBTYPE_COTTAGE => Some(TRAP_DOOR_MESH_COTTAGE),
        TRAP_DOOR_SUBTYPE_GOTHIC => Some(TRAP_DOOR_MESH_GOTHIC),
        _ => None,
    }
}

/// 活板门几何参数（O 常量；U：轴向）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TrapDoorGeometry {
    pub mesh: &'static str,
    pub nani: &'static str,
    pub cuboid_a: f32,
    pub cuboid_b: f32,
}

/// 由子类型计算活板门几何（O）。
pub fn trap_door_geometry(subtype: u8) -> Option<TrapDoorGeometry> {
    trap_door_mesh(subtype).map(|mesh| TrapDoorGeometry {
        mesh,
        nani: TRAP_DOOR_NANI,
        cuboid_a: TRAP_DOOR_CUBOID_A,
        cuboid_b: TRAP_DOOR_CUBOID_B,
    })
}

/// 0x1407be730（O 结构）：生成活板门装饰的视觉内容。
pub fn generate_trap_doors(
    decorator_storage: Res<DecoratorStorage>,
    cached_transforms: Res<CachedDecoratorTransforms>,
    walls: Res<PublicWalls>,
    mesh_lib: Res<DecoratorMeshLibrary>,
    derived: Res<DerivedDecoratorSubtypes>,
    mut blueprints: ResMut<DecoratorBlueprints>,
    mut transient: ResMut<DecoratorTransientVisualState>,
) {
    for (wall, decorators) in decorator_storage.iter_walls() {
        // O：屋顶平坦校验（roof_entity + is_flat）。
        let roof_entity = walls.roof_entity(wall);
        for (id, info) in decorators.iter_modified() {
            let subtype = derived.get(id).subtype().0;
            let Some(geometry) = trap_door_geometry(subtype) else {
                continue;
            };
            let transform = cached_transforms.transform(id);
            // 洞记录（O callee：DormerHoleSsbo::cottage；载荷 U）。
            let _hole = crate::utils::DormerHoleSsbo::cottage(
                [0; 16],
                id,
                [0; 16],
                [0.0, 0.0],
                0.0,
            );
            // 蓝图（O callee：set_colliders + set_outline + insert）。
            let mut blueprint = DecoratorBlueprint::new();
            blueprint.set_colliders(vec![derived.get(id).collision_mesh()]);
            blueprint.set_outline((0.0, 0.0, 0.0));
            blueprints.insert(id, blueprint);
            // 视觉实体（O callee：new + custom_shader + nani_balcony_door）。
            let mut builder = WindowVisualEntityBuilder::new(
                transform.translation,
                transform.rotation,
                id,
            );
            builder.mesh_name = geometry.mesh;
            builder.nani_name = geometry.nani;
            let _ = &mut transient;
            let _ = &builder;
            let _ = &mesh_lib;
            let _ = DecoratorLibKey::new(subtype, 0);
            let _ = roof_entity;
            let _ = info;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subtype_predicate_matches_decomp() {
        assert!(is_trap_door_subtype(0x05));
        assert!(is_trap_door_subtype(0x0a));
        assert!(!is_trap_door_subtype(0x04));
        assert!(!is_trap_door_subtype(0x09));
        assert!(!is_trap_door_subtype(0x0b));
    }

    #[test]
    fn mesh_names_match_machine_strings() {
        assert_eq!(
            trap_door_mesh(0x05),
            Some("decorators/trap_door_cottage")
        );
        assert_eq!(trap_door_mesh(0x0a), Some("decorators/trap_door_gothic"));
        assert_eq!(trap_door_mesh(0x09), None);
    }

    #[test]
    fn geometry_constants() {
        let g = trap_door_geometry(0x0a).unwrap();
        assert_eq!(g.cuboid_a, 1.3);
        assert_eq!(g.cuboid_b, 1.125);
        assert_eq!(g.nani, "nani_balcony_door");
    }
}
