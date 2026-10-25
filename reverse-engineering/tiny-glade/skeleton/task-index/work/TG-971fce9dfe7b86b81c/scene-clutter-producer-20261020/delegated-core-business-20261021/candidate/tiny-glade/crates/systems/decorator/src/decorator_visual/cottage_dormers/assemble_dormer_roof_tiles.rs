//! B `cottage_dormers/assemble_dormer_roof_tiles.rs` — 老虎窗屋顶瓦装配（D1）。
//!
//! 证据：fresh RE-MCP 反编译 0x1407cb1a0（generate_dormer_roofs，57KB）/
//! 0x14128db20（closure_env<7>，54KB）+ 骨架 callees（O）：
//! dormer_roof_height（内联）、is_bay_window（内联）、assemble_dormer_roof_tiles
//! 泛型实例、DormerKey::new（DecoratorLibKey::new 0x140b25c90）、
//! DecoratorArchivist::wall_decorators（0x140a1e190）、LazyTile::from_index
//! （0x140833250）、halver to_f16（0x140c8ffb0）、Roof::get_roof_shape_from_pos_ws_clamped
//! （0x1408e4750）、blueprint 链。
//!
//! 2026-10-19 复核的新事实（O）：
//! - 子类型掩码：`subtype < 0x18 && ((0x800186 >> subtype) & 1) != 0`
//!   （0x1407cb1a0 行 412）→ subtype ∈ **{1, 2, 7, 8, 23}**；
//! - subtype == 0x17（23）单独分支（行 1161 `if (subtype != 0x17)`）；
//! - 哥特分支 subtype == 8 → `assemble_gothic_dormer_tiles`
//!   （0x1407c5650 的哥特老虎窗生成器同样只处理 8）；其余 → `assemble_dormer_roof_tiles`。
//!
//! U 登记：① 瓦片装配数学主体（f16 瓦坐标） ② dormer_roof_height / is_bay_window
//! 的精确公式 ③ 参数排序。

use bevy_ecs::prelude::*;
use country_core::resources::walls::decorator::DecoratorInfo;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes;

use crate::blueprint::DecoratorBlueprints;

/// 子类型掩码（O：0x800186 → 位 1/2/7/8/23）。
pub const DORMER_ROOF_SUBTYPE_MASK: u32 = 0x0080_0186;
/// 掩码上界（O：`subtype < 0x18`）。
pub const DORMER_ROOF_SUBTYPE_LIMIT: u8 = 0x18;
/// 特殊子类型（O：行 1161 `subtype == 0x17`）。
pub const DORMER_ROOF_SUBTYPE_SPECIAL: u8 = 0x17;
/// 哥特老虎窗子类型（O：assemble_gothic 分支）。
pub const DORMER_ROOF_SUBTYPE_GOTHIC: u8 = 0x08;

/// 子类型是否属于老虎窗屋顶装配（O 位测试）。
pub fn is_dormer_roof_subtype(subtype: u8) -> bool {
    subtype < DORMER_ROOF_SUBTYPE_LIMIT
        && ((DORMER_ROOF_SUBTYPE_MASK >> (subtype & 0x1f)) & 1) != 0
}

/// 装配族选择（O：哥特 8；其余走农舍瓦装配）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DormerRoofFamily {
    Gothic,
    Cottage,
}

/// 子类型 → 装配族（O）。
pub fn dormer_roof_family(subtype: u8) -> Option<DormerRoofFamily> {
    if !is_dormer_roof_subtype(subtype) {
        return None;
    }
    Some(if subtype == DORMER_ROOF_SUBTYPE_GOTHIC {
        DormerRoofFamily::Gothic
    } else {
        DormerRoofFamily::Cottage
    })
}

/// 老虎窗屋顶高（内联，I；U：来源）。
///
/// 原版由 `Info` 高度/屋顶形状得（入口 decomp-0x1407cb1a0）；精确式 U，
/// 因此返回 `None`（未恢复），不以 0 冒充。
pub fn dormer_roof_height(_info: &DecoratorInfo) -> Option<f32> {
    None // U。
}

/// 凸窗判定（内联，I；U：判别准则）。
pub fn is_bay_window(_info: &DecoratorInfo) -> Option<bool> {
    None // U。
}

/// 0x1407cb1a0（O 结构）：生成老虎窗屋顶（瓦片装配入口）。
pub fn generate_dormer_roofs(
    decorator_storage: Res<DecoratorStorage>,
    derived: Res<DerivedDecoratorSubtypes>,
    _mesh: Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,
    mut blueprints: ResMut<DecoratorBlueprints>,
    _archivist: Res<country_core::resources::walls::decorator_archivist::DecoratorArchivist<()>>,
    _commands: Commands,
    _cached: Res<
        country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms,
    >,
    _transient: ResMut<crate::decorator_visual_state::DecoratorTransientVisualState>,
) {
    for (_wall, decorators) in decorator_storage.iter_walls() {
        for (id, info) in decorators.iter_modified() {
            let subtype = derived.get(id).0;
            // O：子类型掩码 / 0x17 特殊分支。
            let Some(family) = dormer_roof_family(subtype) else {
                continue;
            };
            // O：凸窗跳过（is_bay_window；U → 不跳过，显式记录）。
            if is_bay_window(info) == Some(true) {
                continue;
            }
            // O：瓦片装配主体（assemble_dormer_roof_tiles / gothic）。
            let _height = dormer_roof_height(info);
            let blueprint = crate::blueprint::DecoratorBlueprint::new();
            match family {
                DormerRoofFamily::Gothic => {
                    // O：哥特分支消费 gothic_dormer_profile_curve(rank)；
                    // rank 非 1..=3 时原函数 panic，这里保持（不伪造）。
                    if matches!(info.rank(), 1 | 2 | 3) && _height.is_some() {
                        let profile =
                            crate::decorator_visual::gothic_dormer::gothic_dormer_profile_curve(
                                info.rank(),
                            );
                        let _tiles =
                            crate::decorator_visual::cottage_dormers::util_assemble_gothic_dormer::assemble_gothic_dormer_tiles(
                                &profile,
                                _height.unwrap(),
                            );
                    }
                }
                DormerRoofFamily::Cottage => {}
            }
            blueprints.insert(id, blueprint);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subtype_mask_bits_match_machine_constant() {
        let expected = [1u8, 2, 7, 8, 23];
        for s in 0u8..24 {
            assert_eq!(
                is_dormer_roof_subtype(s),
                expected.contains(&s),
                "subtype {s}"
            );
        }
        assert!(!is_dormer_roof_subtype(0x18));
        assert!(!is_dormer_roof_subtype(0x17 - 1));
    }

    #[test]
    fn family_selection_picks_gothic_for_eight() {
        assert_eq!(dormer_roof_family(8), Some(DormerRoofFamily::Gothic));
        assert_eq!(dormer_roof_family(1), Some(DormerRoofFamily::Cottage));
        assert_eq!(dormer_roof_family(23), Some(DormerRoofFamily::Cottage));
        assert_eq!(dormer_roof_family(3), None);
    }
}
