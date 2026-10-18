//! 将窗口空间位移应用到游标；捕获期间保持锚点，原始增量仅供相机等动作使用。
use bevy_ecs::prelude::*;
use glam::Vec2;

use super::translate_input_to_cursor_moved::clamp_window_position;
use crate::{
    input::action_set::ActionSetStates,
    resources::{
        mouse_capture::CursorCapturedPosition,
        mouse_terrain_raycast::CursorPositionSS,
        window::{CursorMovedScreenSpace, WindowSize},
    },
};

/// 录制和回放使用的整数像素位置。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CursorPosition {
    pub x: u32,
    pub y: u32,
}
impl CursorPosition {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
    pub fn as_vec2(self) -> Vec2 {
        Vec2::new(self.x as f32, self.y as f32)
    }
}

/// 在游戏消费者之前生成本帧屏幕坐标和事件。
pub fn update_cursor_position(
    states: Res<ActionSetStates>,
    captured: Res<CursorCapturedPosition>,
    window: Res<WindowSize>,
    mut cursor: ResMut<CursorPositionSS>,
    mut events: EventWriter<CursorMovedScreenSpace>,
) {
    let next = captured.0.unwrap_or(cursor.0 + states.analog[1]);
    let next = clamp_window_position(next, window.as_vec2());
    cursor.0 = next;
    events.write(CursorMovedScreenSpace(next));
}
