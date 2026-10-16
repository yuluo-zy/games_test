//! `update_cursor_position.rs` — 游标位置合并（B PDB 签名 @0x1413ee760，正文全内联）。
//!
//! 权威签名（O，`cursor-update-procedures.json` into_configs @0x1413ee760）：
//! `(Res<ActionSetStates>, Res<CursorCapturedPosition>, ResMut<CursorPositionSS>,
//!  EventWriter<CursorMovedScreenSpace>)` —— 消费 `ActionSetStates.analog[0]`
//! （`input_space_cursor_move`，由 `translate_input_to_cursor_moved` 折叠写
//! `ActionSetValues.current.analog[0]` 再经 `calculate_input_delta` 拷贝而来），
//! 累加进 f32 屏幕空间游标 `CursorPositionSS`，非负 clamp 写回，并产出
//! `CursorMovedScreenSpace` 窗口空间事件。
//!
//! `CursorPosition`（u32）仍是 `input_record_replay`（0x1409423d0 param_6）回放写回的
//! 像素游标资源，保留；本系统按 PDB 签名只写 `CursorPositionSS`。
//!
//! 语义标记：非负 clamp 与捕获（`CursorCapturedPosition`）交互为 I（章节"非负 clamp 后
//! 写回"）；`EventWriter<CursorMovedScreenSpace>` 载荷形状 I（待取证）。
use bevy_ecs::prelude::*;
use glam::Vec2;

use crate::{
    input::action_set::ActionSetStates,
    resources::{
        mouse_capture::CursorCapturedPosition, mouse_terrain_raycast::CursorPositionSS,
        window::CursorMovedScreenSpace,
    },
};

use super::translate_input_to_cursor_moved::InputCursorMoved;

/// 像素游标位置（O：0x1409423d0 case 2 写 `CONCAT44(y, x)`；input_record_replay 消费）。
/// 注意：`update_cursor_position` 的 PDB 签名写 f32 `CursorPositionSS`，不写本 u32 资源。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CursorPosition {
    pub x: u32,
    pub y: u32,
}

impl CursorPosition {
    /// 两 u32 游标（输入顺序 x, y）。
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    /// `input_record_replay` 录制的游标 f32 对（O：0x1409423d0 default 分支）。
    pub fn as_vec2(self) -> Vec2 {
        Vec2::new(self.x as f32, self.y as f32)
    }
}

/// 非负 clamp 累加（I：章节"非负 clamp 后写回"；Absolute 直接替换。
/// RelativeWhileCaptured 与同帧 translate_input_to_cursor_moved 捕获路径一致累加——
/// reviewer 二轮复核）。供旧 I 级近似与测试保留。
pub fn fold_cursor_moved(start: Vec2, events: impl IntoIterator<Item = InputCursorMoved>) -> Vec2 {
    let mut position = start;
    for event in events {
        match event {
            InputCursorMoved::Relative(value) | InputCursorMoved::RelativeWhileCaptured(value) => {
                position += value;
            }
            InputCursorMoved::Absolute(value) => {
                position = value;
            }
        }
        position.x = if position.x.is_nan() || position.x < 0.0 { 0.0 } else { position.x };
        position.y = if position.y.is_nan() || position.y < 0.0 { 0.0 } else { position.y };
    }
    position
}

/// 每帧折叠：`CursorPositionSS + analog[0]`（input_space_cursor_move），非负 clamp（I）。
pub fn fold_cursor_state(start: Vec2, input_space_delta: Vec2) -> Vec2 {
    let next = start + input_space_delta;
    Vec2::new(
        if next.x.is_nan() || next.x < 0.0 { 0.0 } else { next.x },
        if next.y.is_nan() || next.y < 0.0 { 0.0 } else { next.y },
    )
}

/// `update_cursor_position`（O 签名 @0x1413ee760；正文全内联 I）：
/// 读 `ActionSetStates`（含 `translate_input_to_cursor_moved` 折叠的 input-space 增量）
/// 与 `CursorCapturedPosition`，累加进 `CursorPositionSS`，发 `CursorMovedScreenSpace`。
/// 与 `translate_input_to_cursor_moved` 同帧不同 stage 消费同一增量流（顺序依赖）。
pub fn update_cursor_position(
    states: Res<ActionSetStates>,
    captured: Res<CursorCapturedPosition>,
    mut cursor: ResMut<CursorPositionSS>,
    mut events: EventWriter<CursorMovedScreenSpace>,
) {
    let _ = captured.0; // 捕获锚点由 mouse_capture 保有；本系统只读其判别（语义 I）。
    let next = fold_cursor_state(cursor.0, states.analog[0]);
    cursor.0 = next;
    events.write(CursorMovedScreenSpace(next));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn relative_accumulates_and_absolute_replaces() {
        let merged = fold_cursor_moved(
            Vec2::new(3.0, 4.0),
            [
                InputCursorMoved::Relative(Vec2::new(1.0, -2.0)),
                InputCursorMoved::Absolute(Vec2::new(50.0, 60.0)),
                InputCursorMoved::RelativeWhileCaptured(Vec2::new(5.0, 5.0)),
            ],
        );
        // RelativeWhileCaptured 与捕获路径一致累加（reviewer 二轮复核）。
        assert_eq!(merged, Vec2::new(55.0, 65.0));
    }

    #[test]
    fn negatives_and_nan_clamp_to_zero() {
        let merged = fold_cursor_moved(
            Vec2::ZERO,
            [InputCursorMoved::Relative(Vec2::new(-5.0, f32::NAN))],
        );
        assert_eq!(merged, Vec2::ZERO);
    }

    #[test]
    fn cursor_state_folds_the_input_space_delta_and_clamps() {
        assert_eq!(fold_cursor_state(Vec2::new(8.0, 5.0), Vec2::new(1.0, -2.0)), Vec2::new(9.0, 3.0));
        assert_eq!(fold_cursor_state(Vec2::new(8.0, 5.0), Vec2::new(-20.0, f32::NAN)), Vec2::ZERO);
    }
}
