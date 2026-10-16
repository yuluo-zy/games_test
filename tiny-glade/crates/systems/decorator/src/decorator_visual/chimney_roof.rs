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
//! 业务（O 结构）：逐墙（iter）逐 modified 装饰 → 屋顶目的地（dst_as_roof 族）
//! → as_chimney 载荷（f32 烟囱高）→ 缓存变换 → 立方体碰撞（new_cuboid）→
//! 蓝图（set_colliders + insert）。
//! U 登记：① 烟囱高/位置几何 ② 参数排序。

use bevy_ecs::prelude::*;
use country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes;
use country_core::resources::walls::public_walls::PublicWalls;

use crate::blueprint::{DecoratorBlueprint, DecoratorBlueprints};

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
            // O 判别：as_chimney（非烟囱 panic）+ 目的地面。
            if info.dst_tag() != 2 {
                // 屋顶烟囱只看屋顶目的地（O 分支；U 精确判别）。
                continue;
            }
            let _height = info.extra().as_chimney().0; // U：f32 载荷语义。
            let _transform = cached_transforms.transform(id);
            let _subtype = derived.get(id).subtype();
            // O：立方体碰撞（RaycastMeshData::new_cuboid）。
            let _collider = crate::utils::new_cuboid_collider(1.0, 1.0, 1.0); // U：尺寸。
            // O：蓝图（set_colliders + insert）。
            let mut blueprint = DecoratorBlueprint::new();
            blueprint.set_colliders(Vec::new());
            blueprints.insert(id, blueprint);
        }
    }
}
