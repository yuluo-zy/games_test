//! 将设备布尔值推进为按下、持续、连发和释放状态。
use crate::input::action_set::{
    ActionSetStates, ActionSetValues, DigitalActionPhase as Phase, DigitalActionState,
};
use bevy_ecs::prelude::{Event, Res, ResMut};

const REPEAT_DELAY_SECONDS: f32 = 0.2;
const REPEAT_INTERVAL_SECONDS: f32 = 0.1;

/// UI 历史按钮事件：零为撤销，其余为重做。
#[derive(Event, Clone, Copy, Debug, PartialEq, Eq)]
pub struct UiHistoryButtonPressed(pub u8);
impl UiHistoryButtonPressed {
    pub fn is_redo(&self) -> bool {
        self.0 != 0
    }
}

pub fn digital_delta(
    previous: bool,
    current: bool,
    state: DigitalActionState,
    delta_seconds: f32,
    auto_repeat: bool,
) -> DigitalActionState {
    let delta_seconds = if delta_seconds.is_finite() {
        delta_seconds.max(0.0)
    } else {
        0.0
    };
    let timer_seconds = if state.timer_seconds.is_finite() {
        state.timer_seconds
    } else {
        0.0
    };
    let (phase, timer_seconds) = match (previous, current) {
        (false, false) => (Phase::Idle, 0.0),
        (false, true) => (Phase::JustPressed, 0.0),
        (true, false) => (Phase::JustReleased, timer_seconds),
        (true, true) if state.phase == Phase::Repeating => {
            // 一帧最多触发一次，慢帧不产生无限追赶循环。
            let countdown = if timer_seconds <= 0.0 {
                REPEAT_INTERVAL_SECONDS
            } else {
                timer_seconds
            };
            (Phase::Repeating, countdown - delta_seconds)
        }
        (true, true) => {
            let held_seconds = (timer_seconds.max(0.0) + delta_seconds).min(f32::MAX);
            if auto_repeat && held_seconds >= REPEAT_DELAY_SECONDS {
                (Phase::Repeating, 0.0)
            } else {
                (Phase::Held, held_seconds)
            }
        }
    };
    DigitalActionState {
        phase,
        timer_seconds,
    }
}

pub fn uses_ui_auto_repeat(slot: usize) -> bool {
    (20..=25).contains(&slot)
}

pub fn update_action_states(
    values: &ActionSetValues,
    states: &mut ActionSetStates,
    delta_seconds: f32,
) {
    states.analog = values.current.analog.map(|value| {
        if value.is_finite() {
            value
        } else {
            glam::Vec2::ZERO
        }
    });
    for (slot, state) in states.digital.iter_mut().enumerate() {
        *state = digital_delta(
            values.previous.digital[slot],
            values.current.digital[slot],
            *state,
            delta_seconds,
            uses_ui_auto_repeat(slot),
        );
    }
}

/// 输入长按使用真实时间，游戏暂停不会阻止菜单导航。
pub fn calculate_input_delta(
    values: Res<ActionSetValues>,
    time: Res<bevy_time::Time>,
    mut states: ResMut<ActionSetStates>,
) {
    update_action_states(&values, &mut states, time.delta_secs());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn press_hold_repeat_release_and_invalid_time() {
        let pressed = digital_delta(false, true, DigitalActionState::default(), 0.1, true);
        assert!(pressed.just_pressed());
        let held = digital_delta(true, true, pressed, 0.1, true);
        assert_eq!(held.phase, Phase::Held);
        let repeated = digital_delta(true, true, held, 0.1, true);
        assert!(repeated.just_pressed());
        let waiting = digital_delta(true, true, repeated, 0.02, true);
        assert!(!waiting.just_pressed());
        let released = digital_delta(true, false, waiting, 0.1, true);
        assert!(released.just_released());
        assert_eq!(
            digital_delta(false, false, released, f32::NAN, true),
            DigitalActionState::default()
        );
        assert!(
            digital_delta(true, true, held, f32::INFINITY, false)
                .timer_seconds
                .is_finite()
        );
    }
    #[test]
    fn ordinary_buttons_never_auto_repeat() {
        let mut state = digital_delta(false, true, DigitalActionState::default(), 1.0, false);
        for _ in 0..20 {
            state = digital_delta(true, true, state, 1.0, false);
            assert!(!state.just_pressed());
        }
        assert_eq!(state.phase, Phase::Held);
    }
}
