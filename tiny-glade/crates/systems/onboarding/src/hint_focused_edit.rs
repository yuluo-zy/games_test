//! hint_focused_edit / hint_clutter_focus_edit（B 主基线）。
//!
//! 证据（skeleton PDB）：
//! - hint_focused_edit_condition 0x14227da70（L49）；hint_focused_edit_hint 0x14227dd60
//!   （L81）：EventWriter + PublicWalls::get + RaycastWorld::legacy_raycast +
//!   geo_src_to_edit_subject + make_focus_or_camera_binding_hint。
//! - hint_clutter_focused_edit_condition 0x141c3d890（L42）；hint_clutter_focused_edit_hint
//!   0x141c3db80（L74）：RaycastWorld::raycast + geo_src_to_edit_subject_w_deco_clutter +
//!   make_focus_or_camera_binding_hint。
//! - make_focus_or_camera_binding_hint 0x141c3de00（L0）：焦点编辑提示绑定
//!   （LMB 组合子 0x141c4e6d0 / HintGraphicParams 0x141c4e8b0）。

use bevy_ecs::prelude::*;
use country_core::resources::walls::public_walls::PublicWalls;
use country_core::systems::collision::world::RaycastWorld;

use crate::{
    exponential_back_off::ExponentialBackOff, BindingHintDisplay, DisplayHintRequest,
    HintGraphicParams, HintType, HintPlacement,
};

/// 编辑焦点（geo_src_to_edit_subject 语义；宿主装配）。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct EditFocus {
    /// 当前编辑目标（墙/装饰物；None = 无焦点）。
    pub target: Option<u64>,
    pub cursor: glam::Vec3,
}

/// 焦点编辑提示绑定（make_focus_or_camera_binding_hint 0x141c3de00）：
/// LMB（或光标）焦点编辑。
pub fn make_focus_or_camera_binding_hint() -> BindingHintDisplay {
    BindingHintDisplay::Key(crate::BindingKey::MouseLeft)
        .alternate(BindingHintDisplay::Graphic(HintGraphicParams {
            graphic: "camera".to_string(),
            size: glam::Vec2::splat(1.0),
            color: 0,
        }))
}

/// hint_focused_edit_condition（0x14227da70）：有编辑焦点且未拖拽。
pub fn hint_focused_edit_condition(
    focus: Res<EditFocus>,
    back_off: Res<ExponentialBackOff>,
) -> bool {
    focus.target.is_some() && back_off.remaining[3] <= 0.0
}

/// hint_focused_edit_hint（0x14227dd60）：焦点编辑提示（raycast 命中 → 事件）。
#[allow(clippy::too_many_arguments)]
pub fn hint_focused_edit_hint(
    focus: Res<EditFocus>,
    walls: Res<PublicWalls>,
    raycast: Res<RaycastWorld>,
    mut back_off: ResMut<ExponentialBackOff>,
    mut ev: EventWriter<DisplayHintRequest>,
) {
    // O：legacy_raycast（光标方向）+ PublicWalls::get 判定命中目标。
    let Some(target) = focus.target else {
        return;
    };
    let hit = crate::raycast_seam::legacy_raycast_hit(&raycast, focus.cursor);
    let _ = walls.get(country_core::resources::walls::WallId(target));
    if hit.is_some() && back_off.trigger(3) {
        let _ = ev.write(DisplayHintRequest {
            display: make_focus_or_camera_binding_hint(),
            hint_type: HintType::FocusEdit,
            placement: HintPlacement::Cursor,
        });
    }
}

/// hint_clutter_focused_edit_condition（0x141c3d890）：杂物焦点编辑条件。
pub fn hint_clutter_focused_edit_condition(
    focus: Res<EditFocus>,
    back_off: Res<ExponentialBackOff>,
) -> bool {
    focus.target.is_some() && back_off.remaining[4] <= 0.0
}

/// hint_clutter_focused_edit_hint（0x141c3db80）：杂物焦点编辑提示
/// （geo_src_to_edit_subject_w_deco_clutter 语义）。
pub fn hint_clutter_focused_edit_hint(
    focus: Res<EditFocus>,
    raycast: Res<RaycastWorld>,
    mut back_off: ResMut<ExponentialBackOff>,
    mut ev: EventWriter<DisplayHintRequest>,
) {
    let Some(_target) = focus.target else {
        return;
    };
    let hit = crate::raycast_seam::raycast_hit(&raycast, focus.cursor);
    if hit.is_some() && back_off.trigger(4) {
        let _ = ev.write(DisplayHintRequest {
            display: make_focus_or_camera_binding_hint(),
            hint_type: HintType::FocusEdit,
            placement: HintPlacement::Cursor,
        });
    }
}
