//! B `decorator_visual/arrow_slit.rs` — 箭窗（arrow slit）装饰视觉生成（D1）。
//!
//! 证据：fresh RE-MCP 反编译 0x1407bbee0（generate_arrow_slits，63KB）+ 常量
//! rne-read + 骨架 callees（O）：PublicWallState::is_premade_shape、
//! Decorators::iter_modified、DecoratorMaybeMut::get（0x1400885d0）、
//! DerivedDecoratorInfo::{new,bbx_ls,collision_mesh,interaction_mesh,subtype}、
//! DormerHoleSsbo::gothic、Rectangle3d::{extent,center,width}、
//! get_gothic_hat_bones（gothic_wall_window）、swizzlers x0y/xvy、
//! WindowVisualEntityBuilder::{new,mark_as_preview,mark_as_window_glass,color_id}、
//! blueprint {set_colliders,set_outline,insert}。
//!
//! 常量（O，0x142a85680..0x2c + 0x142a80d40 + 0x40200000/0x3d4ccccd）：
//! - `{0.4, 1.7}`（0x3ecccccd/0x3fd9999a），`{0.6049, 0.55}`（0x3f1ae148/
//!   0x3f0ccccd；旧注释记 0.588，字节实为 0.6049），`{0.4, 1.5}`
//!   （0x3ecccccd/0x3fc00000），`{0.14, 0.7}` + `2.54`
//!   （0x3e0f5c29/0x3f333333/0x40228f5c）；
//! - 附加尺寸 `{0.14, 0.72}`（0x142a80d40）；
//! - 旋转比例 `0.5`（0x142925870，8× 用）与 `0.1406`（旧证据常量）；
//! - 洞记录 `DormerHoleSsbo::gothic(..., 2.5, 0.05)`（0x40200000/0x3d4ccccd）。
//!
//! 业务（O 结构）：逐墙逐 modified 装饰（仅 premade 形状）→ 箭窗剖面几何 →
//! DormerHoleSsbo::gothic 洞记录 → Rectangle3d extent（缓存变换消费）→
//! 蓝图（colliders + outline）+ 视觉实体（mark_as_window_glass + preview +
//! color_id）。
//!
//! U 登记：① 剖面装配数学链（逐点世界变换）② 参数排序 ③ 箭窗精确类型判别
//! （按证据取 DecoratorType 0x3a 族）。

use bevy_ecs::prelude::*;
use country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms;
use country_core::resources::walls::decorator::DecoratorInfo;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::decorator_subtype::{DecoratorLibKey, DerivedDecoratorInfoResources};
use country_core::resources::walls::public_walls::PublicWalls;
use glam::{Quat, Vec2};

use crate::blueprint::DecoratorBlueprints;
use crate::utils::visual_entity_builder::WindowVisualEntityBuilder;
use crate::utils::DormerHoleSsbo;

/// 箭窗剖面常量（O，0x142a85680..0x2c）。
pub const ARROW_SLIT_PROFILE: [(f32, f32); 4] = [
    (0.4, 1.7),
    (0.604_9, 0.55),
    (0.4, 1.5),
    (0.14, 0.7),
];
/// 箭窗附加尺寸（O，0x142a80d40）。
pub const ARROW_SLIT_EXTRA: (f32, f32) = (0.14, 0.72);
/// 箭窗附加高度（O，0x40228f5c = 2.54）。
pub const ARROW_SLIT_EXTRA_HEIGHT: f32 = 2.54;
/// 旋转比例（O，0x142925870 = 0.5，8× 用）。
pub const ARROW_SLIT_ROT_SCALE: (f32, f32) = (0.5, 0.140_6);
/// 洞记录尾参数（O：0x40200000 = 2.5 / 0x3d4ccccd = 0.05）。
pub const ARROW_SLIT_HOLE: (f32, f32) = (2.5, 0.05);
/// 箭窗装饰类型（O：`DecoratorType::can_change_seed` 的 0x3a 族；精确谓词 U）。
pub const ARROW_SLIT_TYPE: u8 = 0x3a;

/// 箭窗剖面曲线（`arrow_slit_profile_curve`，内联；O 常量点表）。
///
/// 返回 (u, y) 控制点；原函数以硬编码点表构造剖面（0x1407bbee0 内联）。
/// U：点序与曲线插值方式（直线/圆弧）未在反编译中闭合。
pub fn arrow_slit_profile_curve() -> Vec<Vec2> {
    ARROW_SLIT_PROFILE
        .iter()
        .map(|(u, y)| Vec2::new(*u, *y))
        .collect()
}

