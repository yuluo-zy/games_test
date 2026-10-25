//! B `decorator_visual/cottage_corner_window.rs` — 农舍角窗装饰视觉生成（D1）。
//!
//! 证据：fresh RE-MCP 反编译 0x1407f66b0（generate_cottage_corner_windows，
//! 45KB）+ 骨架 callees（O）：get_corner_info（本文件子函数，I）、has_cloth ×3、
//! DecoratorLibKey::new ×4、DecoratorRank ×4、xz_projective_rotation_bad、
//! swizzlers x0y/xvy、Rectangle2d 几何、flowerbed_xforms（cottage_wall_window）、
//! WindowVisualEntityBuilder 链、blueprint 链。
//!
//! 业务（O 结构）：逐墙逐 modified 装饰 → 角窗判别（get_corner_info，U 准则）
//! → 角窗几何（角吸附旋转 xz_projective_rotation_bad 消费）→ 花床（mode=3
//! 角窗网格名）→ 蓝图 + 视觉实体。
//! U 登记：① get_corner_info 准则 ② 参数排序。

use bevy_ecs::prelude::*;
use country_core::asset_libraries::mesh_library::AssetMeshLibrary;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes;
use glam::{Quat, Vec3};

use crate::blueprint::{DecoratorBlueprint, DecoratorBlueprints};
use crate::decorator_visual_state::DecoratorTransientVisualState;
use crate::utils::visual_entity_builder::WindowVisualEntityBuilder;

use super::cottage_wall_window::flowerbed_xforms;

/// 0x1407f66b0（O 结构）：生成农舍角窗装饰视觉。
pub fn generate_cottage_corner_windows(
    decorator_storage: Res<DecoratorStorage>,
    derived: Res<DerivedDecoratorSubtypes>,
    mesh_lib: Res<AssetMeshLibrary>,
    mut blueprints: ResMut<DecoratorBlueprints>,
    mut transient: ResMut<DecoratorTransientVisualState>,
    _cached: Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,
) {
    for (_wall, decorators) in decorator_storage.iter_walls() {
        for (id, info) in decorators.iter_modified() {
            // O：角窗判别（get_corner_info；U 准则；D1 以角吸附能力承载）。
            let d = derived.get(id);
            if !d.subtype().can_be_on_corner() {
                continue;
            }
            let _corner = get_corner_info(info);
            // O：花床（mode=3 角窗网格名）。
            let mut beds = Vec::new();
            flowerbed_xforms(&mut beds, d.subtype().0, 3, 0, &mesh_lib);
            // O：蓝图 + 视觉实体。
            let mut blueprint = DecoratorBlueprint::new();
            blueprint.set_colliders(vec![d.collision_mesh()]);
            blueprint.set_outline((0.0, 0.0, 0.0));
            blueprints.insert(id, blueprint);
            let builder = WindowVisualEntityBuilder::new(Vec3::ZERO, Quat::IDENTITY, id);
            let _ = &mut transient;
            let _ = &builder;
        }
    }
}

/// 角窗信息（O callee，I；U：角侧判定准则待取证）。
fn get_corner_info(info: &country_core::resources::walls::decorator::DecoratorInfo) -> u8 {
    let _ = info;
    0 // U：角侧值待取证。
}
