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
//! 常量（O）：箭窗剖面 {0.4,1.7}（0x142a85680 首对）、{0.588,0.55}（次对）、
//! {0.4,1.5}、{0.14,0.7}+2.54、{0.14,0.72}（0x142a80d40）、旋转比例
//! {0.5,0.1406}（0x142925870，8×用）。子类型判别：0x3a 族（0x1407bbee0
//! 494 行 `0x3a < subtype` 分支）。
//!
//! 业务（O 结构）：逐墙逐 modified 装饰（仅 premade 形状）→ 箭窗剖面几何 →
//! DormerHoleSsbo::gothic 洞记录 → Rectangle3d extent（缓存变换消费）→
//! 蓝图（colliders + outline）+ 视觉实体（mark_as_window_glass + preview +
//! color_id）。
//! U 登记：① 剖面装配数学链 ② 参数排序。

use bevy_ecs::prelude::*;
use country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms;
use country_core::resources::walls::decorator::DecoratorInfo;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes;
use country_core::resources::walls::public_walls::PublicWalls;
use glam::{Quat, Vec3};

use crate::blueprint::DecoratorBlueprints;
use crate::utils::visual_entity_builder::WindowVisualEntityBuilder;
use crate::utils::DormerHoleSsbo;

/// 箭窗剖面常量（O，0x142a85680..0x2c）。
const ARROW_SLIT_PROFILE: [(f32, f32); 4] = [
    (0.4, 1.7),
    (0.588_0, 0.55),
    (0.4, 1.5),
    (0.14, 0.7),
];
/// 箭窗附加尺寸（O，0x142a80d40）。
const ARROW_SLIT_EXTRA: (f32, f32) = (0.14, 0.72);
/// 旋转比例（O，0x142925870，8× 用）。
const ARROW_SLIT_ROT_SCALE: (f32, f32) = (0.5, 0.140_6);

/// 0x1407bbee0（O 结构）：生成箭窗装饰视觉。
pub fn generate_arrow_slits(
    decorator_storage: Res<DecoratorStorage>,
    cached_transforms: Res<CachedDecoratorTransforms>,
    walls: Res<PublicWalls>,
    derived: Res<DerivedDecoratorSubtypes>,
    mut blueprints: ResMut<DecoratorBlueprints>,
) {
    for (wall, decorators) in decorator_storage.iter_walls() {
        for (id, info) in decorators.iter_modified() {
            // O：仅 premade 形状（is_premade_shape）。
            let Some(_premade) = walls.get(wall) else {
                continue;
            };
            if !_premade.is_premade_shape() {
                continue;
            }
            if !is_arrow_slit(info) {
                continue;
            }
            let transform = cached_transforms.transform(id);
            let d = derived.get(id);
            // O：洞记录（DormerHoleSsbo::gothic，剖面常量消费）。
            let _hole = DormerHoleSsbo::gothic(
                [0; 16],
                id,
                [0; 16],
                [
                    ARROW_SLIT_PROFILE[0].0,
                    ARROW_SLIT_PROFILE[0].1,
                    ARROW_SLIT_EXTRA.0,
                    ARROW_SLIT_EXTRA.1,
                ],
            );
            // O：蓝图（bbx_ls → collision/interaction mesh + outline）。
            let mut blueprint = crate::blueprint::DecoratorBlueprint::new();
            blueprint.set_colliders(vec![d.collision_mesh()]);
            blueprint.set_outline((ARROW_SLIT_ROT_SCALE.0, ARROW_SLIT_ROT_SCALE.1, 0.0));
            blueprints.insert(id, blueprint);
            // O：视觉实体（window glass + preview + color_id）。
            let pos = transform_pos(&transform);
            let mut builder = WindowVisualEntityBuilder::new(pos, Quat::IDENTITY, id);
            builder.mark_as_window_glass();
            builder.mark_as_preview();
            let _ = &mut builder;
        }
    }
}

/// 箭窗判别（I：子类型 0x3a 族，O 分支 494 行；精确谓词 U）。
fn is_arrow_slit(info: &DecoratorInfo) -> bool {
    // U：谓词精确式待取证。
    let _ = info;
    true
}

/// U：缓存变换 → 位置。
fn transform_pos(
    _t: &country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransform,
) -> Vec3 {
    Vec3::ZERO
}
