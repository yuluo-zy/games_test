//! 将一帧内的相对/绝对光标事件合成为动作增量和窗口空间位移。
use crate::{
    input::action_set::ActionSetValues,
    resources::{
        mouse_capture::CursorCapturedPosition, mouse_terrain_raycast::CursorPositionSS,
        window::WindowSize,
    },
};
use bevy_ecs::prelude::*;
use glam::Vec2;

/// Semantic names for the three observed native tags 0/1/2. Event IDs and
/// double buffering belong to Bevy, not to this payload.
#[derive(Event, Clone, Copy, Debug, PartialEq)]
pub enum InputCursorMoved {
    Relative(Vec2),
    Absolute(Vec2),
    RelativeWhileCaptured(Vec2),
}
#[derive(Event, Clone, Copy, Debug, PartialEq)]
pub struct InputCursorDelta(pub Vec2);

fn clamp_window_component(value: f32, size: f32) -> f32 {
    let lower = if value.is_nan() || value < 0. {
        0.
    } else {
        value
    };
    if lower > size { size } else { lower }
}

/// 原始鼠标增量可以越界，窗口位置始终限制在有效像素范围内。
pub(super) fn clamp_window_position(position: Vec2, window: Vec2) -> Vec2 {
    Vec2::new(
        clamp_window_component(position.x, window.x),
        clamp_window_component(position.y, window.y),
    )
}

pub fn fold_cursor_events(
    start: Vec2,
    window: Vec2,
    captured: bool,
    events: impl IntoIterator<Item = InputCursorMoved>,
) -> (Vec2, Vec2) {
    let mut position = start;
    let mut delta = Vec2::ZERO;
    for event in events {
        if captured {
            // 捕获路径：tag 0（Relative）与 tag 2（RelativeWhileCaptured）都累加，
            // 仅 tag 1（Absolute）跳过（O：0x140b17a80 `cmp dword [r15+8],1; jne add`，
            // reviewer 二轮复核）。
            match event {
                InputCursorMoved::Relative(value)
                | InputCursorMoved::RelativeWhileCaptured(value) => delta += value,
                InputCursorMoved::Absolute(_) => {}
            }
        } else {
            match event {
                InputCursorMoved::Relative(value) => {
                    let next = position + value;
                    // Original MAXPS + ordered mask converts NaN to zero,
                    // then MINPS limits by integer WindowSize converted to f32.
                    position = clamp_window_position(next, window);
                    delta += value;
                }
                InputCursorMoved::Absolute(value) => {
                    let next = clamp_window_position(value, window);
                    delta += next - position;
                    position = next;
                }
                InputCursorMoved::RelativeWhileCaptured(_) => {}
            }
        }
    }
    (delta, position - start)
}

pub fn translate_input_to_cursor_moved(
    cursor: Res<CursorPositionSS>,
    window: Res<WindowSize>,
    captured: Res<CursorCapturedPosition>,
    mut values: ResMut<ActionSetValues>,
    mut moved: EventReader<InputCursorMoved>,
    mut deltas: EventWriter<InputCursorDelta>,
) {
    let (input, hardware) = fold_cursor_events(
        cursor.0,
        window.as_vec2(),
        captured.0.is_some(),
        moved.read().copied(),
    );
    // Native emits once even for an empty reader, before mutating ActionSetValues.
    deltas.write(InputCursorDelta(input));
    values.current.analog[0] += input;
    values.current.analog[1] += hardware;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn window_clamp_retains_native_signed_zero_and_nan_policy() {
        assert_eq!(
            clamp_window_component(-0.0, 0.0).to_bits(),
            (-0.0f32).to_bits()
        );
        assert_eq!(clamp_window_component(f32::NAN, 10.0).to_bits(), 0);
        assert_eq!(clamp_window_component(f32::INFINITY, 10.0), 10.0);
    }
    #[test]
    fn clamped_relative_then_absolute_uses_the_intermediate_cursor() {
        let (input, hardware) = fold_cursor_events(
            Vec2::new(8., 5.),
            Vec2::splat(10.),
            false,
            [
                InputCursorMoved::Relative(Vec2::new(5., -8.)),
                InputCursorMoved::RelativeWhileCaptured(Vec2::splat(100.)),
                InputCursorMoved::Absolute(Vec2::new(3., 4.)),
            ],
        );
        assert_eq!(input, Vec2::new(-2., -4.));
        assert_eq!(hardware, Vec2::new(-5., -1.));
    }
    #[test]
    fn capture_accumulates_relative_and_while_captured_events() {
        // O：捕获路径 tag 0（Relative）与 tag 2（RelativeWhileCaptured）都累加，
        // 仅 tag 1（Absolute）跳过（reviewer 二轮复核）。
        let result = fold_cursor_events(
            Vec2::splat(5.),
            Vec2::splat(10.),
            true,
            [
                InputCursorMoved::Relative(Vec2::splat(2.)),
                InputCursorMoved::Absolute(Vec2::splat(100.)),
                InputCursorMoved::RelativeWhileCaptured(Vec2::splat(3.)),
            ],
        );
        assert_eq!(result, (Vec2::splat(5.), Vec2::ZERO));
    }
}
