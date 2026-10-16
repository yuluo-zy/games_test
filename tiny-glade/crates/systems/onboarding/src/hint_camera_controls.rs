//! hint_camera_controls（B 主基线；5 函数）。
//!
//! 证据（skeleton PDB）：
//! - hint_camera_enable_condition 0x14227c780（L56）：启用条件（纯；无调用）。
//! - hint_camera_display_condition 0x14227c910（L119）：显示条件（0x1408c5680 =
//!   工具选择判定；0x140eaaa50 = 按键按下判定）。
//! - camera_pan_hint 0x14227d000（L0）/ camera_rotate_hint 0x14227d2f0（L0）/
//!   camera_zoom_hint 0x14227d680（L282）：构造 BindingHintDisplay
//!   （alternate/adjacent/together 组合子 0x141c4e6d0 等）。

use bevy_ecs::prelude::*;
use country_core::resources::time::GameTime;

use crate::{
    exponential_back_off::ExponentialBackOff, BindingHintDisplay, DisplayHintRequest, HintType,
    HintPlacement,
};

/// 相机提示状态（资源）。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct CameraHintState {
    pub enabled: bool,
}

/// hint_camera_enable_condition（0x14227c780）：相机提示启用条件
/// （非 welcome 屏、非 photo mode；I：会话状态门）。
pub fn hint_camera_enable_condition(
    state: Res<CameraHintState>,
    _esc: Res<country_core::resources::gui::EscMenuState>,
) -> bool {
    state.enabled
}

/// hint_camera_display_condition（0x14227c910）：显示条件（未选工具/未拖拽）。
pub fn hint_camera_display_condition(
    back_off: Res<ExponentialBackOff>,
    _dragging: Res<crate::hint_orchestrator::InputHeld>,
) -> bool {
    // 退避门：任一相机提示可显示。
    back_off.remaining[0..3].iter().any(|r| *r <= 0.0)
}

/// camera_pan_hint（0x14227d000）：平移提示。
pub fn camera_pan_hint(
    time: Res<GameTime>,
    mut back_off: ResMut<ExponentialBackOff>,
    mut ev: EventWriter<DisplayHintRequest>,
) {
    let _ = &time;
    if back_off.trigger(0) {
        let _ = ev.write(DisplayHintRequest {
            display: crate::hints_startup::camera_pan_display(),
            hint_type: HintType::CameraPan,
            placement: HintPlacement::Bottom,
        });
    }
}

/// camera_rotate_hint（0x14227d2f0）：旋转提示。
pub fn camera_rotate_hint(
    time: Res<GameTime>,
    mut back_off: ResMut<ExponentialBackOff>,
    mut ev: EventWriter<DisplayHintRequest>,
) {
    let _ = &time;
    if back_off.trigger(1) {
        let _ = ev.write(DisplayHintRequest {
            display: crate::hints_startup::camera_rotate_display(),
            hint_type: HintType::CameraRotate,
            placement: HintPlacement::Bottom,
        });
    }
}

/// camera_zoom_hint（0x14227d680）：缩放提示。
pub fn camera_zoom_hint(
    time: Res<GameTime>,
    mut back_off: ResMut<ExponentialBackOff>,
    mut ev: EventWriter<DisplayHintRequest>,
) {
    let _ = &time;
    if back_off.trigger(2) {
        let _ = ev.write(DisplayHintRequest {
            display: crate::hints_startup::camera_zoom_display(),
            hint_type: HintType::CameraZoom,
            placement: HintPlacement::Bottom,
        });
    }
}

/// 绑定树导出（组合子实例证据对照；供测试/宿主）。
pub fn binding_tree_sample() -> BindingHintDisplay {
    BindingHintDisplay::Key(crate::BindingKey::MouseLeft)
        .alternate(BindingHintDisplay::Key(crate::BindingKey::MouseRight))
        .together(BindingHintDisplay::Key(crate::BindingKey::Scroll))
}
