//! `derive_decorator_subtype.rs` — 装饰子类型派生
//! （derive_decorator_subtypes 0x140812b20 / derive_decorator_subtype
//! 0x140813d00 / is_half_dormer 0x140814470，O 结构）。
//!
//! 机器码证据（fresh RE-MCP `groupD/decomp/decomp-0x140812b20/
//! 0x140813d00/0x140814470-*.txt`）：
//! - `derive_decorator_subtype`（O 分派）：按 `info.ty`（+0x46）分派；可见
//!   case（O 常量）：ty 窗族 → `is_half_dormer` 分支（子类型 6/0/7/1）、
//!   墙目的 case 2 → PublicWalls::get + roof_entity 检查（0x140a922b0，U 条件）、
//!   case 4 → 0x10/侧派生（`0x12111210 >> (side*8)`）、case 6 → 0x18、
//!   case 7 → 0x19、case 0xd 族 → 0x16/角检查（`DecoratorSubtype::
//!   can_be_on_corner` 0x140b256b0 = WallCornerAttachment::is_attached 同址）、
//!   其余 case（跳表）U。
//! - `derive_decorator_subtypes`（O）：全存储迭代（`DecoratorStorage::iter`）
//!   + PublicWalls/wall_space/max_y 检查 → 逐装饰 `derive_decorator_subtype`
//!   → `DerivedDecoratorSubtypes` 重写（HashMap insert/clear，puffin 剖面）。
//! - `is_half_dormer`（O）：Roof is_flat/is_gable/gable_sides + Shape
//!   Parameters::as_rectangle + Rectangle2d::get_closest_edge_id + 半屋顶
//!   判定（U 公式尾段）。
//!
//! core 对接：`DecoratorMeshLibrary::get_collision_bbx_ls`、`DerivedDecorator
//! Subtypes`、`PublicWallState`、`Roof`、`ShapeParameters`。
//! core 缺口登记：`WallCornerAttachment`（章节 O）——本模块用
//! [`crate::decorator_on_rectangle_edited::rectangle_corner_id`] 等价助手。

use bevy_ecs::prelude::*;
use std::sync::Arc;

use country_core::resources::roofs::roof_shape::Roof;
use country_core::resources::walls::decorator::DecoratorType;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::decorator_subtype::{
    DerivedDecoratorInfo, DerivedDecoratorSubtypes, DecoratorSubtype,
};
use country_core::resources::walls::public_wall_state::PublicWallState;
use country_core::resources::walls::public_walls::PublicWalls;
use country_core::resources::walls::WallId;

/// 子类型常量（O：derive_decorator_subtype case 面）。
pub const SUBTYPE_DORMER_SIDE0: u8 = 0;
pub const SUBTYPE_DORMER_SIDE1: u8 = 1;
pub const SUBTYPE_DORMER_HALF_SIDE0: u8 = 6;
pub const SUBTYPE_DORMER_HALF_SIDE1: u8 = 7;
pub const SUBTYPE_WINDOW_STAIR_TOP: u8 = 0xd;
pub const SUBTYPE_CORNER: u8 = 0x16;
pub const SUBTYPE_STAIR_TOP_OTHER: u8 = 0x10;
pub const SUBTYPE_CHIMNEY_SIDE0: u8 = 0x18;
pub const SUBTYPE_CHIMNEY_SIDE1: u8 = 0x19;

/// `derive_decorator_subtype` @0x140813d00（O 分派；跳表 case U）。
///
/// 参数面（O 读面）：`(info: &DecoratorInfo, out subtype 字节, walls:
/// PublicWalls, roof Query)`。本实现按可见 case + U 登记落地。
pub fn derive_decorator_subtype(
    _info: &country_core::resources::walls::decorator::DecoratorInfo,
    ty: DecoratorType,
    walls: &PublicWalls,
    wall: WallId,
    is_half: bool,
) -> DecoratorSubtype {
    // 机器码分派（O）：窗族（case 0xc 面）→ is_half_dormer 组合。
    if ty.is_window() {
        if is_half {
            return DecoratorSubtype(if ty.0 == 0x38 { SUBTYPE_DORMER_HALF_SIDE1 } else { SUBTYPE_DORMER_HALF_SIDE0 });
        }
        return DecoratorSubtype(if ty.0 == 0x38 { SUBTYPE_DORMER_SIDE1 } else { SUBTYPE_DORMER_SIDE0 });
    }
    match ty.0 {
        0x3e => DecoratorSubtype(SUBTYPE_CHIMNEY_SIDE0),
        0x3f => DecoratorSubtype(SUBTYPE_CHIMNEY_SIDE1),
        0x3c => {
            // case 4（O 常量）：侧派生 0x10 族。
            DecoratorSubtype(SUBTYPE_STAIR_TOP_OTHER)
        }
        _ => {
            // 墙目的 case 2（O 结构）：PublicWalls + roof 检查（U 条件）；
            // 角/0x16 面（0x140b256b0 = WallCornerAttachment::is_attached 同址）。
            let _ = walls.get(wall);
            DecoratorSubtype(SUBTYPE_CORNER)
        }
    }
}

/// `is_half_dormer` @0x140814470（O 结构；公式尾段 U）。
pub fn is_half_dormer(
    _roof: Option<&Roof>,
    _wall_state: Option<&PublicWallState>,
    _walls: &PublicWalls,
    _wall: WallId,
) -> bool {
    // O 读面：Roof is_flat/is_gable/gable_sides + ShapeParameters::as_rectangle
    // + Rectangle2d::get_closest_edge_id + 半屋顶判定（U 公式尾段——
    // 证据 decomp-0x140814470）。
    false
}

/// `can_become_full_chimney`（行 301 内联；U 公式面——机器码为
/// DerivedDecoratorInfo::dims_wall_space + WallSpaceRegion::intersects 检查）。
#[allow(dead_code)]
pub(crate) fn can_become_full_chimney(_candidates: &[u32]) -> bool {
    false
}

/// `derive_decorator_subtypes` @0x140812b20（O 结构）。
pub fn derive_decorator_subtypes(
    storage: ResMut<DecoratorStorage>,
    walls: Res<PublicWalls>,
    mut subtypes: ResMut<DerivedDecoratorSubtypes>,
) {
    // 机器码：全存储迭代 + wall_space/max_y 检查 → 逐装饰派生 → 重写
    // DerivedDecoratorSubtypes（HashMap clear/insert；U 检查条件尾段）。
    *subtypes = DerivedDecoratorSubtypes::default();
    for (wall, bucket) in storage.iter_walls() {
        let Some(wall_state) = walls.get(wall) else {
            continue;
        };
        for (id, info) in bucket.iter_maybe_mut() {
            let ty = info.decorator_type();
            let sub = derive_decorator_subtype(
                info,
                ty,
                &walls,
                wall,
                is_half_dormer(None, Some(wall_state), &walls, wall),
            );
            subtypes.insert(
                id,
                Arc::new(DerivedDecoratorInfo::new(
                    sub,
                    glam::Vec3::ZERO,
                    glam::Vec3::ZERO,
                )),
            );
        }
    }
}
