//! hint_deselect_tool / hint_disable_snapping / hint_unlink_windows / hint_undo /
//! hint_stairs / hint_terrain{,_focus,_steepness} / hint_roof_shape（B 主基线；碎而浅）。
//!
//! 证据（skeleton PDB 依赖注解）：
//! - hint_deselect_tool 0x1422853c0（L51）：DigitalActionState::pressed +
//!   AppMode::is_tool_selected + make_focus_or_camera_binding_hint + 0x140eaaa50。
//! - hint_disable_snapping 0x14227e140（L33）：0x1409e8d90 + 0x140eaaa50 +
//!   alternate<Option<KeyCode>> 0x141c4e4f0。
//! - hint_unlink_windows 0x14227d760（L12）：同 disable_snapping 组合。
//! - hint_undo 0x142284220（L24）：DigitalActionState::just_pressed + adjacent/together 组合。
//! - hint_stairs 0x142285a20（L0）：LMB 组合。
//! - hint_terrain_layering 0x142283cc0（L45）/ hint_terrain_focus 0x142283980（L45）/
//!   hint_terrain_steepness 0x142283640（L0）：地形族（make_focus_or_camera_binding_hint /
//!   make_lbm_drag_binding_hint 组合）。

use bevy_ecs::prelude::*;
use country_core::input::action_set::DigitalActionState;
use country_core::resources::time::GameTime;
use country_core::systems::mode_manager::AppMode;

use crate::exponential_back_off::ExponentialBackOff;
use crate::hint_roof_shape::make_lbm_drag_binding_hint;
use crate::{BindingHintDisplay, DisplayHintRequest, HintType, HintPlacement};

/// 提示输入（宿主输入层填充；DigitalActionState 语义）。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct HintInput {
    pub cancel_pressed: bool,
    pub undo_pressed: bool,
    pub tool_selected: bool,
    /// 光标世界位置（raycast 提示用）。
    pub cursor: glam::Vec3,
}

/// hint_deselect_tool（0x1422853c0）：选工具后提示"取消选择"（Esc）。
pub fn hint_deselect_tool(
    time: Res<GameTime>,
    input: Res<HintInput>,
    mode: Res<AppMode>,
    mut back_off: ResMut<ExponentialBackOff>,
    mut ev: EventWriter<DisplayHintRequest>,
) {
    let _ = &time;
    let selected = mode.is_tool_selected() || input.tool_selected;
    if selected && input.cancel_pressed && back_off.trigger(5) {
        let _ = ev.write(DisplayHintRequest {
            display: BindingHintDisplay::Key(crate::BindingKey::Esc),
            hint_type: HintType::DeselectTool,
            placement: HintPlacement::Top,
        });
    }
}

/// hint_disable_snapping（0x14227e140）：吸附禁用提示（Alt 键）。
pub fn hint_disable_snapping(
    time: Res<GameTime>,
    input: Res<HintInput>,
    mut back_off: ResMut<ExponentialBackOff>,
    mut ev: EventWriter<DisplayHintRequest>,
) {
    let _ = &time;
    if input.cancel_pressed && back_off.trigger(6) {
        let _ = ev.write(DisplayHintRequest {
            display: BindingHintDisplay::Key(crate::BindingKey::Alt) // Alt
                .alternate(BindingHintDisplay::Key(crate::BindingKey::Alt)),
            hint_type: HintType::DisableSnapping,
            placement: HintPlacement::Top,
        });
    }
}

/// hint_unlink_windows（0x14227d760）：窗解链提示（Alt 修饰）。
pub fn hint_unlink_windows(
    time: Res<GameTime>,
    input: Res<HintInput>,
    mut back_off: ResMut<ExponentialBackOff>,
    mut ev: EventWriter<DisplayHintRequest>,
) {
    let _ = &time;
    if input.cancel_pressed && back_off.trigger(7) {
        let _ = ev.write(DisplayHintRequest {
            display: BindingHintDisplay::Key(crate::BindingKey::Alt)
                .together(BindingHintDisplay::Key(crate::BindingKey::MouseLeft)),
            hint_type: HintType::UnlinkWindows,
            placement: HintPlacement::Top,
        });
    }
}

/// hint_undo（0x142284220）：撤销提示（Ctrl+Z adjacent）。
pub fn hint_undo(
    time: Res<GameTime>,
    input: Res<HintInput>,
    mut back_off: ResMut<ExponentialBackOff>,
    mut ev: EventWriter<DisplayHintRequest>,
) {
    let _ = &time;
    if input.undo_pressed && back_off.trigger(8) {
        let _ = ev.write(DisplayHintRequest {
            display: BindingHintDisplay::Key(crate::BindingKey::Ctrl)
                .adjacent(BindingHintDisplay::Key(crate::BindingKey::KeyZ)), // Z
            hint_type: HintType::Undo,
            placement: HintPlacement::Top,
        });
    }
}

/// hint_stairs（0x142285a20）：阶梯提示（LMB 拖动）。
pub fn hint_stairs(
    time: Res<GameTime>,
    input: Res<HintInput>,
    mut back_off: ResMut<ExponentialBackOff>,
    mut ev: EventWriter<DisplayHintRequest>,
) {
    let _ = &time;
    if input.tool_selected && back_off.trigger(9) {
        let _ = ev.write(DisplayHintRequest {
            display: make_lbm_drag_binding_hint(),
            hint_type: HintType::Stairs,
            placement: HintPlacement::Cursor,
        });
    }
}

/// hint_terrain_layering（0x142283cc0）：地形分层提示（LBM 拖动）。
pub fn hint_terrain_layering(
    time: Res<GameTime>,
    input: Res<HintInput>,
    mut back_off: ResMut<ExponentialBackOff>,
    mut ev: EventWriter<DisplayHintRequest>,
) {
    let _ = &time;
    if input.tool_selected && back_off.trigger(10) {
        let _ = ev.write(DisplayHintRequest {
            display: make_lbm_drag_binding_hint(),
            hint_type: HintType::TerrainLayering,
            placement: HintPlacement::Cursor,
        });
    }
}

/// hint_terrain_focus（0x142283980）：地形焦点提示。
pub fn hint_terrain_focus(
    time: Res<GameTime>,
    input: Res<HintInput>,
    mut back_off: ResMut<ExponentialBackOff>,
    mut ev: EventWriter<DisplayHintRequest>,
) {
    let _ = &time;
    if input.tool_selected && back_off.trigger(11) {
        let _ = ev.write(DisplayHintRequest {
            display: crate::hint_focused_edit::make_focus_or_camera_binding_hint(),
            hint_type: HintType::TerrainFocus,
            placement: HintPlacement::Cursor,
        });
    }
}

/// hint_terrain_steepness（0x142283640）：地形陡峭提示（LBM 拖动）。
pub fn hint_terrain_steepness(
    time: Res<GameTime>,
    input: Res<HintInput>,
    mut back_off: ResMut<ExponentialBackOff>,
    mut ev: EventWriter<DisplayHintRequest>,
) {
    let _ = &time;
    if input.tool_selected && back_off.trigger(12) {
        let _ = ev.write(DisplayHintRequest {
            display: make_lbm_drag_binding_hint(),
            hint_type: HintType::TerrainSteepness,
            placement: HintPlacement::Cursor,
        });
    }
}

/// DigitalActionState 引用标注（O：pressed/just_pressed 消费）。
#[allow(unused)]
fn action_state_contract(state: DigitalActionState) -> (bool, bool) {
    (state.pressed(), state.just_pressed())
}
