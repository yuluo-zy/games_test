//! hint_roof_shape（O：hint_roof_shape 0x142284ed0 / make_lbm_drag_binding_hint
//! 0x142285270）。
//!
//! `hint_roof_shape`：遍历事件；GameProfile+0x1ff==0 且事件字节==1 且 [+1]!=1、
//! `*in_RCX==0`（无工具）、legacy_raycast 命中、geo_src_to_edit_subject==2、
//! PublicWalls::roof_entity 有效且 Roof::is_flat()==0 → make_lbm_drag_binding_hint 写
//! DisplayHintRequest（tag 7）。
//! `make_lbm_drag_binding_hint`：together(steam, alternate(Graphic("hint/lmb-move")))。

use bevy_ecs::prelude::*;

/// 屋顶形状提示条件（O：hints_startup 初值 2B [0]=profile[0x1ff],[1]=profile[0x1ff]）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct HintRoofShapeConditions {
    pub hint_done: bool,
    pub event_byte: u8,
}

/// make_lbm_drag_binding_hint（O 0x142285270）：together(steam, alternate(Graphic
/// ("hint/lmb-move")))。
/// **未实现**：SteamInput + lang 串未落库。
pub fn make_lbm_drag_binding_hint() -> crate::BindingHintDisplay {
    crate::BindingHintDisplay::Empty
}

/// hint_roof_shape（O 0x142284ed0）：见模块头。
/// **未实现**：NotifyHintSystemRoofShape + geo_src + RaycastWorld 返回链未落库。
pub fn hint_roof_shape(
    _state: ResMut<HintRoofShapeConditions>,
    _profile: Res<country_core::game_profile::GameProfile>,
    _walls: Res<country_core::resources::walls::public_walls::PublicWalls>,
    _raycast: Res<country_core::systems::collision::world::RaycastWorld>,
    _query: Query<&country_core::resources::roofs::roof_shape::Roof>,
) {
    // 未实现：见模块头。
}
