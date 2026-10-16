//! B `decorator_visual/gothic_balcony_door.rs` — 哥特阳台门装饰视觉生成（D1）。
//!
//! 证据：fresh RE-MCP 反编译 0x1407f0900（generate_gothic_balcony_doors，60KB）/
//! 0x1407f3340（check_for_door_stairs，6.5KB）+ 骨架 callees（O）：
//! check_for_door_stairs ×2、get_door_holes、float_grid bilinear_sample
//! （0x140a14df0）、DecoratorLibKey 族、blueprint 链、WindowVisualEntityBuilder 链。
//!
//! 业务（O 结构）：逐墙逐 modified 装饰 → 阳台门判别 → 门前楼梯检测
//! （check_for_door_stairs）→ 门洞（get_door_holes）→ 地形高度采样
//! （bilinear_sample）→ 蓝图 + 视觉实体。
//! check_for_door_stairs（O，5 参面，返回 u64）：门前是否有梯（梯装配消费）。
//! U 登记：① 门/梯几何主体 ② 参数排序。

use bevy_ecs::prelude::*;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes;
use glam::{Quat, Vec3};

use crate::blueprint::{DecoratorBlueprint, DecoratorBlueprints};
use crate::decorator_visual_state::DecoratorTransientVisualState;
use crate::utils::visual_entity_builder::WindowVisualEntityBuilder;

/// 0x1407f3340（O，5 参面，返回 u64）：检查门前楼梯。
/// U：梯检测准则 + 返回载荷。
pub fn check_for_door_stairs(
    _p1: u64,
    _p2: u64,
    _p3: &[f32],
    _p4: u64,
    _p5: u64,
) -> u64 {
    0 // U。
}

/// 0x1407f0900（O 结构）：生成哥特阳台门装饰视觉。
pub fn generate_gothic_balcony_doors(
    decorator_storage: Res<DecoratorStorage>,
    derived: Res<DerivedDecoratorSubtypes>,
    mut blueprints: ResMut<DecoratorBlueprints>,
    mut transient: ResMut<DecoratorTransientVisualState>,
    _mesh_lib: Res<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,
) {
    for (_wall, decorators) in decorator_storage.iter_walls() {
        for (id, info) in decorators.iter_modified() {
            // U：阳台门判别谓词。
            if !is_balcony_door(info) {
                continue;
            }
            let d = derived.get(id);
            // O：门前楼梯检测（check_for_door_stairs）。
            let _stairs = check_for_door_stairs(0, 0, &[], 0, 0);
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

/// 阳台门判别（U：谓词精确式待取证）。
fn is_balcony_door(info: &country_core::resources::walls::decorator::DecoratorInfo) -> bool {
    let _ = info;
    true
}
