//! B 0x140b17a80. Real EventReader order, per-event clamping and separate
//! input-space/window-space deltas.
//!
//! PDB 签名（O，`translate_input_to_cursor_moved.rs.structures.json.gz`）：
//! `(Res<CursorPositionSS>, Res<WindowSize>, Res<CursorCapturedPosition>,
//! ResMut<ActionSetValues>, EventReader<InputCursorMoved>, EventWriter<InputCursorDelta>)`
//! —— 方向为：消费 `InputCursorMoved`（stride 0x18，tag@+8：0=相对/1=绝对/2=跳过），
//! 产出**一个** `InputCursorDelta{pub Vec2}`（stride 0x10 {id, Vec2}）。
//!
//! 折叠语义（O，fresh 0x140b17a80，reviewer 二轮复核）：
//! - 非捕获（`CursorCapturedPosition` 为 None，`iVar2==0`）：tag 0 相对事件按
//!   `acc = clamp(acc + delta, 0, WindowSize)` 累加；tag 1 绝对事件 `sum += value - acc`
//!   并替换 acc；tag 2 跳过；
//! - 捕获（Some，`iVar2!=0`）：tag 0 与 tag 2（RelativeWhileCaptured）都累加原始 delta，
//!   仅 tag 1（Absolute）跳过（O：`cmp dword [r15+8],1; jne add`）；
//! - 尾部：发一个 `InputCursorDelta{sum}`；`ActionSetValues.current.analog[0] += sum`、
//!   `analog[1] += (末 acc - 起始光标)`。
//! 相对/绝对事件的 u32→f32 用 `_UNK_1429e95b0`（`0x4330000000000000`×2 = 2^52 魔数）精确
//! 转换，数值上与 `as f32` 一致。
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
    let lower = if value.is_nan() || value < 0. { 0. } else { value };
    if lower > size { size } else { lower }
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
                    position = Vec2::new(
                        clamp_window_component(next.x, window.x),
                        clamp_window_component(next.y, window.y),
                    );
                    delta += value;
                }
                InputCursorMoved::Absolute(value) => {
                    delta += value - position;
                    position = value;
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
        assert_eq!(clamp_window_component(-0.0, 0.0).to_bits(), (-0.0f32).to_bits());
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
