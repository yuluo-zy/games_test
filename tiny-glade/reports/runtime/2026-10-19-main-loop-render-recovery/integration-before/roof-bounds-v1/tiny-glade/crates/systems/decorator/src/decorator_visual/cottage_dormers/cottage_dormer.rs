//! B `cottage_dormers/cottage_dormer.rs` — 农舍老虎窗装饰（D1）。
//!
//! 证据：fresh RE-MCP 反编译 0x1407c7ac0（generate_cottage_dormers，55KB）/
//! 0x1407ca6c0（is_out_of_roof_bounds，7KB）+ 骨架 callees（O）：
//! cottage_dormer_dims（内联）、get_dormer_panel_xforms（内联）、
//! DormerHoleSsbo::cottage、Roof::get_roof_shape_from_pos_ws_clamped
//! （0x1408e4750）、DecoratorExtraInfo::as_window、Rectangle2d::perimeter ×2、
//! blueprint 链、WindowVisualEntityBuilder 链。
//!
//! is_out_of_roof_bounds（O 结构，5 参面，返回 u64 bool）：老虎窗超出屋顶边界
//! 判定（Rectangle2d::perimeter + 屋顶形状访问器）。
//! U 登记：① 面板/尺寸几何主体 ② 参数排序。

use bevy_ecs::prelude::*;
use country_core::resources::walls::decorator::DecoratorInfo;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes;

use crate::blueprint::{DecoratorBlueprint, DecoratorBlueprints};
use crate::decorator_visual_state::DecoratorTransientVisualState;
use crate::utils::visual_entity_builder::WindowVisualEntityBuilder;

/// 屋顶形状模式系数表（O：&UNK_142a87240 + mode*4；mode 0/1/2 = {2.1, 0.1, 0.15}）。
const ROOF_MODE_TABLE: [f32; 3] = [2.1, 0.1, 0.15];

/// 0x1407ca6c0（O，全文解码）：老虎窗是否超出屋顶边界。
///
/// 机器码（O）：param_1 = &dormer 状态（+0x10 u32、+0x14 f32、+0x18 u32）；
/// param_4 = 模式（>2 fmt panic，源 cottage_dormer.rs 0x142a875e8）；param_5 =
/// &屋顶形状（+0x08 i32 形状类型、+0x4c f32 高）。判断：
/// `pos_x*0.5 + table[mode]*roof_height + 0.1 + state.y <=
///  (roof_height + roof.h) - base`（base = 形状类型 0 → 0.5，否则 0.1）。
/// U：param_2/param_3 精确含义 + FUN_1408e3260 屋顶高访问器。
pub fn is_out_of_roof_bounds(
    state: &DormerRoofBoundsState,
    _param_2: f32,
    param_3: f32,
    mode: u32,
    roof: &RoofBoundsShape,
) -> bool {
    let mode = mode as usize;
    if mode >= 3 {
        // 原 fmt panic（internal error）。
        panic!("is_out_of_roof_bounds: mode out of range");
    }
    let base = if roof.shape_type == 0 { 0.5 } else { 0.1 };
    let roof_height = roof.height;
    let f17 = roof_height * ROOF_MODE_TABLE[mode] + param_3 * 0.5 + 0.1 + state.y;
    let f18 = (roof_height + roof.h) - base;
    f17 <= f18
}

/// 老虎窗边界状态（O 布局面：+0x10 u32、+0x14 f32、+0x18 u32）。
#[derive(Clone, Copy, Debug, Default)]
pub struct DormerRoofBoundsState {
    pub u10: u32,
    pub y: f32,
    pub u18: u32,
}

/// 屋顶形状（O 布局面：+0x08 形状类型、+0x4c 高）。
#[derive(Clone, Copy, Debug, Default)]
pub struct RoofBoundsShape {
    pub shape_type: i32,
    pub height: f32,
    pub h: f32,
}

/// 农舍老虎窗子类型（O：0x1407c7ac0 行 402 `subtype != 0x02 → skip`）。
pub const COTTAGE_DORMER_SUBTYPE: u8 = 0x02;

/// 农舍老虎窗尺寸（内联，I；U：维度来源）。未恢复返回 None。
pub fn cottage_dormer_dims(_info: &DecoratorInfo) -> Option<(f32, f32)> {
    None // U。
}

/// 老虎窗面板变换（内联，I；U：面板几何）。未恢复返回 None。
pub fn get_dormer_panel_xforms(_info: &DecoratorInfo) -> Option<Vec<crate::utils::IsoTransform>> {
    None // U。
}

/// 0x1407c7ac0（O 结构）：生成农舍老虎窗装饰视觉。
pub fn generate_cottage_dormers(
    decorator_storage: Res<DecoratorStorage>,
    derived: Res<DerivedDecoratorSubtypes>,
    mut blueprints: ResMut<DecoratorBlueprints>,
    mut transient: ResMut<DecoratorTransientVisualState>,
    cached: Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,
    _mesh_lib: Res<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,
) {
    for (_wall, decorators) in decorator_storage.iter_walls() {
        for (id, info) in decorators.iter_modified() {
            // O：农舍老虎窗子类型 = 2。
            if derived.get(id).subtype().0 != COTTAGE_DORMER_SUBTYPE {
                continue;
            }
            // O：超出屋顶边界判定（is_out_of_roof_bounds）。
            if is_out_of_roof_bounds(
                &DormerRoofBoundsState::default(),
                0.0,
                0.0,
                0,
                &RoofBoundsShape::default(),
            ) {
                continue;
            }
            let d = derived.get(id);
            let transform = cached.transform(id);
            // O：蓝图 + 视觉实体。
            let mut blueprint = DecoratorBlueprint::new();
            blueprint.set_colliders(vec![d.collision_mesh()]);
            blueprint.set_outline((0.0, 0.0, 0.0));
            blueprints.insert(id, blueprint);
            let builder =
                WindowVisualEntityBuilder::new(transform.translation, transform.rotation, id);
            let _ = &mut transient;
            let _ = &builder;
            let _ = info;
        }
    }
}
