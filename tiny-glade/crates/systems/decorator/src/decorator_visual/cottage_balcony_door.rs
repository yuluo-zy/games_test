//! B `decorator_visual/cottage_balcony_door.rs` — 农舍阳台门装饰视觉生成（D1）。
//!
//! 证据：fresh RE-MCP 反编译 0x1407f3da0（generate_cottage_balcony_doors，65KB）
//! / 0x1407f5e70（add_door_autoclutter，5.8KB）+ 骨架 callees（O）：
//! DecoratorLibKey::new（3×）、DecoratorInfo dst 访问、add_door_autoclutter 递归、
//! GladeType::is_autumn、Rectangle2d/Circle2d 几何、Affine3Packed::from_cols、
//! WindowVisualEntityBuilder 链、blueprint 链。
//!
//! 参数面（O，Ghidra 7 参）：param_1..param_7 复合（storage/变换缓存/蓝图库/
//! 网格库/构建器上下文；U 精确排序待复核）。
//! U 登记：① 阳台门几何生成主体（内联矩形/门框装配细节）② 参数映射
//! ③ add_door_autoclutter 的落点语义。

use bevy_ecs::prelude::*;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary;
use country_core::resources::walls::public_walls::PublicWalls;
use glam::{Quat, Vec3};

use crate::blueprint::DecoratorBlueprints;
use crate::decorator_visual_state::DecoratorTransientVisualState;
use crate::utils::visual_entity_builder::WindowVisualEntityBuilder;

/// 0x1407f3da0（O 结构）：生成农舍阳台门装饰视觉。
///
/// 业务链（O callee）：逐墙逐 modified 装饰 → DecoratorLibKey（3×new =
/// 碰撞/交互/描边键）→ 阳台门几何（Rectangle2d/Circle2d + Affine3Packed 装配，
/// 细节 U）→ 蓝图（set_colliders/set_outline + insert）→ 视觉实体
/// （WindowVisualEntityBuilder 链）→ add_door_autoclutter 附加杂乱
/// （GladeType::is_autumn 分支，O callee）。
pub fn generate_cottage_balcony_doors(
    decorator_storage: Res<DecoratorStorage>,
    _walls: Res<PublicWalls>,
    _mesh_lib: Res<DecoratorMeshLibrary>,
    mut blueprints: ResMut<DecoratorBlueprints>,
    mut transient: ResMut<DecoratorTransientVisualState>,
    _commands: Commands,
    _cached: Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,
) {
    for (_wall, decorators) in decorator_storage.iter_walls() {
        for (_id, info) in decorators.iter_modified() {
            // U：门型判别（0x1407f3da0 内联谓词，精确式待取证）。
            if !is_balcony_door(info) {
                continue;
            }
            // 蓝图（O callee）。
            let mut blueprint = crate::blueprint::DecoratorBlueprint::new();
            blueprint.set_colliders(Vec::new());
            blueprint.set_outline((0.0, 0.0, 0.0));
            blueprints.insert(info.decorator_id, blueprint);
            // 视觉实体（O callee：WindowVisualEntityBuilder 链）。
            let builder = WindowVisualEntityBuilder::new(
                Vec3::ZERO,
                Quat::IDENTITY,
                info.decorator_id,
            );
            let _ = &mut transient;
            let _ = &builder;
            // add_door_autoclutter（O callee；autumn 分支 GladeType::is_autumn）。
            add_door_autoclutter(info.decorator_id, 0.0, 0);
        }
    }
}

/// 0x1407f5e70（O，5.8KB）：阳台门附加杂乱生成（参数面 6 参；
/// FUN_14128cc30 ×2 + FUN_14078ca00 ×2 + GladeType::is_autumn 分支）。
/// U：杂乱落点/autumn 载荷语义待取证。
pub fn add_door_autoclutter(
    _decorator_id: u32,
    _u: f32,
    _wall_side: u32,
) {
    // U：autoclutter 附加逻辑主体待取证（O callee：is_autumn 分支 +
    // 两组装 helper）。
}

/// U：阳台门判别谓词（内联，精确式待取证）。
fn is_balcony_door(info: &country_core::resources::walls::decorator::DecoratorInfo) -> bool {
    let _ = info;
    true // U：谓词待取证（当前不筛除，宁可真生成面）。
}
