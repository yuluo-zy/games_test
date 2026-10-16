//! B `decorator_visual/gothic_wall_window.rs` — 哥特墙窗装饰视觉生成（D1）。
//!
//! 证据：fresh RE-MCP 反编译 0x1407e87b0（generate_gothic_wall_windows，70KB）/
//! 0x1407eb500（get_gothic_hat_bones，6.3KB）/ 0x1407eb160
//! （calc_gothic_half_dormer_profile_ws，4.9KB）+ 骨架 callees（O）：
//! get_window_holes、WallStyle::is_halftimbered（0x1408c5670）、
//! DecoratorLibKey::new ×3、Rectangle3d::extent ×2、swizzlers、inverse_lerp、
//! WindowVisualEntityBuilder 链、blueprint 链。
//!
//! get_gothic_hat_bones（O 结构）：6 参面 {out f32*, ctx(+600 计数), info
//! (+0x44 rank), f32, u32, f32}；rank==1 → 加常量 _UNK_1429ddce8；inverse_lerp
//! 消费 + 哥特剖面 helper 链（FUN_14126a040/0x141269710）。
//! U 登记：① 剖面/hat 数学主体 ② 参数排序。

use bevy_ecs::prelude::*;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes;
use glam::{Quat, Vec3};

use crate::blueprint::{DecoratorBlueprint, DecoratorBlueprints};
use crate::decorator_visual_state::DecoratorTransientVisualState;
use crate::utils::visual_entity_builder::WindowVisualEntityBuilder;

/// 0x1407eb500（O 结构）：哥特窗帽（hat）骨骼点生成。
/// U：骨骼装配主体。
pub fn get_gothic_hat_bones(
    _out: &mut [f32],
    _ctx: &mut u32,
    _rank: u8,
    _f4: f32,
    _u5: u32,
    _f6: f32,
) {
    // U：inverse_lerp + 剖面 helper 装配待取证。
}

/// 0x1407eb160（O 结构）：哥特半老虎窗剖面世界空间计算。
/// U：剖面曲线主体。
pub fn calc_gothic_half_dormer_profile_ws(
    _a: u64,
    _b: u64,
) -> Vec3 {
    Vec3::ZERO // U。
}

/// 0x1407e87b0（O 结构）：生成哥特墙窗装饰视觉。
pub fn generate_gothic_wall_windows(
    decorator_storage: Res<DecoratorStorage>,
    derived: Res<DerivedDecoratorSubtypes>,
    mut blueprints: ResMut<DecoratorBlueprints>,
    mut transient: ResMut<DecoratorTransientVisualState>,
) {
    for (_wall, decorators) in decorator_storage.iter_walls() {
        for (id, info) in decorators.iter_modified() {
            // O：哥特窗 = DecoratorType 0x39 族（core is_window 位运算）。
            if info.decorator_type().0 != 0x39 {
                continue;
            }
            let d = derived.get(id);
            // O：窗洞（get_window_holes，U）。
            let _holes = get_window_holes(id);
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

/// 窗洞记录（O callee，I；U：洞几何主体）。
fn get_window_holes(_id: u32) -> Vec<u8> {
    Vec::new() // U。
}
