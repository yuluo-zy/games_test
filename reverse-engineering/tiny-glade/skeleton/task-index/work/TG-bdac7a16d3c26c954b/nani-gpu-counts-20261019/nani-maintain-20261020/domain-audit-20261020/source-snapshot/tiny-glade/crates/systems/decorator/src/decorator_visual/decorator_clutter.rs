//! B `decorator_visual/decorator_clutter.rs` — 装饰杂乱生成（D1）。
//!
//! 证据：fresh RE-MCP 反编译 0x1407e3370（generate_decorator_clutter，42KB）+
//! 骨架 callees（O）：clutter_mesh_ty_data、DecoratorStorage::iter + iter_modified
//! + as_wall、Roof::is_flat + roof_entity、CachedDecoratorTransforms::transform、
//! Rectangle3d::{set_size,height,length,width}、
//! blueprint {set_colliders,set_outline,insert}、
//! WindowVisualEntityBuilder::{new,has_birdspot,custom_shader}、
//! RaycastMeshData::new_cuboid。
//!
//! 2026-10-19 复核的新事实（O）：
//! - 类型谓词（0x1407e3370 行 564）：
//!   `(type - 0x38) < 8 && (type - 0x38) != 6` → type ∈ {0x38..0x3f} \ {0x3e}；
//! - 网格类型表 `clutter_mesh_ty_data`（0x1408b8e90）仍 U，用于 subtype 位表
//!   （行 585-588 `((bits >> 6) * 8) >> (subtype & 0x3f) & 1`）；
//! - 高度下限（行 688-690）：`height = max(height - 1.0, 0.1)`
//!   （0x14295a0e8 = -1.0、0x142925984 = 0.1）。
//!
//! U 登记：① 杂乱落点/尺寸计算 ② 参数排序 ③ clutter_mesh_ty_data 表。

use bevy_ecs::prelude::*;

use country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms;
use country_core::resources::walls::decorator::DecoratorInfo;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes;
use country_core::resources::walls::public_walls::PublicWalls;
use glam::Vec3;

use crate::blueprint::{DecoratorBlueprint, DecoratorBlueprints};
use crate::decorator_visual_state::DecoratorTransientVisualState;
use crate::utils::visual_entity_builder::WindowVisualEntityBuilder;

/// 杂乱类型下界（O：0x38）。
pub const CLUTTER_TYPE_LOW: u8 = 0x38;
/// 杂乱类型上界（O：`type-0x38 < 8` → 0x3f）。
pub const CLUTTER_TYPE_HIGH: u8 = 0x3f;
/// 排除的类型（O：`type-0x38 != 6` → 0x3e）。
pub const CLUTTER_TYPE_EXCLUDED: u8 = 0x3e;
/// 高度扣减（O：0x14295a0e8 = -1.0）。
pub const CLUTTER_HEIGHT_OFFSET: f32 = -1.0;
/// 高度下限（O：0x142925984 = 0.1）。
pub const CLUTTER_HEIGHT_MIN: f32 = 0.1;

/// 杂乱类型谓词（O：0x1407e3370 行 564）。
pub fn is_clutter_type(ty: u8) -> bool {
    ty.wrapping_sub(CLUTTER_TYPE_LOW) < 8 && ty != CLUTTER_TYPE_EXCLUDED
}

/// 杂乱高度（O：`max(h - 1.0, 0.1)`；输入 h 的来源 U）。
pub fn clutter_height(h: f32) -> f32 {
    let v = h + CLUTTER_HEIGHT_OFFSET;
    if v <= CLUTTER_HEIGHT_MIN {
        CLUTTER_HEIGHT_MIN
    } else {
        v
    }
}

/// 0x1407e3370（O 结构）：生成装饰杂乱（窗台/屋顶杂乱物）。
pub fn generate_decorator_clutter(
    decorator_storage: Res<DecoratorStorage>,
    cached_transforms: Res<CachedDecoratorTransforms>,
    walls: Res<PublicWalls>,
    derived: Res<DerivedDecoratorSubtypes>,
    mut blueprints: ResMut<DecoratorBlueprints>,
    mut transient: ResMut<DecoratorTransientVisualState>,
    _mesh: Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,
) {
    for (wall, decorators) in decorator_storage.iter() {
        // O：屋顶平坦校验（roof_entity + is_flat）。
        let _roof = walls.roof_entity(wall);
        for (id, info) in decorators.iter_modified() {
            // O：杂乱族谓词。
            if !is_clutter_type(info.decorator_type().0) {
                continue;
            }
            // O：杂乱网格数据（clutter_mesh_ty_data 0x1408b8e90，D1 承载于 utils；U）。
            let _mesh_ty = crate::utils::clutter_mesh_ty_data();
            let transform = cached_transforms.transform(id);
            let d = derived.get(id);
            // O：Rectangle3d 尺寸（set_size/height/length/width）。
            let mut rect = d.collision_mesh().bounds;
            let (min, max) = d.bbx_ls();
            let size = Vec3::new(max.x - min.x, clutter_height(max.y - min.y), max.z - min.z);
            rect.set_size(size);
            let _dims = rect.extent();
            // O：蓝图 + 视觉实体（has_birdspot + custom_shader）。
            let mut blueprint = DecoratorBlueprint::new();
            blueprint.set_colliders(vec![d.collision_mesh()]);
            blueprint.set_outline((0.0, 0.0, 0.0));
            blueprints.insert(id, blueprint);
            let mut builder =
                WindowVisualEntityBuilder::new(transform.translation, transform.rotation, id);
            builder.has_birdspot();
            let _ = &mut transient;
            let _ = &mut builder;
        }
    }
}

/// 装饰类型判别（O：0x38..0x3f 除 0x3e；`DecoratorType::spawns_clutter` 只到 0x38/0x39）。
pub fn spawns_clutter(info: &DecoratorInfo) -> bool {
    is_clutter_type(info.decorator_type().0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clutter_type_mask_matches_decomp() {
        for ty in 0x38..=0x3d {
            assert!(is_clutter_type(ty), "type {ty:#x}");
        }
        assert!(!is_clutter_type(0x3e));
        assert!(is_clutter_type(0x3f));
        assert!(!is_clutter_type(0x37));
        assert!(!is_clutter_type(0x40));
    }

    #[test]
    fn height_clamps_to_tenth() {
        assert_eq!(clutter_height(0.5), 0.1);
        assert_eq!(clutter_height(1.0), 0.1);
        assert!((clutter_height(2.0) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn spawns_clutter_predicate_uses_type() {
        let mut info = DecoratorInfo::default();
        info.ty = 0x38;
        assert!(spawns_clutter(&info));
        info.ty = 0x3e;
        assert!(!spawns_clutter(&info));
    }
}
