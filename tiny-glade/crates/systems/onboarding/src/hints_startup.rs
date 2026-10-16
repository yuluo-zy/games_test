//! hints_startup（0x14227b450，源码 L11）。
//!
//! 证据（skeleton PDB）：raw_vec::RawVecInner —— 启动时预置提示状态（欢迎屏/初始提示
//! 队列）。语义：`WelcomeScreenState`（core resources/onboarding.rs）未关闭时排队
//! 初始提示（相机控制族）。

use bevy_ecs::prelude::*;

use crate::hint_camera_controls::CameraHintState;
use crate::{BindingKey, BindingHintDisplay, HintType, HintPlacement};

/// 启动提示状态（资源）。
#[derive(Resource, Clone, Debug, Default)]
pub struct StartupHints {
    pub queued: Vec<HintType>,
    pub shown: bool,
}

/// hints_startup（0x14227b450）：启动预置相机提示队列。
pub fn hints_startup(
    mut state: ResMut<StartupHints>,
    mut cam_state: ResMut<CameraHintState>,
    mut ev: EventWriter<crate::DisplayHintRequest>,
) {
    if state.shown {
        return;
    }
    // 相机控制提示族（O：启动提示 = pan/rotate/zoom 三连）。
    let hints = [
        (HintType::CameraPan, camera_pan_display()),
        (HintType::CameraRotate, camera_rotate_display()),
        (HintType::CameraZoom, camera_zoom_display()),
    ];
    for (ty, display) in hints {
        if !state.queued.contains(&ty) {
            state.queued.push(ty);
        }
        let _ = ev.write(crate::DisplayHintRequest {
            display,
            hint_type: ty,
            placement: HintPlacement::Bottom,
        });
    }
    state.shown = true;
    cam_state.enabled = true;
}

/// 相机平移绑定（O：MouseButton alternate 组合）。
pub fn camera_pan_display() -> BindingHintDisplay {
    BindingHintDisplay::Key(BindingKey::MouseMiddle).alternate(BindingHintDisplay::Key(
        BindingKey::KeyA
    ))
}

/// 相机旋转绑定（O：LMB + 拖动组合）。
pub fn camera_rotate_display() -> BindingHintDisplay {
    BindingHintDisplay::Key(BindingKey::MouseRight)
        .together(BindingHintDisplay::Key(BindingKey::MouseLeft))
}

/// 相机缩放绑定（O：滚轮 + 按键 adjacent）。
pub fn camera_zoom_display() -> BindingHintDisplay {
    BindingHintDisplay::Key(BindingKey::Scroll).adjacent(BindingHintDisplay::Key(
        BindingKey::KeyW
    ))
}
