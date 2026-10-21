//! B `decorator_visual/flag.rs` — 旗装饰视觉生成（D1）。
//!
//! 证据：fresh RE-MCP 反编译 0x1407ff6c0（generate_flags）+ 常量 rne-read +
//! 骨架 callees（O）。
//!
//! 机器码业务（O）：
//! - 逐墙 modified 装饰 → `CachedDecoratorTransforms::transform`（0x140a1aa20）→
//!   `DerivedDecoratorInfo::subtype`（0x140b26970）；
//! - **子类型判别**（0x1407ff6c0 行 483-517）：
//!   `subtype - 0xb <= 1`（0xb/0xc）→ 不产出；
//!   `subtype - 0xd < 2`（0xd/0xe）→ 旗分支；
//!   `subtype == 0xf` → 旗连接分支（flag_connections 消费）；
//!   其余 → fmt panic（UNK_142a8d440 源路径 `flag.rs`）；
//! - 旗分支 → `DecoratorBlueprints::insert`（0x1408258b0）后**蓝图值写**：
//!   `+0x00 = {1, 0}`；`+0x10..+0x1c = cloth 基四元数`（常量
//!   0x142a887e0..0xec = `{-0.0, -0.70710677, -0.0, +0.70710677}`，经
//!   rne-read 复核，是绕 Y 轴 -90° 的四元数，**不是**旧注释的
//!   `{0.5,-0.7071,0.7071,0.5}`）；`+0x20/+0x28/+0x30/+0x38` 为变换/附加槽；
//! - 随后 `DecoratorLibKey::new`（0x140b25c90）+ 网格库查询（0x1411f7dc0）
//!   逐条目构建碰撞/实体；旗布几何 = 该四元数与常量表 0x142a88840
//!   （`0.48`、`0.08` 等）的旋转/下垂合成（O 数学链，槽位语义 U）。
//!
//! U 登记：① 蓝图 +0x20..+0x40 槽位语义；② 网格条目碰撞/实体装配细节；
//! ③ 旗布四角局部偏移的精确映射（本实现仅落地可复核的四元数/下垂常量）。

use bevy_ecs::prelude::*;
use country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::decorator_subtype::{
    DecoratorLibKey, DerivedDecoratorInfoResources,
};
use glam::{Quat, Vec3};

use crate::blueprint::DecoratorBlueprints;
use crate::decorator_visual_state::DecoratorTransientVisualState;
use crate::utils::visual_entity_builder::WindowVisualEntityBuilder;

/// 子类型下界（O：`subtype-0xb <= 1` 的 0xb 侧）。
const FLAG_SUBTYPE_LOW: u8 = 0x0b;
/// 墙旗子类型（O：`subtype-0xd < 2` 的 0xd 侧）。
pub const FLAG_WALL_SUBTYPE: u8 = 0x0d;
/// 屋旗子类型（O 判别族 0xe 侧）。
pub const FLAG_ROOF_SUBTYPE: u8 = 0x0e;
/// 旗连接子类型（O：subtype == 0xf → flag_connections 分支）。
pub const FLAG_CONNECTIONS_SUBTYPE: u8 = 0x0f;
/// 旗布基四元数（O：0x142a887e0..0xec = {-0.0,-0.70710677,-0.0,0.70710677}）。
pub const FLAG_CLOTH_QUAT_XYZW: [f32; 4] = [-0.0, -0.707_106_77, -0.0, 0.707_106_77];
/// 旗布半宽（O：0x142a88848/0x142a88850 = 0.48）。
pub const FLAG_CLOTH_HALF_WIDTH: f32 = 0.48;
/// 旗布下垂系数（O：0x142a88890/0x142a88894 = 0.08）。
pub const FLAG_CLOTH_SAG: f32 = 0.08;

/// 旗子类型分类（O：0x1407ff6c0 行 483-517）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlagSubtype {
    /// 0xb/0xc：本函数不产出（`subtype-0xb <= 1` 提前跳过）。
    HandledElsewhere,
    /// 0xd/0xe：旗。
    Flag,
    /// 0xf：旗连接。
    Connections,
}

/// 分类子类型；越界值返回 None（调用方保持原 fmt panic）。
pub fn classify_flag_subtype(subtype: u8) -> Option<FlagSubtype> {
    if subtype.wrapping_sub(FLAG_SUBTYPE_LOW) <= 1 {
        Some(FlagSubtype::HandledElsewhere)
    } else if subtype.wrapping_sub(FLAG_WALL_SUBTYPE) < 2 {
        Some(FlagSubtype::Flag)
    } else if subtype == FLAG_CONNECTIONS_SUBTYPE {
        Some(FlagSubtype::Connections)
    } else {
        None
    }
}

/// 旗布基四元数（O 常量）。
pub fn flag_cloth_quat() -> Quat {
    Quat::from_xyzw(
        FLAG_CLOTH_QUAT_XYZW[0],
        FLAG_CLOTH_QUAT_XYZW[1],
        FLAG_CLOTH_QUAT_XYZW[2],
        FLAG_CLOTH_QUAT_XYZW[3],
    )
}

/// 旗布四角（O 结构 + U：局部偏移精确映射）。
///
/// 以基四元数旋转本地方向，XZ 半宽 0.48，并按 0.08 系数下垂（常量
/// 0x142a88848/0x142a88890）。角点局部偏移的精确槽位语义 U。
pub fn flag_cloth_corners(origin: Vec3, rotation: Quat) -> [Vec3; 4] {
    let q = rotation * flag_cloth_quat();
    let half = FLAG_CLOTH_HALF_WIDTH;
    let local = [
        Vec3::new(-half, 0.0, 0.0),
        Vec3::new(half, 0.0, 0.0),
        Vec3::new(-half, 0.0, 2.0 * half),
        Vec3::new(half, 0.0, 2.0 * half),
    ];
    let mut out = [Vec3::ZERO; 4];
    for (i, p) in local.iter().enumerate() {
        let mut r = origin + q * *p;
        // 下垂：布角低于原点（相对半宽的比例，常量 0.08）。
        r.y -= FLAG_CLOTH_SAG * half * 2.0;
        out[i] = r;
    }
    out
}

