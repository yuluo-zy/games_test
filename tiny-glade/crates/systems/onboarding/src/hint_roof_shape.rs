//! hint_roof_shape（B 主基线；2 函数）。
//!
//! 证据（skeleton PDB）：hint_roof_shape 0x142284ed0（L0）：EventWriter + Roof::is_flat +
//! PublicWalls::roof_entity + RaycastWorld::legacy_raycast + geo_src_to_edit_subject +
//! make_lbm_drag_binding_hint；make_lbm_drag_binding_hint 0x142285270（L126）：
//! 0x1409e8d80/0x1409e8d90（DigitalActionState）+ 0x141c4e8b0（HintGraphicParams 组合）+
//! together 0x141c4f230。语义：点击平顶屋顶 → 提示拖拽改屋顶形状。

use bevy_ecs::prelude::*;
use country_core::resources::roofs::roof_shape::Roof;
use country_core::resources::walls::public_walls::PublicWalls;
use country_core::systems::collision::world::RaycastWorld;

use crate::{
    exponential_back_off::ExponentialBackOff, BindingHintDisplay, DisplayHintRequest,
    HintGraphicParams, HintType, HintPlacement,
};

/// LBM 拖动绑定（make_lbm_drag_binding_hint 0x142285270）：LMB + 拖动图标。
pub fn make_lbm_drag_binding_hint() -> BindingHintDisplay {
    BindingHintDisplay::Key(crate::BindingKey::MouseLeft).together(
        BindingHintDisplay::Graphic(HintGraphicParams {
            graphic: "drag".to_string(),
            size: glam::Vec2::splat(1.0),
            color: 0,
        }),
    )
}

/// hint_roof_shape（0x142284ed0）：raycast 命中平顶屋顶 → 屋顶形状提示。
#[allow(clippy::too_many_arguments)]
pub fn hint_roof_shape(
    walls: Res<PublicWalls>,
    raycast: Res<RaycastWorld>,
    mut back_off: ResMut<ExponentialBackOff>,
    mut ev: EventWriter<DisplayHintRequest>,
    mut roof_query: Query<&Roof>,
    input: Res<crate::hint_deselect_tool::HintInput>,
) {
    if !input.tool_selected {
        return;
    }
    // O：legacy_raycast（光标方向）+ PublicWalls::roof_entity + Roof::is_flat 门控。
    let cursor = input_position(&input);
    let hit = crate::raycast_seam::legacy_raycast_hit(&raycast, cursor);
    let Some(_dist) = hit else {
        return;
    };
    let mut flat = false;
    for wall in walls.all() {
        if let Some(roof_entity) = walls.roof_entity(wall.0) {
            if let Ok(roof) = roof_query.get(roof_entity) {
                if roof.is_flat() {
                    flat = true;
                    break;
                }
            }
        }
    }
    if flat && back_off.trigger(13) {
        let _ = ev.write(DisplayHintRequest {
            display: make_lbm_drag_binding_hint(),
            hint_type: HintType::RoofShape,
            placement: HintPlacement::Cursor,
        });
    }
}

/// 光标位置（宿主输入缝；HintInput 扩展）。
fn input_position(input: &crate::hint_deselect_tool::HintInput) -> glam::Vec3 {
    input.cursor
}
