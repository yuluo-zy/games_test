//! B `decorator_visual/cottage_wall_window.rs` — 农舍墙窗装饰视觉生成（D1）。
//!
//! 证据：fresh RE-MCP 反编译 0x1407ebfd0（generate_cottage_wall_windows，70KB）/
//! 0x1407ef7b0（flowerbed_xforms，2.6KB，O 全文）/ 0x1407ef990（add_lintels，
//! 14KB）+ 骨架 callees（O）：flowerbed_xforms ×2、add_lintels ×2、
//! subdivide_wall_decorator、has_cloth ×3、Rectangle3d::extent ×4、
//! xz_projective_rotation_bad（0x140c911d0）、Rectangle2d 几何、
//! WindowVisualEntityBuilder 链、blueprint 链。
//!
//! flowerbed_xforms（O，0x1407ef7b0 全文 + 常量 rne-read）：
//! 5 参面 {out, subtype u8, mode u8, ctx, mesh_lib}；跳表（0x142a8ae68 /
//! 0x142a8ae80，subtype-1<3 三键）按 mode 选网格名：
//! - mode 0/1：`decorators/window_cottage_{1x1,2x1,3x1}_flowerbed_locations`
//! - mode 3：`decorators/window_cottage_corner_{1x1,2x1,3x1}_flowerbed_locations`
//! AssetMeshLibrary::get_by_name → Mesh::positions_without_indices →
//! 元素数 = len÷3（Vec3）→ 逐花床变换输出。其余 mode/subtype fmt panic。
//!
//! U 登记：① 花床变换装配（FUN_141260b40 内部）② add_lintels 12 参面语义
//! ③ 参数排序。

use bevy_ecs::prelude::*;
use country_core::asset_libraries::mesh_library::AssetMeshLibrary;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes;
use glam::{Quat, Vec3};

use crate::blueprint::{DecoratorBlueprint, DecoratorBlueprints};
use crate::decorator_visual_state::DecoratorTransientVisualState;
use crate::utils::visual_entity_builder::WindowVisualEntityBuilder;

/// 农舍窗花床网格名（O：0x142a8ae68 跳表，mode<2）。
const FLOWERBED_NAMES: [&str; 3] = [
    "decorators/window_cottage_1x1_flowerbed_locations",
    "decorators/window_cottage_2x1_flowerbed_locations",
    "decorators/window_cottage_3x1_flowerbed_locations",
];
/// 角窗花床网格名（O：0x142a8ae80 跳表，mode==3）。
const CORNER_FLOWERBED_NAMES: [&str; 3] = [
    "decorators/window_cottage_corner_1x1_flowerbed_locations",
    "decorators/window_cottage_corner_2x1_flowerbed_locations",
    "decorators/window_cottage_corner_3x1_flowerbed_locations",
];

/// 0x1407ef7b0（O，全文）：花床变换（网格位置 → 变换列表）。
/// mode <2 → 窗花床网格；mode ==3 → 角窗花床网格；subtype-1<3 三键；其余
/// fmt panic。元素数 = positions.len()/3（Vec3）。
pub fn flowerbed_xforms(
    out: &mut Vec<crate::utils::IsoTransform>,
    subtype: u8,
    mode: u8,
    _ctx: u64,
    mesh_lib: &AssetMeshLibrary,
) {
    let names: &[&str; 3] = match mode {
        0 | 1 => &FLOWERBED_NAMES,
        3 => &CORNER_FLOWERBED_NAMES,
        _ => panic!("flowerbed_xforms: unexpected mode {}", mode),
    };
    let idx = subtype.wrapping_sub(1);
    if idx >= 3 {
        panic!("flowerbed_xforms: unexpected subtype {}", subtype);
    }
    let Some(mesh) = mesh_lib.get_by_name(names[idx as usize]) else { return };
    // O：positions（Vec3 序列；原 ÷3 = Vec3 计数与 core Mesh 返回一致）。
    let positions = mesh.positions_without_indices();
    for p in positions {
        out.push(crate::utils::IsoTransform::new(*p, Quat::IDENTITY));
    }
}

/// 0x1407ebfd0（O 结构）：生成农舍墙窗装饰视觉。
pub fn generate_cottage_wall_windows(
    decorator_storage: Res<DecoratorStorage>,
    _cached: Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,
    derived: Res<DerivedDecoratorSubtypes>,
    mesh_lib: Res<AssetMeshLibrary>,
    mut blueprints: ResMut<DecoratorBlueprints>,
    mut transient: ResMut<DecoratorTransientVisualState>,
) {
    for (_wall, decorators) in decorator_storage.iter_walls() {
        for (id, info) in decorators.iter_modified() {
            if !is_cottage_window(info) {
                continue;
            }
            let d = derived.get(id);
            // O：花床（flowerbed_xforms 消费，mode=0）。
            let mut beds = Vec::new();
            flowerbed_xforms(&mut beds, d.subtype().0, 0, 0, &mesh_lib);
            // O：蓝图 + 视觉实体。
            let mut blueprint = DecoratorBlueprint::new();
            blueprint.set_colliders(vec![d.collision_mesh()]);
            blueprint.set_outline((0.0, 0.0, 0.0));
            blueprints.insert(id, blueprint);
            let builder = WindowVisualEntityBuilder::new(
                Vec3::ZERO,
                Quat::IDENTITY,
                id,
            );
            let _ = &mut transient;
            let _ = &builder;
            // O：add_lintels（窗楣；12 参面 U）。
            add_lintels(id, 0, 0, d.subtype().0);
        }
    }
}

/// 农舍窗判别（I：DecoratorType 0x38 族，O 谓词位运算；见 core DecoratorType
/// is_window）。
fn is_cottage_window(info: &country_core::resources::walls::decorator::DecoratorInfo) -> bool {
    info.decorator_type().is_window()
}

/// 0x1407ef990（O，12 参面）：窗楣添加（FUN_14082b3b0 ×2 = 0x160B 元素 Vec
/// push；OverhangSupportFinderCache::new 0x140c925d0；DecoratorLibKey::new；
/// dst_as_wall 0x140833260）。U：12 参面精确语义 + 楣几何主体。
pub fn add_lintels(
    _id: u32,
    _p2: u64,
    _p3: u64,
    _subtype: u8,
) {
    // U：楣块几何待取证。
}