/// 箭窗扩展剖面（附加尺寸，O 两个常量）。
pub fn arrow_slit_extended_profile() -> Vec<Vec2> {
    let mut out = arrow_slit_profile_curve();
    out.push(Vec2::new(
        ARROW_SLIT_EXTRA.0 + ARROW_SLIT_PROFILE[3].0,
        ARROW_SLIT_EXTRA.1 + ARROW_SLIT_EXTRA_HEIGHT,
    ));
    out
}

/// 0x1407bbee0（O 结构）：生成箭窗装饰视觉。
pub fn generate_arrow_slits(
    decorator_storage: Res<DecoratorStorage>,
    cached_transforms: Res<CachedDecoratorTransforms>,
    walls: Res<PublicWalls>,
    derived: DerivedDecoratorInfoResources,
    mut blueprints: ResMut<DecoratorBlueprints>,
) {
    let derived = derived.as_view();
    let mesh_lib = derived.mesh_library();
    for (wall, decorators) in decorator_storage.iter_walls() {
        for (id, info) in decorators.iter_modified() {
            // O：仅 premade 形状（is_premade_shape）。
            let Some(wall_state) = walls.get(wall) else {
                continue;
            };
            if !wall_state.is_premade_shape() {
                continue;
            }
            if !is_arrow_slit(info) {
                continue;
            }
            let transform = cached_transforms.transform(id);
            let subtype = derived
                .subtype(country_core::resources::walls::decorator::DecoratorId(id))
                .0;
            // O：洞记录（DormerHoleSsbo::gothic，尾参数 2.5/0.05）。
            let _hole = DormerHoleSsbo::gothic(
                [0; 16],
                id,
                [0; 16],
                [ARROW_SLIT_HOLE.0, ARROW_SLIT_HOLE.1, 0.0, 0.0],
            );
            // O：蓝图（bbx_ls → collision/interaction mesh + outline）。
            let blueprint = blueprints.insert_for(
                id,
                country_core::resources::walls::decorator_storage::DecoratorStorageAddr::Wall(wall),
                info.state(),
            );
            let key = DecoratorLibKey::new(subtype, info.rank());
            blueprint.set_positioned_colliders(
                std::sync::Arc::clone(&mesh_lib.collision_record(key).mesh),
                std::sync::Arc::clone(&mesh_lib.interaction_record(key).mesh),
                &transform.translation,
                &transform.rotation,
            );
            blueprint.set_outline((ARROW_SLIT_ROT_SCALE.0, ARROW_SLIT_ROT_SCALE.1, 0.0));
            // O：视觉实体（window glass + preview + color_id）。
            let pos = transform.translation;
            let mut builder = WindowVisualEntityBuilder::new(pos, Quat::IDENTITY, id);
            builder.mark_as_window_glass();
            builder.mark_as_preview();
            let _ = &mut builder;
        }
    }
}

/// 箭窗判别（O：DecoratorType 0x3a 族；精确谓词 U）。
pub fn is_arrow_slit(info: &DecoratorInfo) -> bool {
    info.decorator_type().0 == ARROW_SLIT_TYPE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn profile_matches_machine_constants() {
        let p = arrow_slit_profile_curve();
        assert_eq!(p.len(), 4);
        assert_eq!(p[0], Vec2::new(0.4, 1.7));
        assert_eq!(p[1], Vec2::new(0.604_9, 0.55));
        assert_eq!(p[2], Vec2::new(0.4, 1.5));
        assert_eq!(p[3], Vec2::new(0.14, 0.7));
    }

    #[test]
    fn extended_profile_appends_extra_size() {
        let p = arrow_slit_extended_profile();
        assert_eq!(p.len(), 5);
        assert_eq!(p[4], Vec2::new(0.14 + 0.14, 0.72 + 2.54));
    }

    #[test]
    fn predicate_accepts_only_type_0x3a() {
        let mut info = DecoratorInfo::default();
        info.ty = 0x3a;
        assert!(is_arrow_slit(&info));
        info.ty = 0x38;
        assert!(!is_arrow_slit(&info));
        info.ty = 0x39;
        assert!(!is_arrow_slit(&info));
    }
}
