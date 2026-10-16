//! B `decorator_visual/chimney_wall.rs` — 墙烟囱装饰视觉生成（D1）。
//!
//! 证据：fresh RE-MCP 反编译 0x1407c21b0（generate_wall_chimneys，63KB）+ 骨架
//! callees（O）：DecoratorStorage::iter、iter_modified、
//! DerivedDecoratorInfo::subtype、DecoratorLibKey::new、wall_space::distance
//! （0x140c91df0）、DecoratorExtraInfo::as_chimney、is_valid_body_placement
//! （本文件子函数）、blueprint 链、RaycastMeshData::new_cuboid。
//!
//! 业务（O 结构）：逐墙逐 modified 装饰 → 墙目的地（dst tag<2）→ 放置合法
//! 校验（is_valid_body_placement，U 准则）→ as_chimney 载荷 → 墙空间距离
//! （落点）→ 蓝图 + 碰撞 + 视觉实体。
//! U 登记：① is_valid_body_placement 准则 ② 参数排序。

use bevy_ecs::prelude::*;
use country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes;
use country_core::resources::walls::public_walls::PublicWalls;

use crate::blueprint::{DecoratorBlueprint, DecoratorBlueprints};

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
            // 放置合法性（O callee：is_valid_body_placement）。
            if !is_valid_body_placement(info) {
                continue;
            }
            let _height = info.extra().as_chimney().0;
            let _transform = cached_transforms.transform(id);
            let _subtype = derived.get(id).subtype();
            let _wall_space = 0.0f32; // U：wall_space::distance 消费面。
            let _collider = crate::utils::new_cuboid_collider(1.0, 1.0, 1.0);
            let mut blueprint = DecoratorBlueprint::new();
            blueprint.set_colliders(Vec::new());
            blueprints.insert(id, blueprint);
        }
    }
}

/// 墙烟囱体放置合法性（O callee，I 语义：墙高/边界校验；精确准则 U）。
fn is_valid_body_placement(info: &country_core::resources::walls::decorator::DecoratorInfo) -> bool {
    // U：校验准则待取证（D1 以未过滤承载，宁可真生成面）。
    let _ = info;
    true
}
