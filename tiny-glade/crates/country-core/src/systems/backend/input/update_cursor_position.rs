//! `update_cursor_position.rs` — 游标位置合并（B 全内联无独立符号，行 10，I/O 混合）。
//!
//! 证据：`chapters/systems/backend/input/update_cursor_position.rs.md`：`update_cursor_position`
//! 消费 `Events<InputCursorMoved>`（每元素 0x10 B `{id, Vec2 delta}`）→ 累加进游标位置资源
//! `CursorPosition {x u32, y u32}`（像素坐标，由 `translate_input_to_cursor_moved` 的 f32→u32
//! clamp 提供）；非负 clamp 后写回。与 `translate_input_to_cursor_moved` 同帧不同 stage 消费
//! 同一事件流（顺序依赖）。
//!
//! `CursorPosition` 同时是 `input_record_replay`（0x1409423d0）`param_6`：回放时帧 @+0x40 的
//! 双 f32 经 NaN 安全 clamp 写入该资源（O，见 `input_record_replay` 模块文档）。
use bevy_ecs::prelude::*;
use glam::Vec2;

use super::translate_input_to_cursor_moved::InputCursorMoved;

/// 像素游标位置（O：0x1409423d0 case 2 写 `CONCAT44(y, x)`；update_cursor_position 累加）。
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

/// 非负 clamp 累加（I：章节“非负 clamp 后写回”；Absolute 直接替换。RelativeWhileCaptured
/// 与同帧 translate_input_to_cursor_moved 捕获路径一致累加——reviewer 二轮复核）。
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

/// `update_cursor_position`（行 10，I/O）：消费 `InputCursorMoved`，合并进 `CursorPosition`。
///
/// 与 `translate_input_to_cursor_moved` 同帧不同 stage 消费同一事件流（顺序依赖，I）。
pub fn update_cursor_position(
    mut cursor: ResMut<CursorPosition>,
    mut moved: EventReader<InputCursorMoved>,
) {
    let start = Vec2::new(cursor.x as f32, cursor.y as f32);
    let merged = fold_cursor_moved(start, moved.read().copied());
    cursor.x = merged.x as u32;
    cursor.y = merged.y as u32;
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
}