/// 0x1407ff6c0（O）：生成旗装饰视觉。
pub fn generate_flags(
    decorator_storage: Res<DecoratorStorage>,
    cached_transforms: Res<CachedDecoratorTransforms>,
    derived_subtypes: DerivedDecoratorInfoResources,
    mut blueprints: ResMut<DecoratorBlueprints>,
    mut transient: ResMut<DecoratorTransientVisualState>,
) {
    let derived = derived_subtypes.as_view();
    let mesh_lib = derived.mesh_library();
    for (wall, decorators) in decorator_storage.iter_walls() {
        for (id, info) in decorators.iter_modified() {
            let transform = cached_transforms.transform(id);
            let subtype = derived
                .subtype(country_core::resources::walls::decorator::DecoratorId(id))
                .0;
            match classify_flag_subtype(subtype) {
                Some(FlagSubtype::HandledElsewhere) => continue,
                Some(FlagSubtype::Connections) => continue,
                Some(FlagSubtype::Flag) => {
                    // O 蓝图写面：`+0x00 = {1, 0}`，`+0x10.. = cloth 基四元数`，
                    // 变换槽由缓存变换承载。
                    let blueprint = blueprints.insert_for(
                id,
                country_core::resources::walls::decorator_storage::DecoratorStorageAddr::Wall(wall),
                info.state(),
            );
                    blueprint.u50 = 1;
                    blueprint.flag_cloth_basis = FLAG_CLOTH_QUAT_XYZW;
                    blueprint.transform = Some(transform);
                    // O：网格库查询（DecoratorLibKey::new + 查询链）→ 逐条目构建。
                    let key = DecoratorLibKey::new(subtype, info.rank());
                    blueprint.set_positioned_colliders(
                        std::sync::Arc::clone(&mesh_lib.collision_record(key).mesh),
                        std::sync::Arc::clone(&mesh_lib.interaction_record(key).mesh),
                        &transform.translation,
                        &transform.rotation,
                    );
                    // O：旗布几何（四元数 + 0.48/0.08 常量）。
                    let pos = transform.translation;
                    let corners = flag_cloth_corners(pos, transform.rotation);
                    let _ = corners;
                    // O：视觉实体 builder（沿用 id/位置；mesh 名静态表 U）。
                    let mut builder = WindowVisualEntityBuilder::new(pos, transform.rotation, id);
                    match subtype {
                        FLAG_WALL_SUBTYPE => {
                            builder.flag_wall_cloth();
                        }
                        FLAG_ROOF_SUBTYPE => {
                            builder.flag_roof_cloth();
                        }
                        _ => {}
                    }
                    let _ = &mut transient;
                }
                None => {
                    // 原 fmt panic（UNK_142a8d440 = flag.rs 源路径）。保持 panic 语义。
                    panic!("generate_flags: unexpected decorator subtype {}", subtype);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subtype_classification_matches_decomp() {
        assert_eq!(
            classify_flag_subtype(0x0b),
            Some(FlagSubtype::HandledElsewhere)
        );
        assert_eq!(
            classify_flag_subtype(0x0c),
            Some(FlagSubtype::HandledElsewhere)
        );
        assert_eq!(classify_flag_subtype(0x0d), Some(FlagSubtype::Flag));
        assert_eq!(classify_flag_subtype(0x0e), Some(FlagSubtype::Flag));
        assert_eq!(
            classify_flag_subtype(0x0f),
            Some(FlagSubtype::Connections)
        );
        assert_eq!(classify_flag_subtype(0x10), None);
        assert_eq!(classify_flag_subtype(0x0a), None);
    }

    #[test]
    fn cloth_quat_is_unit_and_rotates_y() {
        let q = flag_cloth_quat();
        assert!((q.length() - 1.0).abs() < 1e-6);
        // 绕 Y -90°：+X → +Z? 以结果正交/长度验证，具体轴向由常量锚定。
        let v = q * Vec3::X;
        assert!((v.length() - 1.0).abs() < 1e-5);
        assert!(v.y.abs() < 1e-6);
    }

    #[test]
    fn cloth_corners_are_distinct_and_below_origin() {
        let c = flag_cloth_corners(Vec3::new(1.0, 2.0, 3.0), Quat::IDENTITY);
        assert!(c.iter().all(|p| p.y < 2.0));
        assert_ne!(c[0], c[1]);
        // 半宽 0.48*2 = 0.96 的横向跨度在某个轴上体现。
        assert!(c.iter().any(|p| (p - c[0]).length() > 0.5));
    }

    #[test]
    fn cloth_quat_constant_layout() {
        // O 常量：x=-0.0, y=-0.70710677, z=-0.0, w=+0.70710677。
        assert_eq!(FLAG_CLOTH_QUAT_XYZW[0], 0.0);
        assert!((FLAG_CLOTH_QUAT_XYZW[1] + std::f32::consts::FRAC_1_SQRT_2).abs() < 1e-6);
        assert_eq!(FLAG_CLOTH_QUAT_XYZW[2], 0.0);
        assert!((FLAG_CLOTH_QUAT_XYZW[3] - std::f32::consts::FRAC_1_SQRT_2).abs() < 1e-6);
    }
}
