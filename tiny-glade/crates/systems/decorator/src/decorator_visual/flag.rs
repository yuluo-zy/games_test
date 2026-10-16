//! B `decorator_visual/flag.rs` — 旗装饰视觉生成（D1）。
//!
//! 证据：fresh RE-MCP 反编译 0x1407ff6c0（generate_flags）+ 常量 rne-read +
//! 骨架 callees（O）。
//!
//! 机器码业务（O）：逐墙 modified 装饰 → CachedDecoratorTransforms::transform
//! （0x140a1aa20）→ DerivedDecoratorInfo::subtype（0x140b26970）→ **子类型判别**：
//! `subtype-0xd < 2` → 旗（墙旗/屋旗，0xd/0xe）；subtype == 0xf → 旗连接分支
//! （flag_connections 消费，见 0x1407df470）；其余走 fmt panic
//! （UNK_142a8d440 "flag.rs" 源文件）。旗分支 → DecoratorBlueprints::insert
//! （0x1408258b0）后**蓝图值写**：`{1, 0, pos_hi, pos_lo, pos, pos,
//! cloth_basis Vec2{0.5,-0.7071}, Vec2{0.7071,0.5}}`（+0x00 起，O 写面）。
//! 随后 DecoratorLibKey::new（0x140b25c90）+ 网格库查询（0x1411f7dc0）逐条目
//! 构建碰撞/实体；旗布几何 = 四元数 {0.5,-0.7071,0.7071,0.5} 与
//! 0x142a88840..0x8c 常量矩阵的旋转合成（O 数学链，语义：旗布四角）。
//!
//! U 登记：① 蓝图 +0x18 之后字段（4/5/6/7 槽）语义 ② 网格条目碰撞/实体装配
//! 细节 ③ 参数排序 SystemParam 映射。

use bevy_ecs::prelude::*;
use country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::decorator_subtype::{
    DecoratorLibKey, DecoratorMeshLibrary, DerivedDecoratorSubtypes,
};
use glam::{Quat, Vec3};

use crate::blueprint::DecoratorBlueprints;
use crate::decorator_visual_state::DecoratorTransientVisualState;
use crate::utils::visual_entity_builder::WindowVisualEntityBuilder;

/// 墙旗子类型（O：subtype-0xd<2 判别族的 0xd 侧；墙旗/屋旗的精确配对 U）。
const FLAG_WALL_SUBTYPE: u8 = 0x0d;
/// 屋旗子类型（O 判别族 0xe 侧）。
const FLAG_ROOF_SUBTYPE: u8 = 0x0e;
/// 旗连接子类型（O：subtype == 0xf → flag_connections 分支）。
const FLAG_CONNECTIONS_SUBTYPE: u8 = 0x0f;

/// 旗布基（O：蓝图 +0x10 写面 {0.5,-0.7071,0.7071,0.5} = 四元数四分量）。
const CLOTH_BASIS: [f32; 4] = [0.5, -0.70710677, 0.70710677, 0.5];

/// 0x1407ff6c0（O）：生成旗装饰视觉。
pub fn generate_flags(
    decorator_storage: Res<DecoratorStorage>,
    cached_transforms: Res<CachedDecoratorTransforms>,
    derived_subtypes: Res<DerivedDecoratorSubtypes>,
    mesh_lib: Res<DecoratorMeshLibrary>,
    mut blueprints: ResMut<DecoratorBlueprints>,
    mut transient: ResMut<DecoratorTransientVisualState>,
) {
    for (wall, decorators) in decorator_storage.iter_walls() {
        for (_id, info) in decorators.iter_modified() {
            let transform = cached_transforms.transform(info.decorator_id);
            let derived = derived_subtypes.get(info.decorator_id);
            let subtype = derived.subtype().0;
            // O 判别：subtype-0xd<2 → 旗；==0xf → 连接旗（flag_connections）；
            // 其余 → 原 fmt panic（flag.rs 源）。
            if subtype.wrapping_sub(FLAG_WALL_SUBTYPE) < 2 {
                // O 蓝图写面：{kind=1, 0, pos, pos, pos, cloth_basis}。
                let mut blueprint = crate::blueprint::DecoratorBlueprint::new();
                blueprint.u50 = 1;
                blueprint.flag_cloth_basis = CLOTH_BASIS;
                blueprint.transform = Some(transform);
                blueprints.insert(info.decorator_id, blueprint);
                // O：网格库查询（DecoratorLibKey::new + 查询链）→ 逐条目构建。
                let key = DecoratorLibKey::new(subtype, 0);
                if let Some(collision) = mesh_lib.collision_mesh(key) {
                    let b = blueprints.get_mut(info.decorator_id).unwrap();
                    b.set_colliders(vec![collision]);
                }
                // 旗布几何（O 数学链）：四元数 CLOTH_BASIS 旋转合成 + 常量矩阵
                // 0x142a88840..0x8c（U：精确装配语义）。
                let pos = transform_pos(&transform);
                let builder = WindowVisualEntityBuilder::new(pos, Quat::IDENTITY, info.decorator_id);
                match subtype {
                    FLAG_WALL_SUBTYPE => {
                        let _ = builder;
                    }
                    FLAG_ROOF_SUBTYPE => {
                        let _ = builder;
                    }
                    _ => {}
                }
                // U：实体落瞬态表（build 反编译缺失；D1 以 builder 记录承载）。
                let _ = &mut transient;
            } else if subtype == FLAG_CONNECTIONS_SUBTYPE {
                // 旗连接由 flag_connections 处理（O 分支）；本函数跳过。
                continue;
            } else {
                // 原 fmt panic（UNK_142a8d440 = flag.rs 源路径）。保持 panic 语义。
                panic!("generate_flags: unexpected decorator subtype {}", subtype);
            }
            let _ = wall;
        }
    }
}

/// U：缓存变换 → 位置（CachedDecoratorTransform 布局待取证）。
fn transform_pos(
    _t: &country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransform,
) -> Vec3 {
    Vec3::ZERO
}
