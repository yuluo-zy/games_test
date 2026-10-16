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
//! 反编译观察（O）：参数面 6 参；逐墙表（0x14B 条目 {u32 id, u32 类型, ...}）
//! 过滤 + 变换表（0x48B 步长）+ 逐墙 modified 装饰（iter_modified 落
//! uStack_208）→ 洞记录 / 蓝图 / 构建器。常量 0x3fa66666（1.3）/0x3f900000
//! （1.125）参与几何阈值。
//!
//! U 登记：① 类型谓词（位表过滤式）精确式 ② 参数排序的 SystemParam 映射
//! ③ 阈值常量用途 ④ 变换表字段语义。以上均待 reviewer/re-analyst 复核。

use bevy_ecs::prelude::*;
use country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms;
use country_core::resources::walls::decorator::DecoratorInfo;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::decorator_subtype::{
    DecoratorLibKey, DecoratorMeshLibrary,
};
use country_core::resources::walls::public_walls::PublicWalls;
use glam::{Quat, Vec3};

use crate::blueprint::{DecoratorBlueprint, DecoratorBlueprints};
use crate::decorator_visual_state::DecoratorTransientVisualState;
use crate::utils::visual_entity_builder::WindowVisualEntityBuilder;

/// 0x1407be730（O 结构）：生成活板门装饰的视觉内容。
///
/// 业务链（O callee 顺序）：逐墙表过滤 → 屋顶平坦校验（Roof::is_flat +
/// roof_entity）→ 缓存变换（CachedDecoratorTransforms::transform）→ 逐墙
/// modified 装饰（Decorators::iter_modified）→ 洞记录（DormerHoleSsbo::cottage）
/// → 蓝图（set_colliders + set_outline + DecoratorBlueprints::insert）→ 视觉
/// 实体（WindowVisualEntityBuilder::new + custom_shader）。
///
/// 几何常量（O）：1.3（0x3fa66666）、1.125（0x3f900000）参与
/// Rectangle2d::contains_point_padding / Circle2d::intersect_circle 阈值
/// （精确语义 U）。
pub fn generate_trap_doors(
    decorator_storage: Res<DecoratorStorage>,
    cached_transforms: Res<CachedDecoratorTransforms>,
    walls: Res<PublicWalls>,
    mesh_lib: Res<DecoratorMeshLibrary>,
    mut blueprints: ResMut<DecoratorBlueprints>,
    mut transient: ResMut<DecoratorTransientVisualState>,
) {
    for (wall, decorators) in decorator_storage.iter_walls() {
        // U：逐墙表过滤（0x14B 条目 {u32,u32,...} 位表谓词）在反编译为原始
        // 字段访问；D1 以 iter_modified 面承载（等价迭代域，谓词语义 U）。
        for (_id, info) in decorators.iter_modified() {
            // U：类型谓词精确式待取证；此处登记 O 观察（活板门 = 屋顶/地形系
            // 装饰，Roof::is_flat + roof_entity 消费）。
            let _roof_flat_check = walls.roof_entity(wall);
            // 缓存变换（O callee）。
            let _transform = cached_transforms.transform(info.decorator_id);
            let pos = Vec3::ZERO; // U：CachedDecoratorTransform 布局待取证。
            // 洞记录（O callee：DormerHoleSsbo::cottage；载荷 U）。
            let _hole = crate::utils::DormerHoleSsbo::cottage(
                [0; 16],
                info.decorator_id,
                [0; 16],
                [0.0, 0.0],
                0.0,
            );
            // 蓝图（O callee：set_colliders + set_outline + insert）。
            let mut blueprint = DecoratorBlueprint::new();
            blueprint.set_colliders(Vec::new());
            blueprint.set_outline((0.0, 0.0, 0.0));
            blueprints.insert(info.decorator_id, blueprint);
            // 视觉实体（O callee：new + custom_shader）。
            let builder = WindowVisualEntityBuilder::new(pos, Quat::IDENTITY, info.decorator_id);
            let _ = &mut transient;
            let _ = &builder;
            let _ = &mesh_lib;
            let _ = (DecoratorLibKey::new(0, 0),);
        }
    }
}

/// 类型谓词登记占位：反编译显示位表过滤（`(bitset >> (t & 0x3f)) & 1`），
/// 精确类型值 U。本函数不参与业务（仅登记证据面）。
#[allow(dead_code)]
fn _type_bitset_predicate(info: &DecoratorInfo) -> bool {
    let _ = info;
    false
}
