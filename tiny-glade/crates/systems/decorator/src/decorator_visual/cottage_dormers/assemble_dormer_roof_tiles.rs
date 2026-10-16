//! B `cottage_dormers/assemble_dormer_roof_tiles.rs` — 老虎窗屋顶瓦装配（D1）。
//!
//! 证据：fresh RE-MCP 反编译 0x1407cb1a0（generate_dormer_roofs，57KB）/
//! 0x14128db20（closure_env<7>，54KB）+ 骨架 callees（O）：
//! dormer_roof_height（内联）、is_bay_window（内联）、assemble_dormer_roof_tiles
//! 泛型实例、DormerKey::new（DecoratorLibKey::new 0x140b25c90）、
//! DecoratorArchivist::wall_decorators（0x140a1e190）、LazyTile::from_index
//! （0x140833250）、halver to_f16（0x140c8ffb0）、Roof::get_roof_shape_from_pos_ws_clamped
//! （0x1408e4750）、blueprint 链。
//!
//! 业务（O 结构）：生成老虎窗屋顶（按墙装饰 + 屋顶形状 → 瓦片装配
//! assemble_dormer_roof_tiles 泛型，瓦坐标 f16 打包）。
//! U 登记：① 瓦片装配数学主体 ② 参数排序。

use bevy_ecs::prelude::*;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes;

use crate::blueprint::DecoratorBlueprints;

/// 老虎窗屋顶高（内联，I；U：来源）。
pub fn dormer_roof_height(_info: &country_core::resources::walls::decorator::DecoratorInfo) -> f32 {
    0.0 // U。
}

/// 凸窗判定（内联，I；U：判别准则）。
pub fn is_bay_window(_info: &country_core::resources::walls::decorator::DecoratorInfo) -> bool {
    false // U。
}

/// 0x1407cb1a0（O 结构）：生成老虎窗屋顶（瓦片装配入口）。
pub fn generate_dormer_roofs(
    decorator_storage: Res<DecoratorStorage>,
    _derived: Res<DerivedDecoratorSubtypes>,
    _mesh: Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,
    mut blueprints: ResMut<DecoratorBlueprints>,
    _archivist: Res<country_core::resources::walls::decorator_archivist::DecoratorArchivist<()>>,
    _commands: Commands,
    _cached: Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,
    _transient: ResMut<crate::decorator_visual_state::DecoratorTransientVisualState>,
    _p10: u64,
) {
    for (_wall, decorators) in decorator_storage.iter_walls() {
        for (id, info) in decorators.iter_modified() {
            // O：凸窗跳过（is_bay_window）。
            if is_bay_window(info) {
                continue;
            }
            // U：瓦片装配主体（assemble_dormer_roof_tiles 泛型实例）。
            let _height = dormer_roof_height(info);
            let blueprint = crate::blueprint::DecoratorBlueprint::new();
            blueprints.insert(id, blueprint);
        }
    }
}
