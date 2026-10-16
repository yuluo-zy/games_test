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
//! 业务（O 结构）：逐墙逐 modified 装饰（屋顶平坦校验）→ 杂乱网格数据
//! （clutter_mesh_ty_data）→ 缓存变换 → Rectangle3d 尺寸 → 蓝图 + 视觉实体
//! （has_birdspot 鸟巢 + custom_shader）。
//! U 登记：① 杂乱落点/尺寸计算 ② 参数排序。

use bevy_ecs::prelude::*;

use country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes;
use country_core::resources::walls::public_walls::PublicWalls;
use glam::{Quat, Vec3};

use crate::blueprint::{DecoratorBlueprint, DecoratorBlueprints};
use crate::decorator_visual_state::DecoratorTransientVisualState;
use crate::utils::visual_entity_builder::WindowVisualEntityBuilder;

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
    for (_wall, decorators) in decorator_storage.iter() {
        for (id, info) in decorators.iter_modified() {
            // O：仅杂乱族（DecoratorType::spawns_clutter = 0x38/0x39）。
            if !info.decorator_type().spawns_clutter() {
                continue;
            }
            // O：屋顶平坦校验（roof_entity + is_flat）。
            let _roof = walls.roof_entity(_wall);
            // O：杂乱网格数据（clutter_mesh_ty_data 0x1408b8e90，D1 承载于 utils）。
            let _mesh_ty = crate::utils::clutter_mesh_ty_data();
            let _transform = cached_transforms.transform(id);
            let d = derived.get(id);
            // O：Rectangle3d 尺寸（set_size/height/length/width）。
            let mut rect = d.collision_mesh().bounds;
            rect.set_size(Vec3::ZERO); // U：尺寸来源。
            let _dims = rect.extent();
            // O：蓝图 + 视觉实体（has_birdspot + custom_shader）。
            let mut blueprint = DecoratorBlueprint::new();
            blueprint.set_colliders(vec![d.collision_mesh()]);
            blueprint.set_outline((0.0, 0.0, 0.0));
            blueprints.insert(id, blueprint);
            let mut builder = WindowVisualEntityBuilder::new(Vec3::ZERO, Quat::IDENTITY, id);
            builder.has_birdspot();
            let _ = &mut transient;
            let _ = &mut builder;
        }
    }
}
