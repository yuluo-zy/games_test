//! B `decorator_visual/gothic_dormer.rs` — 哥特老虎窗装饰（D1）。
//!
//! 证据：fresh RE-MCP 反编译 0x1407c7500（gothic_dormer_profile_curve，2.7KB，
//! O 全文）+ 0x1407c5650（generate_gothic_dormers，57KB）+ 常量 rne-read
//! （0x142a867d0/0x142a86810/0x142a86840 剖面控制点）。
//!
//! `gothic_dormer_profile_curve`（O，全文解码）：按 rank（param_2+0x44）返回
//! 哥特剖面控制点 Vec<Vec2>（game Vec {cap,ptr,len}）：
//! - rank 1：6 点 = 5 常量 + {0.35, 0.0} 终点（半宽 0.35）
//! - rank 2：7 点 = 6 常量 + {0.588, 0.0}（半宽 0.588，0x3f170a3d）
//! - rank 3：9 点 = 8 常量 + {0.76, 0.0}（半宽 0.76，0x3f428f5c）
//! 其余 rank → fmt panic（UNK_142a86dc8 = gothic_dormer.rs 源）。
//!
//! 控制点（O）：rank1 {(0,0.93),(0.15,0.78),(0.26,0.6),(0.32,0.4),(0.35,0.19)}；
//! rank2 {(0,1.07),(0.18,0.95),(0.34,0.81),(0.46,0.63),(0.54,0.44),(0.59,0.23)}；
//! rank3 {(0,1.45),(0.19,1.34),(0.36,1.19),(0.5,1.02),(0.61,0.82),(0.69,0.62),
//! (0.73,0.4),(0.76,0.18)}（机器码仅拷贝前 5/6/8 点）。
//!
//! U 登记：① generate_gothic_dormers 几何主体 ② 参数排序。

use bevy_ecs::prelude::*;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::decorator_subtype::{DecoratorLibKey, DerivedDecoratorInfoResources};
use glam::Vec2;

use crate::blueprint::{DecoratorBlueprint, DecoratorBlueprints};
use crate::decorator_visual_state::DecoratorTransientVisualState;
use crate::utils::visual_entity_builder::WindowVisualEntityBuilder;

/// rank 1 剖面（5 常量 + 终点 {0.35,0}）。
const RANK1_PROFILE: [Vec2; 6] = [
    Vec2::new(0.0, 0.93),
    Vec2::new(0.15, 0.78),
    Vec2::new(0.26, 0.6),
    Vec2::new(0.32, 0.4),
    Vec2::new(0.35, 0.19),
    Vec2::new(0.35, 0.0),
];
/// rank 2 剖面（6 常量 + 终点 {0.588,0}）。
const RANK2_PROFILE: [Vec2; 7] = [
    Vec2::new(0.0, 1.07),
    Vec2::new(0.18, 0.95),
    Vec2::new(0.34, 0.81),
    Vec2::new(0.46, 0.63),
    Vec2::new(0.54, 0.44),
    Vec2::new(0.59, 0.23),
    Vec2::new(0.588, 0.0),
];
/// rank 3 剖面（8 常量 + 终点 {0.76,0}）。
const RANK3_PROFILE: [Vec2; 9] = [
    Vec2::new(0.0, 1.45),
    Vec2::new(0.19, 1.34),
    Vec2::new(0.36, 1.19),
    Vec2::new(0.5, 1.02),
    Vec2::new(0.61, 0.82),
    Vec2::new(0.69, 0.62),
    Vec2::new(0.73, 0.4),
    Vec2::new(0.76, 0.18),
    Vec2::new(0.76, 0.0),
];

/// 哥特老虎窗子类型（O：0x1407c5650 行 430 `subtype != 0x08 → skip`）。
pub const GOTHIC_DORMER_SUBTYPE: u8 = 0x08;

/// 哥特老虎窗剖面控制点（0x1407c7500，O 全文）。
pub fn gothic_dormer_profile_curve(rank: u8) -> Vec<Vec2> {
    match rank {
        1 => RANK1_PROFILE.to_vec(),
        2 => RANK2_PROFILE.to_vec(),
        3 => RANK3_PROFILE.to_vec(),
        _ => panic!("gothic_dormer_profile_curve: unexpected rank {}", rank),
    }
}

/// 0x1407c5650（O 结构）：生成哥特老虎窗装饰视觉。
pub fn generate_gothic_dormers(
    decorator_storage: Res<DecoratorStorage>,
    derived: DerivedDecoratorInfoResources,
    mut blueprints: ResMut<DecoratorBlueprints>,
    mut transient: ResMut<DecoratorTransientVisualState>,
    cached: Res<
        country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms,
    >,
) {
    let derived = derived.as_view();
    let mesh_lib = derived.mesh_library();
    for (_wall, decorators) in decorator_storage.iter_walls() {
        for (id, info) in decorators.iter_modified() {
            // O：哥特老虎窗子类型 = 8（0x1407c5650 行 430）。
            if derived
                .subtype(country_core::resources::walls::decorator::DecoratorId(id))
                .0
                != GOTHIC_DORMER_SUBTYPE
            {
                continue;
            }
            // O：剖面（rank 驱动）。
            let _profile = gothic_dormer_profile_curve(info.rank());
            // O：蓝图 + 视觉实体。
            let transform = cached.transform(id);
            let mut blueprint = DecoratorBlueprint::new();
            let key = DecoratorLibKey::new(GOTHIC_DORMER_SUBTYPE, info.rank());
            blueprint.set_positioned_colliders(
                std::sync::Arc::clone(&mesh_lib.collision_record(key).mesh),
                std::sync::Arc::clone(&mesh_lib.interaction_record(key).mesh),
                &transform.translation,
                &transform.rotation,
            );
            blueprint.set_outline((0.0, 0.0, 0.0));
            blueprints.insert(id, blueprint);
            let builder =
                WindowVisualEntityBuilder::new(transform.translation, transform.rotation, id);
            let _ = &mut transient;
            let _ = &builder;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn profile_tables_end_at_documented_half_widths() {
        assert_eq!(gothic_dormer_profile_curve(1).last(), Some(&Vec2::new(0.35, 0.0)));
        assert_eq!(gothic_dormer_profile_curve(2).last(), Some(&Vec2::new(0.588, 0.0)));
        assert_eq!(gothic_dormer_profile_curve(3).last(), Some(&Vec2::new(0.76, 0.0)));
    }

    #[test]
    #[should_panic]
    fn unexpected_rank_panics() {
        let _ = gothic_dormer_profile_curve(4);
    }
}
